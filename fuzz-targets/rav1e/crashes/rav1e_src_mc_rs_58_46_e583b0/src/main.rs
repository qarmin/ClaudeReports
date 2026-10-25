use rav1e::bench::mc::MotionVector;

fn main() {
    let _ = <rav1e::bench::mc::MotionVector as ::core::ops::Mul<i16>>::mul(MotionVector { row: 0i16, col: -5549i16 }, 10i16);
}
