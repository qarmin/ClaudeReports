use num_bigint_dig::BigInt;

fn main() {
    let _ = num_bigint_dig::prime::probably_prime(&<num_bigint_dig::BigInt as num_bigint_dig::IntoBigUint>::into_biguint(<num_bigint_dig::BigInt as ::core::ops::Rem<isize>>::rem(BigInt::from(0i8), 0isize)).unwrap(), 0usize);
}
