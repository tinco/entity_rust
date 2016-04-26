#[macro_use]
extern crate entity_rust;
#[macro_use]
extern crate lazy_static;
extern crate shared_mutex;

system!( my_system {
	state! { x: i64 }

	on!(my_event, bla) {
		self.x += 1;
	}

});

#[test]
fn generates_functions() {
	my_system::register();
	my_system::my_event();
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