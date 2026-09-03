use std::hint::black_box;

fn main() {
    assert_eq!(black_box(f32::INFINITY) as i128, i128::MAX);
    assert_eq!(black_box(f32::NEG_INFINITY) as i128, i128::MIN);
    assert_eq!(black_box(f32::INFINITY) as u64, u64::MAX);
    assert_eq!(black_box(f32::INFINITY) as i64, i64::MAX);
    assert_eq!(black_box(f32::INFINITY) as u32, u32::MAX);
    assert_eq!(black_box(f64::INFINITY) as u128, u128::MAX);
    assert_eq!(black_box(f64::INFINITY) as i128, i128::MAX);
    println!("ok");
}
