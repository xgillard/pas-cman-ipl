//! Entities are whatever property that can be attached to a given 
//! entity. Typically a position or the fact that an entity can be
//! rendered on the map are both examples of components.
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

use std::time::Instant;

use bracket_lib::{color::ColorPair, terminal::Point};

/// This component indicates that the entity is a character 
/// (they should be rendered on top of both the map and the food)
#[derive(Debug, Clone, Copy)]
pub struct Character(pub &'static [char; 4]);

/// This is going to be our action hero (aka the pizza guy, aka
/// the main character w/ which you usually play on old arcade).
/// The goal of the hero in the game is to eat all of the available
/// seeds that have been laid you on the ground. 
/// The hero also has the ability to eat some magic powerups which 
/// have the side effect of turinging all the villains into edible
/// stuff for a limited period of time.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hero;

/// Tese are the villains of the game. They are trying to go eat 
/// the hero. When the hero eat magic powerups, then the villain
/// are made edible. If the hero collides with a villain as it is
/// edible, then the hero eats the villain. When the villain is 
/// not edible but a collision happens between the hero and a 
/// villain, then it is the villain who eats the hero.
/// 
/// When the villain is eated by the hero, it is respawned somewhere
/// on the map. When the hero gets killed it is likewise also respawned 
/// somewhere on the map.
#[derive(Debug, Clone, Copy)]
pub struct Villain;

/// The hunter has the ability to kill victims
#[derive(Debug, Clone, Copy)]
pub struct Hunter;

/// The victim is being killed whenever it encounters a hunter
#[derive(Debug, Clone, Copy)]
pub struct Victim;

/// The role indicates whether the entity needs to behave as a hunter
/// or as a victim.
#[derive(Debug, Clone, Copy)]
pub enum Role {
    Hunter,
    Victim
}

/// This component indicates that the entity will change role at a given instant
#[derive(Debug, Clone, Copy)]
pub struct SwapRole {
    pub add: Role,
    pub remove: Role,
    pub color: ColorPair,
}

/// The position of an entity on the map
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn into_point(self) -> Point {
        Point::new(self.x, self.y)
    }
}

/// This are the stuffs on the floor which the hero is trying to eat
/// as much of as it possibly can before being eated by a villain.
#[derive(Debug, Clone, Copy)]
pub struct Food(pub char);

/// This is a magic powerup. It can also be eated by the hero just like
/// the regular food. However, whenever the hero eats a powerup, all
/// the villains are made edible for a short (a few seconds) period of
/// time.
#[derive(Debug, Clone, Copy)]
pub struct Powerup;

/// The entity wanders erratically on the map
#[derive(Debug, Clone, Copy)]
pub struct RandomWalk{
    pub time: Instant
}

/// The entity is blood thirsty and will walk the shortest path to the hero
#[derive(Debug, Clone, Copy)]
pub struct SmartBot {
    pub time: Instant
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Down = 0,
    Right= 1,
    Left = 2,
    Up   = 3
}

/// 
#[derive(Debug, Clone, Copy)]
pub struct IntendsToMove(pub Direction);

#[derive(Debug, Clone, Copy)]
pub struct DelayedSwapRole {
    pub time: Instant,
    pub swap: SwapRole,
}

#[derive(Debug, Clone, Copy)]
pub struct Dead;