use rav1e::bench::mc::MotionVector;

fn main() {
    let _ = <rav1e::bench::mc::MotionVector as ::core::ops::Add<rav1e::bench::mc::MotionVector>>::add(MotionVector { row: i16::MIN, col: 0i16 }, MotionVector { row: -10000i16, col: 0i16 });
}
