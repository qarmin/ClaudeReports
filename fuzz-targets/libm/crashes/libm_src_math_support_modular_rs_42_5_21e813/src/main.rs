fn main() {
    let _ = libm::support::linear_mul_reduction::<u16>(0u16, u32::MAX, 1000u16);
}
