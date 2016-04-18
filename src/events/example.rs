use shared_mutex::{ SharedMutex };
use events;
use uuid::{ UuidVersion };

/// Example component
#[derive(PartialEq,Eq,Clone)]
pub struct Argument {
	pub x: i64,
	pub y: i64
}

use uuid::Uuid;
lazy_static! {
	/// Arguments to be passed into the next time trigger is called
	pub static ref Arguments: SharedMutex<Vec<Argument>> = SharedMutex::new(vec![]);
	/// EventUUID is used internally to index events, is randomly
	/// generated at first access.
	pub static ref EventUUID: String = Uuid::new_v4().simple().to_string();
}

/// Listeners are a list of functions that should be called by trigger
pub fn trigger(argument: Argument) {
	events::this_tick_queues::push(&*EventUUID, argument);
}

