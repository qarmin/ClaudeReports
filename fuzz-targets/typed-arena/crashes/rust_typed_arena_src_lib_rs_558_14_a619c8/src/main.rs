fn main() {
    let arena = <typed_arena::Arena<String>>::new();
    let _ = <typed_arena::Arena<String> as ::core::default::Default>::default();
    arena.reserve_extend(10000000000000000000usize);
}
