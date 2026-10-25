use allsorts::tables::Fixed;

fn main() {
    let _ = <allsorts::tables::Fixed as ::core::ops::Add<allsorts::tables::Fixed>>::add(<allsorts::tables::Fixed as allsorts::binary::read::ReadFrom>::read_from(1i32), <allsorts::tables::Fixed as ::core::ops::Div<allsorts::tables::Fixed>>::div(Fixed::from_raw(0i32), Fixed::from(0i32))).abs();
}
