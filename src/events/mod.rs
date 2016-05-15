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
use std::collections::HashSet;
use std::any::{ Any, TypeId };
use shared_mutex::{ SharedMutex };

pub mod example;

// The tick queues hold the data associated with the events for each event
static_any_vec_map! { this_tick_queues, String }
static_any_vec_map! { next_tick_queues, String }

// The new events sets contain the events that have data in their queues so
// are ready to be ran.
lazy_static! {
	pub static ref THIS_TICK_NEW_EVENTS: SharedMutex<HashSet<String>> = SharedMutex::new(HashSet::new());
	pub static ref NEXT_TICK_NEW_EVENTS: SharedMutex<HashSet<String>> = SharedMutex::new(HashSet::new());
}

pub fn trigger_this_tick<T>(event_name: &String, data: T) where T: Any+'static+Sync {
	this_tick_queues::push(event_name, data);
	let mut new_events_set = THIS_TICK_NEW_EVENTS.write().expect("THIS_TICK_NEW_EVENTS mutex was corrupted.");
	new_events_set.insert(event_name.clone());
}

pub fn trigger_next_tick<T>(event_name: &String, data: T) where T: Any+'static+Sync {
	next_tick_queues::push(event_name, data);
	let mut new_events_set = NEXT_TICK_NEW_EVENTS.write().expect("NEXT_TICK_NEW_EVENTS mutex was corrupted.");
	new_events_set.insert(event_name.clone());
}

// Runs a single iteration of the event system. Can be run in a loop to process events
// continuously, but should be interleaved with `next_tick` to progress properly.
pub fn run_events() {
	// {
	//   Get (clone) the events for this tick
	//   Clear the events
	// }
	// Then we get the handlers for the ticks
	// for each handler
	// {
	//  	we obtain the correct component locks
	// 		we run the handlers
	// }
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
			use shared_mutex::{ SharedMutex };
			use std::any::{ Any, TypeId };
			use entity_rust::events;
			use uuid::Uuid;

			/// Example component
			#[derive(PartialEq,Eq,Clone)]
			pub struct Data {
				$(pub $field_name : $field_typ),*
			}

			pub type HandlerFn = fn(Vec<Data>, Vec<&Any>, Vec<&mut Any>);

			pub struct Handler {
				handler_fn: HandlerFn,
				component_types: Vec<TypeId>,
				mut_component_types: Vec<TypeId>
			}

			lazy_static! {
				/// EVENT_UUID is used internally to index events, is randomly
				/// generated at first access.
				pub static ref EVENT_UUID: String = Uuid::new_v4().simple().to_string();
				pub static ref HANDLERS: SharedMutex<Vec<Handler>> = SharedMutex::new(vec![]);
			}

			/// Listeners are a list of functions that should be called by trigger
			pub fn trigger(argument: Data) {
				events::trigger_this_tick(&*EVENT_UUID, argument);
			}

			pub fn register_handler(handler_fn: HandlerFn, component_types: Vec<TypeId>, mut_component_types: Vec<TypeId>) {
				let mut handlers = HANDLERS.write().expect("Events HANDLERS mutex corrupted");
				let handler = Handler {
					handler_fn : handler_fn,
					component_types : component_types.clone(),
					mut_component_types : mut_component_types.clone()
				};
				handlers.push(handler);
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
