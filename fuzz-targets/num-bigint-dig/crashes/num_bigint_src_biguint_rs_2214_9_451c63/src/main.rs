use num_bigint_dig::BigInt;
use num_bigint_dig::Sign;

fn main() {
    let _ = BigInt::from_radix_be(Sign::Minus, &vec![], 0u32);
}
