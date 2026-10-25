use num_bigint_dig::BigUint;

fn main() {
    let big_uint = BigUint::new(vec![1u32]);
    let _ = big_uint.to_radix_le(1u32);
}
