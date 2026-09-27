// A tiny program with real debug info so gcc emits a .debug_info compile-unit
// header. Content is irrelevant; we only need a valid .debug_info section.
#include <stdio.h>

int add(int a, int b) { return a + b; }
int sub(int a, int b) { return a - b; }

int main(void) {
  printf("%d %d\n", add(2, 3), sub(9, 4));
  return 0;
}
