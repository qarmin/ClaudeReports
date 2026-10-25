use zlib_rs::InflateConfig;
use zlib_rs::inflate::Window;

fn main() {
    let mut z_stream = zlib_rs::c_api::z_stream::default();
    let _ = zlib_rs::inflate::back_init(&mut z_stream, InflateConfig { window_bits: 1000i32 }, Window::empty());
}
