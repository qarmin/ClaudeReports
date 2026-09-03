//! TEMPORARY: overall PDF -> PNG pipeline phase breakdown. Requires the
//! matching temporary instrumentation in hayro/src/lib.rs
//! (__debug_phase_timings_ns) and vello_common/src/pixmap.rs
//! (__debug_png_phase_timings_ns, split out of `into_png`'s body) -
//! see hayro_lib_rs_profiling_instrumentation.diff in this folder for the
//! former; the latter followed the same pattern applied to `into_png`.
//! Not wired into CI. See ___OPTIMIZATIONS___/PIPELINE.md for the results.

use hayro::hayro_interpret::InterpreterSettings;
use hayro::hayro_syntax::Pdf;
use hayro::{RenderCache, RenderSettings, render};
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("usage: bench_pipeline_phases <pdf>");
    let iterations: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(20);

    let data = std::fs::read(path).unwrap();

    let mut parse_ns = 0u64;
    let mut encode_ns = 0u64;
    let mut total_ns = 0u64;
    let mut page_count = 0u64;

    for _ in 0..iterations {
        let total_start = Instant::now();

        let t0 = Instant::now();
        let pdf = Pdf::new(data.clone()).unwrap();
        parse_ns += t0.elapsed().as_nanos() as u64;

        let cache = RenderCache::new();
        let settings = InterpreterSettings::default();

        for page in pdf.pages().iter() {
            page_count += 1;
            let pixmap = render(
                page,
                &cache,
                &settings,
                &RenderSettings {
                    x_scale: 1.0,
                    y_scale: 1.0,
                    ..Default::default()
                },
            );

            let t2 = Instant::now();
            let png = pixmap.into_png().unwrap();
            encode_ns += t2.elapsed().as_nanos() as u64;
            std::hint::black_box(&png);
        }

        total_ns += total_start.elapsed().as_nanos() as u64;
    }

    let (interpret_ns, rasterize_ns) = hayro::__debug_phase_timings_ns();
    // Requires a temporary `vello_common` dev-dependency in hayro/Cargo.toml
    // pointing at the same patched local clone (via the `[patch]` already in
    // the workspace root Cargo.toml), so this symbol resolves:
    //   vello_common = { git = "https://github.com/linebender/vello", rev = "..." }
    let (unpremultiply_ns, deflate_ns) = vello_common::pixmap::__debug_png_phase_timings_ns();
    page_count /= iterations as u64;

    let total_ms = total_ns as f64 / 1e6;
    let parse_ms = parse_ns as f64 / 1e6;
    let interpret_ms = interpret_ns as f64 / 1e6;
    let rasterize_ms = rasterize_ns as f64 / 1e6;
    let encode_ms = encode_ns as f64 / 1e6;
    let unpremultiply_ms = unpremultiply_ns as f64 / 1e6;
    let deflate_ms = deflate_ns as f64 / 1e6;
    let accounted = parse_ms + interpret_ms + rasterize_ms + encode_ms;

    println!("{path} ({page_count} page(s), {iterations} iterations)");
    println!(
        "  parse:            {:>8.3} ms  ({:>5.1}%)",
        parse_ms,
        100.0 * parse_ms / total_ms
    );
    println!(
        "  interpret+decode: {:>8.3} ms  ({:>5.1}%)",
        interpret_ms,
        100.0 * interpret_ms / total_ms
    );
    println!(
        "  vello rasterize:  {:>8.3} ms  ({:>5.1}%)",
        rasterize_ms,
        100.0 * rasterize_ms / total_ms
    );
    println!(
        "  encode png total: {:>8.3} ms  ({:>5.1}%)",
        encode_ms,
        100.0 * encode_ms / total_ms
    );
    println!(
        "    of which unpremultiply: {:>8.3} ms  ({:>5.1}% of total)",
        unpremultiply_ms,
        100.0 * unpremultiply_ms / total_ms
    );
    println!(
        "    of which deflate:       {:>8.3} ms  ({:>5.1}% of total)",
        deflate_ms,
        100.0 * deflate_ms / total_ms
    );
    println!(
        "  (sum of phases: {accounted:.3} ms; wall total incl. overhead not captured above: {total_ms:.3} ms)"
    );
}
