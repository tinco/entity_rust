#[macro_use]
extern crate entity_rust;
#[macro_use]
extern crate lazy_static;
extern crate shared_mutex;
extern crate uuid;

use entity_rust::events;
use std::any::{ Any };

event!(test_event);

#[test]
fn events_example_trigger() {
	let arg = events::example::Argument { x: 1, y: 2};
	events::example::trigger(arg.clone());
	let queue = events::this_tick_queues::get(&*events::example::EVENT_UUID);
	assert!(queue.len() == 1);
	assert!(arg == queue[0])
}

fn test_handler(arg : Vec<test_event::Argument>, cs: Vec<&Any>, mut_cs: Vec<&mut Any>) {
	println!("Test");
}

#[test]
fn register_handler_works() {
	test_event::register_handler(test_handler, vec![], vec![]);
}