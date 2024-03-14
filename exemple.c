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
            Spawn spawn = {
                .msgt = SPAWN,
                .id   = id++,
                .item = WALL,
                .pow  = {
                    .x = x,
                    .y = y
                }
            }; 
            Message msg;
            msg.spawn = spawn;

            size_t written = fwrite(&msg, sizeof(Message), 1, f);
            if (written < sizeof(Message)) {
                perror("fwrite");
                exit(1);
            }
        }
    }

    fclose(f);
}