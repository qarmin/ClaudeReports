use csv_core::WriterBuilder;

fn main() {
    let writer_builder = WriterBuilder::default();
    let mut writer = writer_builder.build();
    let _ = writer.field(&vec![10u8], &mut vec![]);
    let _ = writer.terminator(&mut vec![]);
}
