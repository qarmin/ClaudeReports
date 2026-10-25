fn main() {
    let vec = triple_accel::alloc_str(65536usize);
    let _ = triple_accel::rdamerau(&vec, &vec![]);
}
