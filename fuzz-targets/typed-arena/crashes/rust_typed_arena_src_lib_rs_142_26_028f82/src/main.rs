fn main() {
    let _ = <typed_arena::Arena<u32>>::new();
    let _ = <typed_arena::Arena<u8>>::with_capacity(0usize);
    let _ = <typed_arena::Arena<u8>>::with_capacity(10000000000000000000usize);
}
