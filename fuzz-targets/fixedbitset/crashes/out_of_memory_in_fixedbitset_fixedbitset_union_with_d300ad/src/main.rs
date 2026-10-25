use fixedbitset::FixedBitSet;

fn main() {
    let mut fixed_bit_set = FixedBitSet::default();
    fixed_bit_set.union_with(&FixedBitSet::with_capacity(1000000000000000000usize));
}
