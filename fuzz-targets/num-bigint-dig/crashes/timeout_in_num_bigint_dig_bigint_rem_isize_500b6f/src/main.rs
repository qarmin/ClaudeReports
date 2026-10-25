use num_bigint_dig::BigInt;

fn main() {
    let _ = <num_bigint_dig::BigInt as ::core::ops::Rem<isize>>::rem(BigInt::from_signed_bytes_le(&vec![]), 0isize);
}
