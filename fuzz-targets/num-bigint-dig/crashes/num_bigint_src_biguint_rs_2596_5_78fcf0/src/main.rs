use num_bigint_dig::BigInt;

fn main() {
    let big_int = BigInt::from_signed_bytes_le(&vec![176u8]);
    let _ = big_int.to_radix_be(1000000000u32);
}
