use oorandom::Rand32;

fn main() {
    let mut rand32 = Rand32::new_inc(0u64, 0u64);
    let _ = rand32.rand_range(0u32..0u32);
}
