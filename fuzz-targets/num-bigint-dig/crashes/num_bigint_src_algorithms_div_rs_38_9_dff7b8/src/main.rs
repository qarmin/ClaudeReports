use num_bigint_dig::BigUint;

fn main() {
    let _ = num_bigint_dig::algorithms::div_rem(&BigUint::from_bytes_be(&vec![]), &BigUint::default());
}
