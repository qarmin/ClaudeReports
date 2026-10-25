use fixedbitset::FixedBitSet;
use fixedbitset::IntoOnes;

fn main() {
    let usize_v = IntoOnes::last_positive_bit_and_unset(&mut 1usize);
    let fixed_bit_set = FixedBitSet::new();
    let _ = fixed_bit_set.is_superset(&FixedBitSet::with_capacity_and_blocks::<Vec<usize>>(10000000000000000000usize, vec![65535usize, 14542859564429068131usize, 13991429659670356655usize, usize_v, usize_v]));
}
