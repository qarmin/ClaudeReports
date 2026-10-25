use allsorts::tables::Fixed;

fn main() {
    let _ = <allsorts::tables::Fixed as ::core::ops::Neg>::neg(Fixed::from_raw(i32::MIN));
}
