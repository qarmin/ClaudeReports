use num_bigint_dig::BigInt;
use num_bigint_dig::BigUint;
use num_bigint_dig::Sign;

fn main() {
    let big_int = BigInt::from_biguint(Sign::Minus, BigUint::from(0u8));
    let _ = num_bigint_dig::algorithms::jacobi(&big_int, &BigInt::from_signed_bytes_be(&vec![]));
}
