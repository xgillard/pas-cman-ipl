//! The systems are the bits of code providing the game logic. 
//! This module provides an implementation of the systems for when the game is running
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

use std::time::{Duration, Instant};

use bracket_lib::{color::{ColorPair, BLACK, WHITE}, terminal::{to_cp437, DrawBatch, Point, VirtualKeyCode}};
use crate::*;

/// Duration of a powerup
const POWERUP_DURATION : u64 = 3;

/// Time between 2 random walks (ms)
const TIME_BETWEEN_RWALKS : u64 = 760;

/// Time between 2 smart moves(ms)
const TIME_BETWEEN_SMARTMOVES : u64 = 250;

/// Maximum search depth with a*
const MAX_SEARCH_DEPTH: f32 = 25.0;

/// This function creates the ECS schedule which decides when a given system should be run
pub fn run_game_schedule() -> Schedule {
    Schedule::builder()
        //.add_system(render_map_system())
        .add_system(user_input_system())
        .add_system(random_walk_system())
        .add_system(smart_hunter_system())
        .add_system(smart_victims_system())
        .flush()
        .add_system(move_to_next_place_system())
        .flush()
        .add_system(eat_food_system())
        .add_system(kill_victim_system())
        .add_system(eat_powerup_system())
        .flush()
        .add_system(swap_roles_system())
        .add_system(render_food_system())
        .add_system(render_characters_system())
        .add_system(delayed_swaps_system())
        .flush()
        .add_system(remove_dead_system())
        .flush()
        .add_system(has_lost_system())
        .add_system(has_won_system())
        .build()
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
#[read_component(ColorPair)]
#[read_component(Direction)]
pub fn render_characters(ecs: &SubWorld) {
    let mut batch = DrawBatch::new();
    batch.target(2);

    <(&Position, &Character, &Direction, &ColorPair)>::query()
        .iter(ecs)
        .for_each(|(pos, character, direction, color)| {
            batch.set(
                pos.into_point(),
                *color,
                to_cp437(character.0[*direction as usize]),
            );
        });

    batch.submit(10_000).expect("draw entity error");
}

/// This system deals with the user input
#[system]
#[read_component(Hero)]
pub fn user_input(
    ecs: &mut SubWorld,
    #[resource] key: &Option<VirtualKeyCode>,
    cmd: &mut CommandBuffer
) {
    if let Some(key) = key {
        let direction = match key {
            VirtualKeyCode::Left  => Direction::Left,
            VirtualKeyCode::Right => Direction::Right,
            VirtualKeyCode::Up    => Direction::Up,
            _                     => Direction::Down,
        };

        <Entity>::query()
            .filter(component::<Hero>())
            .iter(ecs)
            .for_each(|entity| cmd.add_component(*entity, IntendsToMove(direction)));
    }
}


#[system]
#[write_component(RandomWalk)]
#[write_component(IntendsToMove)]
pub fn random_walk(
        ecs: &mut SubWorld, 
        cmd: &mut CommandBuffer,
        #[resource] rng: &mut RandomNumberGenerator,
    ) {
    let now = Instant::now();
    let next= now + Duration::from_millis(TIME_BETWEEN_RWALKS);

    <(Entity, &mut RandomWalk)>::query()
        .iter_mut(ecs)
        .for_each(|(entity, rwalk)| {
            if rwalk.time <= now {
                let choice = rng.roll_dice(1, 4);
                let direction   = match choice {
                    1 => Direction::Down,
                    2 => Direction::Right,
                    3 => Direction::Left,
                    _ => Direction::Up,
                };

                cmd.add_component(*entity, IntendsToMove(direction));
                rwalk.time = next;   
            }
        })
}


#[system]
#[read_component(Position)]
#[read_component(Hunter)]
#[read_component(Victim)]
#[write_component(SmartBot)]
#[write_component(IntendsToMove)]
pub fn smart_hunter(
        ecs: &mut SubWorld, 
        cmd: &mut CommandBuffer,
        #[resource] map: &Map,
    ) {
    
    let now = Instant::now();
    let next= now + Duration::from_millis(TIME_BETWEEN_SMARTMOVES);

    let victims = <&Position>::query()
        .filter(component::<Victim>())
        .iter(ecs)
        .map(|&Position { x, y }| map.point2d_to_index(Point::new(x, y)))
        .collect::<Vec<_>>();
    
    let dijkstra = DijkstraMap::new(
                                    map.width, 
                                    map.height, 
                                    &victims, 
                                    map, 
                                    MAX_SEARCH_DEPTH);
    
    <(Entity, &Position, &mut SmartBot)>::query()
        .filter(component::<Hunter>())
        .iter_mut(ecs)
        .for_each(|(entity, &Position { x, y }, thirst)| {
            let idx = map.point2d_to_index(Point::new(x, y));
            if thirst.time <= now {
                if let Some(dest) = DijkstraMap::find_lowest_exit(&dijkstra, idx, map) {
                    let dest = map.index_to_point2d(dest);
                    if dest.x < x as i32 {
                        cmd.add_component(*entity, IntendsToMove(Direction::Left));
                    } else if dest.x > x as i32 {
                        cmd.add_component(*entity, IntendsToMove(Direction::Right));
                    } else if dest.y < y as i32 {
                        cmd.add_component(*entity, IntendsToMove(Direction::Up));
                    } else if dest.y > y as i32 {
                        cmd.add_component(*entity, IntendsToMove(Direction::Down));
                    }
                }
                thirst.time = next;   
            }
        })
}


#[system]
#[read_component(Position)]
#[read_component(Hunter)]
#[read_component(Victim)]
#[write_component(SmartBot)]
#[write_component(IntendsToMove)]
pub fn smart_victims(
        ecs: &mut SubWorld, 
        cmd: &mut CommandBuffer,
        #[resource] map: &Map,
    ) {
    
    let now = Instant::now();
    let next= now + Duration::from_millis(TIME_BETWEEN_SMARTMOVES);

    let hunters = <&Position>::query()
        .filter(component::<Hunter>())
        .iter(ecs)
        .map(|&Position { x, y }| map.point2d_to_index(Point::new(x, y)))
        .collect::<Vec<_>>();
    
    let dijkstra = DijkstraMap::new(
                                    map.width, 
                                    map.height, 
                                    &hunters, 
                                    map, 
                                    MAX_SEARCH_DEPTH);
    
    <(Entity, &Position, &mut SmartBot)>::query()
        .filter(component::<Victim>())
        .iter_mut(ecs)
        .for_each(|(entity, &Position { x, y }, thirst)| {
            let idx = map.point2d_to_index(Point::new(x, y));
            if thirst.time <= now {
                if let Some(dest) = DijkstraMap::find_highest_exit(&dijkstra, idx, map) {
                    let dest = map.index_to_point2d(dest);
                    if dest.x < x as i32 {
                        cmd.add_component(*entity, IntendsToMove(Direction::Left));
                    } else if dest.x > x as i32 {
                        cmd.add_component(*entity, IntendsToMove(Direction::Right));
                    } else if dest.y < y as i32 {
                        cmd.add_component(*entity, IntendsToMove(Direction::Up));
                    } else if dest.y > y as i32 {
                        cmd.add_component(*entity, IntendsToMove(Direction::Down));
                    }
                }
                thirst.time = next;   
            }
        })
}


#[system]
#[write_component(Position)]
#[write_component(Direction)]
#[write_component(IntendsToMove)]
pub fn move_to_next_place(ecs: &mut SubWorld, cmd: &mut CommandBuffer, #[resource] map: &Map) {
    <(Entity, &mut Position, &mut Direction, &IntendsToMove)>::query()
        .iter_mut(ecs)
        .for_each(|(entity, position, direction, intention)| {
            cmd.remove_component::<IntendsToMove>(*entity);
            *direction = intention.0;
            *position  = next_position(map, *position, intention.0);
        });
}

#[system]
#[read_component(Position)]
#[read_component(Hero)]
#[read_component(Food)]
pub fn eat_food(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
    let mut hero = Position::default();
    <&Position>::query()
        .filter(component::<Hero>())
        .iter(ecs)
        .for_each(|p| hero = *p);

    <(Entity, &Position)>::query()
        .filter(component::<Food>())
        .iter(ecs)
        .for_each(|(entity, pos)| 
            if *pos == hero { 
                cmd.add_component(*entity, Dead); 
            });
}

#[system]
#[read_component(Position)]
#[read_component(Powerup)]
#[read_component(Hero)]
#[read_component(Villain)]
#[write_component(DelayedSwapRole)]
pub fn eat_powerup(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
    let hero = <&Position>::query()
        .filter(component::<Hero>())
        .iter(ecs)
        .next();

    if let Some(hero) = hero {
        let villain_as_food = <&Position>::query()
            .filter(component::<Powerup>())
            .iter(ecs)
            .any(|pos| pos == hero);
        
        if villain_as_food {
            // after superfood, the hero is no longer a victim
            <Entity>::query()
                .filter(component::<Hero>())
                .iter_mut(ecs)
                .for_each(|entity| {
                    cmd.add_component(*entity, SwapRole{
                        add: Role::Hunter, 
                        remove: Role::Victim,
                        color: ColorPair::new(RED, BLACK)
                    });
                    cmd.add_component(*entity, DelayedSwapRole {
                        time: Instant::now() + Duration::from_secs(POWERUP_DURATION),
                        swap: SwapRole{
                            add: Role::Victim, 
                            remove: Role::Hunter,
                            color: ColorPair::new(WHITE, BLACK)
                        }
                    });
                });
            
            // after superfood, the villains are no longer hunters
            <Entity>::query()
                .filter(component::<Villain>())
                .iter_mut(ecs)
                .for_each(|entity| {
                    cmd.add_component(*entity, SwapRole {
                        add: Role::Victim, 
                        remove: Role::Hunter,
                        color: ColorPair::new(RED, BLACK)
                    });
                    cmd.add_component(*entity, DelayedSwapRole {
                        time: Instant::now() + Duration::from_secs(POWERUP_DURATION),
                        swap: SwapRole{
                            add: Role::Hunter, 
                            remove: Role::Victim,
                            color: ColorPair::new(WHITE, BLACK)
                        }
                    });
                });
        }
    }
}

#[system]
#[read_component(Position)]
#[read_component(Hunter)]
#[read_component(Victim)]
pub fn kill_victim(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
    <&Position>::query()
        .filter(component::<Hunter>())
        .iter(ecs)
        .for_each(|hunter| {
            <(Entity, &Position)>::query()
                .filter(component::<Victim>())
                .iter(ecs)
                .for_each(|(entity, victim)| {
                    if hunter == victim {
                        cmd.add_component(*entity, Dead);
                    }
                })
        });
}

#[system]
#[write_component(SwapRole)]
#[write_component(Hunter)]
#[write_component(Victim)]
#[write_component(ColorPair)]
pub fn swap_roles(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
    <(Entity, &SwapRole, &mut ColorPair)>::query()
        .iter_mut(ecs)
        .for_each(|(entity, swap, color)| {
            match swap.remove {
                Role::Victim => cmd.remove_component::<Victim>(*entity),
                Role::Hunter => cmd.remove_component::<Hunter>(*entity)
            }

            match swap.add {
                Role::Victim => cmd.add_component(*entity, Victim),
                Role::Hunter => cmd.add_component(*entity, Hunter)
            }

            *color = swap.color;
            cmd.remove_component::<SwapRole>(*entity);
        });
}

#[system]
#[write_component(DelayedSwapRole)]
#[write_component(SwapRole)]
pub fn delayed_swaps(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
    let now = Instant::now();

    <(Entity, &DelayedSwapRole)>::query()
        .iter_mut(ecs)
        .for_each(|(entity, action)| {
            if action.time < now {
                cmd.add_component(*entity, action.swap);
                cmd.remove_component::<DelayedSwapRole>(*entity);
            }
        });
}

#[system]
#[read_component(Dead)]
pub fn remove_dead(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
    <Entity>::query()
        .filter(component::<Dead>())
        .iter(ecs)
        .for_each(|entity| cmd.remove(*entity));
}

#[system]
#[read_component(Hero)]
pub fn has_lost(ecs: &mut SubWorld, #[resource] status: &mut GameStatus) {
    let cnt = <Entity>::query()
        .filter(component::<Hero>())
        .iter(ecs)
        .count();
    if cnt == 0 {
        *status = GameStatus::Lost;
    }
}
#[system]
#[read_component(Food)]
pub fn has_won(ecs: &mut SubWorld, #[resource] status: &mut GameStatus) {
    let cnt = <Entity>::query()
        .filter(component::<Food>())
        .iter(ecs)
        .count();
    if cnt == 0 {
        *status = GameStatus::Won;
    }
}
