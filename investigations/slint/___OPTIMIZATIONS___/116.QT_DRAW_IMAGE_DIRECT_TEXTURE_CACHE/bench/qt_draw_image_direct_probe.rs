// Probe for candidate 116: measures the per-paint cost the Qt backend's
// `QtItemRenderer::draw_image_direct` (internal/backends/qt/qt_window.rs) pays today by calling
// `image_to_pixmap()` unconditionally on every call for an SVG-sourced image, versus what the
// added per-source `cached_image_to_pixmap` (same file) pays instead once a `DirectImageCache`
// entry is warm.
//
// Both `image_to_pixmap` and `cached_image_to_pixmap` are the *real* production functions,
// exported behind a temporary `bench-probe` cargo feature (bench_probe.patch) purely so this
// out-of-crate probe can call them directly - "cached" mode is not a re-implementation of the
// cache logic, it calls the exact patched code path.
//
// Usage: qt_draw_image_direct_probe <svg_path> <target_size_px> <mode: uncached|cached> [iters=200]

use i_slint_core::graphics::Image;
use i_slint_core::lengths::PhysicalPx;
use std::env;
use std::process;
use std::time::Instant;

fn main() {
    let mut args = env::args().skip(1);
    let path = args.next().unwrap_or_else(|| {
        eprintln!(
            "usage: qt_draw_image_direct_probe <svg_path> <target_size_px> <mode: uncached|cached> [iters=200]"
        );
        process::exit(1);
    });
    let target_size: u32 = args.next().and_then(|s| s.parse().ok()).unwrap_or(128);
    let mode = args.next().unwrap_or_else(|| "uncached".to_string());
    let iters: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(200);

    i_slint_backend_qt::bench_probe_init();

    let image = Image::load_from_path(std::path::Path::new(&path)).expect("load failed");
    let inner: &i_slint_core::ImageInner = (&image).into();
    let size = Some(euclid::Size2D::<u32, PhysicalPx>::new(target_size, target_size));

    let mut times_us: Vec<u64> = Vec::with_capacity(iters);
    let mut first_size: Option<(u32, u32)> = None;

    // Only relevant to "cached" mode - the same `RefCell<Option<DirectImageCacheEntry>>` a real
    // `QtWindow` keeps for the whole window's lifetime, reused across every draw_image_direct call.
    let cache = i_slint_backend_qt::bench_probe_new_direct_image_cache();

    for i in 0..iters {
        let t0 = Instant::now();
        let pixmap_size = match mode.as_str() {
            "uncached" => {
                // What every draw_image_direct call does today: rasterize unconditionally.
                std::hint::black_box(i_slint_backend_qt::bench_probe_image_to_pixmap(inner, size))
            }
            "cached" => {
                // What the patched draw_image_direct does: ImageCacheKey + size match reuses the
                // stored QPixmap, else rasterizes and stores - the real function, not a reimplementation.
                std::hint::black_box(i_slint_backend_qt::bench_probe_cached_image_to_pixmap(
                    &cache, inner, size,
                ))
            }
            other => {
                eprintln!("unknown mode {other:?}, expected uncached|cached");
                process::exit(1);
            }
        };
        let us = t0.elapsed().as_micros() as u64;

        let pixmap_size = pixmap_size.expect("rasterization returned None");
        let dims = {
            let qsize = pixmap_size.size();
            (qsize.width as u32, qsize.height as u32)
        };
        match first_size {
            None => first_size = Some(dims),
            Some(prev) => {
                assert_eq!(dims, prev, "iter {i} size {dims:?} differs from iter 0 {prev:?}")
            }
        }
        times_us.push(us);
    }

    let (w, h) = first_size.unwrap();
    let warmup = (iters / 10).max(1).min(iters - 1);
    let stable: Vec<u64> = {
        let mut s = times_us[warmup..].to_vec();
        s.sort_unstable();
        s
    };
    let n = stable.len();
    let min = stable[0];
    let p50 = stable[n / 2];
    let p95 = stable[(n * 95) / 100];
    let mean = stable.iter().sum::<u64>() / n as u64;
    let max = stable[n - 1];

    println!("file        = {path}");
    println!("target_size = {target_size}");
    println!("mode        = {mode}");
    println!("iters       = {iters} (warmup discarded = {warmup})");
    println!("out_size    = {w}x{h}  (STABLE across all iters)");
    println!("min         = {:8.3} ms", min as f64 / 1000.0);
    println!("p50         = {:8.3} ms", p50 as f64 / 1000.0);
    println!("mean        = {:8.3} ms", mean as f64 / 1000.0);
    println!("p95         = {:8.3} ms", p95 as f64 / 1000.0);
    println!("max         = {:8.3} ms", max as f64 / 1000.0);
    println!();
    print!("raw_us      = [");
    for (i, t) in times_us.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{t}");
    }
    println!("]");
}
