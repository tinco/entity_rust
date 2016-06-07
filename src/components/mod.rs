/// Components are types that associate an entity with a set of properties
/// for each type a list of all its instances is kept so they can be iterated
/// over efficiently by systems.
///
/// Whenever a new component is created it is added to a list.
///
/// component! { physics, body: physics.RigidBody, physics_id: physics.ID }
///

use std::collections::{ HashMap };
//use std::ops::CoerceUnsized;
use std::any::{ Any, TypeId };
use shared_mutex::{ SharedMutex, MappedSharedMutexReadGuard, MappedSharedMutexWriteGuard };

pub trait MappedSharedMutexGetters {
	fn read_as_any<'mutex>(&self) -> MappedSharedMutexReadGuard<'mutex, Any>;
	fn write_as_any<'mutex>(&self) -> MappedSharedMutexWriteGuard<'mutex, Any>;
}

pub struct Component {
	pub name: TypeId,
	pub getters: Box<MappedSharedMutexGetters+Sync>
}

lazy_static! {
	pub static ref COMPONENTS: SharedMutex<HashMap<TypeId, Component>> = SharedMutex::new(HashMap::new());
}

pub fn register(component : Component) {
	let mut components = COMPONENTS.write().expect("COMPONENTS lock corrupted");
	components.insert(component.name, component);
}

pub fn get_components_read_lock<'mutex>(id : TypeId) -> MappedSharedMutexReadGuard<'mutex, Any> {
	let components = COMPONENTS.read().expect("COMPONENTS lock corrupted");
	let component = components.get(&id).expect("Unknown component type requested");
	component.getters.read_as_any()
}

pub fn get_components_write_lock<'mutex>(id : TypeId) -> MappedSharedMutexWriteGuard<'mutex, Any> {
	let components = COMPONENTS.read().expect("COMPONENTS lock corrupted");
	let component = components.get(&id).expect("Unknown component type requested");
	component.getters.write_as_any()
}

#[macro_export]
macro_rules! component {
	( $component_name:ident , $( $name:ident : $field:ty ),* ) => (
		pub mod $component_name {
			use shared_mutex::{ SharedMutex, SharedMutexWriteGuard, SharedMutexReadGuard, MappedSharedMutexReadGuard, MappedSharedMutexWriteGuard };
			use entity_rust::entities::{ ComponentList, EntityID };
			use entity_rust::components;
			use std::any::{ Any, TypeId };


			#[derive(Default)]
			pub struct Component {
				$(pub $name : $field),*
			}

			pub struct ListGetters;

			impl components::MappedSharedMutexGetters for ListGetters {
				fn read_as_any<'mutex>(&self) -> MappedSharedMutexReadGuard<'mutex, Any> {
					let list = LIST.read().expect("COMPONENT_LIST corrupted");
					list.into_mapped().map(|v| v as &Any)
				}
				fn write_as_any<'mutex>(&self) -> MappedSharedMutexWriteGuard<'mutex, Any> {
					let mut list = LIST.write().expect("COMPONENT_LIST corrupted");
					list.into_mapped().map(|v| v as &mut Any)
				}
			}

			lazy_static! {
				pub static ref LIST: SharedMutex<ComponentList<Component>> = SharedMutex::new(Vec::new());
			}

			pub fn add(entity: EntityID, component: Component) {
				let mut list = LIST.write().expect("COMPONENT_LIST corrupted");
				list.push((entity, component));
			}

			pub fn add_with_lock(mut list: MappedSharedMutexWriteGuard<ComponentList<Component>>, entity: EntityID, $($name : $field),*) {
				let c = Component { $($name : $name),* };
				list.push((entity,c));
			}

			pub fn register() {
				let type_id = TypeId::of::<Component>();
				let component_entry = components::Component {
					name : type_id,
					getters : Box::new(ListGetters)
				};

				components::register(component_entry);
			}
		}
	)
}
