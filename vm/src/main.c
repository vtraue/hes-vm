#include <SDL3/SDL_messagebox.h>
#include <stdio.h>
#include <stddef.h>
#include <SDL3/SDL.h>
#include "os.h"
#include "leb128.h"

int main() {
	int leb_numbers[] = {0x45, 0x42, 0x30, 0x6C};
	int64_t res = 0;

	uint64_t ures = 0;
	leb128_read_u64((uint8_t*)leb_numbers, 0, sizeof(leb_numbers), &ures);
	printf("%ld\n", ures);
	os_assert(ures == 69);

	leb128_read_i64((uint8_t*)leb_numbers, 0, sizeof(leb_numbers), &res);
	printf("%ld\n", res);
	os_assert(res == -59);
}
