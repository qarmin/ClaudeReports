// Probe for candidate 113: measures the per-paint cost that NativeButton::actual_icon()
// (internal/backends/qt/qt_widgets/button.rs) pays today by calling
// i_slint_backend_qt::qt_window::image_to_pixmap() unconditionally on every paint for an
// SVG-sourced icon, versus what a per-widget (Image, icon_size) -> QPixmap cache would pay
// instead (one real rasterization, then a cheap Image::eq + integer compare per subsequent
// paint - exactly the check the proposed fix in button.rs/tabwidget.rs would do).
//
// `bench_probe_image_to_pixmap` is a temporary, feature-gated (`bench-probe`) export added to
// i-slint-backend-qt's qt_window.rs purely so this out-of-crate probe can call the real
// pub(crate) `image_to_pixmap` directly - not shipped, reverted at the end of the session.
//
// Usage: qt_icon_probe <svg_path> <icon_size_px> <mode: uncached|cached> [iters=50]

use i_slint_core::graphics::Image;
use std::env;
use std::process;
use std::time::Instant;

fn main() {
    let mut args = env::args().skip(1);
    let path = args.next().unwrap_or_else(|| {
        eprintln!("usage: qt_icon_probe <svg_path> <icon_size_px> <mode: uncached|cached> [iters=50]");
        process::exit(1);
    });
    let icon_size: u32 = args.next().and_then(|s| s.parse().ok()).unwrap_or(32);
    let mode = args.next().unwrap_or_else(|| "uncached".to_string());
    let iters: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(50);

    let image = Image::load_from_path(std::path::Path::new(&path)).expect("load failed");
    let inner: &i_slint_core::ImageInner = (&image).into();

    let mut times_us: Vec<u64> = Vec::with_capacity(iters);
    let mut first_size: Option<(u32, u32)> = None;

    // Simulates the per-widget cache the patch adds: the "current" (image, icon_size) the
    // widget would compare against on every paint.
    let mut cached_key: Option<(Image, u32, (u32, u32))> = None;

    for i in 0..iters {
        let t0 = Instant::now();
        let size = match mode.as_str() {
            "uncached" => {
                // What every paint does today: rasterize unconditionally.
                std::hint::black_box(i_slint_backend_qt::bench_probe_image_to_pixmap(
                    inner, icon_size,
                ))
            }
            "cached" => {
                // What the patched actual_icon() would do: reuse when (image, icon_size)
                // still match what's cached, else recompute - same PartialEq the real fix uses.
                match &cached_key {
                    Some((cached_image, cached_size, dims))
                        if *cached_image == image && *cached_size == icon_size =>
                    {
                        std::hint::black_box(Some(*dims))
                    }
                    _ => {
                        let dims = i_slint_backend_qt::bench_probe_image_to_pixmap(inner, icon_size);
                        if let Some(d) = dims {
                            cached_key = Some((image.clone(), icon_size, d));
                        }
                        std::hint::black_box(dims)
                    }
                }
            }
            other => {
                eprintln!("unknown mode {other:?}, expected uncached|cached");
                process::exit(1);
            }
        };
        let us = t0.elapsed().as_micros() as u64;

        let size = size.expect("rasterization returned None");
        match first_size {
            None => first_size = Some(size),
            Some(prev) => assert_eq!(size, prev, "iter {i} size {size:?} differs from iter 0 {prev:?}"),
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

    println!("file       = {path}");
    println!("icon_size  = {icon_size}");
    println!("mode       = {mode}");
    println!("iters      = {iters} (warmup discarded = {warmup})");
    println!("out_size   = {w}x{h}  (STABLE across all iters)");
    println!("min        = {:8.3} ms", min as f64 / 1000.0);
    println!("p50        = {:8.3} ms", p50 as f64 / 1000.0);
    println!("mean       = {:8.3} ms", mean as f64 / 1000.0);
    println!("p95        = {:8.3} ms", p95 as f64 / 1000.0);
    println!("max        = {:8.3} ms", max as f64 / 1000.0);
    println!();
    print!("raw_us     = [");
    for (i, t) in times_us.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{t}");
    }
    println!("]");
}
