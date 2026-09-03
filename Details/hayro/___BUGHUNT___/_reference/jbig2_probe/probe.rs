use std::env;
use std::fs;
use std::time::Instant;

struct NullDecoder;
impl hayro_jbig2::Decoder for NullDecoder {
    fn push_pixel(&mut self, _black: bool) {}
    fn push_pixel_chunk(&mut self, _black: bool, _chunk_count: u32) {}
    fn next_line(&mut self) {}
}

fn main() {
    let path = env::args().nth(1).expect("path");
    let data = fs::read(&path).unwrap();
    let t0 = Instant::now();
    match hayro_jbig2::Image::new(&data) {
        Ok(image) => {
            println!("parsed ok: width={} height={} in {:?}", image.width(), image.height(), t0.elapsed());
            let t1 = Instant::now();
            let r = image.decode(&mut NullDecoder);
            println!("decode result: {:?} in {:?}", r.is_ok(), t1.elapsed());
        }
        Err(e) => {
            println!("parse error: {:?} in {:?}", e, t0.elapsed());
        }
    }
}
