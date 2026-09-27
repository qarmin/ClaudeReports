//! Pure measurement candidate: what does `fdeflate::Decompressor::new()` cost, and is it worth
//! avoiding on every APNG frame (`ZlibStream::reset()` in `src/decoder/zlib.rs` does
//! `*self.state = Decompressor::new()` once per `fcTL`)?
//!
//! `Decompressor::new()` allocates two boxed lookup tables (`litlen_table: Box<[u16; 4096]>`,
//! `dist_table: Box<[u16; 512]>` per fdeflate 0.3.7's source) - this measures that cost directly
//! (fdeflate is a direct dependency here too, so no `png`-side change is needed to observe it),
//! and separately how total decoder allocations scale with APNG frame count, to see how much of
//! that scaling this specific cost could plausibly explain.

use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

static ALLOCS: AtomicU64 = AtomicU64::new(0);
struct CountingAlloc;
unsafe impl GlobalAlloc for CountingAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        System.alloc(layout)
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        System.alloc_zeroed(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        System.realloc(ptr, layout, new_size)
    }
}
#[global_allocator]
static GLOBAL: CountingAlloc = CountingAlloc;

fn main() {
    // Part 1: raw allocation count and time for Decompressor::new(), in isolation.
    let iters = 100_000;
    let before = ALLOCS.load(Ordering::Relaxed);
    let start = Instant::now();
    for _ in 0..iters {
        let d = fdeflate::Decompressor::new();
        black_box(&d);
    }
    let elapsed = start.elapsed();
    let after = ALLOCS.load(Ordering::Relaxed);
    let allocs_per_call = (after - before) as f64 / iters as f64;
    println!(
        "RESULT part=decompressor_new n={iters} allocs_per_call={allocs_per_call:.2} total_ns={} ns_per_call={:.1}",
        elapsed.as_nanos(),
        elapsed.as_nanos() as f64 / iters as f64,
    );

    // Part 2: how do total decoder allocations scale with APNG frame count? (real decode path,
    // via the crate's own public API - builds an N-frame APNG in memory, then decodes it fully.)
    for frames in [1u32, 2, 4, 8, 16, 32] {
        let png_bytes = build_apng(frames, 64);
        let before = ALLOCS.load(Ordering::Relaxed);
        let start = Instant::now();
        let mut decoder = png::Decoder::new(std::io::Cursor::new(&png_bytes));
        decoder.set_ignore_text_chunk(true);
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0u8; reader.output_buffer_size().unwrap()];
        loop {
            match reader.next_frame(&mut buf) {
                Ok(_) => {}
                Err(png::DecodingError::Parameter(_)) => break, // no more frames
                Err(e) => panic!("{e}"),
            }
        }
        let elapsed = start.elapsed();
        let after = ALLOCS.load(Ordering::Relaxed);
        println!(
            "RESULT part=apng_decode frames={frames} allocs={} time_us={:.1}",
            after - before,
            elapsed.as_secs_f64() * 1e6,
        );
    }
}

/// Build a minimal N-frame APNG (via the crate's own encoder) of `size`x`size` RGBA frames.
fn build_apng(frames: u32, size: u32) -> Vec<u8> {
    let mut out = Vec::new();
    let mut encoder = png::Encoder::new(&mut out, size, size);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_animated(frames, 0).unwrap();
    encoder.set_frame_delay(1, 30).unwrap();
    let mut writer = encoder.write_header().unwrap();
    let frame_data = vec![0x42u8; (size * size * 4) as usize];
    for _ in 0..frames {
        writer.write_image_data(&frame_data).unwrap();
    }
    writer.finish().unwrap();
    out
}
