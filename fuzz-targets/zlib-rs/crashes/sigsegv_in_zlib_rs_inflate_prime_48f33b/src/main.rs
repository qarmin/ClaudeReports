use zlib_rs::InflateConfig;
use zlib_rs::inflate::InflateStream;

fn main() {
    let mut inflate_stream = InflateStream::new(InflateConfig { window_bits: 0i32 });
    let _ = zlib_rs::inflate::end(&mut inflate_stream);
    let _ = zlib_rs::inflate::prime(&mut inflate_stream, 1i32, 0i32);
}
