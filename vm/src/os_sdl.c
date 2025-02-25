#include "os.h"
#include <SDL3/SDL_assert.h>

void os_assert(bool condition) {
	SDL_assert(condition);
}

