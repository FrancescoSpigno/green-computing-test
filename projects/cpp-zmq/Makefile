CC := gcc
FLAGS := -O3 -std=c++17 -Wall -Werror -Wuninitialized -Wshadow -Wwrite-strings -Wconversion -Wunreachable-code

.SILENT: run clean

.PHONY: all

all: run

run: main.cpp
	$(CC) $(FLAGS) main.cpp -o main
	./main

clean:
	rm -f main