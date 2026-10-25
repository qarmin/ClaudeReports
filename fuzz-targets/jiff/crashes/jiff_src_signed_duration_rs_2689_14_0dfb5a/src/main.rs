use jiff::SignedDuration;

fn main() {
    let _ = <jiff::SignedDuration as ::core::ops::Div<i32>>::div(SignedDuration::from_millis(0i64), 0i32);
}
