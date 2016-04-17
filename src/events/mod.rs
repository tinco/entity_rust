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
/// 1. We need to register which events have unacknowledged messages.
/// 2. We need to register handlers.
/// 3. We need to have a this_tick and a next_tick queue
/// 4. We need a mechanism for actually receiving the triggers and
///    handling the special 'tick' behaviour.
///

use std::collections::HashMap;
use std::any::Any;
use shared_mutex::{ SharedMutex, MappedSharedMutexReadGuard };

pub mod example;

lazy_static! {
	pub static ref EventQueues: SharedMutex<HashMap<String, Box<Any+'static+Sync>>> = SharedMutex::new(HashMap::new());
}

pub fn event_queue_push<T>(event_name: &String, event: T) where T: Any+'static+Sync {
	let mut map = EventQueues.write().expect("EventQueues is not initialized.");
	let mut entry = map.entry(event_name.clone()).or_insert(Box::new(Vec::<T>::new()));
	let mut casted_entry = &mut **entry as &mut Any;
	let mut vec = casted_entry.downcast_mut::<Vec<T>>().expect("Could not cast created entry to Vec<T>");
	vec.push(event);
}

pub fn event_queue_get<T>(event_name: &String) -> MappedSharedMutexReadGuard<Vec<T>> where T: Any+'static+Sync {
	let map = EventQueues.read().expect("EventQueues is not initialized.");
	let vec = map.into_mapped().map(|m| {
		let entry = m.get(event_name).expect("Could not get a particular event queue.");
		let casted_entry = & **entry as & Any;
		return casted_entry.downcast_ref::<Vec<T>>().expect("Could not cast gotten entry to Vec<T>");
	});
	return vec;
}

pub fn event_queue_clear<T>(event_name: &String) where T: Any+'static+Sync {
	let mut map = EventQueues.write().expect("EventQueues is not initialized.");
	let mut entry = map.entry(event_name.clone()).or_insert(Box::new(Vec::<T>::new()));
	let mut casted_entry = &mut **entry as &mut Any;
	let mut vec = casted_entry.downcast_mut::<Vec<T>>().expect("Could not cast created entry to Vec<T>");
	vec.clear();
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
