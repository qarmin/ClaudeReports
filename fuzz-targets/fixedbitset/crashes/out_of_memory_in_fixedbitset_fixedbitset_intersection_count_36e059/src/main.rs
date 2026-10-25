use fixedbitset::FixedBitSet;

fn main() {
    let fixed_bit_set = FixedBitSet::new();
    let _ = fixed_bit_set.intersection_count(&FixedBitSet::with_capacity_and_blocks::<Vec<usize>>(usize::MAX, vec![11usize, 2usize]));
}
