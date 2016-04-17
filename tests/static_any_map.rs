#[macro_use]
extern crate entity_rust;

static_any_map! { my_map, String }

#[test]
fn event_queue_initial_set() {
	let i : i64 = 5;
	let name = String::from("my_queue");
	my_map::clear::<i64>(&name);
	my_map::push(&name, i);
	let queue = my_map::get(&name);
	assert!(queue.len() == 1);
	assert!(i == queue[0]);
}