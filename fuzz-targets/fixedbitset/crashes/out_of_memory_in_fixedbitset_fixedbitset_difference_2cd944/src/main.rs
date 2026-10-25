use fixedbitset::FixedBitSet;

fn main() {
    let fixed_bit_set = FixedBitSet::default();
    let fixed_bit_set2 = FixedBitSet::with_capacity(10000000000000000000usize);
    let _ = fixed_bit_set.difference(&fixed_bit_set2);
}
