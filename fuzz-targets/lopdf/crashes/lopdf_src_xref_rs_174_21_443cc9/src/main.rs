use lopdf::xref::XrefEntry;

fn main() {
    let xref_entry = XrefEntry::Free;
    let _ = xref_entry.encode_for_xref_stream(&[100usize, 0usize, 0usize]);
}
