//! The systems are the bits of code providing the game logic. 
//! This module provides an implementation of the systems for when the game is running
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

use std::io::{self, Write};

use bracket_lib::{color::{ColorPair, BLACK, WHITE}, terminal::{to_cp437, DrawBatch, Point}};
use crate::*;

/// This function creates the ECS schedule which decides when a given system should be run
pub fn run_game_schedule() -> Schedule {
    Schedule::builder()
        .add_system(user_input_system())
        .add_system(render_map_system())
        .flush()
        .add_system(move_to_next_place_system())
        .flush()
        .add_system(render_food_system())
        .add_system(render_characters_system())
        .flush()
        .add_system(remove_gone_system())
        .build()
}

fn _direction_to_stdout(direction: Direction) {
    let mut stdout = io::stdout();

    let dir: [u8; 4] = direction.into();
    stdout.write_all(&dir).expect("could not write to stdout");
    stdout.flush().expect("could not flush stdout");
}

/// This system deals with the user input
#[system]
pub fn user_input(
    #[resource] key: &Option<VirtualKeyCode>,
) {
    match key {
        Some(VirtualKeyCode::Left)  => { _direction_to_stdout(Direction::Left);  },
        Some(VirtualKeyCode::Right) => { _direction_to_stdout(Direction::Right); },
        Some(VirtualKeyCode::Up)    => { _direction_to_stdout(Direction::Up);    },
        Some(VirtualKeyCode::Down)  => { _direction_to_stdout(Direction::Down);  },
        _                           => {/* do nothing */ }
    }
}

/// This system renders the world map
#[system]
pub fn render_map(#[resource] map: &Map) {
    let mut drawbatch = DrawBatch::new();
    drawbatch.target(0);

    for y in 0..map.height {
        for x in 0..map.width {
            let pos   = Position{x, y};
            let glyph = match map[pos] {
                TileType::Wall  => to_cp437('0'),
                TileType::Floor => to_cp437(' '),
            };
            drawbatch.set(
                Point::new(x,y), 
                ColorPair::new(WHITE, BLACK), 
                glyph);
        }
    }

    drawbatch.submit(0).expect("draw error");
}

/// This system renders all entities in the world
#[system]
#[read_component(Food)]
#[read_component(Position)]
pub fn render_food(ecs: &SubWorld) {
    let mut batch = DrawBatch::new();
    batch.target(1);

    <(&Position, &Food)>::query()
        .iter(ecs)
        .filter(|(pos, _food)| pos.is_valid())
        .for_each(|(pos, food)| {
            batch.set(
                pos.into_point(),
                ColorPair::new(WHITE, BLACK),
                to_cp437(food.0),
            );
        });

    batch.submit(5_000).expect("draw entity error");
}

#[system]
#[read_component(Character)]
#[read_component(Position)]
#[read_component(Direction)]
pub fn render_characters(ecs: &SubWorld) {
    let mut batch = DrawBatch::new();
    batch.target(2);

    <(&Position, &Character, &Direction)>::query()
        .iter(ecs)
        .filter(|(pos, _character, _direction)| pos.is_valid())
        .for_each(|(pos, character, direction)| {
            batch.set(
                pos.into_point(),
                ColorPair::new(WHITE, BLACK),
                to_cp437(character.0[*direction as usize]),
            );
        });

    batch.submit(10_000).expect("draw entity error");
}

#[system]
#[write_component(Position)]
#[write_component(Direction)]
#[write_component(IntendsToMove)]
pub fn move_to_next_place(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
    <(Entity, &mut Position, &mut Direction, &IntendsToMove)>::query()
        .iter_mut(ecs)
        .for_each(|(entity, position, direction, intention)| {
            cmd.remove_component::<IntendsToMove>(*entity);
            let Position { x, y } = intention.0;

            if x > position.x {
                *direction = Direction::Right;
            } else if x < position.x {
                *direction = Direction::Left;
            } else if y > position.y {
                *direction = Direction::Down;
            } else if y < position.y {
                *direction = Direction::Up;
            }
            
            *position  = intention.0;
        });
}

#[system]
#[read_component(LeftGame)]
pub fn remove_gone(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
    <Entity>::query()
        .filter(component::<LeftGame>())
        .iter(ecs)
        .for_each(|entity| {
            cmd.remove(*entity)
        });
}
