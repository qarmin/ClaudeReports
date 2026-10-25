use fixedbitset::FixedBitSet;

fn main() {
    let fixed_bit_set = FixedBitSet::new();
    let _ = fixed_bit_set.difference_count(&FixedBitSet::with_capacity_and_blocks::<Vec<usize>>(10000000000000000000usize, vec![]));
}
