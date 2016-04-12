/// Example component
pub struct Argument {
	x: f64,
	y: f64
}

use uuid::Uuid;
lazy_static! {
	/// Arguments to be passed into the next time trigger is called
	pub static ref Arguments: Mutex<Vec<Argument>> = Mutex::new(vec![]);
	/// EventUUID is used internally to index events, is randomly
	/// generated at first access.
	pub static ref EventUUID: &str = Uuid::new_v4().simple();
}

/// Listeners are a list of functions that should be called by trigger
pub fn trigger(argument: Argument) {
	match events::get_event_queue_mut::<Argument>(EventUUID) {
		Some(event_queue) => event_queue.append(argument),
		None => events::set_event_queue::<Argument>(EventUUID, argument)
	}
}

