#ifndef __SERVER_SHARED__
#define __SERVER_SHARED__

#include <stddef.h>
#include <sys/types.h>
#include <stdbool.h>

#include "pascman.h"

#define NB_PLAYERS 2

// Tous les éléments du jeu ont un identifiant qui peut être 
// choisi arbitrairement. Par facilité, on va opter pour le
// schéma suivant:
// - Les items de type FOOD et SUPERFOOD sont dans le range
//   (0..MAP_SIZE), ce qui veut dire qu'on peut directement 
//   convertir l'identifiant en position sur la map et vice
//   versa.
// - Les items de type WALL et FLOOR ont un identifiant dans
//   le range (MAP_SIZE, 2*MAP_SIZE) parce qu'en fait, on 
//   n'aura jamais besoin de manipuler leurs id.
// - Les itesm de type PLAYER1, PLAYER2 sont dans le range 3*MAP_SIZE
//   et 3*MAP_SIZE + 1. Ce qui permet de connaitre immédiatement
//   l'id d'un joueur, de retrouver le joueur en fonction de
//   son id.
#define PLAYER1_ID (3 * MAP_SIZE)
#define PLAYER2_ID (3 * MAP_SIZE + 1)

// Juste histoire de rendre le code plus facile à lire.
typedef int FileDescriptor;

//#############################################################################
// SHARED STATE (SHM)
//#############################################################################

// Il s'agit ici de l'état partagé par tous les processus
// qui tournent sur le server. C'est lui qui sera stocké en
// mémoire partagée.
struct GameState
{
    // Pour chaque position de la carte, on va stocker le
    // type d'item qui se trouve à la position. Les joueurs, 
    // par contre, ne sont pas stockés comme éléments de la 
    // carte: leur position est gérée à part.
    // Dans la pratique, ca nous permettra de savoir:
    // 1. Si un mouvement est possible (destionation != wall)
    // 2. Quelle food ou superfood on a mangé.
    enum Item map[MAP_SIZE];
    // Ce tableau stocke le score de chacun des deux joueurs.
    int scores[NB_PLAYERS];
    // Compte le nombre d'éléménts qui peuvent encore être mangés sur le plateau.
    int food_count;
    // Ce tableau stocke la position de chacun des deux joueurs.
    struct Position positions[NB_PLAYERS];
    // la partie est-elle en cours ou bien terminée ?
    bool game_over;
};

//#############################################################################
// INITIALISATION
//#############################################################################

// Cette réinitialise un objet GameState ce qui permet de s'assurer
// que toutes les valeurs soient correctement initialisées
// (par exemple en mettant -1 partout dans le champ 'food').
void reset_gamestate(struct GameState *state);

// Cette fonction lit la map stockée dans le fichier 'fdmap' et génère une suite
// de messages qui sont écrits l'un à la suite de lautre sur le pipe 'fdbcast'.
// 
// De plus, va peupler une structure de type GameState passée en parametre qui 
// sera utilisée pour maintenir une l'état courant du jeu.
//
// NOTE: Cette fonction ne ferme AUCUN FileDescriptor. C'est dont l'appelant
//       qui doit s'en charger.
void load_map(FileDescriptor fdmap, FileDescriptor fdbcast, struct GameState *state);

// Cette fonction ecrit le message approprié pour signifier à un client qu'il enregistré
// et qu'il peut commencer à jouer.
void send_registered(uint32_t player, FileDescriptor socket);

//#############################################################################
// COEUR DU JEU
//#############################################################################

// Cette fonction traite une commande de l'utilisateur dans son 
// intégralité. Elle calcule la position suivante du joueur, 
// modifie l'état partagé (state) et envoie les messages nécessaires
// sur le fdbcast.
//
// Par ailleurs, cette fonction renvoie 'true' si la partie est 
// terminée, false sinon.
bool process_user_command(struct GameState* state, enum Item player, enum Direction dir, FileDescriptor fdbcast);

#endif //__SERVER_SHARED__
