CC=gcc
CFLAGS=-std=c11 -pedantic -Wall -Wvla -Werror -D_DEFAULT_SOURCE

all: exemple

exemple: exemple.o game.o utils_v1.o
	$(CC) $(CFLAGS) -o exemple exemple.o game.o utils_v1.o

exemple.o: exemple.c
	$(CC) $(CFLAGS) -c exemple.c
	
game.o: game.h game.c
	$(CC) $(CFLAGS) -c game.c $(INCLUDES)

utils_v1.o: utils_v1.h utils_v1.c
	$(CC) $(CFLAGS) -c utils_v1.c $(INCLUDES)

clean: 
	rm -rf *.o

mrpropre: clean
	rm -rf exemple