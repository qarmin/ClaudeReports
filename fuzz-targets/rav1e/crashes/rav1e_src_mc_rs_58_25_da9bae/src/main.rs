use rav1e::bench::mc::MotionVector;

fn main() {
    let _ = <rav1e::bench::mc::MotionVector as ::core::ops::Mul<i16>>::mul(<rav1e::bench::mc::MotionVector as ::core::ops::Shl<u8>>::shl(MotionVector { row: 1i16, col: 0i16 }, 10u8), 100i16);
}
