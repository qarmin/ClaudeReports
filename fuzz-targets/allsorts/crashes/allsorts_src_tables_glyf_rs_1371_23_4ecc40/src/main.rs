use allsorts::tables::glyf::Point;

fn main() {
    let _ = <allsorts::tables::glyf::Point as ::core::ops::Add<allsorts::tables::glyf::Point>>::add(Point(0i16, i16::MAX), Point(0i16, 1i16));
}
