pub mod my_system {
	#[derive(Default)]
	struct State {
		i: i64
	}

	lazy_static! {
		pub static ref state: SharedMutex<State> = SharedMutex::new(State::default());
	}

	impl State {
		fn event_name(&mut self, data: &Vec<event_name::Data>, positions: &Vec<Position>, descriptions: &mut Vec<Description> ) {
			let position = positions[0];
			descriptions[0].set(format!("Position = {},{}", position.x, position.y));
		}
	}

	pub fn event_name(&Vec<event_name::Data> event_data, components: Vec<Any>) {
		let components_iter = components.iter();
		let positions : &Vec<Position> = components
			.next().expect("Event components list too short.")
			.downcast_ref().expect("Event component not of expected type.");
		let positions : &mut Vec<Description> = components
			.next().expect("Event components list too short.")
			.downcast_ref().expect("Event component not of expected type.");
		let state = state.write().expect("System lock corrupted.");
		state.event_name(event_data, positions, descriptions);
	}

	pub fn register() {
		events::register_handler(event_name, vec![Position::type_id()], vec![Description::type_id()]);
	}
}