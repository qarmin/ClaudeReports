fn main() {
    let _ = bytemuck::allocation::zeroed_rc_slice::<char>(10000000000000000000usize);
}
