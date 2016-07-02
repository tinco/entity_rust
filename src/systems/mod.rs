	
/// 
/// Systems are a set of event handlers and optionally some associated state.
///
/// The event handlers are invoked with a list of event data and have access to component
/// lists. This access can be immutable or mutable which is made explicit so that
/// the event handler scheduler might parallelize handler invocations.
///
/// To properly enforce this I guess the scheduler would acquire the locks on the component
/// lists.
///
/// So the context of the handler function would consist of:
///
///   The handler state
///   The event data list
///   Immutable components lists
///   Mutable components lists
/// 
/// For example the following invocation:
///
/// system! my_system {
///   state! { i: i64 }
///   on!(event_name, { positions: Position}, { descriptions: Description }) {
///     state.i += 1;
///     let position = positions[0];
///     descriptions[0].set(format!("Position = {},{}", position.x, position.y));
///   } 
/// }
///
/// Expands to:
///
/// pub mod my_system {
///   struct State { i: i64 }
///   
///   pub fn event_name(state: &mut State, data: &Vec<event_name::Data>, positions: &Vec<Position>, descriptions: &mut Vec<Description> ) {
///     let position = positions[0];
///     descriptions[0].set(format!("Position = {},{}", position.x, position.y));
///   }
///
///   pub fn register() {
///      events::register_handler(event_name); // etc..
///   }
/// }
///
/// How are we going to register this handler at the event loop?
///
/// Ideally we would have just one 'register' call per system. But to have that the register call should enable
/// all nested handlers. We could also just have one handler per system. That would make things easier..
/// Maybe there's some scheme using recursive macros that we could use to generate a register_system that collects
/// all on! calls. Ok this is definitely a possibility. We could simply match the on! macro in the system! macro
/// and use that to populate the register function.
///
/// How are we going to store/invoke these handlers.
/// 
/// To invoke the handler it will require the state, the components and the data list passed in the right order.
/// we could have a separate invoker function that invokes event_name with the arguments passed in. Its arguments
/// would be the component lists in two vecs, as well as the event data list.
///
/// This could work!
///

#[macro_export]
macro_rules! system {
	( $system_name:ident { $($contents:tt)* } ) => {
		pub mod $system_name {
			use std::any::Any;
			use shared_mutex::{ SharedMutex };
			#[allow(unused_imports)]
			use $crate::entities::{ ComponentList };
			use shared_mutex::{ MappedSharedMutexReadGuard, MappedSharedMutexWriteGuard };
			use std::sync::Arc;

			system_contents!{ ( $($contents)* ) [ ] [ ] }
		}
	} 
}

#[macro_export]
macro_rules! append_path_component {
	($($path_component:ident)::+, $extension:ident) => ( $($path_component)::*::$extension )
}

#[macro_export]
macro_rules! system_contents {
	// Consume on! invocations
	(
		(
			on $event_name:ident, {$($event_declaration_mut:tt)*}, {$($event_declaration:tt)*},
			($_self:ident, $_data:ident) => $event_body:block $($rest:tt)*
		) [
			$( $saved_decl:tt ),*
		] [
			$( $saved_sync_decl:tt ),*
		]
	) => (
		on! { ($event_name, { $( $event_declaration_mut )*  }, { $( $event_declaration)* }  ) $_self , $_data => $event_body }

		system_contents!{ 
			( $($rest)* )
			[ ( $event_name, { $( $event_declaration_mut)* }, {$( $event_declaration)* } ) $(, $saved_decl)* ]
			[ $($saved_sync_decl),* ]
		}
	);

	// Consume on_sync! invocations
	(
		(
			on_sync	$event_name:ident, ($_self:ident, $_data:ident) => $event_body:block $($rest:tt)*
		) [ 
			$( $saved_decl:tt ),*
		] [ 
			$( $saved_sync_decl:tt ),*
		] 
	) => (
		on_sync! { ($event_name, $_self, $_data) $event_body }

		system_contents!{ 
			( $($rest)* )
			[ $($saved_decl),* ]
			[ $event_name $(, $saved_sync_decl)* ]
		}
	);


	(
		(
			state! { $($state_declaration:tt)* } $($rest:tt)*
		) [
			$( $saved_decl:tt ),*
		] [ 
			$( $saved_sync_decl:tt ),*
		]
	) => (
		state! { $($state_declaration)* }

		system_contents!{
			( $($rest)* )
			[ $( $saved_decl ),* ]
			[ $(, $saved_sync_decl)* ]
		}
	);

	(
		(
			$token_tree:item $($rest:tt)*
		) [
			$( $saved_decl:tt ),*
		] [ 
			$( $saved_sync_decl:tt ),*
		]
	) => (
		$token_tree

		system_contents!{
			( $($rest)* )
			[ $( $saved_decl ),* ]
			[ $(, $saved_sync_decl)* ]
		}

	);

	// When all content has been consumed emit register macro
	(
		() [ $( $event_declaration:tt ),* ] [ $( $sync_event_declaration:tt ),* ]
	) => (
		system_register!{ ($( $event_declaration ),*) ($( $sync_event_declaration ),*) }
	)
}

#[macro_export]
macro_rules! on {
	( ($event_name:ident, { $( $mut_name:ident : $mut_typ:tt )* } , { $($name:ident : $typ:tt)* } ) 
		$_self:ident, $_data:ident => $event_body:block ) => (

		impl State {
			#[allow(unused_variables)]
			pub fn $event_name(&mut $_self,
				$_data: &Vec<$event_name::Data>,
				$( $name : &MappedSharedMutexReadGuard<ComponentList<append_path_component!($typ,Component)>>),*
				$( $mut_name : &MappedSharedMutexWriteGuard<ComponentList<append_path_component!($mut_typ,Component)>> ),* ) $event_body
		}

		#[allow(unused_variables)]
		#[allow(unused_mut)]
		pub fn $event_name(
				state: Arc<Any>,
				data: &Vec<$event_name::Data>,
				components: Vec<MappedSharedMutexReadGuard<Any>>,
				mut_components: Vec<MappedSharedMutexWriteGuard<Any>>
			) {
			let mut components_iter = components.into_iter();
			let mut mut_components_iter = mut_components.into_iter();

			$(
				let $name : MappedSharedMutexReadGuard<ComponentList<append_path_component!($typ,Component)>> = components_iter
					.next().expect("Event components list too short.")
					.map(|v| v.downcast_ref().expect("Event component not of expected type."));
			)*

			$(
				let $mut_name: MappedSharedMutexWriteGuard<ComponentList<append_path_component!($mut_typ,Component)>> = mut_components_iter
					.next().expect("Event mut_components list too short.")
					.map(|v| v.downcast_mut().expect("Event component not of expected type."));
			)*

			let state_casted : &SharedMutex<State> = state.downcast_ref().expect("State not of expected type.");

			state_casted.write().expect("Event state corrupted").$event_name(
				data,
				$(&$name),*
				$(&$mut_name),*
			);
		}
	)
}

#[macro_export]
macro_rules! on_sync {
	( ($event_name:ident, $_self:ident, $_data:ident) $event_body:block ) => (

		impl State {
			#[allow(unused_variables)]
			pub fn $event_name(&mut $_self,
				$_data: $event_name::Argument) $event_body

		}

		#[allow(unused_variables)]
		#[allow(unused_mut)]
		pub fn $event_name(
				state: Arc<Any+Send+Sync>,
				data: $event_name::Argument
			) {
			let state_ref : &Any = &*state;
			let state_casted : &SharedMutex<State> = state_ref.downcast_ref().expect("State not of expected type.");
			state_casted.write().expect("Event state corrupted").$event_name(data);
		}
	)
}

#[macro_export]
macro_rules! state {
	( $( $name:ident : $field:ty ),* ) => (
		#[derive(Default)]
		pub struct State {
			$(pub $name : $field),*
		}
	)
}

#[macro_export]
macro_rules! system_register {
	(

	(
		$( 
			(
				$event_name:ident,
				{ $( $mut_name:ident : $mut_typ:ident )* } ,
				{ $( $name:ident : $typ:ident )* }
			)
		),*
	)

	( 
		$(
			$sync_event_name:ident
		),*
	)

	) => (
		pub fn register() {
			#[allow(unused_imports)]
			use std::any::TypeId;

			let state = Arc::new(SharedMutex::new(State::default()));

			$(
				let mut_ts = vec![ $( TypeId::of::< $mut_typ::Component >() ),* ];
				let ts = vec![ $( TypeId::of::< $typ::Component >() ),* ];
				$event_name::register_handler(state.clone(), $event_name, ts, mut_ts);
			)*

			$(
				$sync_event_name::register_handler(state.clone(), $sync_event_name);
			)*
		}
	)
}
