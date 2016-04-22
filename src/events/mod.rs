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
/// 2. We need to register handlers.
/// 4. We need a mechanism for actually receiving the triggers and
///    handling the special 'tick' behaviour.
///
/// Event handlers: 
/// 
///  We call them with our arguments, but they need references to
///  the components they want too. Mega awesome would be if at some
///  point we can dispatch handlers in parallel depending on whether
///  they mutate the components.
///
///  For this we need to know what components the handler uses and
///  whether they mutate them or not.

use std::collections::HashSet;
use std::any::Any;
use shared_mutex::{ SharedMutex };

pub mod example;

static_any_vec_map! { this_tick_queues, String }
static_any_vec_map! { next_tick_queues, String }

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

pub fn run_loop() {

}

/// Defines an event.
macro_rules! event {
	() => ()
}

/// Queues an event to be dispatched.
/// This means that the argument is put into the trigger queue for the
/// event and the event handlers will be invoked either at the next run loop.
/// or immediately.
macro_rules! trigger {
	() => ()
}
