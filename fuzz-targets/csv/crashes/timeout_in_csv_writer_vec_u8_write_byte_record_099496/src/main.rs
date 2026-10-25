use csv::ByteRecord;
use csv::WriterBuilder;

fn main() {
    let mut writer_builder = WriterBuilder::default();
    let _ = writer_builder.buffer_capacity(0usize);
    let mut writer = writer_builder.from_writer::<Vec<u8>>(vec![]);
    let _ = writer.write_byte_record(&ByteRecord::default());
}
