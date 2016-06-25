#![feature(type_macros)]

#[macro_use]
extern crate entity_rust;
#[macro_use]
extern crate lazy_static;
extern crate shared_mutex;
extern crate uuid;

// use std::any::Any;

use entity_rust::{ events };

pub struct Bla<'a> { pub x: &'a i64 }

event!{ test_event , x: i64, y: i64 }
sync_event! { test_sync_event, x: &'a mut super::Bla<'b> }
component! { test_component, a: i64, b: i64 }

system!( test_system {
	use super::test_component;
	use super::test_event;
	use super::test_sync_event;

	state! { x: i64 }

	on test_event, { positions: test_component }, {}, (self, data) => {
		assert!(data.len() > 0);
		self.x += data[0].x;
		assert!(positions.len() > 0);
		self.x += positions[0].1.a;
	}

	on_sync test_sync_event, (self, x) => {
		self.x += *x.x;
		assert!(true);
	}
});

fn reset_state() {
	let mut state = test_system::STATE.write().expect("System lock corrupted.");
	state.x = 0;
	test_sync_event::clear_handlers();
	test_event::clear_handlers();
}

#[test]
fn run_event_runs_system_events() {
	reset_state();
	test_system::register();
	test_component::register();

	test_component::add(1, test_component::Component { a: 2, b: 10 });

	{
		let state = test_system::STATE.read().expect("System lock corrupted");
		assert!(state.x == 0);
	}

	test_event::trigger(1, 39);
	events::run_events();
	{
		let state = test_system::STATE.read().expect("System lock corrupted");
		assert_eq!(state.x, 3);
	}
}


#[test]
fn run_sync_event() {
	reset_state();
	test_system::register();
	let mut x = 0;
	let mut b = Bla { x: &mut x };
	test_sync_event::trigger(&mut b);
}
