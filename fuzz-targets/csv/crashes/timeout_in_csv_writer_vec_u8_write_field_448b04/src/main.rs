use csv::WriterBuilder;

fn main() {
    let mut writer_builder = WriterBuilder::new();
    let _ = writer_builder.buffer_capacity(1usize);
    let mut writer = writer_builder.from_writer::<Vec<u8>>(include_bytes!("image_0.jpg").to_vec());
    let _ = writer.write_field::<Vec<u8>>(include_bytes!("rustfmt.toml").to_vec());
}
