use rav1e::bench::mc::MotionVector;

fn main() {
    let _ = <rav1e::bench::mc::MotionVector as ::core::ops::Shl<u8>>::shl(MotionVector { row: 0i16, col: 0i16 }, 53u8);
}
