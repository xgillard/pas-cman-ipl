//! Ce header comprend les définitions de types dont vous aurez besoin pour
//! interagir avec l'interface graphique du jeu que vous allez développer
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

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
typedef struct Position {
    uint32_t x;
    uint32_t y;
} Position;

/// Un item est tout type d'élément qui peut exister sur le plateau de jeu.
/// Au début du jeu, tous les items sont introduits à l'aide de messages 
/// te type 'spawn'.
typedef enum Item {
    WALL     = 1, // un mur - type de tuile qui constitue un obstacle sur la carte  
    FLOOR    = 2, // du sol - type de tuile sur lesquelles on peut marcher sur la carte
    FOOD     = 3, // de la nourriture - les resources que le hero doit collecter pour gagner
    VILLAIN  = 4, // un méchant qui veut tuer le heros
    HERO     = 5  // le héros qui veut manger toute la nourriture
} Item;

/// Le type de message qui est envoyé depuis l'extérieur à notre interface de jeu
typedef enum MessageType {
    /// To introduce an item in the game
    SPAWN = 1,
    /// To indicate that a given item moves on the map
    MOVEMENT = 2,
} MessageType;

/// Spawn est le message qui sert à introduire un item dans le jeu.
/// 
/// Tous les items ont un identifiant numérique qui leur est attaché tout au cours de la 
/// partie. Chaque item possède aussi un type d'item et une position
typedef struct Spawn {
    /// Ce messagetype devra toujours avoir la valeur SPAWN
    MessageType msgt;
    /// L'identifiant unique de la ressource à introduire
    uint32_t id;
    /// Le type de l'item qu'on introduit
    Item item;
    /// La position sur le plateau de jeu ou cet item doit etre introduit
    Position pos;
} Spawn;


/// La direction dans laquelle un item se déplace
typedef enum Direction {
    DOWN  = 0,
    RIGHT = 1,
    LEFT  = 2,
    UP    = 3
} Direction;

/// Spawn est le message qui sert à introduire un item dans le jeu.
/// 
/// Tous les items ont un identifiant numérique qui leur est attaché tout au cours de la 
/// partie. Chaque item possède aussi un type d'item et une position
typedef struct Movement {
    /// Ce messagetype devra toujours avoir la valeur MOVEMENT
    MessageType msgt;
    /// L'identifiant unique de l'item qui doit se déplacer sur la carte
    uint32_t id;
    /// La nouvelle position de l'item 
    Direction dir;
} Movement;

/// Cette union encapsule tous les messages que vous pourriez vouloir envoyer à l'interface
/// graphique de votre jeu depuis votre programme.
typedef union Message {
    MessageType msgt;
    Spawn spawn;
    Movement movement;
} 