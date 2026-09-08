/* Multi-function source designed to produce a non-contiguous DW_AT_ranges
 * entry (a real .debug_rnglists range list) when compiled with
 * -ffunction-sections -gdwarf-5 -O2, so the compile unit has several
 * disjoint address ranges rather than one low_pc/high_pc pair. */

volatile int sink;

__attribute__((noinline)) int func_a(int x) {
    sink += x;
    return x * 2 + 1;
}

__attribute__((noinline)) int func_b(int x) {
    sink -= x;
    return x * 3 - 1;
}

__attribute__((noinline)) int func_c(int x) {
    sink ^= x;
    return x ^ 0x5a5a;
}

__attribute__((noinline)) int func_d(int x) {
    sink *= x;
    return x + x;
}

int main(void) {
    return func_a(1) + func_b(2) + func_c(3) + func_d(4);
}
