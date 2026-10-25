use fixedbitset::FixedBitSet;

fn main() {
    let fixed_bit_set = FixedBitSet::with_capacity_and_blocks::<Vec<usize>>(usize::MAX, vec![1usize, 256usize, 2147483648usize, 255usize, usize::MAX, usize::MAX]);
    let fixed_bit_set2 = FixedBitSet::with_capacity(0usize);
    let _ = fixed_bit_set.symmetric_difference(&fixed_bit_set2);
}
