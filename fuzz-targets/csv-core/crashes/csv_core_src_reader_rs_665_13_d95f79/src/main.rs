use csv_core::ReaderBuilder;

fn main() {
    let reader_builder = ReaderBuilder::new();
    let mut reader = reader_builder.build();
    reader.set_line(u64::MAX);
    let _ = reader.read_record(&vec![10u8], &mut vec![0u8], &mut vec![0usize]);
}
