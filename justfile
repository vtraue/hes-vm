build:
	clang src/*.c -Werror -Wall -fsanitize=address -o out/debug.bin
run:
	./out/debug.bin
