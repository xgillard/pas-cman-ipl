#ifndef __PASCMAN__
#define __PASCMAN__
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
#define MAP_SIZE (30*20)

/// Lorsqu'un utilisateur utilisera les flèches de son clavier au sein de
/// l'interface graphique, celle-ci écrira une direction (haut, bas, gauche, droite)
/// sur la sortie standard. De cette façon, vous pourrez toujours savoir ce que
/// l'utilisateur voulait faire meme si ce n'est pas vous qui avez programmé 
/// les interactions clavier.
enum Direction {
    DOWN  = 0,
    RIGHT = 1,
    LEFT  = 2,
    UP    = 3
};

/// Une position représente la position d'un item sur la map. Il s'agit donc 
/// d'une position qui peut aller de {x: 0, y: 0} (coin supérieur gauche) à
/// {x: 29, y: 19} (coin inférieur droit).
struct Position {
    uint32_t x;
    uint32_t y;
};

/// Un item est tout type d'élément qui peut exister sur le plateau de jeu.
/// Au début du jeu, tous les items sont introduits à l'aide de messages 
/// de type 'spawn'.
enum Item {
    WALL      = 1, // un mur - type de tuile qui constitue un obstacle sur la carte  
    FLOOR     = 2, // du sol - type de tuile sur lesquelles on peut marcher sur la carte
    FOOD      = 3, // de la nourriture - les resources que les joueures doivent collecter pour gagner
    SUPERFOOD = 4, // de la superfood qui rapporte plus de points que la nourriture normale
    PLAYER1   = 5, // le joueur 1
    PLAYER2   = 6, // le joueur 1
};

/// Le type de message qui est envoyé depuis l'extérieur à notre interface de jeu
enum MessageType {
    /// To tell the system that you've been registered with the server.
    REGISTRATION = 0,
    /// To introduce an item in the game
    SPAWN = 1,
    /// To indicate that a given item moves on the map
    MOVEMENT = 2,
    /// To indicate that someone ate some food
    EAT_FOOD = 3,
    /// To tell that the game is over
    GAME_OVER = 4,
};


/// Registration est le message qui sert à dire au jeu qu'on est un joueur en particulier.
struct Registration {
    /// Ce messagetype devra toujours avoir la valeur REGISTRATION
    enum MessageType msgt;
    /// L'identifiant du joueur
    uint32_t player;
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

/// Indique que le jeu est fini.
struct GameOver {
    /// Ce messagetype devra toujours avoir la valeur GAME_OVER
    enum MessageType msgt;
    uint32_t winner;
};

/// Cette union encapsule tous les messages que vous pourriez vouloir envoyer à l'interface
/// graphique de votre jeu depuis votre programme.
union Message {
    enum MessageType msgt;
    struct Registration registration;
    struct Spawn spawn;
    struct Movement movement;
    struct EatFood eat_food;
    struct GameOver game_over;
};

#endif //__PASCMAN__