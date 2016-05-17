use shared_mutex::{ SharedMutex };
use std::any::{ Any, TypeId };
use events;
use uuid::Uuid;

/// Example component
#[derive(PartialEq,Eq,Clone)]
pub struct Data {
	pub x: i64,
	pub y: i64
}

type HandlerFn = Fn(Vec<Data>, Vec<&Any>, Vec<&mut Any>);

struct Handler<'a> {
	handler_fn: &'a HandlerFn,
	component_types: Vec<TypeId>,
	mut_component_types: Vec<TypeId>
}

lazy_static! {
	/// Arguments to be passed into the next time trigger is called
	pub static ref ARGUMENTS: SharedMutex<Vec<Data>> = SharedMutex::new(vec![]);
	/// EVENT_UUID is used internally to index events, is randomly
	/// generated at first access.
	pub static ref EVENT_UUID: String = Uuid::new_v4().simple().to_string();
}

/// Listeners are a list of functions that should be called by trigger
pub fn trigger(argument: Data) {
	events::trigger_this_tick(&*EVENT_UUID);
}

