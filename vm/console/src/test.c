#define BUILTIN
__attribute__((import_module("env"), import_name("io_print_string"))) void vm_print(char* ptr, int size);
__attribute__((import_module("env"), import_name("gfx_paint"))) void vm_paint(char* framebuffer, int width, int height);

#define WASM_PAGE_SIZE 65536
#define FB_WIDTH 800
#define FB_HEIGHT 600
typedef int size_t;
typedef char u8;
typedef short u16;
typedef int u32;

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
}

void fill_framebuffer(char* buffer, char r, char g, char b) {
  u8* row = buffer;
  int color = (4 << 24) | (b << 16) | (g << 8) | r;

  for(int y = 0; y < 600; y++) {
    for(int x = 0; x < 800; x++) {
      *row++ = r;
      *row++ = g;
      *row++ = b;
      *row++ = 0;

    }
    row = buffer + ((800 * 4) * y);
  }
  /*
  for(int y = 0; y < 600; y++) {
    u32* pixel = (u32*)row;
    for(int x = 0; x < 800; x++) {
      *pixel++ = color;
    }
    row += 800 * 4;
  }
  */
}

void draw_rectangle(u8* dest_buffer, u32 x, u32 y, u32 width, u32 height, u8 r, u8 g, u8 b) {
  u32* s = ((u32*) dest_buffer); 
  u32* d = s;
  int color = r << 24 | g << 16 | b << 8 | 900;
  for(int _y = 0; _y < height; _y++) {
    for(int _x = 0; _x < width; _x++) {
      *d = color;    
      d++;
    }
    d = s + ((_y + y) * 800) + x;
  }
  
}

void run(char* framebuffer, int framebuffer_width, int framebuffer_height) {
  //cstr_print("Hello from run!\n");
  fill_framebuffer(framebuffer, 0, 0, 200);
  //draw_rectangle(framebuffer, 0, 0, 100, 100, 200, 200, 200);

  vm_paint(framebuffer, 800, 600);
}
