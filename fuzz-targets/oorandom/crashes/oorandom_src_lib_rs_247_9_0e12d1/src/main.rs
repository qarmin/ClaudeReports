use oorandom::Rand64;

fn main() {
    let u128 = Rand64::DEFAULT_INC;
    let mut rand64 = Rand64::new(u128);
    let _ = rand64.rand_range(10000000000000000000u64..3145772903569345471u64);
}
