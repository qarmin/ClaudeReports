use fixedbitset::FixedBitSet;

fn main() {
    let mut fixed_bit_set = FixedBitSet::with_capacity(0usize);
    fixed_bit_set.intersect_with(&FixedBitSet::with_capacity_and_blocks::<Vec<usize>>(usize::MAX, vec![]));
}
