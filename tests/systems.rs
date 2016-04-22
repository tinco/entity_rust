#[macro_use]
extern crate entity_rust;
#[macro_use]
extern crate lazy_static;
extern crate shared_mutex;

system!( my_system {

	on!(my_event, bla) {
		// 
	}

});

#[test]
fn register_function() {
	my_system::register();
}