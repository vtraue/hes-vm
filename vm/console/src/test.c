#include <stdint.h>
#include <stddef.h>
#define BUILTIN
__attribute__((import_module("env"), import_name("io_print_string"))) void vm_print(char* ptr, int size);
__attribute__((import_module("env"), import_name("gfx_paint"))) void vm_paint(char* framebuffer, int width, int height);

#define WASM_PAGE_SIZE 65536
#define FB_WIDTH 800
#define FB_HEIGHT 600
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

char* alloc_pages(int pages) {
    return (char*)(__builtin_wasm_memory_grow(0, pages));
}

char* alloc_framebuffer() {
  int size_bytes = (FB_WIDTH * FB_HEIGHT * 4);
  int size_pages = size_bytes / WASM_PAGE_SIZE;   
  return alloc_pages(size_pages + 1);
}


char* init() {
    cstr_print("Hello from init!\n");
    return alloc_framebuffer();
    return 0;
}

void memset(void *dst, int value, unsigned long size) {
  __builtin_memset(dst, value, size);
}


void fill_framebuffer(u8* buffer, u8 r, u8 g, u8 b, u8 a) {
  u8* row = buffer;
  int color = ((u32)(a) << 24) | ((u32)(b) << 16) | ((u32)(g) << 8) | r;

  for(int y = 0; y < FB_HEIGHT; y++) {
    u32* pixel = (u32*)row;
    for(int x = 0; x < FB_WIDTH; x++) {
      *pixel++ = color;
    }
    row += 800 * 4;
  }
}

void draw_rectangle(u8* dest_buffer, u32 x, u32 y, u32 width, u32 height, u8 r, u8 g, u8 b) {
  u32* s = ((u32*) dest_buffer) + ((y * FB_WIDTH) + x); 
  u32* d = s;
  int color = (0 << 24) | b << 16 | g << 8 | r;
  for(int _y = 0; _y < height; _y++) {
    for(int _x = 0; _x < width; _x++) {
      *d = color;    
      d++;
    }
    d = s + ((_y) * FB_WIDTH);
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
void run(u8* framebuffer, u32 framebuffer_width, u32 framebuffer_height) {
  //cstr_print("Hello from run!\n");
  fill_framebuffer(framebuffer, 0, 200, 0, 0);
  //render_weird_gradient(framebuffer, global_xoffset, global_yoffset);
  draw_rectangle(framebuffer, 40, 40, 100, 100, 250, 50, 0);
  global_xoffset += 2;
  global_yoffset += 2;

  vm_paint(framebuffer, FB_WIDTH, FB_HEIGHT);
}
