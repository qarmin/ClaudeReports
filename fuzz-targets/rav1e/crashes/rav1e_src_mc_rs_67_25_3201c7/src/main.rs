use rav1e::bench::mc::MotionVector;

fn main() {
    let _ = <rav1e::bench::mc::MotionVector as ::core::ops::Mul<u16>>::mul(MotionVector { row: 100i16, col: 0i16 }, 10000u16);
}
