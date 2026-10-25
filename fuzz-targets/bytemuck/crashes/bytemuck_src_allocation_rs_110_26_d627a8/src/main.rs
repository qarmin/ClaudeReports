fn main() {
    let _ = bytemuck::allocation::zeroed_vec::<bool>(usize::MAX);
}
