use jiff::SignedDuration;

fn main() {
    let _ = <jiff::SignedDuration as ::core::ops::Mul<i32>>::mul(SignedDuration::MIN, 1000000000i32);
}
