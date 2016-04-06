mod example {
	use std::sync::{ Mutex };

	lazy_static! { static ref List: Mutex<Vec<Component>> = Mutex::new(vec![]); }

	/// Example component
	struct Component {
		x: f64,
		y: f64
	}

	impl Component {
		fn create(x: f64, y: f64) {
			let c = Component { x: x, y: y };
			let mut list = List.lock().unwrap();
			list.push(c);
		}
	}
}