use flate2::Compress;
use flate2::Compression;

fn main() {
    let mut compress = Compress::new(Compression::default(), false);
    let _ = compress.set_level(Compression::new(1000000000u32));
}
