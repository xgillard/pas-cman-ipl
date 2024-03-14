//! The protocol for the pas cman game. This contains the stuctures and enum
//! you will use throughout the game.
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

/// Une map est constituée de 30 x 20 tuiles. Chacunes de ces tuiles peut etre
/// soit un mur, soit du sol. Il n'est possible de placer de la nourriture que
/// sur les cases de qui sont du sol. Il n'est aussi possible de se déplacer 
/// que sur des cases qui sont du sol.
pub const MAP_SIZE: usize = 30*20; 

/// Une position représente la position d'un item sur la map. Il s'agit donc 
/// d'une position qui peut aller de {x: 0, y: 0} (coin supérieur gauche) à
/// {x: 29, y: 19} (coin inférieur droit).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

/// Un item est tout type d'élément qui peut exister sur le plateau de jeu.
/// Au début du jeu, tous les items sont introduits à l'aide de messages 
/// te type 'spawn'.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Item {
    WALL     = 1, // un mur - type de tuile qui constitue un obstacle sur la carte  
    FLOOR    = 2, // du sol - type de tuile sur lesquelles on peut marcher sur la carte
    FOOD     = 3, // de la nourriture - les resources que le hero doit collecter pour gagner
    VILLAIN  = 4, // un méchant qui veut tuer le heros
    HERO     = 5  // le héros qui veut manger toute la nourriture
}

/// Le type de message qui est envoyé depuis l'extérieur à notre interface de jeu
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    /// To introduce an item in the game
    SPAWN = 1,
    /// To indicate that a given item moves on the map
    MOVEMENT = 2,
}

/// Spawn est le message qui sert à introduire un item dans le jeu.
/// 
/// Tous les items ont un identifiant numérique qui leur est attaché tout au cours de la 
/// partie. Chaque item possède aussi un type d'item et une position
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Spawn {
    /// Ce messagetype devra toujours avoir la valeur SPAWN
    pub msgt: MessageType,
    /// L'identifiant unique de la ressource à introduire
    pub id  : u32,
    /// Le type de l'item qu'on introduit
    pub item: Item,
    /// La position sur le plateau de jeu ou cet item doit etre introduit
    pub pos : Position
}


/// La direction dans laquelle un item se déplace
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    DOWN  = 0,
    RIGHT = 1,
    LEFT  = 2,
    UP    = 3
}

/// Spawn est le message qui sert à introduire un item dans le jeu.
/// 
/// Tous les items ont un identifiant numérique qui leur est attaché tout au cours de la 
/// partie. Chaque item possède aussi un type d'item et une position
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Movement {
    /// Ce messagetype devra toujours avoir la valeur MOVEMENT
    pub msgt: MessageType,
    /// L'identifiant unique de l'item qui doit se déplacer sur la carte
    pub id:  u32,
    /// La nouvelle position de l'item 
    pub dir: Direction
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union Message {
    pub msgt: MessageType,
    pub spawn: Spawn,
    pub movement: Movement,
} 