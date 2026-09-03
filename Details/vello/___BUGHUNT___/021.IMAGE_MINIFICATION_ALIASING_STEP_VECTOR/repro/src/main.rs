// Reproduction for: image minification (downscale, transform scale < 1.0) aliases instead of
// being properly filtered (linebender/vello#1289).
//
// Uses vello_cpu's public API end-to-end: build a real source `Pixmap`, use it as a real `Image`
// paint via `RenderContext::set_paint`, apply a real downscaling `paint_transform`, `fill_rect`,
// and `render` - the same sequence any real vello_cpu caller uses. The aliasing happens inside
// the real image-sampling path (`fine::common::image`), not in hand-constructed internal state.
//
// The source image is a "striped" pattern: mostly black with a single bright column every
// `period` source pixels. A correct minification filter (a box filter / mipmap) averages a full
// `period`-wide block of the source per output pixel, so every output pixel should end up close
// to the same dim, uniform gray value (255 / period), regardless of x. A filter whose sampling
// kernel stays a fixed small size (2x2 bilinear / 4x4 bicubic) instead of growing with the
// minification factor instead aliases: whether its few sample points land on/near a bright column
// depends on phase, producing either a large systematic brightness bias (if the phase stays
// aligned across the whole row, as happens for an exact integer downscale factor) or visible
// banding/high variance (if the phase drifts, as happens for a non-integer downscale factor).

use std::sync::Arc;
use vello_cpu::kurbo::{Affine, Rect};
use vello_cpu::peniko::{Extend, ImageQuality, ImageSampler};
use vello_cpu::{Image, ImageSource, Pixmap, RenderContext, Resources};

/// Builds a `size`x`size` source pixmap that is black everywhere except a single bright
/// (255-value) column every `period` pixels.
fn striped_source(size: u16, period: u16) -> Pixmap {
    let mut pm = Pixmap::new(size, size);
    for y in 0..size {
        for x in 0..size {
            let on = x % period == period / 2;
            let v = if on { 255 } else { 0 };
            pm.set_pixel(
                x,
                y,
                vello_cpu::peniko::color::PremulRgba8 {
                    r: v,
                    g: v,
                    b: v,
                    a: 255,
                },
            );
        }
    }
    pm
}

/// Renders `striped_source(src_size, period)` downscaled by `scale` (paint_transform) to a
/// `out_size`x`out_size` output, through vello_cpu's real public rendering API, and returns
/// (mean, variance, min, max) of luma across output row 0 (the source pattern is y-invariant, so
/// row 0 is representative of every row).
fn render_and_measure(
    src_size: u16,
    period: u16,
    out_size: u16,
    scale: f64,
    quality: ImageQuality,
) -> (f64, f64, u32, u32) {
    let source = striped_source(src_size, period);

    let mut ctx = RenderContext::new(out_size, out_size);
    let mut resources = Resources::new();

    ctx.set_paint(Image {
        image: ImageSource::Pixmap(Arc::new(source)),
        sampler: ImageSampler {
            x_extend: Extend::Pad,
            y_extend: Extend::Pad,
            quality,
            ..Default::default()
        },
    });

    // paint_transform maps paint space -> device space; `scale` < 1.0 is downscaling
    // (minification) - the exact transform-scale-below-1.0 case this candidate is about.
    ctx.set_paint_transform(Affine::scale(scale));
    ctx.fill_rect(&Rect::from_points(
        (0.0, 0.0),
        (out_size as f64, out_size as f64),
    ));

    ctx.flush();
    let mut target = Pixmap::new(out_size, out_size);
    ctx.render(&mut target, &mut resources);

    let row: Vec<u32> = (0..out_size as u32)
        .map(|x| target.sample(x as u16, 0).r as u32)
        .collect();
    let n = row.len() as f64;
    let mean = row.iter().sum::<u32>() as f64 / n;
    let variance = row.iter().map(|&v| (v as f64 - mean).powi(2)).sum::<f64>() / n;
    let min = *row.iter().min().unwrap();
    let max = *row.iter().max().unwrap();

    (mean, variance, min, max)
}

/// A test case is considered "aliased" (bug reproduces) if the measured mean deviates from the
/// ideal box-filtered value by more than 25% (a wide margin - the actual deviations observed are
/// far larger, ~300%), or the variance across the row is far from zero (banding) when the ideal
/// is a uniform value.
fn check(label: &str, src_size: u16, period: u16, out_size: u16, scale: f64, quality: ImageQuality) -> bool {
    let ideal = 255.0 / period as f64;
    let (mean, variance, min, max) = render_and_measure(src_size, period, out_size, scale, quality);

    let mean_dev_pct = 100.0 * (mean - ideal) / ideal;
    let aliased = mean_dev_pct.abs() > 25.0 || variance > 100.0;

    println!("--- {label} ---");
    println!(
        "source {src_size}x{src_size} (stripe period {period}px, quality {quality:?}), scale {scale:.4} -> output {out_size}x{out_size}"
    );
    println!("ideal (proper minification filter): value ~= {ideal:.1} uniformly, variance ~= 0");
    println!(
        "actual: mean = {mean:.2} ({mean_dev_pct:+.0}% vs ideal), variance = {variance:.2}, min = {min}, max = {max}"
    );
    println!(
        "=> {}",
        if aliased {
            "ALIASED (bug reproduces)"
        } else {
            "OK (matches proper minification filter)"
        }
    );

    aliased
}

fn main() {
    let mut any_aliased = false;

    // Case 1: exact 8x downscale, matching issue #1289's own worked example. The sample stride
    // lands in-phase with the stripe every time, giving a large systematic brightness bias.
    any_aliased |= check(
        "exact 8x downscale (phase-aligned, bilinear)",
        512,
        8,
        64,
        1.0 / 8.0,
        ImageQuality::Medium,
    );

    // Case 2: a slightly non-integer ~8.4x downscale, so the sampling phase drifts across the
    // row - this produces visible banding/variance instead of a phase-locked bias.
    any_aliased |= check(
        "non-integer ~8.4x downscale (phase drifts, bilinear)",
        512,
        8,
        61,
        61.0 / 512.0,
        ImageQuality::Medium,
    );

    // Case 3: same exact 8x downscale as case 1, but bicubic (High quality) filtering - its
    // fixed 4x4 kernel is wider than bilinear's 2x2 but still doesn't scale with an 8x step.
    any_aliased |= check(
        "exact 8x downscale (phase-aligned, bicubic)",
        512,
        8,
        64,
        1.0 / 8.0,
        ImageQuality::High,
    );

    if any_aliased {
        eprintln!("\nREPRODUCED - at least one case showed minification aliasing");
        std::process::exit(1);
    } else {
        println!("\ndid not reproduce - all cases matched a proper minification filter");
    }
}
