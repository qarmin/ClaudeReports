use csv::WriterBuilder;

fn main() {
    let mut writer_builder = WriterBuilder::new();
    let _ = writer_builder.buffer_capacity(10000000000000000000usize);
    let _ = writer_builder.from_writer::<Vec<u8>>(include_bytes!("image_0.jpg").to_vec());
}
