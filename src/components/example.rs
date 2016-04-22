use std::sync::{ Mutex };
use entities::*;

/// The coolest way to implement this would be to have two lists references.
/// One mutable, one is immutable.
/// 
/// The event system will have a trigger function that takes an
/// argument that decides if the event will be triggered this tick
/// or the next tick. The default should be next tick to prevent
/// inadvertent infinite loop bugs.
///
/// The event system can then schedule all events handlers that
/// do not mutate components to be run in parallel.
/// All handlers that do mutate the component list can be run
/// sequentially after that. Perhaps we can even schedule
/// those with non-overlapping mutable component lists to
/// run in parallel.
///
/// So the next step at least is to make a mutable list iterator
/// that we can use and have all events be executed sequentially.
///
/// An easy way could be that the event loop is responsible for taking
/// the lock and all the functions here take a reference to the unlocked
/// component list given to them by the event system.

lazy_static! { pub static ref LIST: Mutex<ComponentList<Component>> = Mutex::new(vec![]); }

/// Example component
pub struct Component {
	x: f64,
	y: f64
}

pub fn create(list: &mut ComponentList<Component>, entity: EntityID, x: f64, y: f64) {
	let c = Component { x: x, y: y };
	list.push((entity,c));
}
