use rav1e::bench::mc::MotionVector;

fn main() {
    let _ = <rav1e::bench::mc::MotionVector as ::core::ops::Shr<u8>>::shr(MotionVector::default(), 100u8);
}
