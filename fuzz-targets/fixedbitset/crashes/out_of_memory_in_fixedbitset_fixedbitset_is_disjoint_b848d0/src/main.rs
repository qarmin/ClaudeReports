use fixedbitset::FixedBitSet;

fn main() {
    let fixed_bit_set = FixedBitSet::default();
    let _ = fixed_bit_set.is_disjoint(&FixedBitSet::with_capacity(usize::MAX));
}
