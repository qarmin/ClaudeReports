use rav1e::bench::mc::MotionVector;

fn main() {
    let _ = <rav1e::bench::mc::MotionVector as ::core::ops::Mul<u16>>::mul(MotionVector { row: 0i16, col: 10000i16 }, 10000u16);
}
