use allsorts::tables::F2Dot14;
use allsorts::tables::Fixed;

fn main() {
    let _ = F2Dot14::from(<allsorts::tables::Fixed as ::core::ops::Div<allsorts::tables::Fixed>>::div(Fixed::from(0.0f32), Fixed::from(0.0f32)));
}
