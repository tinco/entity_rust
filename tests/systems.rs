#![feature(type_macros)]

#[macro_use]
extern crate entity_rust;
#[macro_use]
extern crate lazy_static;
extern crate shared_mutex;
extern crate uuid;

event!{ my_event , x: i64, y: i64 }
event!{ my_event_2 , x: i64, y: i64 }

system!( my_system {
	use super::my_event;
	use super::my_event_2;

	component! { position,
		x: i64,
		y: i64
	}

	state! { x: i64 }

	on!( my_event, { positions: position }, {}) self, data => {
		self.x += data[0].x;
		self.x += positions[0].1.x;
	}

	on!( my_event_2, { positions: position }, {}) self, data => {
		// do nothing
	}
});

fn reset_state() {
	let mut state = my_system::STATE.write().expect("System lock corrupted.");
	state.x = 0;
}

#[test]
fn generates_functions() {
	my_system::register();
}

#[test]
fn generates_state() {
	reset_state();
	let s = my_system::State::default();
	assert!(s.x == 0);

	{
		let mut state = my_system::STATE.write().expect("System lock corrupted.");
		state.x = 2;
	}

	{
		let state = my_system::STATE.read().expect("System lock corrupted");
		assert!(state.x == 2);
	}
}

/*
#[test]
fn on_event_works() {
	reset_state();
	let data = vec![my_event::Data{ x: 1, y: 2}];
	let positions = &vec![Position{ x: 3, y: 5}] as &Any;
	let components = vec![positions];
	let mut_positions = &mut vec![Position{ x: 3, y: 5}] as &mut Any;
	let mut mut_components = vec![mut_positions];

	my_system::my_event(&data, components, mut_components);

	let state = my_system::state.read().expect("System state corrupted");
	assert!(state.x == 4);
}
*/