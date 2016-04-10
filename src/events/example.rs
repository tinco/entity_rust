/// Example component
pub struct Argument {
	x: f64,
	y: f64
}

/// Arguments to be passed into the next time trigger is called
 lazy_static! { pub static ref Arguments: Mutex<Vec<Argument>> = Mutex::new(vec![]); }


/// Listeners are a list of functions that should be called by trigger

pub fn trigger(arguments: &mut Vec<Argument>) {

}
