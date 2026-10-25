fn main() {
    let _ = bytemuck::allocation::zeroed_slice_box::<bool>(usize::MAX);
}
