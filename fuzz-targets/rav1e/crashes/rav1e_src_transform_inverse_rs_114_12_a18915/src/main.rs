fn main() {
    rav1e::bench::transform::inverse::av1_iflipadst4(&[100000000i32, 0i32, 0i32, 0i32], &mut vec![0i32, 0i32, 0i32, 0i32], 0usize);
}
