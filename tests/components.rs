#[macro_use]
extern crate entity_rust;
#[macro_use]
extern crate lazy_static;
extern crate shared_mutex;

component! { test_component, a: i64, b: i64 }

fn reset_state() {
	let mut state = test_component::LIST.write().expect("Component lock corrupted.");
	state.clear();
}

#[test]
fn generates_functions() {
	reset_state();
	{
		let components = test_component::LIST.write().expect("Component lock corrupted.").into_mapped();
		test_component::add_with_lock(components, 1, 2, 3);
	}
	{
		let components = test_component::LIST.read()
			.expect("Component lock corrupted.");
		let mut components_iter = components.iter();
		let component = components_iter.next().expect("Component list too short");
		assert!(component.0 == 1);
		assert!(component.1.a == 2);
	}
}

use entity_rust::components;
use std::any::{ TypeId };

#[test]
fn gettable_lock() {
	reset_state();
	let type_id = TypeId::of::<test_component::Component>();
	test_component::register();

	let cs = components::get_components_write_lock(type_id);
	let components = cs.map(|v|
		v.downcast_mut::<entity_rust::entities::ComponentList<test_component::Component>>()
			.expect("Components mutex was not of expected type")
	);
	test_component::add_with_lock(components, 1, 2, 3);
}
