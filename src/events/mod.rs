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

static_any_vec_map! { this_tick_queues, String }
static_any_vec_map! { next_tick_queues, String }

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
