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

use std::collections::HashMap;

lazy_static! {
	pub static ref EventQueues: Mutex<HashMap<str, &mut Vec<Any>>> = Mutex::new(HashMap::new());
}

/// The event loop should trigger every n ms and execute any
/// events that are queued up. Every event has a function
/// that is called by the trigger! macro that puts the event
/// on the queue.
pub fn run_loop() {

}

pub fn get_event_queue_mut<T>(event_name: &str) -> &Vec<T> {
	let map = EventQueues.lock().unwrap();
	return map.get_mut(event_name).downcast_mut::<T>()
}

pub fn set_event_queue<T>(event_name: &str, initial_value: T) {
	let map = EventQueues.lock().unwrap();
	map.insert(event_name, vec![T]);
}

///

/// Defines an event.
macro_rules! event {
	() => ()
}

/// Queues an event to be dispatched.
/// This means that the argument is put into the trigger queue for the
/// event and the event handlers will be invoked either at the next run loop.
macro_rules! trigger {
	//
	// Implementation idea: since we can't initialize events implicitly
	// in any way (not even through a rustc plugin atm) maybe we can
	// do it like lazy_static does it and initialize the event on the
	// first invocation of trigger.
	// 
	// Initialisation that needs to happen:
	//   The trigger queue for this particular event needs to be initialized
	//   The run loop needs to be handed a reference to the trigger queue
	//
	() => ()
}

