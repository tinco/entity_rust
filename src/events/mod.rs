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
use std::any::Any;
use shared_mutex::{ SharedMutex };

pub mod example;

lazy_static! {
	pub static ref EventQueues: SharedMutex<HashMap<String, Box<Any+'static+Sync>>> = SharedMutex::new(HashMap::new());
}

/// The event loop should trigger every n ms and execute any
/// events that are queued up. Every event has a function
/// that is called by the trigger! macro that puts the event
/// on the queue.
pub fn run_loop() {

}

pub fn event_queue_push<T>(event_name: &String, event: T) where T: Any+'static+Sync {
	let mut map = EventQueues.write().unwrap();
	if map.contains_key(event_name) {
		let any_value = *map.get_mut(event_name).unwrap();
		let any_value_casted = any_value as Box<Any +'static>;
		let queue : &mut Vec<T> = any_value_casted.downcast_mut::<Vec<T>>().unwrap();
		queue.push(event);
	} else {
		let queue = Box::new(vec![event]);
		map.insert(event_name.clone(), queue);
	}
}
/*
pub fn get_events<T>(event_name: &String) -> Box<Vec<T>> where T: Any+Sync {
	let map = EventQueues.read().unwrap();
	let any_value = **map.get(event_name).unwrap();
	let queue : Vec<T> = any_value.downcast_ref::<Vec<T>>().unwrap();
	return *(map.get(event_name).unwrap());
}*/

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

