#[macro_use]
extern crate entity_rust;
#[macro_use]
extern crate lazy_static;
extern crate shared_mutex;

use std::collections::HashMap;

static_any_vec_map! { my_map, String }

#[test]
fn static_map_initial_set() {
	let i : i64 = 5;
	let name = String::from("my_queue");
	my_map::clear::<i64>(&name);
	my_map::push(&name, i);
	let queue = my_map::get(&name);
	assert!(queue.len() == 1);
	assert!(i == queue[0]);
}

#[test]
fn static_map_multi_set() {
	let i : i64 = 5;
	let j : i64 = 352;
	let k : i64 = 234;
	let name = String::from("my_queue");
	my_map::clear::<i64>(&name);
	my_map::push(&name, i);
	my_map::push(&name, j);
	my_map::push(&name, k);
	let queue = my_map::get(&name);
	assert!(queue.len() == 3);
	assert!(i == queue[0]);
	assert!(j == queue[1]);
	assert!(k == queue[2]);
}

#[test]
fn static_map_multi_queue_set() {
	let i : i64 = 5;
	let j : i64 = 352;
	let name = String::from("my_queue");
	let name2 = String::from("next_queue");
	my_map::clear::<i64>(&name);
	my_map::clear::<i64>(&name2);
	my_map::push(&name, i);
	my_map::push(&name2, j);
	{
		let queue = my_map::get(&name);
		assert!(queue.len() == 1);
		assert!(i == queue[0]);
	}
	{
		let queue2 = my_map::get(&name2);
		assert!(queue2.len() == 1);
		assert!(j == queue2[0]);
	}
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