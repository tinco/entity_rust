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
use shared_mutex::{ SharedMutex, SharedMutexReadGuard, SharedMutexWriteGuard, MappedSharedMutexReadGuard };

pub mod example;

trait MappedSharedMutexGetters {
	pub fn read_as_any() -> MappedSharedMutexReadGuard<'mutex, Any>;
	pub fn write_as_any() -> MappedSharedMutexWriteGuard<'mutex, Any>;
}

#[derive(Clone)]
pub struct Component {
	pub name: TypeId,
	pub getters: Box<MappedSharedMutexGetters>
}

lazy_static! {
	pub static ref COMPONENTS: SharedMutex<HashMap<TypeId, Component>> = SharedMutex::new(HashMap::new());
}

pub fn register(component : Component) {
	let mut components = COMPONENTS.write().expect("COMPONENTS lock corrupted");
	components.insert(component.name, component);
}

pub fn get_components_lock<'mutex>(id : TypeId) -> MappedSharedMutexReadGuard<'mutex, Any> {
	let components = COMPONENTS.read().expect("COMPONENTS lock corrupted");
	let component = components.get(&id).expect("Unknown component type requested");
	let list = (component.list_getter)();
	list.read()
}

#[macro_export]
macro_rules! component {
	( $component_name:ident , $( $name:ident : $field:ty ),* ) => (
		pub mod $component_name {
			use shared_mutex::{ SharedMutex, SharedMutexWriteGuard, SharedMutexReadGuard };
			use entity_rust::entities::{ ComponentList, EntityID };
			use entity_rust::components;
			use std::any::{ Any, TypeId };


			#[derive(Default)]
			pub struct Component {
				pub $($name : $field),*
			}

			pub struct ListContainer {
				pub list: &'static SharedMutex<ComponentList<Component>>
			}

			lazy_static! {
				pub static ref LIST: SharedMutex<ComponentList<Component>> = SharedMutex::new(Vec::new());
			}

			pub fn create(mut list: SharedMutexWriteGuard<ComponentList<Component>>, entity: EntityID, $($name : $field),*) {
				let c = Component { $($name : $name),* };
				list.push((entity,c));
			}

			pub fn get_list_as_any() -> MappedSharedMutexReadGuard<Any> {
				let list = LIST.read().expect("COMPONENT_LIST corrupted");
				let result = list.into_mapped() {}
			}

			pub fn register() {
				let type_id = TypeId::of::<Component>();
				let component_entry = components::Component {
					name : type_id,
					get_component_list : get_list
				};

				components::register(component_entry)
			}
		}
	)
}
