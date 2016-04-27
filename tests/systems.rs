#[macro_use]
extern crate entity_rust;
#[macro_use]
extern crate lazy_static;
extern crate shared_mutex;

pub mod my_event {
	pub struct Data {
		x: i64
	}
}

system!( my_system {
	state! { x: i64 }

	on!( my_event, { positions: State }, {}) {
		self.x += 1;
	}

});

#[test]
fn generates_functions() {
	my_system::register();
}

#[test]
fn generates_state() {
	let s = my_system::State::default();
	assert!(s.x == 0);

	{
		let mut state = my_system::state.write().expect("System lock corrupted.");
		state.x = 2;
	}

	{
		let state = my_system::state.read().expect("System lock corrupted");
		assert!(state.x == 2);
	}

}