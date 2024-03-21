#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>  // usleep

#include "pascman.h"

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

/* Cette fonction lit la map stockée dans le fichier 'resources/map.txt' et génère une suite
 * de messages qui sont écrits l'un à la suite de lautre sur la sortie standard du programme.
 * 
 * Cette fonction peuble le tableau passé en argument avec deux entiers: 
 * le premier entier est l'identifiant du héros
 * le deuxierme entier est l'identifiant du méchant
 */
void load_map(uint32_t *res) {
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
        // - Lorsqu'on rencontrera un caractere '*' on ajoutera un powerup (type spécial de nourriture dont vous n'avez besoin que si vous faites le bonus)
        // - Lorsqu'on rencontrera un caractere ' ' on ajoutera uniquement une tuile de sol.
        // - Lorsqu'on rencontrera un caractere '@' on injectera le heros
        // - Lorsqu'on rencontrera un caractere '!' on ajoutera un méchant
        switch (c) {
            case '#': 
                spawn_item(id++, x, y, WALL);
                x++;
                break;
            case '.':
                spawn_item(id++, x, y, FLOOR);
                spawn_item(id++, x, y, FOOD);
                x++;
                break;
             case '*':
                spawn_item(id++, x, y, FLOOR);
                spawn_item(id++, x, y, POWERUP);
                x++;
                break;
            case ' ':
                spawn_item(id++, x, y, FLOOR);
                x++;
                break;
            case '@':
                res[0] = spawn_item(id++, x, y, HERO);
                x++;
                break;
            case '!':
                res[1] = spawn_item(id++, x, y, VILLAIN);
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

/* Cette fonction ecrit le message 'special mode' pour indiquer que l'entité 'id' 
 * entre ou sort du mode spécial.
 */
void special_mode(uint32_t id, bool special) {
    union Message msg;
    struct SpecialMode mode = {
        .msgt    = SPECIAL_MODE,
        .id      = id,
        .active  = special
    };
    msg.special  = mode;

    size_t nb_ecrit = fwrite(&msg, sizeof(union Message), 1, stdout);
    if (nb_ecrit != 1) {
        perror("je n'ai pas su écrire mon special mode");
        exit(1);
    }
}

int main(int argc, const char const* const* argv) {
    uint32_t hero_villain[2];
    load_map(hero_villain);
    fflush(stdout);

    uint32_t id_hero    = hero_villain[0];
    uint32_t id_villain = hero_villain[1];

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
    
    int tour_len     = 10;
    int tour_villain = 0;
    int tour_hero    = 4;
    int nb_tours     = 3;

    bool special     = false;
    for(int i = 0; i < nb_tours * 10; i++) {
        usleep(250000);
        move_item(id_hero,    tour[tour_hero]);
        move_item(id_villain, tour[tour_villain]);

        // juste pour l'illustration, on montre ce que ca donne de faire
        // passer un héro et un méchant dans le mode spécial pendant la 
        // moitié du tour. Dans l'implémentation du bonus, faire passer
        // un héros et les méchants en mode spécial nécessiterait que le
        // héros ait d'abord mangé un powerup.
        if (i % 5 == 0) {
            special = !special;
            special_mode(id_hero,    special);
            special_mode(id_villain, special);
        }
      
        tour_hero    = (tour_hero + 1) % tour_len;
        tour_villain = (tour_villain+1)% tour_len;
        fflush(stdout);
    }

   
    union Message left_game = {.left_game = {.msgt = LEFT_GAME, .id = id_villain }};
    size_t nb_ecrit = fwrite(&left_game, sizeof(union Message), 1, stdout);
    if (nb_ecrit != 1) {
        perror("je n'ai pas su écrire le fait que le mechant a quitté le jeu");
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