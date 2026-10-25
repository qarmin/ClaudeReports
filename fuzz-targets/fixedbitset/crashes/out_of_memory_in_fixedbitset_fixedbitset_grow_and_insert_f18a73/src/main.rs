use fixedbitset::FixedBitSet;

fn main() {
    let mut fixed_bit_set = FixedBitSet::default();
    fixed_bit_set.grow_and_insert(1000000000000000000usize);
}
