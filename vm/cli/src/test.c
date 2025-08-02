#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
const uint8_t cat_data[] =
{
  #embed "resources/malu.bmp"
};

const uint8_t font_data[] =
{
  #embed "resources/font2.bmp"
};


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

#define BUILTIN
__attribute__((import_module("env"), import_name("io_print_string"))) void vm_print(char* ptr, int size);
__attribute__((import_module("env"), import_name("gfx_paint"))) void vm_paint(uint8_t* framebuffer, int width, int height);
__attribute__((import_module("env"), import_name("gfx_clear_buffer_rgb"))) void vm_clear(uint8_t* framebuffer, int r, int g, int b);
__attribute__((import_module("env"), import_name("gfx_draw_rect_rgb"))) void vm_draw_rect_rgb(uint8_t* framebuffer, int x, int y, int width, int height, int r, int g, int b);
__attribute__((import_module("env"), import_name("io_print_sint"))) void vm_print_int(int32_t num);
__attribute__((import_module("env"), import_name("io_print_sint64"))) void vm_print_int64(int64_t num);
__attribute__((import_module("env"), import_name("clock_get_time_passed_ms"))) int64_t vm_get_time_ms();
__attribute__((import_module("env"), import_name("rand_range_sint32"))) int32_t vm_rand_range(int32_t min, int32_t max);
__attribute__((import_module("env"), import_name("input_get_key_state"))) bool vm_get_key(Key_Code key);
__attribute__((import_module("env"), import_name("system_shutdown"))) void vm_shutdown();


#define WASM_PAGE_SIZE 65536
#define FB_WIDTH 512
#define FB_HEIGHT 288
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


typedef struct Point {
  i32 x;
  i32 y;
} Point;

typedef struct Snake {
  Point body[200];
  u32 len;
  Point direction;
} Snake;

void snake_push_segment(Snake* snake, Point p) {
  snake->len += 1;
  snake->body[snake->len - 1] = p;
}

Point* snake_head(Snake* snake) {
  return &snake->body[0];
}
Point* snake_tail(Snake* snake) {
  return &snake->body[snake->len - 1];
}

void snake_init(Snake* snake) {
  snake_push_segment(snake, (Point) {2,0});
  snake_push_segment(snake, (Point) {1,0});
  snake_push_segment(snake, (Point) {0,0});
  snake->direction = (Point) {1, 0};
}

void snake_draw(Snake* snake, u8* framebuffer) {
  for(int i = 0; i < snake->len; ++i) {
    Point p = snake->body[i];
    vm_draw_rect_rgb(framebuffer, p.x * 16, p.y * 16, 16, 16, 100, 0, 50);
  }
  Point head = snake->body[0];
  vm_draw_rect_rgb(framebuffer, head.x * 16, head.y * 16, 16, 16, 90, 100, 0);
}

void snake_update(Snake* snake) {
  for(int i = snake->len - 1; i > 0; i--) {
    snake->body[i] = snake->body[i - 1];
  }
  i32 width = (FB_WIDTH) / 16;
  i32 height = (FB_HEIGHT) / 16;

  Point* head = snake_head(snake);
  head->x = (head->x + snake->direction.x) % width;
  head->y = (head->y + snake->direction.y) % height;

  if(head->x < 0) {
    head->x = (width - 1);
  }
  if(head->y < 0) {
    head->y = (height - 1);
  }
}

void point_random(Point* p) {
  p->x = vm_rand_range(0, FB_WIDTH) % (FB_WIDTH / 16);
  p->y = vm_rand_range(0, FB_HEIGHT) % (FB_HEIGHT / 16);
}

void fruit_draw(u8* framebuffer, Point* fruit) {
  vm_draw_rect_rgb(framebuffer, fruit->x * 16, fruit->y * 16, 16, 16, 255, 192, 203);
}
typedef struct Bitmap {
  u16 bits_per_pixel; 
  u32 width;
  u32 height;
  u8* data;
} Bitmap;

Bitmap bitmap_load(const u8* bitmap_data) {
  u16 bits_per_pixel = *(u16*)(bitmap_data + 0x1C);
  u32 width = *(i32*)(bitmap_data + 0x12);
  u32 height = *(i32*)(bitmap_data + 0x16);
  u32 data_offset = *(u32*)(bitmap_data + 0x0A);
  u8* data = (u8*)(bitmap_data + data_offset);
  return (Bitmap) {
    bits_per_pixel,
    width,
    height,
    data
  };  
}

typedef struct Game_Data {
  u8 framebuffer[FB_WIDTH * FB_HEIGHT * 4];
  bool keys[KEYCODE_COUNT];
  i32 cat_positions_x[200];
  i32 cat_positions_y[200];
  i32 position_x;
  i32 position_y;
  i32 current_speed;
  Bitmap malu;
  Bitmap font;
  // Bitmap clouds;
  u32 frame_count;
  Snake snake;
  Point fruit;
  float test;
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

bool is_uppercase_letter(char c) {
  return c >= 'A' && c <= 'Z';  
}

bool is_lowercase_letter(char c) {
  return c >= 'a' && c <= 'z';  
}

bool is_number(char c) {
  return c >= '0' && c <= '9';
}

Game_Data* init() {
    cstr_print("Hello from init!\n");
    assert(cat_data[0] == 0x42);
    assert(cat_data[1] == 0x4D);
    Game_Data* data = alloc_game_data();
    data->malu = bitmap_load(cat_data);
    data->font = bitmap_load(font_data);

    for(int i = 0; i < 200; i++) {
      data->cat_positions_x[i] = vm_rand_range(0, 300);
      data->cat_positions_y[i] = vm_rand_range(0, 300);
    }
    data->current_speed = 2;
    snake_init(&data->snake);

    point_random(&data->fruit);

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
          u32 source_image_height,
          u32 src_x, u32 src_y,
          u32 src_width, u32 src_height) {

  u8* dst = dest + ((dest_y * (FB_WIDTH * 4)) + dest_x * 4); 
  u8* src = source + ((source_image_height - 1 - src_y) * source_pitch) + (src_x * 4);

  for(int y = 0; y < src_height; y++) {
    for(int x = 0; x < src_width; x++) {
      u32 pixel = *(u32*)(src + (x * 4));
      u8 a = (pixel >> 24);
      u8 r = (pixel >> 16);
      u8 g = (pixel >> 8);
      u8 b = (pixel >> 0);
      
      if(a > 0) {
        *(u32*)(dst + (x * 4)) = (a << 24) + (b << 16) + (g << 8) + r;
        
      }
    }
    dst += FB_WIDTH * 4;
    src -= source_pitch;
  }
}

void blit_bitmap(u8* dest,
                 u32 dest_x, u32 dest_y,
                 Bitmap* src,
                 u32 src_x, u32 src_y,
                 u32 src_width, u32 src_height) {
  
    blit(dest, dest_x, dest_y, src->data, src->width * 4, src->height, src_x, src_y, src_width, src_height);
}

void blit_glyph(u8* dest, Bitmap* font, u32 dest_x, u32 dest_y, u32 x_index, u32 y_index) {
  // vm_print_int(x_index);
  // vm_print_int(y_index);
  blit_bitmap(dest, dest_x, dest_y, font, x_index * 6, y_index * 10, 6, 10);  
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

Point get_glyph_index(char c) {
  u32 offset = 0;
  if(is_lowercase_letter(c)) {
     offset = (c - 'a') + (13 * 2);
  }
  else if(is_uppercase_letter(c)) {
     offset = (c - 'A');
  }
  else if(is_number(c)) {
     offset = (c - '0') + (13 * 4);
  }
  else {
    return (Point){0};
  } 

  u32 x = offset % 13;
  u32 y = offset / 13;
  return (Point){x, y};
}

void draw_glyph(u8* framebuffer, Bitmap* font, u32 x, u32 y, char glyph) {
  Point p = get_glyph_index(glyph);
  blit_glyph(framebuffer, font, x, y, p.x, p.y);
}


void draw_string(u8* framebuffer, Bitmap* font, u32 x, u32 y, char* str, u32 len) {
  u32 x_offset = 0;  
  u32 y_offset = 0;
  char* s = str;
  u32 n = 0;
  while(n < len) {
    if(*s == '\n') {
      y_offset += 10;
      x_offset = 0;
      s++;
    } else if(*s == ' ') {
      x_offset += 6;
      s++;
    }
    else {
      draw_glyph(framebuffer, font, x + x_offset, y + y_offset, *s);
      x_offset += 6;
      s++;
    }
    n++;
  }
}
void draw_cstring(u8* framebuffer, Bitmap* font, u32 x, u32 y, char* str) {
  draw_string(framebuffer, font, x,y, str, cstr_len(str));
}

int itoa(int value, char *sp, int radix)
{
    char tmp[32];// be careful with the length of the buffer
    char *tp = tmp;
    int i;
    unsigned v;

    int sign = (radix == 10 && value < 0);    
    if (sign)
        v = -value;
    else
        v = (unsigned)value;

    while (v || tp == tmp)
    {
        i = v % radix;
        v /= radix;
        if (i < 10)
          *tp++ = i+'0';
        else
          *tp++ = i + 'a' - 10;
    }

    int len = tp - tmp;

    if (sign) 
    {
        *sp++ = '-';
        len++;
    }

    while (tp > tmp)
        *sp++ = *--tp;

    return len;
}

void draw_number(u8* framebuffer, Bitmap* font, u32 x, u32 y, int num) {
  char num_string[32] = {0};
  int str_len = itoa(num, num_string, 10);
  draw_string(framebuffer, font, x, y, num_string, str_len);

}

void run(Game_Data* game, u32 framebuffer_width, u32 framebuffer_height) {

  game->frame_count += 1;
  int64_t then = vm_get_time_ms();
  game->test += 1;
  // 
  //cstr_print("Hello from rint64_t numun!\n");
  //fill_framebuffer(game->framebuffer, 0, 255, 255, 255);
  // render_weird_gradient(game->framebuffer, global_xoffset, global_yoffset);
  vm_clear(game->framebuffer, 0, 0, 200);
  if(vm_get_key(KEYCODE_UP)) {
    if(game->position_y - game->current_speed + 16 > 0) {
      game->position_y -= game->current_speed;
    }
  }

  if(vm_get_key(KEYCODE_DOWN)) {
    if((game->position_y + game->current_speed + 16) < FB_HEIGHT) {
      game->position_y += game->current_speed;
      
    }
  }

  if(vm_get_key(KEYCODE_LEFT)) {
    if(game->position_x - game->current_speed - 16 > 0) {
      game->position_x -= game->current_speed;
    }
  }

  if(vm_get_key(KEYCODE_RIGHT)) {
    if((game->position_x + game->current_speed + 16) < FB_WIDTH) {
      game->position_x += game->current_speed;
    }
  }
  if(vm_get_key(KEYCODE_X)) {
    vm_shutdown();
  }
  /*
  vm_print_int(game->position_x);
  vm_print_int(game->position_y);
  */
  //draw_rectangle(game->framebuffer, game->position_x, game->position_y, 16, 16, 250 ,0, 250);

  
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

    blit_bitmap(game->framebuffer, position_x, position_y, &game->malu, frame * 16, 0, 16, 16);
    
  }
  //vm_draw_rect_rgb(game->framebuffer, game->position_x, game->position_y, 16, 17, 0, 0, 0);

  global_xoffset += 4;
  global_yoffset += 4;

  i64 time_passed = vm_get_time_ms() - then;
  //cstr_print("Time passed: ");
  //vm_print_int64(time_passed);
  //cstr_print("\n");
  last_time_passed = time_passed;

  if(vm_get_key(KEYCODE_DOWN)) {
    game->snake.direction = (Point) {0, 1};
  }
  if(vm_get_key(KEYCODE_UP)) {
    game->snake.direction = (Point) {0, -1};
  }
  if(vm_get_key(KEYCODE_RIGHT)) {
    game->snake.direction = (Point) {1, 0};
  }
  if(vm_get_key(KEYCODE_LEFT)) {
    game->snake.direction = (Point) {-1, 0};
  }

  if(game->frame_count % 2 == 0) { 
    snake_update(&game->snake);
    Point* head = snake_head(&game->snake);
    Point* fruit = &game->fruit;

    if(head->x == fruit->x && head->y == fruit->y) {
      Point* tail = snake_tail(&game->snake);
      snake_push_segment(&game->snake, *tail);
      point_random(&game->fruit);
    }
  }

  // vm_print_int(game->snake.x);
  // vm_print_int(snake_head(&game->snake)->x);
  // vm_print_int(snake_head(&game->snake)->y);
  // blit(game->framebuffer, 0, 0, game->clouds.data, game->clouds.width * 4, 0, 0, game->clouds.width , game->clouds.height);

  snake_draw(&game->snake, game->framebuffer);
  fruit_draw(game->framebuffer, &game->fruit);
  

  /*
  cstr_print("Random number: ");
  vm_print_int(vm_rand_range(-10, 10));
  cstr_print("\n");
  */
  //blit(game->framebuffer, 16, 16, game->font.data, game->font.width * 4, 0, 0, game->font.width , game->font.height);
  ///vm_print_int(*(u8*)(game->font.data + 3));
  // blit_bitmap(game->framebuffer, 0, 0, &game->font, 0, 0, 6, 10);
  // blit_bitmap(game->framebuffer, 6, 0, &game->font, 6, 0, 6, 10);
  //draw_glyph(game->framebuffer, &game->font, 0, 0, 'C');
  //draw_cstring(game->framebuffer, &game->font, 0, 0, "HES VM praesentiert\nHallo Welt 123450\nPunktestand 500");
  
  draw_number(game->framebuffer, &game->font, 0, 0, time_passed);
  draw_number(game->framebuffer, &game->font, 20, 20, (i32)game->test);

  vm_paint(game->framebuffer, FB_WIDTH, FB_HEIGHT);
 
}
