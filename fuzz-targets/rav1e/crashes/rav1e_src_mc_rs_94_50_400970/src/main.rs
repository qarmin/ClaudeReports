use rav1e::bench::mc::MotionVector;

fn main() {
    let _ = <rav1e::bench::mc::MotionVector as ::core::ops::Add<rav1e::bench::mc::MotionVector>>::add(MotionVector { row: 0i16, col: i16::MAX }, MotionVector { row: 0i16, col: 1i16 });
}
