use bytesize::Unit;

fn main() {
    let _ = <bytesize::Unit as ::core::ops::Add<u64>>::add(Unit::TebiByte, u64::MAX);
}
