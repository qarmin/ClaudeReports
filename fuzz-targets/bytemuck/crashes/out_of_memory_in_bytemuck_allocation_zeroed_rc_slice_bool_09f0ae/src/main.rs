fn main() {
    let _ = bytemuck::allocation::zeroed_rc_slice::<bool>(1000000000000000000usize);
}
