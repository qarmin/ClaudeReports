use fixedbitset::FixedBitSet;

fn main() {
    let fixed_bit_set = FixedBitSet::new();
    let fixed_bit_set2 = FixedBitSet::with_capacity(1000000000000000000usize);
    let _ = fixed_bit_set.intersection(&fixed_bit_set2);
}
