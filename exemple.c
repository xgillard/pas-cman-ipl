#include <stdio.h>
#include <stdlib.h>
#include "pascman.h"

int main(int argc, const char const* const* argv) {
    FILE* f = fopen("output.bin", "w");
    if (!f) {
        perror("fopen");
        exit(1);
    }

    int id = 0;
    for(uint32_t x = 0; x < WIDTH; x++) {
        for (uint32_t y=0; y < HEIGHT; y++) {

            if (x % 2 == 0 ) {
                struct Spawn spawn = {
                    .msgt = SPAWN,
                    .id   = id++,
                    .item = WALL,
                    .pos  = {
                        .x = x,
                        .y = y
                    }
                }; 
                union Message msg;
                msg.spawn = spawn;

                size_t written = fwrite(&msg, sizeof(union Message), 1, f);
                if (written < 1) {
                    perror("fwrite");
                    exit(1);
                }
            } else {
                {
                    struct Spawn spawn = {
                        .msgt = SPAWN,
                        .id   = id++,
                        .item = FLOOR,
                        .pos  = {
                            .x = x,
                            .y = y
                        }
                    }; 
                    union Message msg;
                    msg.spawn = spawn;

                    size_t written = fwrite(&msg, sizeof(union Message), 1, f);
                    if (written < 1) {
                        perror("fwrite");
                        exit(1);
                    }
                } 

                {
                    struct Spawn spawn = {
                        .msgt = SPAWN,
                        .id   = id++,
                        .item = FOOD,
                        .pos  = {
                            .x = x,
                            .y = y
                        }
                    }; 
                    union Message msg;
                    msg.spawn = spawn;

                    size_t written = fwrite(&msg, sizeof(union Message), 1, f);
                    if (written < 1) {
                        perror("fwrite");
                        exit(1);
                    }
                }
            }
        }
    }

    fclose(f);
}