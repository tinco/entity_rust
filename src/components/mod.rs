/// Components are types that associate an entity with a set of properties
/// for each type a list of all its instances is kept so they can be iterated
/// over efficiently by systems.
///
/// Whenever a new component is created it is added to a list.
///
/// component! { physics, body: physics.RigidBody, physics_id: physics.ID }
///

use std::collections::{ HashSet, HashMap };
use std::any::{ Any, TypeId };
use shared_mutex::{ SharedMutex, SharedMutexReadGuard, SharedMutexWriteGuard };

pub mod example;

#[derive(Clone)]
pub struct Component {
	pub name: TypeId,
	pub get_component_list: fn () -> Box<Any + 'static>
}

lazy_static! {
	pub static ref COMPONENTS: SharedMutex<HashMap<TypeId, Component>> = SharedMutex::new(HashMap::new());
}

pub fn register(component : Component) {
	let mut components = COMPONENTS.write().expect("COMPONENTS lock corrupted");
	components.insert(component.name, component);
}

/*
pub fn get_components_lock<'mutex, T>(id : TypeId) -> SharedMutexReadGuard<'mutex, T> where T : Any {
	let components = COMPONENTS.read().expect("COMPONENTS lock corrupted");
	let component = components.get(&id).expect("Unknown component type requested");
	let component_list_any = (component.get_component_list)();
	let component_list : &'static SharedMutex<T> = *component_list_any.downcast_ref().expect("Invalid type for components list");
	return component_list.read().expect("Component LIST lock corrupted"); 
}

pub fn get_components_lock_mut<'mutex, T>(id : TypeId) -> SharedMutexWriteGuard<'mutex, T> where T : Any {
	let components = COMPONENTS.read().expect("COMPONENTS lock corrupted");
	let component = components.get(&id).expect("Unknown component type requested");
	let component_list_any = (component.get_component_list)();
	let component_list : &'static SharedMutex<T> = *component_list_any.downcast_ref().expect("Invalid type for components list");
	return component_list.write().expect("Component LIST lock corrupted"); 
}
*/

#[macro_export]
macro_rules! component {
	( $component_name:ident , $( $name:ident : $field:ty ),* ) => (
		pub mod $component_name {
			use shared_mutex::{ SharedMutex, SharedMutexWriteGuard };
			use entity_rust::entities::{ ComponentList, EntityID };

			#[derive(Default)]
			pub struct Component {
				pub $($name : $field),*
			}

			lazy_static! {
				pub static ref LIST: SharedMutex<ComponentList<Component>> = SharedMutex::new(Vec::new());
			}

			pub fn create(mut list: SharedMutexWriteGuard<ComponentList<Component>>, entity: EntityID, $($name : $field),*) {
				let c = Component { $($name : $name),* };
				list.push((entity,c));
			}
		}
	)
}
