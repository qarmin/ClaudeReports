use lopdf::xref::XrefEntry;

fn main() {
    let xref_entry = XrefEntry::Compressed { container: 0u32, index: 0u16 };
    let _ = xref_entry.encode_for_xref_stream(&[10000000000000000000usize, 2147483648usize, 5830499564928007129usize]);
}
