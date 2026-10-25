use num_bigint_dig::BigUint;

fn main() {
    let _ = num_bigint_dig::prime::probably_prime_miller_rabin(&BigUint::from(1usize), 0usize, false);
}
