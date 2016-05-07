/// Components are types that associate an entity with a set of properties
/// for each type a list of all its instances is kept so they can be iterated
/// over efficiently by systems.
///
/// Whenever a new component is created it is added to a list.
///
///
///
/// component! { physics, body: physics.RigidBody, physics_id: physics.ID }
///


pub mod example;

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
