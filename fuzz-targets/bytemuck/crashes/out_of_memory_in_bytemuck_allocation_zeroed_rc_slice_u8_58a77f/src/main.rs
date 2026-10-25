fn main() {
    let _ = bytemuck::allocation::zeroed_rc_slice::<u8>(4294967296usize);
}
