__attribute__((import_module("env"), import_name("dbg_print_string"))) void print_string(char* ptr, int size);

void run() {
  print_string("hello world!\n", 13);
}
