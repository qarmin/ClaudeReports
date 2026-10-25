fn main() {
    let _ = bytemuck::allocation::zeroed_arc_slice::<char>(1000000000usize);
}
