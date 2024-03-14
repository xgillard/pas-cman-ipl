//! The systems are the bits of code providing the game logic. 
//! This module provides an implementation of the systems for when the game 
//! is finished and the hero has won
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

use bracket_lib::prelude::*;
use legion::system;

use crate::GameStatus;

#[system]
pub fn proceed_to_restart(#[resource] key: &Option<VirtualKeyCode>, #[resource] status: &mut GameStatus) {
    if key.is_some() {
        *status = GameStatus::NotStarted;
    }
}