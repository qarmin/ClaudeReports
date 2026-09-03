use std::hint::black_box;

fn main() {
    assert_eq!(black_box(f32::INFINITY) as u128, u128::MAX);
    assert_eq!(black_box(f32::INFINITY) as u128, u128::MAX);
    println!("ok");
}
