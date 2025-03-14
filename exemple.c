#include <stdio.h>
#include <stdlib.h>
#include <string.h>  // memset
#include <unistd.h>  // usleep

#include "pascman.h"

// ********************************************************************************
// CE PROGRAMME VOUS EST FOURNI A TITRE D'EXEMPLE. IL VOUS PERMET DE COMPRENDRE
// LA FACON D'INTERAGIR AVEC L'INTERFACE GRAPHIQUE DU JEU QUI VOUS EST FOURNIE.
// CE PROGRAMME VOUS DONNE AUSSI UNE SERIE DE STRUCTURES ET DE FONCTIONS QUI 
// SONT AUTANT D'INDICES POUVANT VOUS ORIENTER POUR BIEN DEMARRER VOTRE PROJET.
// ================================================================================
// VOUS AVEZ PARFAITEMENT LE DROIT DE REUTILISER, ET D'ADAPTER LE CODE FOURNI
// POUR L'INTEGRER A VOTRE PROJET SI VOUS JUGEZ QUE C'EST PERTINENT. NOUS 
// ATTIRONS TOUTEFOIS VOTRE ATTENTION SUR LE FAIT QUE CE PROGRAMME N'EST PAS
// MODULARISE ALORS QU'IL EST ATTENDU QUE VOTRE CODE SOIT CORRECTEMENT 
// DECOUPE EN MODULES.
// ********************************************************************************

/* Cette fonction ecrit le message approprié pour injecter la resource voulue dans le jeu
 *
 * Cette fonction renvoie l'identifiant de la ressource qui a été ajoutée 
 */
uint32_t spawn_item(uint32_t id, uint32_t x, uint32_t y, enum Item item) {
    union Message msg;
    struct Spawn spawn = {
        .msgt = SPAWN,
        .id   = id,
        .item = item,
        .pos  = {
            .x = x,
            .y = y
        }
    };
    msg.spawn = spawn;

    size_t nb_ecrit = fwrite(&msg, sizeof(union Message), 1, stdout);
    if (nb_ecrit != 1) {
        perror("je n'ai pas su écrire mon item");
        exit(1);
    }
    return id;
}

/* Cette fonction ecrit le message pour faire bouger un élément du jeu
 */
void move_item(uint32_t id, struct Position pos) {
    union Message msg;
    struct Movement movement = {
        .msgt = MOVEMENT,
        .id   = id,
        .pos  = pos
    };
    msg.movement = movement;

    size_t nb_ecrit = fwrite(&msg, sizeof(union Message), 1, stdout);
    if (nb_ecrit != 1) {
        perror("je n'ai pas su écrire mon mouvement");
        exit(1);
    }
}

/* Cette fonction ecrit le message pour signifier à l'interface graphique
 * qu'un joueur donné a mangé une certaine nourriture sur le plateau.
 */
void eat_food(uint32_t player_id, uint32_t food_id) {
    union Message msg;
    struct EatFood eat_food = {
        .msgt  = EAT_FOOD,
        .eater = player_id,
        .food  = food_id
    };
    msg.eat_food = eat_food;

    size_t nb_ecrit = fwrite(&msg, sizeof(union Message), 1, stdout);
    if (nb_ecrit != 1) {
        perror("je n'ai pas su écrire le fait que j'ai mangé la nourriture");
        exit(1);
    }
}

// Cette structure permet de mémoriser les informations nécessaires pour 
// connaitre l'état du jeu en cours. Vous pouvez vous en inspirer pour
// la modélisation du contenu de votre mémoire partagée (mais vous n'etes
// pas obligés de le faire). Par ailleurs, cette structure n'est pas 
// totalement adaptée pour implémenter ce qui vous est demandé. Vous 
// devrez donc y faire certains changements vous même pour pouvoir 
// gérer correctement l'état de votre jeu du coté server.
typedef struct {
    // ce champ garde en mémoire la position de tous les murs.
    bool walls[MAP_SIZE]; 
    // ce champ nous permet de savoir pour chaque position sur
    // la carte s'il y a de la nourriture à cet endroit, et si 
    // oui, de connaitre l'identifiant de la resource pour 
    // l'interface graphique. Lorsqu'une tuile de la carte ne 
    // contient aucune nourriture, la valeur -1 est indiquée dans
    // cette carte.
    int32_t food[MAP_SIZE];
    // Ce champ permet de connaitre immédiatement l'identifiant
    // du joueur 1 tel qu'il est connu par l'interface graphique.
    uint32_t id_player1;
    // Ce champ permet de connaitre immédiatement l'identifiant
    // du joueur 2 tel qu'il est connu par l'interface graphique.
    uint32_t id_player2;
    // ce champ nous permet de connaitre immédiatement la position
    // de chacun des joueurs sur le plateau.
    struct Position positions[2];
} GameState;

// Cette fonction convertit une position x,y en un indice
// qui peut être utilisé pour retrouver une entrée sur la carte.
size_t at(uint32_t x, uint32_t y) {
    return x * WIDTH + y;
}
// Cette fonction renvoie convertit une position x,y en un indice
// qui peut être utilisé pour retrouver une entrée sur la carte.
// La seule différence avec la fonction 'at', c'est qu'ici on 
// utilise la structure 'Position' qui vous est founie.
size_t at_pos(struct Position pos) {
    return at(pos.x, pos.y);
}
// Cette réinitialise un objet GameState ce qui permet de s'assurer
// que toutes les valeurs soient correctement initialisées
// (par exemple en mettant -1 partout dans le champ 'food').
void reset_gamestate(GameState *state) {
    if(!memset(state->walls, 0, sizeof(state->walls))) {
        perror("memset walls:");
        exit(1);
    }
    if(!memset(state->food, -1, sizeof(state->food))) {
        perror("memset food:");
        exit(1);
    }
    if(!memset(state->positions, 0, sizeof(state->positions))) {
        perror("memset positions:");
        exit(1);
    }
    state->id_player1 = 0;
    state->id_player2 = 0;
}

/* Cette fonction lit la map stockée dans le fichier 'resources/map.txt' et génère une suite
 * de messages qui sont écrits l'un à la suite de lautre sur la sortie standard du programme.
 * 
 * De plus, va peupler une structure de type GameState passée en parametre que vous pouvez
 * utiliser pour maintenir une copie l'état courant du jeu.
 */
void load_map(GameState *state) {
    reset_gamestate(state);

    FILE* f = fopen("resources/map.txt", "r");
    if (!f) {
        perror("fopen");
        exit(1);
    }

    // note: chaque élément du jeu a un id, on va donc garder un id global qui sera mis à jour
    uint32_t id = 0;
    uint32_t x  = 0;
    uint32_t y  = 0;
    char c = fgetc(f);
    while(!feof(f)) {
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
                spawn_item(id++, x, y, WALL);
                state->walls[at(x, y)] = true;
                x++;
                break;
            case '.':
                spawn_item(id++, x, y, FLOOR);
                spawn_item(id++, x, y, FOOD);
                state->food[at(x, y)] = id-1;
                x++;
                break;
             case '*':
                spawn_item(id++, x, y, FLOOR);
                spawn_item(id++, x, y, SUPERFOOD);
                state->food[at(x, y)] = id-1;
                x++;
                break;
            case ' ':
                spawn_item(id++, x, y, FLOOR);
                x++;
                break;
            case '@':
                spawn_item(id++, x, y, PLAYER1); // player 1
                state->id_player1     = id-1;
                state->positions[0].x = x;
                state->positions[0].y = y;
                x++;
                break;
            case '!':
                spawn_item(id++, x, y, PLAYER2); // player 2
                state->id_player2     = id-1;
                state->positions[1].x = x;
                state->positions[1].y = y;
                x++;
                break;
            case '\n':
                y ++;
                x = 0;
                break;
            default:
                // par défaut on ne fait simplement rien
                break;
        }
        c = fgetc(f);
    }
}

int main(int argc, const char const* const* argv) {
    GameState state;
    load_map(&state);
    fflush(stdout);

    // Dans cet exemple, nos deux joueurs vont simplement faire un
    // petit tour sur la carte, ce qui va créer une animation un
    // peu idiote mais qui devrait permettre aux étudiants de 
    // comprendre comment ils doivent utiliser le code qui leur 
    // est fourni.
    struct Position tour[10] = {
        {.x = 12, .y = 9},
        {.x = 13, .y = 9},
        {.x = 14, .y = 9},
        {.x = 15, .y = 9},
        {.x = 16, .y = 9},
        {.x = 16, .y =10},
        {.x = 15, .y =10},
        {.x = 14, .y =10},
        {.x = 13, .y =10},
        {.x = 12, .y =10},
    };
    // Les deux joueurs font le même tour sur la carte. Le tour comprend
    // 10 pas. Mais, le player 1 commence le tour à la e position définie
    // tandis que le player 2 commence son tour à la toute premiere position.
    // Ce petit décalage sert simplement à donner l'impression que le p2 
    // poursuit le p1.
    int tour_len     = 10;
    int step_p1      = 4;
    int step_p2      = 0;
    int nb_tours     = 3;

    // on attend un peu avant de commencer notre animation, parce que 
    // parfois le systeme prend un peu de temps pour afficher l'interface.
    usleep(2000000);

    for(int i = 0; i < nb_tours * 10; i++) {
        usleep(500000);
        
        // on écrit les messages de mouvement sur stdout pour que 
        // l'interface graphique sache quoi faire.
        move_item(state.id_player1, tour[step_p1]);
        move_item(state.id_player2, tour[step_p2]);

        // on garde une trace de ces changements dans notre game state.
        state.positions[0] = tour[step_p1];
        state.positions[1] = tour[step_p2];

        // si un des joueurs se trouve sur une case contenant de la nourriture
        // on informe l'interface graphique que cette nourriture a été 
        // consommée.
        // == JOUEUR 1 == 
        int32_t food_id = state.food[at_pos(state.positions[0])];
        if (food_id != -1) {
            // on informe l'interface graphique
            eat_food(state.id_player1, food_id);
            // on consomme la nourriture dans notre état.
            state.food[at_pos(state.positions[0])] = -1;
        }
        // == JOUEUR 2 == 
        food_id = state.food[at_pos(state.positions[1])];
        if (food_id != -1) {
            // on informe l'interface graphique
            eat_food(state.id_player2, food_id);
            // on consomme la nourriture dans notre état.
            state.food[at_pos(state.positions[1])] = -1;
        }
        
        // on passe au pas suivant dans notre tour.
        step_p1 = (step_p1 + 1) % tour_len;
        step_p2 = (step_p2 + 1) % tour_len;
        fflush(stdout);
    }

   
    union Message left_game = {.left_game = {.msgt = LEFT_GAME, .id = state.id_player2 }};
    size_t nb_ecrit = fwrite(&left_game, sizeof(union Message), 1, stdout);
    if (nb_ecrit != 1) {
        perror("je n'ai pas su écrire le fait que le 2nd joueur a quitté le jeu");
        exit(1);
    }
    fflush(stdout);

    // on attend un peu histoire de pouvoir voir que le joueur est bien parti pour du vrai 
    usleep(1000000);

    // apres 3 tours, on va afficher un victoire (meme si personne n'a gagné)
    union Message msg = {.victory = {.msgt = VICTORY }};
    nb_ecrit = fwrite(&msg, sizeof(union Message), 1, stdout);
    if (nb_ecrit != 1) {
        perror("je n'ai pas su écrire ma victoire");
        exit(1);
    }
    fflush(stdout);

    return 0;
}