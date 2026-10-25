use csv::WriterBuilder;

fn main() {
    let mut writer_builder = WriterBuilder::default();
    let _ = writer_builder.buffer_capacity(1000000000000000000usize);
    let writer = writer_builder.from_writer::<Vec<u8>>(vec![]);
    let _ = writer.get_ref();
}
