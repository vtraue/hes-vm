#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
const uint8_t cat_data[] =
{
  #embed "resources/malu.bmp"
};


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
#define FB_WIDTH 640
#define FB_HEIGHT 360
typedef uint8_t u8;
typedef uint16_t u16;
typedef uint32_t u32;
typedef uint64_t u64;

typedef int8_t i8;
typedef int16_t i16;
typedef int32_t i32;
typedef int64_t i64;
i64 last_time_passed = 0;

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
  i32 cat_positions_x[200];
  i32 cat_positions_y[200];
  i32 position_x;
  i32 position_y;
  i32 current_speed;
  i32 cat_image_width;
  i32 cat_image_height;
  u8* cat_image_data;
} Game_Data;

Game_Data* alloc_game_data() {
  int size_bytes = sizeof(Game_Data);
  int size_pages = size_bytes / WASM_PAGE_SIZE;   
  Game_Data* data = (Game_Data*)(alloc_pages(size_pages + 1));
  return data;
}


void assert(bool cond) {
  if(!cond) {
    __builtin_unreachable();
  }
}

Game_Data* init() {
    cstr_print("Hello from init!\n");
    assert(cat_data[0] == 0x42);
    assert(cat_data[1] == 0x4D);
    Game_Data* data = alloc_game_data();
    data->cat_image_width = *(i32*)(cat_data + 0x12);
    data->cat_image_height = *(i32*)(cat_data + 0x16);
    u32 data_offset = *(u32*)(cat_data + 0x0A);
    data->cat_image_data = (u8*)(cat_data + data_offset);
    for(int i = 0; i < 200; i++) {
      data->cat_positions_x[i] = vm_rand_range(0, 300);
      data->cat_positions_y[i] = vm_rand_range(0, 300);
    }

    u16 bits_per_pixel = *(u16*)(cat_data + 0x1C);

    cstr_print("width: ");
    vm_print_int((i32)data->cat_image_width);

    cstr_print("height: ");
    vm_print_int((i32)data->cat_image_height);

    cstr_print("bits per pixel: ");
    vm_print_int((i32) bits_per_pixel);

    u32 test = *(u32*)(data->cat_image_data);
    vm_print_int((i32) test);
    data->current_speed = 2;

    cstr_print("init done\n");
    return data;
}

void memset(void *dst, uint32_t value, unsigned long size) {
  __builtin_memset(dst, value, size);
}
void memcpy(void *dst, const void *src, unsigned long size) {
    __builtin_memcpy(dst, src, size);
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

void blit(u8* dest,
          u32 dest_x, u32 dest_y,
          u8* source,
          u32 source_pitch,
          u32 src_x, u32 src_y,
          u32 src_width, u32 src_height) {

  u8* dst = dest + ((dest_y * (FB_WIDTH * 4)) + dest_x * 4); 
  u8* src = source + ((((src_height - 1) + src_y) * source_pitch) + ((src_width - 1) + src_x) * 4);

  for(int y = 0; y < src_height; y++) {
    for(int x = 0; x < src_width; x++) {
      u32 pixel = *(u32*)(src - (x * 4));
      u8 a = (u8)(pixel << 0);
      u8 r = (u8)(pixel << 16);
      u8 g = (u8)(pixel << 8);
      u8 b = (u8)(pixel << 0);
      
      if(a > 0) {
        *(u32*)(dst + (x * 4)) = pixel;
      }
    }
    dst += FB_WIDTH * 4;
    src -= source_pitch;
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
  vm_clear(game->framebuffer, 0, 128, 128);
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
  //vm_draw_rect_rgb(game->framebuffer, game->position_x, game->position_y, 16, 17, 0, 0, 0);

  
  i64 index = vm_get_time_ms() / 200;
  u32 frame = index % 6;
  //vm_print_int64(index % 6);

  for(int i = 0; i < 200; i++) {
    i32 position_x = game->cat_positions_x[i];
    i32 position_y = game->cat_positions_y[i];

    /*
    cstr_print("x: ");
    vm_print_int(position_x);
    cstr_print("y: ");
    vm_print_int(position_y);
    */
    //TODO: Fixe das!
    if(position_x > FB_WIDTH || position_x < 0 || position_y > FB_HEIGHT || position_y < 0) {
      continue;
    }

    blit(game->framebuffer, position_x, position_y, game->cat_image_data, game->cat_image_width * 4, frame * 16, 0, 16, 16);
    
  }
  global_xoffset += 4;
  global_yoffset += 4;

  i64 time_passed = vm_get_time_ms() - then;
  
  cstr_print("Time passed: ");
  vm_print_int64(time_passed);
  cstr_print("\n");
  last_time_passed = time_passed;
  /*
  cstr_print("Random number: ");
  vm_print_int(vm_rand_range(-10, 10));
  cstr_print("\n");
  */

  vm_paint(game->framebuffer, FB_WIDTH, FB_HEIGHT);
 
}
