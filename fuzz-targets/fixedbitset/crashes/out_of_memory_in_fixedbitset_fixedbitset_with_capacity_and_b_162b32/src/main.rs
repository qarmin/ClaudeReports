use fixedbitset::FixedBitSet;

fn main() {
    let _ = FixedBitSet::with_capacity_and_blocks::<Vec<usize>>(1000000000000000000usize, vec![]);
}
