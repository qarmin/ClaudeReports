use num_bigint::BigInt;
use num_bigint::Sign;

fn main() {
    let _ = BigInt::from_radix_le(Sign::NoSign, &vec![], 0u32);
}
