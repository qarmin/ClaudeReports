use num_bigint_dig::BigUint;

fn main() {
    let _ = num_bigint_dig::algorithms::div_rem_digit(<num_bigint_dig::BigUint as ::core::ops::Shl<usize>>::shl(BigUint::from_bytes_be(&vec![]), usize::MAX), 1u64);
}
