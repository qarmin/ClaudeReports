fn main() {
    let _ = bytemuck::allocation::zeroed_rc_slice::<char>(1000000000usize);
}
