use csv_core::Reader;

fn main() {
    let mut reader = Reader::new();
    reader.set_line(u64::MAX);
    let _ = reader.read_field(&vec![10u8], &mut vec![73u8]);
}
