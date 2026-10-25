fn main() {
    let _ = bytemuck::allocation::zeroed_arc_slice::<bool>(1000000000000000000usize);
}
