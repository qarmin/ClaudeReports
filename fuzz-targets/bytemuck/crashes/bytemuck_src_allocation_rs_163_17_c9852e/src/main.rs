fn main() {
    let _ = bytemuck::allocation::zeroed_arc_slice::<u8>(usize::MAX);
}
