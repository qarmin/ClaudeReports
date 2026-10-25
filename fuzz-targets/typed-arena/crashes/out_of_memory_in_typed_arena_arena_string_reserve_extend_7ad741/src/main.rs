fn main() {
    let arena = <typed_arena::Arena<String>>::new();
    arena.reserve_extend(1000000000usize);
}
