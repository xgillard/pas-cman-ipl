//! The systems are the bits of code providing the game logic. 
//! This module provides an implementation of the systems for when the game 
//! is finished and the hero has lost
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

use std::process::exit;

use bracket_lib::prelude::*;
use legion::{Schedule, system};
use crate::{proceed_to_restart_system, GameStatus, Map, Player};

pub fn game_over_schedule() -> Schedule {
    Schedule::builder()
        .add_system(render_gameover_screen_system())
        .add_system(proceed_to_restart_system())
        .build()
}


#[system]
pub fn render_gameover_screen(
    #[resource] map: &Map, 
    #[resource] player: &Player,
    #[resource] status: &GameStatus,
    #[resource] key: &Option<VirtualKeyCode>,
) {
    if let &GameStatus::Over { winner } = status {
        if let Some(VirtualKeyCode::Return) = key {
            exit(0);
        }

        let me = player.0;

        let mut batch = DrawBatch::new();
        batch.target(3);
        batch.set_all_alpha(1.0, 1.0);

        let w = map.width * 2;
        let h = map.height* 2;
        
        batch.draw_box(Rect::with_size(w/4, h/4, w/2, h/2), ColorPair::new(WHITE, BLACK));

        if me == winner {
            batch.print_color_centered(h/2-2, "Congratulations, you won !", ColorPair::new(YELLOW, BLACK));
        } else {
            batch.print_color_centered(h/2-2, "Too bad, you lost :( ",      ColorPair::new(RED, BLACK));
        }

        batch.print_color_centered(h/2 + 2, "Press ENTER to end", ColorPair::new(TAN, BLACK));

        batch.submit(5_000).expect("error submitting draw batch");
    }
}