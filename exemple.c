#include <stdio.h>
#include <stdlib.h>
#include <string.h>  // memset
#include <unistd.h>  // usleep
#include <fcntl.h>

#include "utils_v1.h"
#include "pascman.h"
#include "game.h"

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

int main(int argc, char** argv) {
    struct GameState state;
    FileDescriptor sout = 1;
    FileDescriptor map  = sopen("./resources/map.txt", O_RDONLY, 0);
    load_map(map, sout, &state);
    sclose(map);

    // Dans cet exemple, nos deux joueurs vont simplement faire un
    // petit tour sur la carte, ce qui va créer une animation un
    // peu idiote mais qui devrait permettre aux étudiants de 
    // comprendre comment ils doivent utiliser le code qui leur 
    // est fourni.
    enum Direction tour[10] = {
        RIGHT, RIGHT, RIGHT, RIGHT,
        DOWN,
        LEFT, LEFT, LEFT, LEFT,
        UP
    };
    // Les deux joueurs font le même tour sur la carte. Le tour comprend
    // 10 pas. Mais, le player 1 commence le tour à la e position définie
    // tandis que le player 2 commence son tour à la toute premiere position.
    // Ce petit décalage sert simplement à donner l'impression que le p2 
    // poursuit le p1.
    int tour_len     = 10;
    int step_p1      = 4;
    int step_p2      = 0;
    int nb_tours     = 2;

    // on attend un peu avant de commencer notre animation, parce que 
    // parfois le systeme prend un peu de temps pour afficher l'interface
    // et on n'a pas envie que les étudiants ne voient pas la super
    // animation qu'on leur a préparé.
    usleep(2000000);

    // On va faire croire a notre interface graphique qu'elle est en 
    // train de jouer avec le joueur 1.
    send_registered(1, sout);

    for(int i = 0; i < nb_tours * 10; i++) {
        usleep(500000);
        
        process_user_command(&state, PLAYER1, tour[step_p1], sout);
        process_user_command(&state, PLAYER2, tour[step_p2], sout);
        
        // on passe au pas suivant dans notre tour.
        step_p1 = (step_p1 + 1) % tour_len;
        step_p2 = (step_p2 + 1) % tour_len;
    }

    // Ici on va tricher: on va dire au systeme que le jeu est fini
    // et que c'est le joueur 1 qui a gagné.
    state.scores[0] = 10000;
    state.scores[1] = 0; // joueur 2, tu as perdu.
    state.game_over = true;

    // normalement, la fonction process_user_command génère ce message
    // pour vous automatiquement si l'état du jeu est cohérent.
    //send_game_over(PLAYER1, sout);

    return 0;
}
