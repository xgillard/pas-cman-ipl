//! This library provides all the implementation routines that are
//! required to create a pacman-like game.
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

/// some utility functions
pub mod utils;
/// any resource manipulated by the game
pub mod resources;
/// properties of the entities
pub mod components;
/// game logic
pub mod systems;
/// the game state
pub mod game_state;
/// how to spawn stuffs in the game
pub mod spawn;

pub use utils::*;
pub use resources::*;
pub use components::*;
pub use systems::*;
pub use game_state::*;
pub use spawn::*;

pub use bracket_lib::prelude::*;
pub use legion::*;
pub use legion::world::*;
pub use legion::systems::*;