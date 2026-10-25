use fixedbitset::FixedBitSet;

fn main() {
    let fixed_bit_set = FixedBitSet::with_capacity(usize::MAX);
    let _ = fixed_bit_set.ones();
}
