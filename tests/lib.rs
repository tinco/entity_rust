extern crate entity_rust;

use entity_rust::events;

#[test]
fn event_queue_accessors() {
	let i : i64 = 5;
	events::set_event_queue("my_queue", i);
	let updated = events::event_queue_apply("my_queue", |queue| {
		assert!(queue.len() == 1);
		assert!(i == queue[0])
	});
	if !updated {
		assert!(false)
	}
}
