build:
	clang src/*.c -std=c23 -Werror -Wall -fsanitize=address -lSDL3 -o out/debug.bin
run:
	./out/debug.bin
