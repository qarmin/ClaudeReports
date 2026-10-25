use fixedbitset::FixedBitSet;

fn main() {
    let mut fixed_bit_set = FixedBitSet::default();
    fixed_bit_set.difference_with(&FixedBitSet::with_capacity_and_blocks::<Vec<usize>>(1000000000000000000usize, vec![]));
}
