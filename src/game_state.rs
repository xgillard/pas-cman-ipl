//! The game state is the shared state that needs to be updated 
//! upon each game 'tick'. This is typically the data which you
//! will store in the shared memory as you implement your multi-player
//! version of the game.
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

use std::sync::mpsc::Receiver;

use legion::{world::World, Resources, Schedule};
use crate::{pascman_protocol::Item, *};

use self::pascman_protocol::MessageType;

#[derive(Debug, Clone, Copy)]
pub enum GameStatus {
    NotStarted,
    Running,
    Lost,
    Won
}

pub struct State {
    pub ecs: World,
    pub resources: Resources,
    pub running: Schedule,
    pub won: Schedule,
    pub lost: Schedule,
    pub map_file: String,
}

impl State {
    pub fn new(channel: std::sync::mpsc::Receiver<pascman_protocol::Message>) -> Self {
        let ecs = World::default();
        let running = run_game_schedule();
        let won = won_game_schedule();
        let lost= lost_game_schedule();
        let mut resources = Resources::default();
        let rng = RandomNumberGenerator::new();
        resources.insert(rng);
        resources.insert(GameStatus::Running);
        resources.insert(Map{width: 30, height: 20, tiles: vec![TileType::Floor;30*20] });
        resources.insert(channel);
        Self { ecs, resources, running, won, lost, map_file: String::new() }
    }

    fn process_message(ecs: &mut World, resources: &Resources, msg: pascman_protocol::Message) {
        unsafe {
            match msg.msgt {
                MessageType::SPAWN => {
                    let spawn = msg.spawn;
                    match spawn.item {
                        Item::FLOOR   => {
                            let mut map = resources.get_mut::<Map>();
                            let map = map.as_deref_mut().unwrap();
                            let idx = map.point2d_to_index(Point::new(spawn.pos.x, spawn.pos.y));
                            map.tiles[idx] = TileType::Floor;
                        },
                        Item::WALL    => {
                            let mut map = resources.get_mut::<Map>();
                            let map = map.as_deref_mut().unwrap();
                            let idx = map.point2d_to_index(Point::new(spawn.pos.x, spawn.pos.y));
                            map.tiles[idx] = TileType::Wall;
                        },
                        Item::FOOD    => {
                            spawn_seed(ecs, spawn.id, Position { x: spawn.pos.x as usize, y: spawn.pos.y as usize});
                        },
                        Item::HERO    => {
                            spawn_hero(ecs, spawn.id, Position { x: spawn.pos.x as usize, y: spawn.pos.y as usize});
                        },
                        Item::VILLAIN => {
                            spawn_villain(ecs, spawn.id, Position { x: spawn.pos.x as usize, y: spawn.pos.y as usize});
                        },
                    }
                },
                MessageType::MOVEMENT => {
                    let mvmt = msg.movement;
                    let dir = match mvmt.dir {
                        pascman_protocol::Direction::DOWN => Direction::Down,
                        pascman_protocol::Direction::LEFT => Direction::Left,
                        pascman_protocol::Direction::RIGHT=> Direction::Right,
                        pascman_protocol::Direction::UP   => Direction::Up
                    };
                    let entity = <(Entity, &Id)>::query()
                        .iter(ecs)
                        .find(|(_entity, id)| id.0 == mvmt.id)
                        .map(|(entity, _)| *entity);

                    if let Some(entity) = entity {
                        if let Some(mut entry) = ecs.entry(entity) {
                            entry.add_component(IntendsToMove(dir));
                        }
                    }
                }
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
        ctx.set_active_console(0); // the map
        ctx.cls();
        ctx.set_active_console(1); // food
        ctx.cls();
        ctx.set_active_console(2); // characters
        ctx.cls();
        ctx.set_active_console(3); // message
        ctx.cls();
        ctx.set_all_alpha(0.0, 0.0); // by default the message console is transparent

        { // fetch messages
            let resources = &self.resources;
            let mut rx = resources.get_mut::<Receiver<pascman_protocol::Message>>();
            let rx = rx.as_deref_mut().unwrap();
            while let Ok(msg) = rx.try_recv() {
                Self::process_message(&mut self.ecs, resources, msg);
            }
        }

        let status = self.resources.get::<GameStatus>().as_deref().copied().unwrap();
        match status {
            GameStatus::NotStarted => {
                self.ecs.clear();
                self.resources.insert(GameStatus::Running);
            },
            GameStatus::Running => {
                self.running.execute(&mut self.ecs, &mut self.resources)},
            GameStatus::Lost => 
                self.lost.execute(&mut self.ecs, &mut self.resources),
            GameStatus::Won => 
                self.won.execute(&mut self.ecs, &mut self.resources),
        }
        // 
        
        // effectively draw everything on screen (in batch to be more efficient)
        render_draw_buffer(ctx).expect("could not render");
    }
}