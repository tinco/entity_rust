#[macro_export]
macro_rules! static_any_vec_map {
	($v:ident, $t:ty) => {
		pub mod $v {
			use std::collections::HashMap;
			use std::any::Any;
			use shared_mutex::{ SharedMutex, MappedSharedMutexReadGuard };

			lazy_static! {
				pub static ref Map: SharedMutex<HashMap<$t, Box<Any+'static+Sync>>> = SharedMutex::new(HashMap::new());
			}

			pub fn push<T>(event_name: & $t, event: T) where T: Any+'static+Sync {
				let mut map = Map.write().expect("Static Map is not initialized.");
				let mut entry = map.entry(event_name.clone()).or_insert(Box::new(Vec::<T>::new()));
				let mut casted_entry = &mut **entry as &mut Any;
				let mut vec = casted_entry.downcast_mut::<Vec<T>>().expect("Could not cast created entry to Vec<T>");
				vec.push(event);
			}

			pub fn get<T>(event_name: & $t) -> MappedSharedMutexReadGuard<Vec<T>> where T: Any+'static+Sync {
				let map = Map.read().expect("Static Map is not initialized.");
				let vec = map.into_mapped().map(|m| {
					let entry = m.get(event_name).expect("Could not get a particular map entry.");
					let casted_entry = & **entry as & Any;
					return casted_entry.downcast_ref::<Vec<T>>().expect("Could not cast gotten entry to Vec<T>");
				});
				return vec;
			}

			pub fn clear<T>(event_name: & $t) where T: Any+'static+Sync {
				let mut map = Map.write().expect("Static Map is not initialized.");
				let mut entry = map.entry(event_name.clone()).or_insert(Box::new(Vec::<T>::new()));
				let mut casted_entry = &mut **entry as &mut Any;
				let mut vec = casted_entry.downcast_mut::<Vec<T>>().expect("Could not cast created entry to Vec<T>");
				vec.clear();
			}
		}
	}
}