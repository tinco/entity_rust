#[macro_use]
extern crate entity_rust;
#[macro_use]
extern crate lazy_static;
extern crate shared_mutex;
extern crate uuid;

// use std::any::Any;

use entity_rust::{ entities, events, components };

event!{ test_event , x: i64, y: i64 }
component! { test_component, a: i64, b: i64 }

system!( test_system {
	state! { x: i64 }

	on!( test_event, { positions: super::test_component::Component }, {}) self, data => {
		assert!(data.len() > 0);
		self.x += data[0].x;
		assert!(positions.len() > 0);
		self.x += positions[0].1.a;
	}
});

fn reset_state() {
	let mut state = test_system::state.write().expect("System lock corrupted.");
	state.x = 0;
}

#[test]
fn run_event_runs_system_events() {
	test_system::register();
	test_component::register();

	test_component::add(1, test_component::Component { a: 2, b: 10 });

	test_event::trigger(test_event::Data { x: 1, y: 6 });
	events::run_events();
	let state = test_system::state.read().expect("System lock corrupted");
	assert!(state.x == 3);
}