#include <SDL3/SDL_messagebox.h>
#include <stdio.h>
#include <stddef.h>
#include <SDL3/SDL.h>
#include "test.h" 

int main() {
	SDL_ShowSimpleMessageBox(0, "Hey", "Test", nullptr);
	printf("Hey %d\n", get_num_things());	
}
