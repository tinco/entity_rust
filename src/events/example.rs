use shared_mutex::{ SharedMutex };
use events;
use uuid::{ UuidVersion };

/// Example component
pub struct Argument {
	x: f64,
	y: f64
}

use uuid::Uuid;
lazy_static! {
	/// Arguments to be passed into the next time trigger is called
	pub static ref Arguments: SharedMutex<Vec<Argument>> = SharedMutex::new(vec![]);
	/// EventUUID is used internally to index events, is randomly
	/// generated at first access.
	pub static ref EventUUID: &'static str =
		format! { "{}", Uuid::new(UuidVersion::Random).unwrap().simple() };
}

/// Listeners are a list of functions that should be called by trigger
pub fn trigger(argument: Argument) {
	let updated = events::event_queue_apply::<Argument>(EventUUID, |event_queue| {
		event_queue.append(argument);
	});

	if !updated {
		events::set_event_queue::<Argument>(EventUUID, argument);
	}
}

