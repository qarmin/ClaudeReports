use jiff::SignedDuration;

fn main() {
    let _ = <jiff::SignedDuration as ::core::ops::Neg>::neg(SignedDuration::MIN);
}
