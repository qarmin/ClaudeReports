use fixedbitset::FixedBitSet;
use fixedbitset::IntoOnes;

fn main() {
    let usize_v = IntoOnes::last_positive_bit_and_unset(&mut 1usize);
    let fixed_bit_set = FixedBitSet::new();
    let _ = fixed_bit_set.is_subset(&FixedBitSet::with_capacity_and_blocks::<Vec<usize>>(10000000000000000000usize, vec![2147483648usize, 4294967296usize, 13usize, usize::MAX, 2usize, usize_v]));
}
