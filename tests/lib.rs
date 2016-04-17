extern crate entity_rust;

use std::collections::HashMap;

use entity_rust::events;

#[test]
fn event_queue_initial_set() {
	let i : i64 = 5;
	let name = String::from("my_queue");
	events::event_queue_clear::<i64>(&name);
	events::event_queue_push(&name, i);
	let queue = events::event_queue_get(&name);
	assert!(queue.len() == 1);
	assert!(i == queue[0]);
}

#[test]
fn event_queue_multi_set() {
	let i : i64 = 5;
	let j : i64 = 352;
	let k : i64 = 234;
	let name = String::from("my_queue");
	events::event_queue_clear::<i64>(&name);
	events::event_queue_push(&name, i);
	events::event_queue_push(&name, j);
	events::event_queue_push(&name, k);
	let queue = events::event_queue_get(&name);
	assert!(queue.len() == 3);
	assert!(i == queue[0]);
	assert!(j == queue[1]);
	assert!(k == queue[2]);
}

#[test]
fn test_how_string_works_as_hashmap_index() {
	let name = String::from("my_queue");
	let mut m : HashMap<String, i64> = HashMap::new();
	let i = 543;
	m.insert(name.clone(), i);
	let j = *m.get(&name.clone()).unwrap();
	assert!(i == j);
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