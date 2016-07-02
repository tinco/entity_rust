#[macro_use]
extern crate entity_rust;
#[macro_use]
extern crate lazy_static;
extern crate shared_mutex;
extern crate uuid;

event!(test_event, x: i64, y: i64 );
sync_event! { test_sync_event, x: &'a i64 }
sync_event! { test_sync_event_2, x: i64 }


fn sync_event_handler(x: &i64) {
	assert!(*x == 1);
}

/*#[test]
fn sync_events() {
	test_sync_event::register_handler(sync_event_handler);
	let x = 1;
	test_sync_event::trigger(&x);
	assert!(x == 1);
}*/

// use entity_rust::events;
/* use std::any::{ Any };


#[test]
fn events_example_trigger() {
	/*
	let arg = events::example::Data { x: 1, y: 2};
	events::example::trigger(arg.clone());
	let queue = events::this_tick_queues::get(&*events::example::EVENT_UUID);
	assert!(queue.len() == 1);
	assert!(arg == queue[0])
	*/
}
/*
pub fn test_handler(arg : &Vec<test_event::Data>, cs: Vec<&Any>, mut_cs: Vec<&mut Any>) {
	println!("Test");
}

#[test]
fn register_handler_works() {
	let handler : test_event::HandlerFn = test_handler;
	test_event::register_handler(handler, vec![], vec![]);
}*/
*/