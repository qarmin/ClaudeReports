//! Standalone bench for candidate 016: does moving (instead of cloning) `raw_bytes` in
//! `parse_captured` for a chunk type with no specific parser reduce decoder allocations?
//!
//! Builds a real PNG with a normal encoder (1x1 grayscale), then splices in a hand-built
//! unrecognized ancillary chunk ("zzZz", never matched by any specific parser and thus never
//! captured-and-then-reparsed) of a given size before IDAT. Decodes the result N times with
//! `set_captured_chunks(&[zzZz])`, counting allocations/reallocations/bytes via a wrapping
//! GlobalAlloc, and verifying the captured chunk bytes come back correct every iteration.

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

/// Build a 1x1 grayscale PNG (via the crate's own encoder), then splice a hand-built,
/// unrecognized "zzZz" ancillary chunk of `chunk_len` bytes in right before IDAT.
fn build_png(chunk_len: usize) -> (Vec<u8>, Vec<u8>) {
    let mut base = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut base, 1, 1);
        encoder.set_color(png::ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&[128u8]).unwrap();
    }

    let idat_pos = base
        .windows(4)
        .position(|w| w == b"IDAT")
        .expect("no IDAT in encoder output")
        - 4;

    // Payload with a distinct, checkable byte pattern rather than all-same-byte, so a
    // move-vs-clone bug that hands back stale/garbage bytes would be caught.
    let payload: Vec<u8> = (0..chunk_len).map(|i| (i % 251) as u8).collect();
    let chunk = build_chunk(b"zzZz", &payload);

    let mut out = Vec::with_capacity(base.len() + chunk.len());
    out.extend_from_slice(&base[..idat_pos]);
    out.extend_from_slice(&chunk);
    out.extend_from_slice(&base[idat_pos..]);
    (out, payload)
}

fn decode_once(png_bytes: &[u8]) -> Vec<u8> {
    let mut options = png::DecodeOptions::default();
    options
        .set_captured_chunks(&[png::chunk::ChunkType(*b"zzZz")])
        .unwrap();
    let decoder = png::Decoder::new_with_options(png_bytes, options);
    let reader = decoder.read_info().unwrap();
    let info = reader.info();
    assert_eq!(info.captured_chunks.len(), 1);
    info.captured_chunks[0].data.clone()
}

fn bench(label: &str, png_bytes: &[u8], expected: &[u8], iters: usize) {
    let warmup = iters / 10;
    let mut samples = Vec::with_capacity(iters);
    let mut allocs_sum = 0u64;
    let mut reallocs_sum = 0u64;
    let mut bytes_sum = 0u64;
    let mut last_len = 0usize;

    for i in 0..iters {
        let before = Snapshot::now();
        let start = Instant::now();
        let r = decode_once(black_box(png_bytes));
        let elapsed = start.elapsed();
        let after = Snapshot::now();

        assert_eq!(r, expected, "captured chunk mismatch for {label}");
        last_len = r.len();
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
        "RESULT cfg={label} len={last_len} n={n} p50={:.4}ms allocs_avg={:.1} reallocs_avg={:.1} bytes_avg_KiB={:.2}",
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

    for chunk_len in [0usize, 1024, 65536, 1_048_576] {
        let (png_bytes, expected) = build_png(chunk_len);
        // Correctness spot-check before timing.
        let got = decode_once(&png_bytes);
        assert_eq!(got, expected, "mismatch before timing for chunk_len={chunk_len}");
        bench(&format!("chunk{chunk_len}B"), &png_bytes, &expected, iters);
    }
}
