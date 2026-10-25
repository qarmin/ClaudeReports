use qoi::Channels;
use qoi::ColorSpace;
use qoi::Header;

fn main() {
    let header = Header { width: u32::MAX, height: u32::MAX, channels: Channels::Rgb, colorspace: ColorSpace::default() };
    let _ = header.n_bytes();
}
