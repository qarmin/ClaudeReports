// Small translation unit that produces a real .debug_info + .debug_abbrev
// with gcc -g. A couple of functions so the compile-unit's DIE tree and
// abbrev table are non-trivial.
int helper(int x) { return x * 3 + 1; }

int compute(int a, int b) {
  int s = 0;
  for (int i = a; i < b; i++)
    s += helper(i);
  return s;
}

int main(void) { return compute(1, 10); }
