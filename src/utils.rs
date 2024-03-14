//! This module only comprises a bunch of utility functions
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

use bracket_lib::terminal::Point;

use crate::{Direction, Map, Position};

/// Returns the next position after the movement has been applied. If the movement is not
/// legal, then the position is simply not updated
pub fn next_position(map: &Map, curr: Position, direction: Direction) -> Position {
    let mut x = curr.x as isize;
    let mut y = curr.y as isize;

    match direction {
        Direction::Up    => y -= 1,
        Direction::Down  => y += 1,
        Direction::Left  => x -= 1,
        Direction::Right => x += 1,
    };

    if !map.can_enter(Point::new(x, y)) {
        curr
    } else {
        Position {x: x as usize, y: y as usize}
    }
}