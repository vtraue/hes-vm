#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#define BUILTIN
__attribute__((import_module("env"), import_name("io_print_string"))) void vm_print(char* ptr, int size);
__attribute__((import_module("env"), import_name("gfx_paint"))) void vm_paint(uint8_t* framebuffer, int width, int height);
__attribute__((import_module("env"), import_name("gfx_clear_buffer_rgb"))) void vm_clear(uint8_t* framebuffer, int r, int g, int b);
__attribute__((import_module("env"), import_name("gfx_draw_rect_rgb"))) void vm_draw_rect_rgb(uint8_t* framebuffer, int x, int y, int width, int height, int r, int g, int b);
__attribute__((import_module("env"), import_name("io_print_sint"))) void vm_print_int(int32_t num);
__attribute__((import_module("env"), import_name("io_print_sint64"))) void vm_print_int64(int64_t num);
__attribute__((import_module("env"), import_name("clock_get_time_passed_ms"))) int64_t vm_get_time_ms();
__attribute__((import_module("env"), import_name("rand_range_sint32"))) int32_t vm_rand_range(int32_t min, int32_t max);


#define WASM_PAGE_SIZE 65536
#define FB_WIDTH 352
#define FB_HEIGHT 240
typedef uint8_t u8;
typedef uint16_t u16;
typedef uint32_t u32;
typedef uint64_t u64;

typedef int8_t i8;
typedef int16_t i16;
typedef int32_t i32;
typedef int64_t i64;

uint8_t global_xoffset = 0;
uint8_t global_yoffset = 0;
size_t cstr_len(const char* str) {
  size_t counter = 0;
  for(;;) {
    if(str[counter] == 0) {
      return counter;
    }
    counter++;
  }
}

void cstr_print(const char* str) {
  size_t l = cstr_len(str);
  vm_print(str, l);
}

uint8_t* alloc_pages(int pages) {
    return (uint8_t*)(__builtin_wasm_memory_grow(0, pages));
}

typedef enum Key_Code : uint32_t {
  KEYCODE_UP = 0,
  KEYCODE_DOWN = 1,
  KEYCODE_LEFT = 2,
  KEYCODE_RIGHT = 3,
  KEYCODE_A = 4,
  KEYCODE_B = 5,
  KEYCODE_X = 6,
  KEYCODE_Y = 7,
  KEYCODE_R = 8,
  KEYCODE_L = 9,
  KEYCODE_COUNT = 10,
} Key_Code;

typedef struct Game_Data {
  u8 framebuffer[FB_WIDTH * FB_HEIGHT * 4];
  bool keys[KEYCODE_COUNT];
  int32_t position_x;
  int32_t position_y;
  int32_t current_speed;
} Game_Data;

Game_Data* alloc_game_data() {
  int size_bytes = sizeof(Game_Data);
  int size_pages = size_bytes / WASM_PAGE_SIZE;   
 
  return (Game_Data*)(alloc_pages(size_pages + 1));
}


Game_Data* init() {
    cstr_print("Hello from init!\n");
    Game_Data* data = alloc_game_data();
    data->current_speed = 2;

    return data;
}

void memset(void *dst, uint32_t value, unsigned long size) {
  __builtin_memset(dst, value, size);
}


void fill_framebuffer(u8* buffer, u8 r, u8 g, u8 b, u8 a) {
  u8* row = buffer;
  int color = ((u32)(a) << 24) | ((u32)(b) << 16) | ((u32)(g) << 8) | r;

  for(int y = 0; y < FB_HEIGHT; y++) {
    u32* pixel = (u32*)row;
    memset((void*)pixel, color, FB_WIDTH * 4);
    row += FB_WIDTH * 4;
  }
}

void draw_rectangle(u8* dest_buffer, u32 x, u32 y, u32 width, u32 height, u8 r, u8 g, u8 b) {
  u8* s = dest_buffer + ((y * FB_WIDTH * 4) + x * 4); 
  u64* d = (u64*) s;
  int color = (0 << 24) | b << 16 | g << 8 | r;
  for(int _y = 0; _y < (height); _y++) {
    for(int _x = 0; _x < (width / 2); _x++) {
      *d++ = (u64) color << 32 | color;    
    }
    d = (u64*)(s + ((_y) * FB_WIDTH * 4));
  }
}

void render_weird_gradient(u8* dest_buffer, int x_offset, int y_offset) {
  int width = FB_WIDTH / 2;
  int height = FB_HEIGHT;
  int pitch = FB_WIDTH * 4;

  u8* row = dest_buffer;
  for(int y = 0; y < height; ++y) {
    u64* pixel = (u64*)row;
    for(int x = 0; x < width; ++x) {
      uint8_t blue = (x + x_offset);
      uint8_t green = (y + y_offset);
      uint32_t p = ((blue << 16) | (green << 8));
      *pixel++ = (u64) p << 32 | p;
    }
    row += pitch;
  }
}

void input(Game_Data* game, uint32_t key, bool down) {
  game->keys[key] = down; 
}

void run(Game_Data* game, u32 framebuffer_width, u32 framebuffer_height) {
  int64_t then = vm_get_time_ms();

  //cstr_print("Hello from rint64_t numun!\n");
  //fill_framebuffer(game->framebuffer, 0, 255, 255, 255);
  //render_weird_gradient(game->framebuffer, global_xoffset, global_yoffset);
  vm_clear(game->framebuffer, 200, 0, 0);
  if(game->keys[KEYCODE_UP]) {
    if(game->position_y - game->current_speed + 16 > 0) {
      game->position_y -= game->current_speed;
    }
  }

  if(game->keys[KEYCODE_DOWN]) {
    if((game->position_y + game->current_speed + 16) < FB_HEIGHT) {
      game->position_y += game->current_speed;
      
    }
  }

  if(game->keys[KEYCODE_LEFT]) {
    if(game->position_x - game->current_speed - 16 > 0) {
      game->position_x -= game->current_speed;
    }
  }

  if(game->keys[KEYCODE_RIGHT]) {
    if((game->position_x + game->current_speed + 16) < FB_WIDTH) {
      game->position_x += game->current_speed;
    }
  }

  /*
  vm_print_int(game->position_x);
  vm_print_int(game->position_y);
  */
  //draw_rectangle(game->framebuffer, game->position_x, game->position_y, 16, 16, 250 ,0, 250);
  vm_draw_rect_rgb(game->framebuffer, game->position_x, game->position_y, 16, 17, 0, 0, 0);

  global_xoffset += 4;
  global_yoffset += 4;

  i64 time_passed = vm_get_time_ms() - then;

  /*
  cstr_print("Time passed: ");
  vm_print_int64(time_passed);
  cstr_print("\n");
  
  cstr_print("Random number: ");
  vm_print_int(vm_rand_range(-10, 10));
  cstr_print("\n");
  */
  vm_paint(game->framebuffer, FB_WIDTH, FB_HEIGHT);
 
}
