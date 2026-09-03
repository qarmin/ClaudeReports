//! Self-contained benchmark for `Pixmap::into_png`'s PNG compression
//! settings. Real, path-dependency on `vello_common`. Uses a real rendered
//! page (595x841, premultiplied RGBA8, from the user's own `img.pdf` test
//! file - see `testdata/img_page_rgba.bin`) rather than synthetic noise,
//! since PNG compression ratio (and therefore relative speed of different
//! DEFLATE backends) is very sensitive to real content patterns.
//!
//! `into_png()` itself doesn't expose a compression-level parameter, so
//! this replicates its ~4-line body locally, varying only
//! `encoder.set_compression(...)`, to compare candidates against the real,
//! unmodified `take_unpremultiplied()` from the crate under test.
//!
//! Run with: `cargo run --release -- [iterations]`

use std::time::{Duration, Instant};
use vello_common::peniko::color::Rgba8;
use vello_common::pixmap::Pixmap;

const RAW: &[u8] = include_bytes!("../testdata/img_page_rgba.bin");

fn load_pixmap() -> Pixmap {
    let width = u32::from_le_bytes(RAW[0..4].try_into().unwrap()) as u16;
    let height = u32::from_le_bytes(RAW[4..8].try_into().unwrap()) as u16;
    let bytes = &RAW[8..];
    let premul: Vec<vello_common::peniko::color::PremulRgba8> = bytemuck::cast_slice(bytes).to_vec();
    Pixmap::from_parts(premul, width, height)
}

fn encode_with(pixmap: &Pixmap, compression: png::Compression) -> Vec<u8> {
    let mut data = Vec::new();
    let mut encoder = png::Encoder::new(&mut data, pixmap.width() as u32, pixmap.height() as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_compression(compression);
    let mut writer = encoder.write_header().unwrap();
    writer
        .write_image_data(bytemuck::cast_slice(&pixmap.clone().take_unpremultiplied()))
        .unwrap();
    writer.finish().unwrap();
    data
}

fn decode_to_rgba(png_bytes: &[u8]) -> Vec<Rgba8> {
    let decoder = png::Decoder::new(std::io::Cursor::new(png_bytes));
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0u8; reader.output_buffer_size().unwrap()];
    reader.next_frame(&mut buf).unwrap();
    bytemuck::cast_slice(&buf).to_vec()
}

fn run_bench(name: &str, pixmap: &Pixmap, compression: png::Compression, iterations: usize) -> Vec<u8> {
    let mut durations = Vec::with_capacity(iterations);
    let mut out = Vec::new();
    for _ in 0..iterations {
        let t = Instant::now();
        out = encode_with(pixmap, compression);
        durations.push(t.elapsed());
    }
    let warmup = (iterations / 10).max(1);
    let mut stable: Vec<Duration> = durations[warmup..].to_vec();
    stable.sort();
    let n = stable.len();
    println!(
        "[{name:10}] min={:>8.3?} p50={:>8.3?} mean={:>8.3?} max={:>8.3?}  output={} bytes",
        stable[0],
        stable[n / 2],
        stable.iter().sum::<Duration>() / n as u32,
        stable[n - 1],
        out.len(),
    );
    out
}

fn main() {
    let iterations: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(30);

    let pixmap = load_pixmap();
    println!(
        "input: {}x{} real rendered page (img.pdf), {iterations} iterations",
        pixmap.width(),
        pixmap.height()
    );

    let baseline = run_bench("Balanced", &pixmap, png::Compression::Balanced, iterations);
    let fast = run_bench("Fast", &pixmap, png::Compression::Fast, iterations);
    let fastest = run_bench("Fastest", &pixmap, png::Compression::Fastest, iterations);
    let no_compression = run_bench("NoCompression", &pixmap, png::Compression::NoCompression, iterations);
    let high = run_bench("High", &pixmap, png::Compression::High, iterations);

    println!();
    println!("Correctness: decoding every candidate's PNG and comparing pixels to the baseline's decode:");
    let baseline_pixels = decode_to_rgba(&baseline);
    for (name, bytes) in [
        ("Fast", &fast),
        ("Fastest", &fastest),
        ("NoCompression", &no_compression),
        ("High", &high),
    ] {
        let pixels = decode_to_rgba(bytes);
        let identical = pixels == baseline_pixels;
        println!("  {name:14}: decoded pixels identical to Balanced = {identical}");
    }
}
