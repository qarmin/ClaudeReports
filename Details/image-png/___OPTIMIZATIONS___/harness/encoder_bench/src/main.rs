//! Encoder-side benchmark harness for the `png` crate.
//!
//! The repo ships `benches/decoder.rs` (+ unfilter/adam7/expand_paletted behind the `benchmarks`
//! feature) but nothing at all for the encoder, which is where most of this workspace's backlog
//! lives. This is that harness: one binary with several modes, extended with new modes/flags as
//! new questions come up rather than forked into one bench per finding.
//!
//! What it measures per configuration:
//!   * wall time (min / p50 / mean / p95 / max over N iterations, first 10% discarded as warmup)
//!   * encoded output size, and the compression ratio against the raw pixel buffer
//!   * allocation count, reallocation count and total allocated bytes (a wrapping global
//!     allocator), because several backlog items are allocation-shaped, not time-shaped
//!   * process peak RSS (`VmHWM`), monotonic - see `--only` below
//!
//! Correctness is checked on every iteration, not just once: the encoded bytes must hash
//! identically across iterations (determinism), and the output is decoded back and compared to
//! the source pixels (round trip). A configuration that fails either prints `verify=FAIL` and is
//! not reported as a valid measurement.
//!
//! Run `encoder_bench --help` for usage.

use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use png::{BitDepth, ColorType, DeflateCompression, Decoder, Encoder, Filter};

// ---------------------------------------------------------------------------------------------
// Allocation accounting
// ---------------------------------------------------------------------------------------------

static ALLOCS: AtomicU64 = AtomicU64::new(0);
static REALLOCS: AtomicU64 = AtomicU64::new(0);
static ALLOC_BYTES: AtomicU64 = AtomicU64::new(0);

struct CountingAlloc;

unsafe impl GlobalAlloc for CountingAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        ALLOC_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        System.alloc(layout)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        ALLOC_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        System.alloc_zeroed(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        REALLOCS.fetch_add(1, Ordering::Relaxed);
        ALLOC_BYTES.fetch_add(new_size.saturating_sub(layout.size()) as u64, Ordering::Relaxed);
        System.realloc(ptr, layout, new_size)
    }
}

#[global_allocator]
static GLOBAL: CountingAlloc = CountingAlloc;

#[derive(Clone, Copy, Default)]
struct AllocSnapshot {
    allocs: u64,
    reallocs: u64,
    bytes: u64,
}

impl AllocSnapshot {
    fn now() -> Self {
        Self {
            allocs: ALLOCS.load(Ordering::Relaxed),
            reallocs: REALLOCS.load(Ordering::Relaxed),
            bytes: ALLOC_BYTES.load(Ordering::Relaxed),
        }
    }

    fn since(self, start: Self) -> Self {
        Self {
            allocs: self.allocs - start.allocs,
            reallocs: self.reallocs - start.reallocs,
            bytes: self.bytes - start.bytes,
        }
    }
}

/// Process-wide peak resident set size in KiB, or 0 where `/proc` is unavailable.
///
/// This is a high-water mark for the whole process, so it only distinguishes configurations when
/// the harness runs one at a time (`--only`).
fn peak_rss_kib() -> u64 {
    let Ok(status) = std::fs::read_to_string("/proc/self/status") else {
        return 0;
    };
    for line in status.lines() {
        if let Some(rest) = line.strip_prefix("VmHWM:") {
            return rest
                .trim()
                .trim_end_matches(" kB")
                .trim()
                .parse()
                .unwrap_or(0);
        }
    }
    0
}

// ---------------------------------------------------------------------------------------------
// Image generation
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pattern {
    /// Constant color - compresses to almost nothing, isolates fixed overhead.
    Flat,
    /// Smooth 2D gradient - the easy case every filter handles well.
    Gradient,
    /// Gradient + ripples + a little noise - the realistic middle ground (default).
    Photo,
    /// Uniform noise - incompressible, and the only pattern that reaches
    /// `FdeflateUltraFast`'s stored-output fallback.
    Noise,
}

impl Pattern {
    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "flat" => Pattern::Flat,
            "gradient" => Pattern::Gradient,
            "photo" => Pattern::Photo,
            "noise" => Pattern::Noise,
            _ => return None,
        })
    }

    fn name(self) -> &'static str {
        match self {
            Pattern::Flat => "flat",
            Pattern::Gradient => "gradient",
            Pattern::Photo => "photo",
            Pattern::Noise => "noise",
        }
    }
}

#[derive(Clone, Copy)]
struct ImageSpec {
    width: u32,
    height: u32,
    color: ColorType,
    depth: BitDepth,
}

impl ImageSpec {
    fn channels(&self) -> usize {
        match self.color {
            ColorType::Grayscale | ColorType::Indexed => 1,
            ColorType::GrayscaleAlpha => 2,
            ColorType::Rgb => 3,
            ColorType::Rgba => 4,
        }
    }

    fn bits_per_sample(&self) -> usize {
        self.depth as usize
    }

    /// Same shape as the crate's own `raw_row_length_from_width` minus the filter byte, so it is
    /// correct for the sub-byte depths that real corpus files use, not just 8/16.
    fn row_bytes(&self) -> usize {
        (self.width as usize * self.channels() * self.bits_per_sample()).div_ceil(8)
    }

    fn raw_bytes(&self) -> usize {
        self.row_bytes() * self.height as usize
    }

    fn label(&self) -> String {
        let ct = match self.color {
            ColorType::Grayscale => "gray",
            ColorType::GrayscaleAlpha => "graya",
            ColorType::Rgb => "rgb",
            ColorType::Rgba => "rgba",
            ColorType::Indexed => "indexed",
        };
        format!("{ct}{}", self.depth as u8)
    }
}

/// Deterministic pseudo-random source - same bytes on every run and every machine, so two
/// measurements of the same configuration are comparing the same work.
struct Lcg(u64);

impl Lcg {
    fn next_u16(&mut self) -> u16 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.0 >> 33) as u16
    }
}

fn generate_pixels(spec: &ImageSpec, pattern: Pattern) -> Vec<u8> {
    let channels = spec.channels();
    let mut out = Vec::with_capacity(spec.raw_bytes());
    let mut rng = Lcg(0x2026_0902);
    let max = if spec.depth == BitDepth::Sixteen {
        u16::MAX as u32
    } else {
        u8::MAX as u32
    };

    for y in 0..spec.height {
        for x in 0..spec.width {
            for c in 0..channels {
                let value: u32 = match pattern {
                    Pattern::Flat => max / 3,
                    Pattern::Gradient => {
                        let fx = x * max / spec.width.max(1);
                        let fy = y * max / spec.height.max(1);
                        (fx + fy) / 2
                    }
                    Pattern::Photo => {
                        let fx = x * max / spec.width.max(1);
                        let fy = y * max / spec.height.max(1);
                        let ripple = (((x * 7 + y * 13 + c as u32 * 31) % 64) * max) / 512;
                        let noise = u32::from(rng.next_u16() % 17);
                        ((fx + fy) / 2 + ripple + noise) % (max + 1)
                    }
                    Pattern::Noise => u32::from(rng.next_u16()) % (max + 1),
                };

                if spec.depth == BitDepth::Sixteen {
                    let v = value as u16;
                    out.extend_from_slice(&v.to_be_bytes());
                } else {
                    out.push(value as u8);
                }
            }
        }
    }

    debug_assert_eq!(out.len(), spec.raw_bytes());
    out
}

/// Decodes a real PNG and hands back its pixels as an encoder input, so measurements are not
/// hostage to whatever the synthetic generator happens to produce. The image is used exactly as
/// the decoder returns it (no transformations), so an indexed file stays indexed and is
/// re-encoded against its own palette.
fn load_image(path: &str) -> Result<(ImageSpec, Vec<u8>, Option<Vec<u8>>), String> {
    let file = std::fs::File::open(path).map_err(|e| format!("{path}: {e}"))?;
    let decoder = Decoder::new(std::io::BufReader::new(file));
    let mut reader = decoder.read_info().map_err(|e| format!("{path}: {e}"))?;

    let palette = reader.info().palette.as_ref().map(|p| p.to_vec());
    let (color, depth) = reader.output_color_type();
    let mut buf = vec![0u8; reader.output_buffer_size().ok_or("output_buffer_size: None")?];
    let out = reader
        .next_frame(&mut buf)
        .map_err(|e| format!("{path}: {e}"))?;
    buf.truncate(out.buffer_size());

    let spec = ImageSpec {
        width: out.width,
        height: out.height,
        color,
        depth,
    };
    if spec.raw_bytes() != buf.len() {
        return Err(format!(
            "{path}: decoded {} bytes but the spec implies {}",
            buf.len(),
            spec.raw_bytes()
        ));
    }
    Ok((spec, buf, palette))
}

/// A 256-entry RGB palette, needed for `ColorType::Indexed`.
fn generate_palette() -> Vec<u8> {
    (0..256u32)
        .flat_map(|i| [i as u8, (255 - i) as u8, (i * 7 % 256) as u8])
        .collect()
}

// ---------------------------------------------------------------------------------------------
// Encoding under test
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct EncodeParams {
    filter: Filter,
    compression: DeflateCompression,
    /// `None` = single image via `write_image_data`; `Some(n)` = animated, `n` frames.
    frames: Option<u32>,
    /// `None` = buffered `write_image_data`; `Some(n)` = `stream_writer_with_size(n)`.
    stream_buffer: Option<usize>,
}

impl EncodeParams {
    fn simple(filter: Filter, compression: DeflateCompression) -> Self {
        Self {
            filter,
            compression,
            frames: None,
            stream_buffer: None,
        }
    }
}

fn encode(spec: &ImageSpec, data: &[u8], palette: Option<&[u8]>, params: &EncodeParams) -> Vec<u8> {
    let mut out = Vec::new();
    {
        let mut encoder = Encoder::new(&mut out, spec.width, spec.height);
        encoder.set_color(spec.color);
        encoder.set_depth(spec.depth);
        encoder.set_filter(params.filter);
        encoder.set_deflate_compression(params.compression);
        if let Some(palette) = palette {
            encoder.set_palette(palette);
        }
        if let Some(frames) = params.frames {
            encoder.set_animated(frames, 0).expect("set_animated");
        }

        let mut writer = encoder.write_header().expect("write_header");

        match params.stream_buffer {
            None => {
                let frames = params.frames.unwrap_or(1);
                for _ in 0..frames {
                    writer
                        .write_image_data(black_box(data))
                        .expect("write_image_data");
                }
            }
            Some(buffer_size) => {
                let frames = params.frames.unwrap_or(1);
                let mut stream = writer
                    .stream_writer_with_size(buffer_size)
                    .expect("stream_writer_with_size");
                for _ in 0..frames {
                    stream.write_all(black_box(data)).expect("stream write_all");
                }
                stream.finish().expect("stream finish");
            }
        }

        writer.finish().expect("finish");
    }
    out
}

// ---------------------------------------------------------------------------------------------
// Verification
// ---------------------------------------------------------------------------------------------

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for &b in bytes {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(0x1000_0000_01b3);
    }
    hash
}

/// Decodes `encoded` back and compares every frame against the source pixels.
fn verify_roundtrip(encoded: &[u8], expected: &[u8], frames: u32) -> Result<(), String> {
    let decoder = Decoder::new(std::io::Cursor::new(encoded));
    let mut reader = decoder.read_info().map_err(|e| format!("read_info: {e}"))?;
    let size = reader
        .output_buffer_size()
        .ok_or_else(|| "output_buffer_size: None".to_string())?;
    let mut buf = vec![0u8; size];

    for frame in 0..frames {
        let info = reader
            .next_frame(&mut buf)
            .map_err(|e| format!("next_frame({frame}): {e}"))?;
        let decoded = &buf[..info.buffer_size()];
        if decoded != expected {
            let first = decoded
                .iter()
                .zip(expected)
                .position(|(a, b)| a != b)
                .unwrap_or(0);
            return Err(format!(
                "frame {frame}: pixel mismatch at byte {first} (len {} vs {})",
                decoded.len(),
                expected.len()
            ));
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Measurement
// ---------------------------------------------------------------------------------------------

struct Stats {
    n: usize,
    min: Duration,
    p50: Duration,
    mean: Duration,
    p95: Duration,
    max: Duration,
}

fn stats(mut samples: Vec<Duration>) -> Stats {
    samples.sort_unstable();
    let n = samples.len();
    let total: Duration = samples.iter().sum();
    let idx = |q: f64| samples[((n as f64 * q) as usize).min(n - 1)];
    Stats {
        n,
        min: samples[0],
        p50: idx(0.50),
        mean: total / n as u32,
        p95: idx(0.95),
        max: samples[n - 1],
    }
}

struct RunConfig {
    iters: usize,
    quiet_verify: bool,
}

fn run_one(
    mode: &str,
    label: &str,
    spec: &ImageSpec,
    data: &[u8],
    palette: Option<&[u8]>,
    params: &EncodeParams,
    cfg: &RunConfig,
) {
    let frames = params.frames.unwrap_or(1);
    let warmup = (cfg.iters / 10).max(1);
    let mut samples = Vec::with_capacity(cfg.iters);
    let mut expected_hash: Option<u64> = None;
    let mut verify = "OK".to_string();
    let mut output_len = 0usize;
    let mut alloc = AllocSnapshot::default();

    for i in 0..(cfg.iters + warmup) {
        let before = AllocSnapshot::now();
        let start = Instant::now();
        let encoded = encode(spec, data, palette, params);
        let elapsed = start.elapsed();
        let encoded = black_box(encoded);
        let after = AllocSnapshot::now();

        let hash = fnv1a(&encoded);
        match expected_hash {
            None => {
                expected_hash = Some(hash);
                if !cfg.quiet_verify {
                    if let Err(e) = verify_roundtrip(&encoded, data, frames) {
                        verify = format!("FAIL({e})");
                    }
                }
            }
            Some(h) if h != hash => verify = "FAIL(nondeterministic output)".to_string(),
            Some(_) => {}
        }

        if i >= warmup {
            samples.push(elapsed);
            output_len = encoded.len();
            alloc = after.since(before);
        }
    }

    let s = stats(samples);
    let raw_total = spec.raw_bytes() * frames as usize;
    let throughput = raw_total as f64 / s.p50.as_secs_f64() / (1024.0 * 1024.0);
    let ratio = raw_total as f64 / output_len.max(1) as f64;

    println!(
        "RESULT mode={mode} cfg={label} img={}x{}x{} pattern_bytes={raw_total} n={} \
         min={:.3}ms p50={:.3}ms mean={:.3}ms p95={:.3}ms max={:.3}ms \
         thr={throughput:.1}MiB/s out={output_len}B ratio={ratio:.2}x \
         allocs={} reallocs={} alloc_MiB={:.2} peak_rss_MiB={:.1} verify={verify}",
        spec.width,
        spec.height,
        spec.label(),
        s.n,
        s.min.as_secs_f64() * 1e3,
        s.p50.as_secs_f64() * 1e3,
        s.mean.as_secs_f64() * 1e3,
        s.p95.as_secs_f64() * 1e3,
        s.max.as_secs_f64() * 1e3,
        alloc.allocs,
        alloc.reallocs,
        alloc.bytes as f64 / (1024.0 * 1024.0),
        peak_rss_kib() as f64 / 1024.0,
    );
}

/// Decode side of an APNG - the one decode path `benches/decoder.rs` does not cover.
fn run_decode_apng(
    spec: &ImageSpec,
    data: &[u8],
    palette: Option<&[u8]>,
    frames: u32,
    cfg: &RunConfig,
) {
    let params = EncodeParams {
        filter: Filter::Adaptive,
        compression: DeflateCompression::Level(6),
        frames: Some(frames),
        stream_buffer: None,
    };
    let encoded = encode(spec, data, palette, &params);
    let warmup = (cfg.iters / 10).max(1);
    let mut samples = Vec::with_capacity(cfg.iters);
    let mut alloc = AllocSnapshot::default();
    let mut verify = "OK".to_string();

    for i in 0..(cfg.iters + warmup) {
        let before = AllocSnapshot::now();
        let start = Instant::now();

        let decoder = Decoder::new(std::io::Cursor::new(black_box(&encoded)));
        let mut reader = decoder.read_info().expect("read_info");
        let mut buf = vec![0u8; reader.output_buffer_size().expect("output_buffer_size")];
        for _ in 0..frames {
            reader.next_frame(&mut buf).expect("next_frame");
        }

        let elapsed = start.elapsed();
        let buf = black_box(buf);
        let after = AllocSnapshot::now();

        if buf[..data.len()] != *data {
            verify = "FAIL(pixel mismatch)".to_string();
        }

        if i >= warmup {
            samples.push(elapsed);
            alloc = after.since(before);
        }
    }

    let s = stats(samples);
    let raw_total = spec.raw_bytes() * frames as usize;
    let throughput = raw_total as f64 / s.p50.as_secs_f64() / (1024.0 * 1024.0);
    println!(
        "RESULT mode=decode-apng cfg={frames}frames img={}x{}x{} pattern_bytes={raw_total} n={} \
         min={:.3}ms p50={:.3}ms mean={:.3}ms p95={:.3}ms max={:.3}ms \
         thr={throughput:.1}MiB/s in={}B ratio=- \
         allocs={} reallocs={} alloc_MiB={:.2} peak_rss_MiB={:.1} verify={verify}",
        spec.width,
        spec.height,
        spec.label(),
        s.n,
        s.min.as_secs_f64() * 1e3,
        s.p50.as_secs_f64() * 1e3,
        s.mean.as_secs_f64() * 1e3,
        s.p95.as_secs_f64() * 1e3,
        s.max.as_secs_f64() * 1e3,
        encoded.len(),
        alloc.allocs,
        alloc.reallocs,
        alloc.bytes as f64 / (1024.0 * 1024.0),
        peak_rss_kib() as f64 / 1024.0,
    );
}

// ---------------------------------------------------------------------------------------------
// Modes
// ---------------------------------------------------------------------------------------------

const FILTERS: &[(&str, Filter)] = &[
    ("NoFilter", Filter::NoFilter),
    ("Sub", Filter::Sub),
    ("Up", Filter::Up),
    ("Avg", Filter::Avg),
    ("Paeth", Filter::Paeth),
    ("Adaptive", Filter::Adaptive),
    ("MinEntropy", Filter::MinEntropy),
];

fn compressions() -> Vec<(String, DeflateCompression)> {
    let mut v = vec![
        ("NoCompression".to_string(), DeflateCompression::NoCompression),
        (
            "FdeflateUltraFast".to_string(),
            DeflateCompression::FdeflateUltraFast,
        ),
    ];
    for level in [1u8, 3, 6, 9] {
        v.push((format!("Level{level}"), DeflateCompression::Level(level)));
    }
    v
}

fn color_types() -> Vec<(ColorType, BitDepth)> {
    vec![
        (ColorType::Grayscale, BitDepth::Eight),
        (ColorType::GrayscaleAlpha, BitDepth::Eight),
        (ColorType::Rgb, BitDepth::Eight),
        (ColorType::Rgba, BitDepth::Eight),
        (ColorType::Rgb, BitDepth::Sixteen),
        (ColorType::Rgba, BitDepth::Sixteen),
        (ColorType::Indexed, BitDepth::Eight),
    ]
}

struct Args {
    modes: Vec<String>,
    size: u32,
    iters: usize,
    pattern: Pattern,
    frames: u32,
    compression: DeflateCompression,
    only: Option<String>,
    input: Option<String>,
    corpus: Option<String>,
}

fn parse_compression(s: &str) -> Option<DeflateCompression> {
    match s {
        "none" | "NoCompression" => Some(DeflateCompression::NoCompression),
        "fdeflate" | "FdeflateUltraFast" => Some(DeflateCompression::FdeflateUltraFast),
        other => other
            .strip_prefix("level")
            .or_else(|| other.strip_prefix("Level"))
            .and_then(|n| n.parse::<u8>().ok())
            .map(DeflateCompression::Level),
    }
}

fn usage() -> ! {
    eprintln!(
        "\
encoder_bench - encoder-side benchmark harness for the `png` crate

USAGE:
    encoder_bench [--mode M]... [OPTIONS]

MODES (repeatable; default: all)
    filter        one run per Filter variant (isolates filtering cost)
    compress      one run per DeflateCompression setting
    color         one run per color type / bit depth
    apng          animated encoding, --frames frames via write_image_data
    stream        StreamWriter with several buffer sizes
    decode-apng   decoding an animated PNG (the gap in benches/decoder.rs)
    corpus        re-encode every *.png in --corpus DIR, one line per file
    all           every mode above except corpus (which needs --corpus)

OPTIONS
    --size N          square image edge in pixels (default 512)
    --iters N         measured iterations per configuration (default 20, +10% warmup)
    --pattern P       flat | gradient | photo | noise (default photo)
    --frames N        frames for apng/decode-apng (default 8)
    --compression C   compression for the filter/color/apng/stream modes
                      none | fdeflate | levelN (default fdeflate)
    --only SUBSTR     run only configurations whose label contains SUBSTR
                      (use this when peak_rss_MiB matters - it is a process-wide high-water mark)
    --input FILE      decode FILE and use its real pixels as the source image instead of the
                      synthetic pattern (applies to filter/compress/apng/stream; the `color` mode
                      always generates its own images, since varying the color type is its point)
    --corpus DIR      directory of *.png files for the `corpus` mode
                      (the repo\'s own is tests/benches/)
    -h, --help        this text

Every result line starts with `RESULT ` and is key=value throughout, so it can be parsed by
___OPTIMIZATIONS___/tools/*.sh without touching this program."
    );
    std::process::exit(2)
}

fn parse_args() -> Args {
    let mut args = Args {
        modes: Vec::new(),
        size: 512,
        iters: 20,
        pattern: Pattern::Photo,
        frames: 8,
        compression: DeflateCompression::FdeflateUltraFast,
        only: None,
        input: None,
        corpus: None,
    };

    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        let mut value = || it.next().unwrap_or_else(|| usage());
        match arg.as_str() {
            "--mode" => args.modes.push(value()),
            "--size" => args.size = value().parse().unwrap_or_else(|_| usage()),
            "--iters" => args.iters = value().parse().unwrap_or_else(|_| usage()),
            "--frames" => args.frames = value().parse().unwrap_or_else(|_| usage()),
            "--pattern" => args.pattern = Pattern::parse(&value()).unwrap_or_else(|| usage()),
            "--compression" => {
                args.compression = parse_compression(&value()).unwrap_or_else(|| usage())
            }
            "--only" => args.only = Some(value()),
            "--input" => args.input = Some(value()),
            "--corpus" => args.corpus = Some(value()),
            "-h" | "--help" => usage(),
            _ => usage(),
        }
    }

    if args.modes.is_empty() || args.modes.iter().any(|m| m == "all") {
        let mut modes: Vec<String> = ["filter", "compress", "color", "apng", "stream", "decode-apng"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        if args.corpus.is_some() {
            modes.push("corpus".to_string());
        }
        args.modes = modes;
    }
    args
}

fn main() {
    let args = parse_args();
    let keep = |label: &str| args.only.as_ref().map_or(true, |s| label.contains(s.as_str()));

    let (source, source_pixels, source_palette) = match args.input.as_deref() {
        Some(path) => load_image(path).unwrap_or_else(|e| {
            eprintln!("--input: {e}");
            std::process::exit(1)
        }),
        None => {
            let spec = ImageSpec {
                width: args.size,
                height: args.size,
                color: ColorType::Rgba,
                depth: BitDepth::Eight,
            };
            let pixels = generate_pixels(&spec, args.pattern);
            (spec, pixels, None)
        }
    };
    let source_palette = source_palette.as_deref();
    let palette = generate_palette();
    let cfg = RunConfig {
        iters: args.iters,
        quiet_verify: false,
    };

    println!(
        "# encoder_bench: source={} img={}x{}x{} iters={} frames={} \
         (allocation counting is always on; see README.md for its overhead)",
        args.input.as_deref().unwrap_or(args.pattern.name()),
        source.width,
        source.height,
        source.label(),
        args.iters,
        args.frames
    );

    for mode in &args.modes {
        match mode.as_str() {
            "filter" => {
                for (name, filter) in FILTERS {
                    if !keep(name) {
                        continue;
                    }
                    run_one(
                        "filter",
                        name,
                        &source,
                        &source_pixels,
                        source_palette,
                        &EncodeParams::simple(*filter, args.compression),
                        &cfg,
                    );
                }
            }
            "compress" => {
                for (name, compression) in compressions() {
                    if !keep(&name) {
                        continue;
                    }
                    run_one(
                        "compress",
                        &name,
                        &source,
                        &source_pixels,
                        source_palette,
                        &EncodeParams::simple(Filter::Adaptive, compression),
                        &cfg,
                    );
                }
            }
            "color" => {
                for (color, depth) in color_types() {
                    let spec = ImageSpec {
                        width: args.size,
                        height: args.size,
                        color,
                        depth,
                    };
                    let label = spec.label();
                    if !keep(&label) {
                        continue;
                    }
                    let pixels = generate_pixels(&spec, args.pattern);
                    let palette = (color == ColorType::Indexed).then_some(palette.as_slice());
                    run_one(
                        "color",
                        &label,
                        &spec,
                        &pixels,
                        palette,
                        &EncodeParams::simple(Filter::Adaptive, args.compression),
                        &cfg,
                    );
                }
            }
            "apng" => {
                let label = format!("{}frames", args.frames);
                if keep(&label) {
                    run_one(
                        "apng",
                        &label,
                        &source,
                        &source_pixels,
                        source_palette,
                        &EncodeParams {
                            filter: Filter::Adaptive,
                            compression: args.compression,
                            frames: Some(args.frames),
                            stream_buffer: None,
                        },
                        &cfg,
                    );
                }
            }
            "stream" => {
                for size in [1usize << 10, 1 << 12, 1 << 13, 1 << 15, 1 << 16] {
                    let label = format!("buf{}KiB", size / 1024);
                    if !keep(&label) {
                        continue;
                    }
                    run_one(
                        "stream",
                        &label,
                        &source,
                        &source_pixels,
                        source_palette,
                        &EncodeParams {
                            filter: Filter::Adaptive,
                            compression: args.compression,
                            frames: None,
                            stream_buffer: Some(size),
                        },
                        &cfg,
                    );
                }
            }
            "decode-apng" => {
                let label = format!("{}frames", args.frames);
                if keep(&label) {
                    run_decode_apng(&source, &source_pixels, source_palette, args.frames, &cfg);
                }
            }
            "corpus" => {
                let Some(dir) = args.corpus.as_deref() else {
                    eprintln!("`corpus` mode needs --corpus DIR");
                    usage();
                };
                let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(dir)
                    .unwrap_or_else(|e| {
                        eprintln!("--corpus {dir}: {e}");
                        std::process::exit(1)
                    })
                    .flatten()
                    .map(|e| e.path())
                    .filter(|p| p.extension().is_some_and(|e| e == "png"))
                    .collect();
                files.sort();
                for path in files {
                    let name = path.file_name().unwrap().to_string_lossy().to_string();
                    if !keep(&name) {
                        continue;
                    }
                    match load_image(&path.to_string_lossy()) {
                        Ok((spec, pixels, pal)) => run_one(
                            "corpus",
                            &name,
                            &spec,
                            &pixels,
                            pal.as_deref(),
                            &EncodeParams::simple(Filter::Adaptive, args.compression),
                            &cfg,
                        ),
                        Err(e) => println!("# skipped: {e}"),
                    }
                }
            }
            other => {
                eprintln!("unknown mode: {other}");
                usage();
            }
        }
    }
}
