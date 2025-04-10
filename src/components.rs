//! Entities are whatever property that can be attached to a given 
//! entity. Typically a position or the fact that an entity can be
//! rendered on the map are both examples of components.
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

use bracket_lib::terminal::Point;

/// Le joueur qui joue une partie.
#[derive(Debug, Clone, Copy)]
pub struct Player(pub u32);

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

    pub fn is_valid(self) -> bool {
        self.x < 30 && self.y < 20
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
pub struct Superfood;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Down = 0,
    Right= 1,
    Left = 2,
    Up   = 3
}
impl From<Direction> for [u8; 4] {
    fn from(direction : Direction) -> [u8; 4] {
        match direction {
            Direction::Down => 0_u32.to_ne_bytes(),
            Direction::Right=> 1_u32.to_ne_bytes(),
            Direction::Left => 2_u32.to_ne_bytes(),
            Direction::Up   => 3_u32.to_ne_bytes(),
        }
    }
}

/// To tell that a given entity intends to move somewhere in the game.
#[derive(Debug, Clone, Copy)]
pub struct IntendsToMove(pub Position);

#[derive(Debug, Clone, Copy)]
pub struct Id(pub u32);

pub struct EatFood{
    pub eater: u32,
    pub food: u32
}
pub struct Kill {
    pub killer: u32,
    pub killed: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct LeftGame;