__attribute__((import_module("env"), import_name("dbg_print_string"))) void vm_print_string(char* ptr, int size);
__attribute__((import_module("env"), import_name("dbg_print_u32"))) void vm_print_u32(int data);

typedef int size_t;

struct Bump_Allocator {
    void* base;    
    size_t offset;
};

void* bump_alloc(struct Bump_Allocator* alloc, size_t size) {
  void* ptr = alloc->base + alloc->offset;
  alloc->offset += size;
  return ptr;
}

void memcpy_slow(const char* src, char* dst, size_t size) {
  for(size_t i = 0; i < size; ++i) {
      dst[i] = src[i];
  }
}

size_t cstr_len(const char* str) {
  size_t counter = 0;
  for(;;) {
    if(str[counter] == 0) {
      vm_print_u32(counter);
      return counter;
    }
    counter++;
  }
}

typedef struct Str {
  char* data;
  size_t len;
} Str;

void print_numbers(int a, int b, int c) {
  vm_print_u32(a);
  vm_print_u32(b);
  vm_print_u32(c);
}

void cstr_print(const char* str) {
  size_t l = cstr_len(str);
  vm_print_string(str, l);
}

Str str_alloc_init(struct Bump_Allocator* alloc, const char* data) {
  size_t l = cstr_len(data);
  vm_print_u32(l);
  if(l == 0) {
    cstr_print("Empty!\n");
    Str s = {0, 0};
    return s;
  }
  size_t len = l;
  char* str_data = bump_alloc(alloc, len);
  memcpy_slow(data, str_data, len);
  Str s = {str_data, len};
  return s; 
} 

Str str_alloc_concat(struct Bump_Allocator* alloc, Str a, Str b) {
  size_t new_len = a.len + b.len;
  char* data = (char*)bump_alloc(alloc, new_len);
  memcpy_slow(a.data, data, a.len);  
  memcpy_slow(b.data, (data + a.len), b.len);
  Str res = {data, new_len};
  return res;
}

void str_print(Str str) {
  vm_print_string(str.data, str.len);
}

void str_println(Str str) {
  vm_print_string(str.data, str.len);
  vm_print_string("\n", 1);  
}

void run() {
  char mem[1028];
  struct Bump_Allocator alloc = {(void*)mem, 0}; 
  vm_print_u32(cstr_len("hallo!"));
  const char* message = "abc"; 
  vm_print_u32(cstr_len(message));

  print_numbers(1, 2, 3);
  Str str_a = str_alloc_init(&alloc, "abc");
  Str str_b = str_alloc_init(&alloc, "def\n");
  Str str_c = str_alloc_concat(&alloc, str_a, str_b);
  str_println(str_c);
   
  //print_u32(str_a.len);
  
}
