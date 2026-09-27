use std::hint::black_box;

fn main() {
    assert_eq!(black_box(1e38f32) as i128, 99999996802856924650656260769173209088i128);
    assert_eq!(black_box(f32::INFINITY) as i128, i128::MAX);
    println!("ok");
}
