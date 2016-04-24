#[macro_use]
extern crate entity_rust;
#[macro_use]
extern crate lazy_static;
extern crate shared_mutex;

system!( my_system {
	state! { x: i64 }

	on!(my_event, bla) {
		// 
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
}