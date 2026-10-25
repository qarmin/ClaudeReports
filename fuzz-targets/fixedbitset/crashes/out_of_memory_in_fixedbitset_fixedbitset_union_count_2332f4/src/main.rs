use fixedbitset::FixedBitSet;

fn main() {
    let fixed_bit_set = FixedBitSet::with_capacity_and_blocks::<Vec<usize>>(0usize, vec![]);
    let _ = fixed_bit_set.union_count(&FixedBitSet::with_capacity(1000000000000000000usize));
}
