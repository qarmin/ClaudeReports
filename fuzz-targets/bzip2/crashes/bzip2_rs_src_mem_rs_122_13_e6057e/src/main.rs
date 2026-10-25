use bzip2::Compress;
use bzip2::Compression;

fn main() {
    let compression = Compression::best();
    let _ = Compress::new(compression, 1000000000u32);
}
