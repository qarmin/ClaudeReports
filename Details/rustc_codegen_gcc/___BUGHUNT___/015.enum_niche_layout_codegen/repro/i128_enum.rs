#![feature(repr128)]
use std::hint::black_box;

#[repr(i128)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum Big {
    NegOne = -1,
    Zero = 0,
    One = 1,
    Huge = i128::MAX,
    Neg = i128::MIN,
}

fn classify(b: Big) -> i32 {
    match b {
        Big::NegOne => 0,
        Big::Zero => 1,
        Big::One => 2,
        Big::Huge => 3,
        Big::Neg => 4,
    }
}

fn main() {
    assert_eq!(classify(black_box(Big::NegOne)), 0);
    assert_eq!(classify(black_box(Big::Zero)), 1);
    assert_eq!(classify(black_box(Big::One)), 2);
    assert_eq!(classify(black_box(Big::Huge)), 3);
    assert_eq!(classify(black_box(Big::Neg)), 4);
    assert_eq!(black_box(Big::NegOne) as i128, -1);
    assert_eq!(black_box(Big::Huge) as i128, i128::MAX);
    assert_eq!(black_box(Big::Neg) as i128, i128::MIN);
    println!("i128 enum ok");
}
