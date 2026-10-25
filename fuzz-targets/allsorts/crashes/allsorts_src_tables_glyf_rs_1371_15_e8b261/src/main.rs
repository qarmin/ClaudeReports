use allsorts::tables::glyf::Point;

fn main() {
    let _ = <allsorts::tables::glyf::Point as ::core::ops::Add<allsorts::tables::glyf::Point>>::add(Point(i16::MAX, 0i16), Point(1i16, 0i16));
}
