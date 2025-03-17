//! The systems are the bits of code providing the game logic.
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

/// Systems that can be used accross several modes
pub mod common;
/// The logic for when the game is running
pub mod running;
/// The logic for when the game is over
pub mod over;

pub use common::*;
pub use running::*;
pub use over::*;