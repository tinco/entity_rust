#![feature(type_macros)]

//! # Goal
//!
//! We want to define components, store them in an efficient manner and
//! make them accessible to systems.
//!
//! We want to define events along with their types and make them available
//! to systems.
//!
//! We want to define systems that operate on lists of components, are
//! triggered by other systems through events.
//!

//! # Implementation
//!
//! For each component type there will be a list that is a tuple of an
//! entity ID and the component values. There will also be a map from
//! entity IDs to component list indexes.
//! 
//! A system will consist of state, iterators over components its subscribed
//! to and any number of functions that are triggered by events.
//!

//! # Syntax
//! 
//! ``` 
//! component! { Physics, body: physics.RigidBody, physics_id: physics.ID }
//!
//! // event! { GameStarted } // This one is implicitly defined
//! event! { PhysicsTick, dt: u64 }
//! event! { Bump, e1: EntityID, e2: EntityID }
//!
//! system! { PhysicsSystem,
//!
//!   state! { world: physics.World }
//!
//!   on! { GameStarted, {
//!      state.world = physics.World::new(event.name);
//!      state.world.on_collision = |e1, e2| {
//!        unwrap_entity = |e| { e.user_data.downcast_ref<EntityID>() }
//!        trigger! { Bump, unwrap_entity(e1), unwrap_entity(e2) }
//!      };
//!   }}
//!
//!   on! { PhysicsTick, {
//!     state.world.step(event.dt);
//!   }}
//!
//!   component_added! { Physics, {
//!     let id = state.world.add_body(component.body);
//!     component.physics_id = id;
//!   }}
//!
//!   component_removed! { Physics, {
//!     state.world.remove_body(component.physics_id);
//!   }}
//!
//! }
//!
//! system! { BumpSystem, {
//!   on! { Bump, {
//!     println!("Entity {:?} bumped into entity {:?}!", e1, e2);
//!   }}
//! }}
//! ```
#[macro_use]
extern crate lazy_static;
extern crate shared_mutex;
extern crate uuid;

#[macro_use]
pub mod helpers;
#[macro_use]
pub mod components;
pub mod entities;
#[macro_use]
pub mod events;
#[macro_use]
pub mod systems;

use std::thread;
pub use std::time::{ Duration, Instant };

event!{ tick, step: super::Duration }

pub fn run(ticks_per_second: u32) {
	let events_thread = ticker(ticks_per_second, true);
	let _ = events_thread.join();
}

pub fn ticker(ticks_per_second: u32, sleep: bool) -> thread::JoinHandle<()> {
	let step = Duration::from_secs(1) / ticks_per_second;
	
	let mut last_tick = Instant::now();

	thread::spawn(move || {
		loop {
			let current_time = Instant::now();
			let next_tick = last_tick + step;
			if next_tick > current_time {
				if sleep {
					thread::sleep(Duration::from_millis(1));
				}
			} else {
				tick::trigger(step);
				events::next_tick();
				last_tick = Instant::now();
				events::run_events();
			}
		}
	})
}
