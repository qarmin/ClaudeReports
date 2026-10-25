use rawler::bits::LookupTable;

fn main() {
    let _ = LookupTable::new_with_bits(&[0u16], 10000u32);
}
