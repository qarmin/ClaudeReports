use fixedbitset::IntoOnes;

fn main() {
    let _ = IntoOnes::last_positive_bit_and_unset(&mut 0usize);
}
