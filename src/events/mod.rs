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
use std::mem;
use shared_mutex::{ SharedMutex };

pub mod example;

lazy_static! {
	pub static ref EventQueues: SharedMutex<HashMap<String, &'static mut usize>> = SharedMutex::new(HashMap::new());
}

/// The event loop should trigger every n ms and execute any
/// events that are queued up. Every event has a function
/// that is called by the trigger! macro that puts the event
/// on the queue.
pub fn run_loop() {

}

pub fn event_queue_push<T>(event_name: &String, event: T) {
	let map = &mut EventQueues.write().unwrap();
	if map.contains_key(event_name) {
		let queue = transmute_from_generic_ref(map.get(event_name).unwrap());
		queue.push(event)
	} else {
		let queue = Box::new(vec![event]);
		map.insert(event_name.clone(), transmute_to_generic_ref(queue));
	}
}


/// src/events/mod.rs:45:38: 45:52 error: mutating transmuted &mut T from &T may cause undefined behavior,consider instead using an UnsafeCell, #[deny(mutable_transmutes)] on by default
fn transmute_from_generic_ref<'a,T>(value: &'a &mut usize) -> &'a mut Box<Vec<T>> {
	let r : &mut Box<Vec<T>> = unsafe { mem::transmute(value) };
	return r;
}

fn transmute_to_generic_ref<'a,T>(value: Box<Vec<T>>) -> &'a mut usize {
	let r : &'a mut usize = unsafe { mem::transmute(value) };
	return r;
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

