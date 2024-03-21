//! Ce header comprend les définitions de types dont vous aurez besoin pour
//! interagir avec l'interface graphique du jeu que vous allez développer
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

#include <stdbool.h>
#include <stdint.h>

/// Par définition, on considere que la map qu'on crée dans notre jeu a une
/// dimension de 30 colonnes et 20 lignes
#define WIDTH 30

/// Par définition, on considere que la map qu'on crée dans notre jeu a une
/// dimension de 30 colonnes et 20 lignes
#define HEIGHT 20

/// Une map est constituée de 30 x 20 tuiles. Chacunes de ces tuiles peut etre
/// soit un mur, soit du sol. Il n'est possible de placer de la nourriture que
/// sur les cases de qui sont du sol. Il n'est aussi possible de se déplacer 
/// que sur des cases qui sont du sol.
#define MAP_SIZE 30*20

/// Une position représente la position d'un item sur la map. Il s'agit donc 
/// d'une position qui peut aller de {x: 0, y: 0} (coin supérieur gauche) à
/// {x: 29, y: 19} (coin inférieur droit).
struct Position {
    uint32_t x;
    uint32_t y;
};

/// Un item est tout type d'élément qui peut exister sur le plateau de jeu.
/// Au début du jeu, tous les items sont introduits à l'aide de messages 
/// te type 'spawn'.
enum Item {
    WALL     = 1, // un mur - type de tuile qui constitue un obstacle sur la carte  
    FLOOR    = 2, // du sol - type de tuile sur lesquelles on peut marcher sur la carte
    FOOD     = 3, // de la nourriture - les resources que le hero doit collecter pour gagner
    VILLAIN  = 4, // un méchant qui veut tuer le heros
    HERO     = 5, // le héros qui veut manger toute la nourriture
    POWERUP  = 6  // un power up (uniquement nécessaire si vous voulez faire le bonus)
};

/// Le type de message qui est envoyé depuis l'extérieur à notre interface de jeu
enum MessageType {
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
    /// To indicate that a player left the game (without being purposedly killed by somone)
    LEFT_GAME = 8,
};

/// Spawn est le message qui sert à introduire un item dans le jeu.
/// 
/// Tous les items ont un identifiant numérique qui leur est attaché tout au cours de la 
/// partie. Chaque item possède aussi un type d'item et une position
struct Spawn {
    /// Ce messagetype devra toujours avoir la valeur SPAWN
    enum MessageType msgt;
    /// L'identifiant unique de la ressource à introduire
    uint32_t id;
    /// Le type de l'item qu'on introduit
    enum Item item;
    /// La position sur le plateau de jeu ou cet item doit etre introduit
    struct Position pos;
};

/// Spawn est le message qui sert à introduire un item dans le jeu.
/// 
/// Tous les items ont un identifiant numérique qui leur est attaché tout au cours de la 
/// partie. Chaque item possède aussi un type d'item et une position
struct Movement {
    /// Ce messagetype devra toujours avoir la valeur MOVEMENT
    enum MessageType msgt;
    /// L'identifiant unique de l'item qui doit se déplacer sur la carte
    uint32_t id;
    /// La nouvelle position de l'item 
    struct Position pos;
};


/// Indique que le qqn a mangé de la nourriture
struct EatFood {
    /// Ce messagetype devra toujours avoir la valeur EAT_FOOD
    enum MessageType msgt;
    uint32_t eater;
    uint32_t food;
};

/// Indique que quelqu'un a tué qqn d'autre
struct Kill {
    /// Ce messagetype devra toujours avoir la valeur KILL_VICTIM
    enum MessageType msgt;
    uint32_t killer;
    uint32_t killed;
};

/// Indique que quelqu'un a quitté le jeu sans forcément avoir été tué (déconnection)
struct LeftGame {
    /// Ce messagetype devra toujours avoir la valeur LEFT_GAME
    enum MessageType msgt;
    /// Identitfiant du joueur qui a quitté le jeu
    uint32_t id;
};

/// Indique que le joueur a gagné
struct Victory {
    /// Ce messagetype devra toujours avoir la valeur VICTORY
    enum MessageType msgt;
};

/// Indique que le joueur a perdu
struct Defeat {
    /// Ce messagetype devra toujours avoir la valeur DEFEAT
    enum MessageType msgt;
};

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
struct SpecialMode {
    /// Ce messagetype devra toujours avoir la valeur SPECIAL_MODE
    enum MessageType msgt;
    uint32_t id;
    bool active;
};


/// Cette union encapsule tous les messages que vous pourriez vouloir envoyer à l'interface
/// graphique de votre jeu depuis votre programme.
union Message {
    enum MessageType msgt;
    struct Spawn spawn;
    struct Movement movement;
    struct EatFood eat_food;
    struct Kill kill_victim;
    struct Victory victory;
    struct Defeat defeat;
    struct SpecialMode special;
    struct LeftGame left_game;
};