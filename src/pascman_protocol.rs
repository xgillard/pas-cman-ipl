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
    HERO     = 5, // le héros qui veut manger toute la nourriture
    POWERUP  = 6  // un power up (uniquement nécessaire si vous voulez faire le bonus)
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
    /// To indicate that someone killed someone else
    KILL_VICTIM = 4,
     /// To indiacate that the user won the game
    VICTORY = 5,
    /// To indicate that user lost the game
    DEFEAT = 6,
    /// To indicate that a hero or villain is entering/leaving the special mode
    /// (ONLY USEFUL IF YOU IMPLEMENT THE BONUS)
    SPECIAL_MODE = 7,
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

/// Indique que quelqu'un a tué qqn d'autre
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Kill {
    /// Ce messagetype devra toujours avoir la valeur KILL_VICTIM
    pub msgt: MessageType,
    pub killer: u32,
    pub killed: u32,
}


/// **********************************************************************
/// Ce message n'est utile que si et seulement si vous voulez implémenter
/// le bonus
/// **********************************************************************
/// 
/// Active ou désactive le mode 'spécial' (super pouvoir du ou des héros).
/// 
/// Activer le mode spécial pour un héros signifie que ce heros pourra manger
/// les méchants qu'il rencontre. Activer le mode spécial pour un méchant 
/// indique simplement qu'il pourra se faire manger par un héros ayant des 
/// super pouvoirs. Concrètement, le seul effet de ce message est de modifier
/// l'aspect visuel du monstre ou du héros de sorte que le joueur sache qu'il
/// peut manger des méchants ou se faire manger par un héros.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SpecialMode {
    /// Ce messagetype devra toujours avoir la valeur SPECIAL_MODE
    pub msgt: MessageType,
    pub id: u32,
    pub active: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union Message {
    pub msgt: MessageType,
    pub spawn: Spawn,
    pub movement: Movement,
    pub eat_food: EatFood,
    pub kill_victim: Kill,
    pub victory: Victory,
    pub defeat: Defeat,
    pub special: SpecialMode,
} 