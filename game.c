#include <stdlib.h>
#include <string.h>

#include "utils_v1.h"

#include "game.h"

// Renvoie le début du range d'id pour ce type d'items.
static uint32_t __base_id(enum Item item) {
    switch (item) {
    case FOOD:
    case SUPERFOOD:
        return 0;
    case WALL:
    case FLOOR:
        return MAP_SIZE;
    case PLAYER1:
    case PLAYER2:
        return 3*MAP_SIZE;
    default:
        perror("The given item type is invalid");
        exit(EXIT_FAILURE);
    }
}

// Cette fonction utilitaire permet de connaitre l'identifiant 
// d'une resource qui se trouve à une position donnée sur la carte.
int32_t id_at(struct Position pos, enum Item item) {
    return id(pos.x, pos.y, item);
}

// Cette fonction utilitaire permet de connaitre l'identifiant 
// d'une resource qui se trouve à une position donnée sur la carte.
int32_t id(uint32_t x, uint32_t y, enum Item item) {
    if (item == PLAYER1) {
        return PLAYER1_ID;
    }
    if (item == PLAYER2) {
        return PLAYER2_ID;
    }
    return __base_id(item) + (y * WIDTH + x);
}
// Cette fonction utilitaire permet de connaitre l'offset d'une 
// position dans la carte.
size_t position2index(struct Position pos) {
    return (size_t) (pos.y * WIDTH + pos.x);
}

// Cette réinitialise un objet GameState ce qui permet de s'assurer
// que toutes les valeurs soient correctement initialisées
// (par exemple en mettant -1 partout dans le champ 'food').
void reset_gamestate(struct GameState *state) {
    state->game_over = true;
    state->food_count= 0;
    if(!memset(state->map, 0, sizeof(state->map))) {
        perror("memset map:");
        exit(EXIT_FAILURE);
    }
    if(!memset(state->positions, 0, sizeof(state->positions))) {
        perror("memset positions:");
        exit(EXIT_FAILURE);
    }
    if(!memset(state->scores, 0, sizeof(state->scores))) {
        perror("memset scores:");
        exit(EXIT_FAILURE);
    }
}

/* Cette fonction lit la map stockée dans le fichier 'resources/map.txt' et génère une suite
 * de messages qui sont écrits l'un à la suite de lautre sur la sortie standard du programme.
 * 
 * De plus, va peupler une structure de type GameState passée en parametre que vous pouvez
 * utiliser pour maintenir une copie l'état courant du jeu.
 */
void load_map(FileDescriptor fdmap, FileDescriptor fdbcast, struct GameState *state) {
    reset_gamestate(state);

    size_t pos  = 0;
    uint32_t x  = 0;
    uint32_t y  = 0;
    char c      = '\0';
    while(sread(fdmap, &c, sizeof(char)) > 0) {
        // on a lu tout le fichier en une fois, maintenant on peut le parcourir charactere par
        // charactere pour voir creer les messages nécessaires à dessiner la map. 
        // - Lorsqu'on rencontrera un caractere '#' on ajoutera un mur
        // - Lorsqu'on rencontrera un caractere '.' on ajoutera un tuile de sol et de la nourriture
        // - Lorsqu'on rencontrera un caractere '*' on ajoutera un tuile de sol et de la superfood
        // - Lorsqu'on rencontrera un caractere ' ' on ajoutera uniquement une tuile de sol.
        // - Lorsqu'on rencontrera un caractere '@' on injectera le 1er joueur
        // - Lorsqu'on rencontrera un caractere '!' on injectera le 2nd joueur
        switch (c) {
            case '#': 
                send_spawn_item(x, y, WALL, fdbcast);
                state->map[pos] = WALL;
                x++;
                pos++;
                break;
            case '.':
                send_spawn_item(x, y, FLOOR, fdbcast);
                send_spawn_item(x, y, FOOD, fdbcast);
                state->map[pos] = FOOD;
                state->food_count++;
                x++;
                pos++;
                break;
            case '*':
                send_spawn_item(x, y, FLOOR, fdbcast);
                send_spawn_item(x, y, SUPERFOOD, fdbcast);
                state->map[pos] = SUPERFOOD;
                state->food_count++;
                x++;
                pos++;
                break;
            case ' ':
                send_spawn_item(x, y, FLOOR, fdbcast);
                state->map[pos] = FLOOR;
                x++;
                pos++;
                break;
            case '@':
                send_spawn_item(x, y, PLAYER1, fdbcast); // player 1
                send_spawn_item(x, y, FLOOR, fdbcast);
                state->map[pos] = FLOOR;
                state->positions[0].x = x;
                state->positions[0].y = y;
                x++;
                pos++;
                break;
            case '!':
                send_spawn_item(x, y, PLAYER2, fdbcast); // player 2
                send_spawn_item(x, y, FLOOR, fdbcast);
                state->map[pos] = FLOOR;
                state->positions[1].x = x;
                state->positions[1].y = y;
                x++;
                pos++;
                break;
            case '\n':
                y ++;
                x = 0;
                break;
            default:
                // par défaut on ne fait simplement rien
                break;
        }
    }

    if (state->food_count == 0) {
        state->game_over = true;
        send_game_over(PLAYER1, fdbcast);
    } else {
        state->game_over = false;
    }
}

// Cette fonction ecrit le message approprié pour signifier à un client qu'il est
void send_registered(uint32_t player, FileDescriptor socket) {
    union Message msg = {
        .registration = {
            .msgt = REGISTRATION,
            .player   = player
        }
    };

    swrite(socket, &msg, sizeof(union Message));
}

// Cette fonction ecrit le message approprié pour signifier aux clients qu'une 
// resource donnée est introduite dans le jeu.
void send_spawn_item(uint32_t x, uint32_t y, enum Item item, FileDescriptor fdbcast) {
    union Message msg = {
        .spawn = {
            .msgt = SPAWN,
            .id   = id(x, y, item),
            .item = item,
            .pos  = {
                .x = x,
                .y = y
            }
        }
    };

    swrite(fdbcast, &msg, sizeof(union Message));
}

// Cette fonction ecrit le message approprié pour signifier aux clients qu'un 
// des joueurs a bougé sur le plateau de jeu.
void send_player_moved(enum Item player, struct Position to, FileDescriptor fdbcast) {
    union Message msg = {
        .movement = {
            .msgt = MOVEMENT,
            .id   = id_at(to, player),
            .pos  = to
        }
    };
    swrite(fdbcast, &msg, sizeof(union Message));
}

// Cette fonction ecrit le message approprié pour signifier aux clients que
// de la nourriture ou superfood a été mangée par un joueur.
void send_eat_food(enum Item player, enum Item food, struct Position to, FileDescriptor fdbcast) {
    union Message msg = {
        .eat_food = {
            .msgt  = EAT_FOOD,
            .eater = id_at(to, player),
            .food  = id_at(to, food),
        }
    };

    swrite(fdbcast, &msg, sizeof(union Message));
}

// Cette fonction ecrit le message approprié pour signifier aux clients que
// la partie est terminée.
void send_game_over(enum Item winner, FileDescriptor fdbcast) {
    union Message msg = {
        .game_over = {
            .msgt   = GAME_OVER,
            .winner = winner == PLAYER1 ? 1 : 2
        }
    };
    swrite(fdbcast, &msg, sizeof(union Message));
}

// Cette fonction renvoie la prochaine position du joueur après
// avoir traité le déplacement dans la direction 'dir'. Il est
// important de noter que la position renvoyée peut être impossible
//  à atteindre.
static struct Position __next_position(struct Position pos, enum Direction dir) {
    struct Position next = pos;
    switch (dir) {
    case UP:
        if (next.y > 0) {
            next.y -= 1;
        }
        break;
    case DOWN:
        if (next.y < HEIGHT-1) {
            next.y += 1;
        }
        break;
    case LEFT:
        if (next.x > 0) {
            next.x -= 1;
        }
        break;
    case RIGHT:
        if (next.x < WIDTH -1) {
            next.x += 1;
        }
        break;
    }
    return next;
}

// Cette fonction traite une commande de l'utilisateur dans son 
// intégralité. Elle calcule la position suivante du joueur, 
// modifie l'état partagé (state) et envoie les messages nécessaires
// sur le fdbcast.
//
// Par ailleurs, cette fonction renvoie 'true' si la partie est 
// terminée, false sinon.
bool process_user_command(struct GameState* state, enum Item player, enum Direction dir, FileDescriptor fdbcast) {
    if (state->game_over) {
        enum Item winner = state->scores[0] > state->scores[1] ? PLAYER1 : PLAYER2;
        send_game_over(winner, fdbcast);
        return true;
    }

    size_t player_offset  = player == PLAYER1 ? 0 : 1;
    struct Position next  = __next_position(state->positions[player_offset], dir);
    struct Position other = state->positions[(player_offset + 1) % 2];

    // Si l'autre joueur se trouve sur la case destination, le jeu est fini.
    if (next.x == other.x && next.y == other.y) {
        state->game_over = true;
        enum Item winner = state->scores[0] > state->scores[1] ? PLAYER1 : PLAYER2;
        send_game_over(winner, fdbcast);
        return true;
    }

    // La partie n'est pas finie, il faut mettre l'état à jour et envoyer une série de messages.
    size_t next_offset = position2index(next);
    enum Item at_next  = state->map[next_offset];
    switch (at_next) {
    case FLOOR:
        state->positions[player_offset] = next;
        send_player_moved(player, next, fdbcast);
        break;
    case FOOD:
        state->map[next_offset] = FLOOR;
        state->positions[player_offset] = next;
        state->scores[player_offset] += 1;
        state->food_count --;
        if (state->food_count == 0) {
            state->game_over = true;
        }
        send_player_moved(player, next, fdbcast);
        send_eat_food(player, at_next, next, fdbcast);
        break;
    case SUPERFOOD:
        state->map[next_offset] = FLOOR;
        state->positions[player_offset] = next;
        state->scores[player_offset] += 17;
        state->food_count --;
        if (state->food_count == 0) {
            state->game_over = true;
        }
        send_player_moved(player, next, fdbcast);
        send_eat_food(player, at_next, next, fdbcast);
        break;
    default:
        /* do nothing */
        break;
    }

    if (state->game_over) {
        enum Item winner = state->scores[0] > state->scores[1] ? PLAYER1 : PLAYER2;
        send_game_over(winner, fdbcast);
    }

    return state->game_over;
}