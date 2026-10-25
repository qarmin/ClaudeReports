use num_bigint_dig::BigInt;

fn main() {
    let big_int = <num_bigint_dig::BigInt as ::core::ops::Rem<u8>>::rem(BigInt::from(0u32), 0u8);
    let _ = big_int.to_str_radix(0u32);
}
