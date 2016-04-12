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
use std::sync::{ Mutex };
use std::any::Any;
use std::mem;

lazy_static! {
	pub static ref EventQueues: Mutex<HashMap<&'static str, &'static mut usize>> = Mutex::new(HashMap::new());
}

/// The event loop should trigger every n ms and execute any
/// events that are queued up. Every event has a function
/// that is called by the trigger! macro that puts the event
/// on the queue.
pub fn run_loop() {

}

pub fn get_event_queue_mut<T>(event_name: &str) -> Option<&Vec<T>> {
	let map = &mut EventQueues.lock().unwrap();
	return match map.get_mut(event_name) {
		Some(v) => {
			// :(
			let r : &Vec<T> = unsafe { mem::transmute(v) };
			Some(r) 
		},
		None => None
	}
}

pub fn set_event_queue<T>(event_name: &'static str, initial_value: T) {
	let map = &mut EventQueues.lock().unwrap();
	let queue = Box::new(vec![initial_value]);

	// :(
	let v : &mut usize = unsafe { mem::transmute(queue) };
	map.insert(event_name, v);
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

