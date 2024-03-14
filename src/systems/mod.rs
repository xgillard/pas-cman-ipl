//! The systems are the bits of code providing the game logic.
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

/// Systems that can be used accross several modes
pub mod common;
/// The logic for when the game is running
pub mod running;
/// The logic for when the game is finished and the user has won
pub mod won;
/// The logic for when the game is over and the user has lost
pub mod lost;

pub use common::*;
pub use running::*;
pub use won::*;
pub use lost::*;