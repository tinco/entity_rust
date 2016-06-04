/// The event system will have a trigger function that takes an
/// argument that decides if the event will be triggered this tick
/// or the next tick. The default should be next tick to prevent
/// inadvertent infinite loop bugs.
///
/// The event system can then schedule all events handlers that
/// do not mutate components to be run in parallel.
/// All handlers that do mutate the component list can be run
/// sequentially after that. Perhaps we can even schedule
/// those with non-overlapping mutable component lists to
/// run in parallel.
///
/// The event loop should trigger every n ms and execute any
/// events that are queued up. Every event has a function
/// that is called by the trigger! macro that puts the event
/// on the queue.
///
use std::collections::{ HashSet, HashMap };
use std::any::{ Any, TypeId };
use shared_mutex::{ SharedMutex, MappedSharedMutexReadGuard, MappedSharedMutexWriteGuard };

use components;

pub mod example;

pub trait Handler {
	fn run(&self, Vec<MappedSharedMutexReadGuard<Any>>, Vec<MappedSharedMutexWriteGuard<Any>>);
	fn component_types(&self) -> Vec<TypeId>;
	fn mut_component_types(&self) -> Vec<TypeId>;
}

#[derive(Clone)]
pub struct Event {
	pub name: String,
	pub get_handler_instances: fn () -> Vec<Box<Handler>>
}

// The new events sets contain the events that have data in their queues so
// are ready to be ran.
lazy_static! {
	pub static ref THIS_TICK_NEW_EVENTS: SharedMutex<HashSet<String>> = SharedMutex::new(HashSet::new());
	pub static ref NEXT_TICK_NEW_EVENTS: SharedMutex<HashSet<String>> = SharedMutex::new(HashSet::new());
	pub static ref REGISTERED_EVENTS: SharedMutex<HashMap<String, Event>> = SharedMutex::new(HashMap::new());
}

pub fn trigger_this_tick(event_name: &String) {
	let mut new_events_set = THIS_TICK_NEW_EVENTS.write().expect("THIS_TICK_NEW_EVENTS mutex was corrupted.");
	new_events_set.insert(event_name.clone());
}

pub fn trigger_next_tick(event_name: &String) {
	let mut new_events_set = NEXT_TICK_NEW_EVENTS.write().expect("NEXT_TICK_NEW_EVENTS mutex was corrupted.");
	new_events_set.insert(event_name.clone());
}

pub fn register_event(event : Event) {
	let mut events = REGISTERED_EVENTS.write().expect("REGISTERED_EVENTS mutex was corrupted.");
	events.insert(event.name.clone(), event);
}

// Runs a single iteration of the event system. Can be run in a loop to process events
// continuously, but should be interleaved with `next_tick` to progress properly.
pub fn run_events() {
	let events : Vec<Event>;
	{
		let event_names : Vec<String>;
		{
			let mut event_names_lock = THIS_TICK_NEW_EVENTS.write().expect("THIS_TICK_NEW_EVENTS mutex was corrupted");
			event_names = event_names_lock.drain().collect();
		}

		let events_lock = REGISTERED_EVENTS.read().expect("REGISTERED_EVENTS mutex was corrupted");
		events = event_names.iter().map(|n|
			events_lock.get(&*n).cloned().expect("Unknown event triggered")
		).collect();
	}

	let handlers = events.iter().flat_map(|e| (e.get_handler_instances)() );

	// TODO sort and parellize handler invocations
	for handler in handlers {
		let component_types = handler.component_types();
		let mut_component_types = handler.mut_component_types();

		let mut locks : Vec<MappedSharedMutexReadGuard<Any>>= vec![];
		let mut mut_locks : Vec<MappedSharedMutexWriteGuard<Any>>= vec![];

		// we obtain the correct component locks
		for typ in component_types {
			locks.push(components::get_components_read_lock(typ));
		}
		for typ in mut_component_types {
			mut_locks.push(components::get_components_write_lock(typ));
		}
		
		// we run the handlers
		handler.run(locks, mut_locks)
	}
}

// Progresses the system to the next tick.
// Interleave `next_tick` between `run_events` invocations to make sure events that are
// scheduled to run only once per tick are triggered.
// A good scheme could be to run `next_tick` every 16ms, while `run_events` is ran continuously.
pub fn next_tick() {
	// get NEXT_TICK_NEW_EVENTS
	// get THIS_TICK_NEW_EVENTS
	// append NEXT_TICK_NEW_EVENTS to THIS_TICK_NEW_EVENTS
	// same for queues
}

#[macro_export]
macro_rules! event {
	( $name:ident, $( $field_name:ident : $field_typ:ty ),* ) => (
		pub mod $name {
			use shared_mutex::{ SharedMutex, MappedSharedMutexWriteGuard, MappedSharedMutexReadGuard };
			use std::any::{ Any, TypeId };
			use entity_rust::events;
			use uuid::Uuid;

			/// Example component
			#[derive(PartialEq,Eq,Clone)]
			pub struct Data {
				$(pub $field_name : $field_typ),*
			}

			pub type HandlerFn = fn(&Vec<Data>, Vec<MappedSharedMutexReadGuard<Any>>, Vec<MappedSharedMutexWriteGuard<Any>>);

			pub struct Handler {
				handler_fn: HandlerFn,
				component_types: Vec<TypeId>,
				mut_component_types: Vec<TypeId>
			}

			pub struct HandlerInstance {
				handler_fn: HandlerFn,
				component_types: Vec<TypeId>,
				mut_component_types: Vec<TypeId>,
				data: Vec<Data>
			}

			impl HandlerInstance {
				pub fn new(h: &Handler, d: Vec<Data>) -> HandlerInstance {
					HandlerInstance {
						handler_fn: h.handler_fn,
						// TODO are these clones really necessary? would be cool if they could be static refs
						component_types: h.component_types.clone(),
						mut_component_types: h.mut_component_types.clone(),
						data: d
					}
				}
			}

			impl events::Handler for HandlerInstance {
				fn run(&self, components: Vec<MappedSharedMutexReadGuard<Any>>, mut_components: Vec<MappedSharedMutexWriteGuard<Any>>) {
					let handler_fn = self.handler_fn;
					let data = &self.data;
					handler_fn(data, components, mut_components) 
				}

				fn component_types(&self) -> Vec<TypeId> { self.component_types.clone() }
				fn mut_component_types(&self) -> Vec<TypeId> { self.mut_component_types.clone() }
			}

			lazy_static! {
				/// EVENT_UUID is used internally to index events, is randomly
				/// generated at first access.
				pub static ref EVENT_UUID: String = Uuid::new_v4().simple().to_string();
				pub static ref HANDLERS: SharedMutex<Vec<Handler>> = SharedMutex::new(vec![]);
				pub static ref THIS_TICK_DATA: SharedMutex<Vec<Data>> = SharedMutex::new(vec![]);
				pub static ref NEXT_TICK_DATA: SharedMutex<Vec<Data>> = SharedMutex::new(vec![]);
			}

			/// Listeners are a list of functions that should be called by trigger
			pub fn trigger(argument: Data) {
				let mut data = THIS_TICK_DATA.write().expect("THIS_TICK_DATA mutex corrupted");
				data.push(argument);

				events::trigger_this_tick(&*EVENT_UUID);
			}

			pub fn register_handler(handler_fn: HandlerFn, component_types: Vec<TypeId>, mut_component_types: Vec<TypeId>) {
				let mut handlers = HANDLERS.write().expect("Events HANDLERS mutex corrupted");
				let handler = Handler {
					handler_fn : handler_fn,
					// TODO are these clones really necessary? would be cool if they could be static refs
					component_types : component_types.clone(),
					mut_component_types : mut_component_types.clone()
				};
				handlers.push(handler);

				let event = events::Event {
					name: (&*EVENT_UUID).clone(),
					get_handler_instances: get_handler_instances
				};

				events::register_event(event)
			}

			pub fn get_handler_instances() -> Vec<Box<events::Handler>> {
				let mut data_old = THIS_TICK_DATA.write().expect("TICK DATA mutex corrupted");
				let data : Vec<Data> = data_old.drain(..).collect();

				let handlers_lock = HANDLERS.read().expect("HANDLERS mutex corrupted");
				handlers_lock.iter().map(|h|
					//TODO eliminate this data.clone
					Box::new(HandlerInstance::new(h, data.clone())) as Box<events::Handler>
				).collect()
			}
		}
	)
}

/// Queues an event to be dispatched.
/// This means that the argument is put into the trigger queue for the
/// event and the event handlers will be invoked either at the next run loop.
/// or immediately.
macro_rules! trigger {
	() => ()
}
