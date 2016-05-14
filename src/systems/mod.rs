
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
			use std::collections::HashMap;
			use std::any::Any;
			use shared_mutex::{ SharedMutex, MappedSharedMutexReadGuard };

			system_contents!{ ( $($contents)* ) [ ] }
		}
	} 
}

#[macro_export]
macro_rules! system_contents {
	// Consume on! invocations
	(
		(
			on! (
				$event_name:ident, $($event_declaration:tt)*
			) $_self:ident, $_data:ident => $event_body:block $($rest:tt)*
		) [ 
			$( $saved_decl:tt ),*
		] 
	) => (
		on! { ($event_name, $( $event_declaration)* ) $_self , $_data => $event_body }

		system_contents!{ 
			( $($rest)* )
			[ ( $event_name, $( $event_declaration)* ) $(, $saved_decl)* ]
		}
	);

	(
		(
			state! { $($state_declaration:tt)* } $($rest:tt)*
		) [
			$( $saved_decl:tt ),*
		] 
	) => (
		state! { $($state_declaration)* }

		system_contents!{
			( $($rest)* )
			[ $( $saved_decl ),* ]
		}
	);

	// When all content has been consumed emit register macro
	(
		() [ $( $event_declaration:tt ),* ]
	) => (
		system_register!{ $( $event_declaration ),* }
	)
}

#[macro_export]
macro_rules! on {
	( ($event_name:ident, { $( $mut_name:ident : $mut_typ:ty )* } , { $($name:ident : $typ:ty)* } ) 
		$_self:ident, $_data:ident => $event_body:block ) => (

		use super::$event_name;

		impl State {
			pub fn $event_name(&mut $_self,
				$_data: Vec<$event_name::Data>,
				$( $name : &Vec<$typ>),*
				$( $mut_name : &Vec<$mut_typ> ),* ) $event_body

		}

		pub fn $event_name(data: Vec<$event_name::Data>, components: Vec<&Any>, mut mut_components: Vec<&mut Any>) {
			let mut components_iter = components.iter();
			let mut mut_components_iter = mut_components.iter_mut();

			$(
				let $name : &Vec<$typ> = components_iter
					.next().expect("Event components list too short.")
					.downcast_ref().expect("Event component not of expected type.");
			)*

			$(
				let $mut_name: &mut Vec<$mut_typ> = mut_components_iter
					.next().expect("Event mut_components list too short.")
					.downcast_mut().expect("Event component not of expected type.");
			)*

			state.write().expect("Event state corrupted").$event_name(
				data,
				$($name),*
				$($mut_name),*
			);
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

		lazy_static! {
			pub static ref state: SharedMutex<State> = SharedMutex::new(State::default());
		}
	)
}

#[macro_export]
macro_rules! system_register {
	( $( (
			$event_name:ident,
			{ $( $mut_name:ident : $mut_typ:ty )* } ,
			{ $( $name:ident : $typ:ty )* }
		)
	),* ) => (
		pub fn register() {
			use std::any::TypeId;
			$(
				use $event_name;
				let mut_ts = vec![ $( TypeId::of::< $mut_typ >() ),* ];
				let ts = vec![ $( TypeId::of::< $typ >() ),* ];
				$event_name::register_handler($event_name, ts, mut_ts);
			)*
		}
	)
}
