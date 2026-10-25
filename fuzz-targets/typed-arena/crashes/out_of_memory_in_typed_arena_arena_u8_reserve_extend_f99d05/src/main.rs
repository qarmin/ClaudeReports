fn main() {
    let arena = <typed_arena::Arena<u8>>::new();
    arena.reserve_extend(2147483648usize);
}
