extern crate entity_rust;

use entity_rust::events;

#[test]
fn events_example_trigger() {
	let arg = events::example::Argument { x: 1, y: 2};
	events::example::trigger(arg.clone());
	let queue = events::this_tick_queues::get(&*events::example::EventUUID);
	assert!(queue.len() == 1);
	assert!(arg == queue[0])
}
