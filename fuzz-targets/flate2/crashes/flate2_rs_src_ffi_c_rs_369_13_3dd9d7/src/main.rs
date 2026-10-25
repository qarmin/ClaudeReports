use flate2::Compression;

fn main() {
    let _ = <flate2::write::GzEncoder<Vec<u8>>>::new(vec![], Compression::new(1000u32));
}
