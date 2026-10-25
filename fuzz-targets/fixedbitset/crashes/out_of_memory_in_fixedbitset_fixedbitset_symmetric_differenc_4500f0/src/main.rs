use fixedbitset::FixedBitSet;

fn main() {
    let fixed_bit_set = FixedBitSet::with_capacity_and_blocks::<Vec<usize>>(0usize, vec![]);
    let _ = fixed_bit_set.symmetric_difference_count(&FixedBitSet::with_capacity(usize::MAX));
}
