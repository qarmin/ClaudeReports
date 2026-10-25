use num_bigint_dig::BigUint;

fn main() {
    let _ = num_bigint_dig::prime::probably_prime_miller_rabin(&<num_bigint_dig::BigUint as ::core::ops::Div<u64>>::div(BigUint::new(vec![45u32]), 21u64), 1usize, false);
}
