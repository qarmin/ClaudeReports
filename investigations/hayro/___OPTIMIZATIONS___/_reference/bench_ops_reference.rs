// TEMPORARY: breakdown of interpret+decode time by operation type. Extended for candidate 007
// with the whole-match-arm buckets from interpret_dispatch_instrumentation.diff (state_stack,
// transform, path_build, path_draw_whole, color, text_state, text_font_op, text_show_whole,
// xobject_misc) - copy back alongside that diff plus pipeline_breakdown_profiling_instrumentation.diff
// to reproduce (see ../README.md's "Instrumentation added for this workspace"). Extended again
// for candidate 011 with the xobject_misc-minus-draw_image line.
use hayro::hayro_interpret::InterpreterSettings;
use hayro::hayro_syntax::Pdf;
use hayro::{RenderCache, RenderSettings, render};
use std::time::Instant;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let iterations: usize = std::env::args().nth(2).and_then(|s| s.parse().ok()).unwrap_or(50);
    let data = std::fs::read(&path).unwrap();

    let mut total_ns = 0u64;
    for _ in 0..iterations {
        let t = Instant::now();
        let pdf = Pdf::new(data.clone()).unwrap();
        let cache = RenderCache::new();
        let settings = InterpreterSettings::default();
        for page in pdf.pages().iter() {
            let pixmap = render(page, &cache, &settings, &RenderSettings { x_scale: 1.0, y_scale: 1.0, ..Default::default() });
            std::hint::black_box(&pixmap);
        }
        total_ns += t.elapsed().as_nanos() as u64;
    }

    let (img_ns, path_ns, glyph_ns, other_ns) = hayro::__debug_op_timings_ns();
    let total_ms = total_ns as f64 / 1e6;
    let img_ms = img_ns as f64 / 1e6;
    let path_ms = path_ns as f64 / 1e6;
    let glyph_ms = glyph_ns as f64 / 1e6;
    let other_ms = other_ns as f64 / 1e6;

    println!("{path} ({iterations} iterations), total render() time = {total_ms:.3}ms");
    println!("  draw_image (incl. decode/convert): {:>8.3} ms  ({:>5.1}%)", img_ms, 100.0*img_ms/total_ms);
    println!("  draw_path/draw_rect:               {:>8.3} ms  ({:>5.1}%)", path_ms, 100.0*path_ms/total_ms);
    println!("  draw_glyph:                        {:>8.3} ms  ({:>5.1}%)", glyph_ms, 100.0*glyph_ms/total_ms);
    println!("  clip/group ops:                    {:>8.3} ms  ({:>5.1}%)", other_ms, 100.0*other_ms/total_ms);
    let accounted = img_ms + path_ms + glyph_ms + other_ms;
    println!("  (accounted: {accounted:.3}ms of {total_ms:.3}ms total; remainder is content-stream parsing/dispatch overhead)");

    let (lookup_ns, miss_ns, glyph_count) = hayro::__debug_glyph_timings_ns();
    println!(
        "  glyph outline: {} calls, cache-hit lookup={:.3}ms, cache-miss (extract outline)={:.3}ms",
        glyph_count / iterations as u64,
        lookup_ns as f64 / 1e6,
        miss_ns as f64 / 1e6
    );

    let (interpret_total_ns, final_rasterize_ns) = hayro::__debug_toplevel_timings_ns();
    let interpret_total_ms = interpret_total_ns as f64 / 1e6;
    let final_rasterize_ms = final_rasterize_ns as f64 / 1e6;
    let dispatch_overhead_ms = interpret_total_ms - accounted;
    println!();
    println!("  interpret_page total:    {:>8.3} ms  ({:>5.1}%)  [= draw_image+path+glyph+other+dispatch]", interpret_total_ms, 100.0*interpret_total_ms/total_ms);
    println!("    of which dispatch/dedup overhead not in any draw_* call: {:>8.3} ms  ({:>5.1}%)", dispatch_overhead_ms, 100.0*dispatch_overhead_ms/total_ms);
    println!("  final ctx.render() (rasterize to pixels): {:>8.3} ms  ({:>5.1}%)", final_rasterize_ms, 100.0*final_rasterize_ms/total_ms);

    let show_text_ns = hayro::hayro_interpret::__debug_show_text_ns();
    let show_text_ms = show_text_ns as f64 / 1e6;
    let pre_glyph_ms = show_text_ms - glyph_ms;
    println!();
    println!("  show_text_string total (incl. draw_glyph calls it makes): {:>8.3} ms  ({:>5.1}%)", show_text_ms, 100.0*show_text_ms/total_ms);
    println!("    of which BEFORE reaching device.draw_glyph (font/glyph resolution, code decoding, advance): {:>8.3} ms  ({:>5.1}%)", pre_glyph_ms, 100.0*pre_glyph_ms/total_ms);

    let (tok_ns, op_count) = hayro::hayro_interpret::__debug_tokenize_ns();
    let tok_ms = tok_ns as f64 / 1e6;
    println!();
    println!("  content-stream tokenizing (ops.next(), {} ops/render): {:>8.3} ms  ({:>5.1}%)", op_count / iterations as u64, tok_ms, 100.0*tok_ms/total_ms);

    println!();
    println!("  -- candidate 007: whole-match-arm buckets (may double-count time already shown above where an arm calls into a device method or show_text_string) --");
    let buckets = hayro::hayro_interpret::__debug_op_buckets_ns();
    let mut bucket_total_ms = 0.0;
    for (name, ns) in buckets {
        let ms = ns as f64 / 1e6;
        bucket_total_ms += ms;
        println!("    {:<16} {:>8.3} ms  ({:>5.1}%)", name, ms, 100.0*ms/total_ms);
    }
    println!("    {:<16} {:>8.3} ms  ({:>5.1}%)  [sum of buckets above]", "sum", bucket_total_ms, 100.0*bucket_total_ms/total_ms);
    println!("    path_draw bucket minus draw_path/draw_rect (path-building prep before device call): {:.3} ms ({:.1}%)",
        (buckets[3].1 as f64 / 1e6) - path_ms, 100.0 * ((buckets[3].1 as f64 / 1e6) - path_ms) / total_ms);
    println!("    text_show bucket minus draw_glyph (font/glyph resolution + text-state work before device call): {:.3} ms ({:.1}%)",
        (buckets[7].1 as f64 / 1e6) - glyph_ms, 100.0 * ((buckets[7].1 as f64 / 1e6) - glyph_ms) / total_ms);
    println!("    xobject_misc bucket minus draw_image (candidate 011: XObject/InlineImage/Shading/marked-content prep before device call, NOT net of nested Form-XObject recursion - see 011.xobject_misc_decompose/RESULT.md caveat): {:.3} ms ({:.1}%)",
        (buckets[8].1 as f64 / 1e6) - img_ms, 100.0 * ((buckets[8].1 as f64 / 1e6) - img_ms) / total_ms);
    let tagged_sum_ms = buckets[0].1 as f64 / 1e6 // state_stack
        + buckets[1].1 as f64 / 1e6 // transform
        + ((buckets[3].1 as f64 / 1e6) - path_ms).max(0.0) // path_draw prep only
        + buckets[4].1 as f64 / 1e6 // color
        + buckets[5].1 as f64 / 1e6 // text_state
        + buckets[6].1 as f64 / 1e6 // text_font_op
        + ((buckets[7].1 as f64 / 1e6) - glyph_ms).max(0.0) // text_show prep only
        + ((buckets[8].1 as f64 / 1e6) - img_ms).max(0.0) // xobject_misc prep only
        + buckets[2].1 as f64 / 1e6; // path_build
    println!("    sum of all non-device-call bucket time (state_stack+transform+path_build+path_draw_prep+color+text_state+text_font_op+text_show_prep+xobject_misc_prep): {:.3} ms ({:.1}%) of total render, vs. dispatch_overhead_ms={:.3}ms above",
        tagged_sum_ms, 100.0*tagged_sum_ms/total_ms, dispatch_overhead_ms);
}
