use num_bigint_dig::BigInt;

fn main() {
    let big_int = <num_bigint_dig::BigInt as ::core::ops::Div<i64>>::div(BigInt::default(), 1i64);
    let _ = big_int.get_limb(0usize);
}
