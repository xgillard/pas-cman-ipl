//! The systems are the bits of code providing the game logic. 
//! This module provides an implementation of the systems for when the game 
//! is finished and the hero has lost
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

use bracket_lib::prelude::*;
use legion::{Schedule, system};
use crate::{proceed_to_restart_system, Map};

pub fn lost_game_schedule() -> Schedule {
    Schedule::builder()
        .add_system(render_defeat_screen_system())
        .add_system(proceed_to_restart_system())
        .build()
}


#[system]
pub fn render_defeat_screen(#[resource] map: &Map) {
    let mut batch = DrawBatch::new();
    batch.target(3);
    batch.set_all_alpha(1.0, 1.0);

    let w = map.width * 2;
    let h = map.height* 2;
    
    batch.draw_box(Rect::with_size(w/4, h/4, w/2, h/2), ColorPair::new(WHITE, BLACK));

    batch.print_color_centered(h/2-2,   "You died",                 ColorPair::new(RED, BLACK));
    batch.print_color_centered(h/2 + 2, "Press any key to restart", ColorPair::new(TAN, BLACK));

    batch.submit(5_000).expect("error submitting draw batch");
}