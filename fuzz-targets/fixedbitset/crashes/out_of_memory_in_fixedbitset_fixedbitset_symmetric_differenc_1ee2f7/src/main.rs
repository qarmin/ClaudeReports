use fixedbitset::FixedBitSet;

fn main() {
    let mut fixed_bit_set = FixedBitSet::with_capacity_and_blocks::<Vec<usize>>(0usize, vec![]);
    fixed_bit_set.symmetric_difference_with(&FixedBitSet::with_capacity(usize::MAX));
}
