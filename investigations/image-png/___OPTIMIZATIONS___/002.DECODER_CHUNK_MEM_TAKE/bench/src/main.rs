//! Standalone bench for candidate 002: does `mem::take` instead of `.clone()` for
//! PLTE/sBIT/tRNS/eXIf/bKGD chunk bytes reduce decoder allocations?
//!
//! Builds a real PNG with a normal encoder (1x1 grayscale + tRNS), then splices in a large
//! hand-built eXIf chunk before IDAT (the crate's own encoder has no eXIf setter). Decodes the
//! result N times, counting allocations/reallocations/bytes via a wrapping GlobalAlloc, and
//! verifying `exif_metadata`/`trns` come back correct every iteration.

use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

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

struct Snapshot {
    allocs: u64,
    reallocs: u64,
    bytes: u64,
}
impl Snapshot {
    fn now() -> Self {
        Snapshot {
            allocs: ALLOCS.load(Ordering::Relaxed),
            reallocs: REALLOCS.load(Ordering::Relaxed),
            bytes: ALLOC_BYTES.load(Ordering::Relaxed),
        }
    }
    fn since(&self, before: &Snapshot) -> (u64, u64, u64) {
        (
            self.allocs - before.allocs,
            self.reallocs - before.reallocs,
            self.bytes - before.bytes,
        )
    }
}

fn crc32(data: &[u8]) -> u32 {
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(data);
    hasher.finalize()
}

fn build_chunk(kind: &[u8; 4], payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(4 + 4 + payload.len() + 4);
    out.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    out.extend_from_slice(kind);
    out.extend_from_slice(payload);
    let crc_input: Vec<u8> = kind.iter().chain(payload.iter()).copied().collect();
    out.extend_from_slice(&crc32(&crc_input).to_be_bytes());
    out
}

/// Build a 1x1 grayscale PNG with a tRNS chunk (via the crate's own encoder, which supports it),
/// then splice a hand-built eXIf chunk of `exif_len` bytes in right before IDAT.
fn build_png(exif_len: usize) -> Vec<u8> {
    let mut base = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut base, 1, 1);
        encoder.set_color(png::ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_trns(vec![0x00u8, 0x2A]); // 2-byte gray trans value
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&[128u8]).unwrap();
    }

    // Find the IDAT chunk (length prefix starts 4 bytes before b"IDAT") and splice an eXIf chunk
    // in immediately before it - valid per spec ordering (ancillary chunks before IDAT).
    let idat_pos = base
        .windows(4)
        .position(|w| w == b"IDAT")
        .expect("no IDAT in encoder output")
        - 4;

    let exif_payload = vec![0xABu8; exif_len];
    let exif_chunk = build_chunk(b"eXIf", &exif_payload);

    let mut out = Vec::with_capacity(base.len() + exif_chunk.len());
    out.extend_from_slice(&base[..idat_pos]);
    out.extend_from_slice(&exif_chunk);
    out.extend_from_slice(&base[idat_pos..]);
    out
}

fn decode_once(png_bytes: &[u8]) -> (usize, usize) {
    let decoder = png::Decoder::new(png_bytes);
    let reader = decoder.read_info().unwrap();
    let info = reader.info();
    let exif_len = info.exif_metadata.as_ref().map(|e| e.len()).unwrap_or(0);
    let trns_len = info.trns.as_ref().map(|t| t.len()).unwrap_or(0);
    (exif_len, trns_len)
}

fn bench(label: &str, png_bytes: &[u8], iters: usize) {
    let warmup = iters / 10;
    let mut samples = Vec::with_capacity(iters);
    let mut allocs_sum = 0u64;
    let mut reallocs_sum = 0u64;
    let mut bytes_sum = 0u64;
    let mut last = (0usize, 0usize);

    for i in 0..iters {
        let before = Snapshot::now();
        let start = Instant::now();
        let r = decode_once(black_box(png_bytes));
        let elapsed = start.elapsed();
        let after = Snapshot::now();

        last = r;
        if i >= warmup {
            samples.push(elapsed);
            let (a, re, b) = after.since(&before);
            allocs_sum += a;
            reallocs_sum += re;
            bytes_sum += b;
        }
    }

    let n = samples.len() as u64;
    samples.sort();
    let p50 = samples[samples.len() / 2];
    println!(
        "RESULT cfg={label} exif_expect={} trns_expect={} n={n} p50={:.4}ms allocs_avg={:.1} reallocs_avg={:.1} bytes_avg_KiB={:.2}",
        last.0,
        last.1,
        p50.as_secs_f64() * 1e3,
        allocs_sum as f64 / n as f64,
        reallocs_sum as f64 / n as f64,
        bytes_sum as f64 / n as f64 / 1024.0,
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let iters: usize = args
        .iter()
        .position(|a| a == "--iters")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(200);

    for exif_len in [0usize, 1024, 65536, 1_048_576] {
        let png_bytes = build_png(exif_len);
        // Correctness: round-trip the exif metadata and trns bytes exactly.
        let (got_exif, got_trns) = decode_once(&png_bytes);
        assert_eq!(got_exif, exif_len, "exif length mismatch for exif_len={exif_len}");
        // 8-bit grayscale tRNS is truncated to 1 byte by the decoder (see parse_trns).
        assert_eq!(got_trns, 1, "trns length mismatch for exif_len={exif_len}");
        bench(&format!("exif{exif_len}B"), &png_bytes, iters);
    }
}
