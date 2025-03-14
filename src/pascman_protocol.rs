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
/// de type 'spawn'.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Item {
    WALL      = 1, // un mur - type de tuile qui constitue un obstacle sur la carte  
    FLOOR     = 2, // du sol - type de tuile sur lesquelles on peut marcher sur la carte
    FOOD      = 3, // de la nourriture - les resources que les joueures doivent collecter pour gagner
    SUPERFOOD = 4, // de la superfood qui rapporte plus de points que la nourriture normale
    PLAYER1   = 5, // le joueur 1
    PLAYER2   = 6, // le joueur 1
}

/// Le type de message qui est envoyé depuis l'extérieur à notre interface de jeu
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum MessageType {
    /// To introduce an item in the game
    SPAWN = 1,
    /// To indicate that a given item moves on the map
    MOVEMENT = 2,
    /// To indicate that someone ate some food
    EAT_FOOD = 3,
    /// To indicate that two players were killed because they collided into one another
    COLLISION = 4,
     /// To indiacate that the user won the game
    VICTORY = 5,
    /// To indicate that user lost the game
    DEFEAT = 6,
    /// To indicate that a player left the game (without being purposedly killed by somone)
    LEFT_GAME = 8,
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
    pub pos: Position
}


/// Indique que le joueur a gagné
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Victory {
    /// Ce messagetype devra toujours avoir la valeur VICTORY
    pub msgt: MessageType,
}

/// Indique que le joueur a perdu
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Defeat {
    /// Ce messagetype devra toujours avoir la valeur DEFEAT
    pub msgt: MessageType,
}

/// Indique que le qqn a mangé de la nourriture
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EatFood {
    /// Ce messagetype devra toujours avoir la valeur EAT_FOOD
    pub msgt: MessageType,
    pub eater: u32,
    pub food: u32,
}

/// Indique que deux joueurs sont morts parce qu'ils sont entrés en collision
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Collision {
    /// Ce messagetype devra toujours avoir la valeur COLLISION
    pub msgt: MessageType,
    pub player_a: u32,
    pub player_b: u32,
}

/// Indique que quelqu'un a quitté le jeu sans forcément avoir été tué (déconnection)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LeftGame {
    /// Ce messagetype devra toujours avoir la valeur LEFT_GAME
    pub msgt: MessageType,
    /// Identitfiant du joueur qui a quitté le jeu
    pub id: u32,
}


#[repr(C)]
#[derive(Clone, Copy)]
pub union Message {
    pub msgt: MessageType,
    pub spawn: Spawn,
    pub movement: Movement,
    pub eat_food: EatFood,
    pub collision: Collision,
    pub victory: Victory,
    pub defeat: Defeat,
    pub left_game: LeftGame,
} 