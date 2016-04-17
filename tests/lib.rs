extern crate entity_rust;

use entity_rust::events;

#[test]
fn event_queue_accessors() {
	let i : i64 = 5;
	let name = String::from("my_queue");
	events::event_queue_push(&name, i);
	let queue = events::event_queue_get(&name);
	assert!(queue.len() == 1);
	assert!(i == queue[0]);
}

/*
#[test]
fn events_example_trigger() {
	let arg = events::example::Argument { x: 1, y: 2};
	events::example::trigger(arg);
	let existed = events::event_queue_apply(events::example::EventUUID, |queue| {
		assert!(queue.len() == 1);
		assert!(arg == queue[0])
	});
	if !existed {
		assert!(false)
	}
}
*/