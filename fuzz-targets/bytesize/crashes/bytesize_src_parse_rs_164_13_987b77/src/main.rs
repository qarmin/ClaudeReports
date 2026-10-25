use bytesize::Unit;

fn main() {
    let _ = <bytesize::Unit as ::core::ops::Mul<u64>>::mul(Unit::GigaByte, 10000000000000000000u64);
}
