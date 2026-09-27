// Candidate 074: does the real vello_cpu rendering pipeline (the actual public
// `RenderContext`/`Pixmap` API, no hand-constructed internal state) ever produce a premultiplied
// pixel whose R/G/B channel exceeds its own alpha channel? `vello_common::util::unpremultiply`'s
// `scalar`/`simd` helpers (reached from the public `Pixmap::take_unpremultiplied`) are only sound
// if that invariant holds for every pixel - see RESULT.md for why.
//
// This program does two independent things:
//   1. An *isolated* re-implementation of the reciprocal/div_256 arithmetic (not calling the
//      crate's private `util::unpremultiply` module - it's `pub(crate)`) to demonstrate, in
//      principle, what happens if the invariant is ever violated. This part always "succeeds" -
//      it's just documenting the arithmetic, not testing whether real rendering hits it.
//   2. A real end-to-end fuzz-style search that drives `vello_cpu::RenderContext`'s actual public
//      API (random solid-color fills, random `Mix`/`Compose` blend modes, random layer nesting
//      with `push_layer`/`pop_layer`, fractional/anti-aliased rect coverage, both `RenderMode`
//      pipelines) and inspects the resulting `Pixmap`'s raw premultiplied buffer via
//      `Pixmap::data()` for any pixel where `r > a || g > a || b > a`, *before* ever calling
//      `take_unpremultiplied()`. If one is ever found, it then calls the real
//      `take_unpremultiplied()` on a fresh render of the same scene and prints the resulting
//      (wrong) unpremultiplied pixel.
//
// Exit code convention: 0 = invariant never violated in the search (bug not reachable via any
// scene tried), non-zero = a violation was found and demonstrated end-to-end.

use vello_common::kurbo::Rect;
use vello_common::peniko::color::{AlphaColor, Srgb};
use vello_common::peniko::{BlendMode, Compose, Mix};
use vello_common::pixmap::Pixmap;
use vello_cpu::{RasterizerSettings, RenderContext, RenderMode, Resources};

const fn reciprocals() -> [u16; 256] {
    let mut values = [0; 256];
    let mut alpha = 1;
    while alpha < 256 {
        let round_factor = alpha / 2;
        values[alpha] = ((255 * 256 + round_factor) / alpha) as u16;
        alpha += 1;
    }
    values
}

/// Isolated re-implementation of `vello_common::util::unpremultiply::scalar`'s arithmetic (that
/// module is `pub(crate)`, so this reproduces its exact formula rather than calling it directly)
/// - used only to document what the arithmetic does, not part of the end-to-end search below.
fn isolated_scalar_explicit(component: u8, reciprocal: u16) -> (u16, u16, u8) {
    let product = u16::from(component).wrapping_mul(reciprocal);
    // The real `div_256` uses a plain `+`, which panics on overflow under `debug-assertions`.
    // This demo uses `wrapping_add` throughout to show the *release*-mode (silently wrapping)
    // behavior without aborting.
    let added = product.wrapping_add(128);
    let divided = added >> 8;
    (product, divided, divided as u8)
}

fn print_isolated_math_demo() {
    println!("=== Part 1: isolated arithmetic demonstration (not calling the crate's private API) ===");
    let recips = reciprocals();
    // A hypothetical (component=153, alpha=152) pixel: alpha is a plausible near-90%-opacity
    // value, and component is only 1 higher than alpha - the exact class of off-by-one rounding
    // delta the introducing PR's own commit message describes as expected during blending.
    let alpha = 152u8;
    let component = 153u8; // violates the documented `component <= alpha` precondition by 1
    let reciprocal = recips[alpha as usize];
    let (product, divided, out) = isolated_scalar_explicit(component, reciprocal);
    println!(
        "hypothetical premultiplied pixel: component={component}, alpha={alpha} (violates the \
         component<=alpha precondition by 1)"
    );
    println!("  reciprocal(alpha={alpha}) = {reciprocal}");
    println!("  product = component * reciprocal = {product}");
    println!("  (product + 128) >> 8 = {divided}  <-- correct answer should be near 255 (white)");
    println!("  ... but `as u8` truncation/wraparound yields: {out}  <-- collapses to near-BLACK");
    assert_eq!(out, 0, "expected the isolated computation to collapse to 0");
    println!(
        "Confirms: IF this precondition is ever violated by a real caller, the isolated \
         arithmetic silently produces a drastically wrong (near-black instead of near-white) \
         result. Part 2 below checks whether real rendering can ever actually violate it."
    );
    println!();
}

fn rand_u32(state: &mut u64) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    (*state >> 32) as u32
}

fn rand_f64(state: &mut u64) -> f64 {
    (rand_u32(state) as f64) / (u32::MAX as f64)
}

const MIX_MODES: &[Mix] = &[
    Mix::Normal,
    Mix::Multiply,
    Mix::Screen,
    Mix::Overlay,
    Mix::Darken,
    Mix::Lighten,
    Mix::ColorDodge,
    Mix::ColorBurn,
    Mix::HardLight,
    Mix::SoftLight,
    Mix::Difference,
    Mix::Exclusion,
    Mix::Hue,
    Mix::Saturation,
    Mix::Color,
    Mix::Luminosity,
];

const COMPOSE_MODES: &[Compose] = &[
    Compose::Clear,
    Compose::Copy,
    Compose::SrcOver,
    Compose::DestOver,
    Compose::SrcIn,
    Compose::DestIn,
    Compose::SrcOut,
    Compose::DestOut,
    Compose::SrcAtop,
    Compose::DestAtop,
    Compose::Xor,
    Compose::Plus,
];

fn run_search(iterations: u32) -> usize {
    let mut state: u64 = 0x1234_5678_9abc_def1;
    let mut resources = Resources::new();
    let mut found = 0usize;

    for attempt in 0..iterations {
        let w = 8u16;
        let h = 8u16;
        let mut ctx = RenderContext::new(w, h);

        let use_layer = rand_u32(&mut state) % 3 == 0;
        if use_layer {
            let opacity = rand_f64(&mut state) as f32;
            let mix = MIX_MODES[(rand_u32(&mut state) as usize) % MIX_MODES.len()];
            let compose = COMPOSE_MODES[(rand_u32(&mut state) as usize) % COMPOSE_MODES.len()];
            ctx.push_layer(
                None,
                Some(BlendMode::new(mix, compose)),
                Some(opacity),
                None,
                None,
            );
        }

        let n_fills = 1 + (rand_u32(&mut state) % 6);
        for _ in 0..n_fills {
            let mix = MIX_MODES[(rand_u32(&mut state) as usize) % MIX_MODES.len()];
            let compose = COMPOSE_MODES[(rand_u32(&mut state) as usize) % COMPOSE_MODES.len()];
            let blend_mode = BlendMode::new(mix, compose);

            let r = (rand_f64(&mut state) * 255.0) as u8;
            let g = (rand_f64(&mut state) * 255.0) as u8;
            let b = (rand_f64(&mut state) * 255.0) as u8;
            let a = (rand_f64(&mut state) * 255.0) as u8;
            let color: AlphaColor<Srgb> = AlphaColor::from_rgba8(r, g, b, a);

            ctx.set_paint(color);
            ctx.set_blend_mode(blend_mode);

            // Fractional-coverage rect to force anti-aliased partial-pixel blending, the
            // scenario the introducing PR's commit message specifically calls out as producing
            // rounding deltas.
            let x0 = rand_f64(&mut state) * (w as f64);
            let y0 = rand_f64(&mut state) * (h as f64);
            let x1 = x0 + rand_f64(&mut state) * (w as f64 - x0).max(0.1);
            let y1 = y0 + rand_f64(&mut state) * (h as f64 - y0).max(0.1);
            let rect = Rect::new(x0, y0, x1.max(x0 + 0.1), y1.max(y0 + 0.1));

            ctx.fill_rect(&rect);
        }
        if use_layer {
            ctx.pop_layer();
        }
        ctx.flush();

        let render_mode = if attempt % 2 == 0 {
            RenderMode::OptimizeQuality
        } else {
            RenderMode::OptimizeSpeed
        };
        let settings = RasterizerSettings {
            render_mode,
            ..Default::default()
        };

        let mut pixmap = Pixmap::new(w, h);
        ctx.render_with(&mut pixmap, &mut resources, settings);

        let data = pixmap.data();
        for (idx, px) in data.iter().enumerate() {
            if px.r > px.a || px.g > px.a || px.b > px.a {
                found += 1;
                println!(
                    "VIOLATION at attempt {attempt} pixel {idx} (render_mode={render_mode:?}): \
                     r={} g={} b={} a={} - precondition `component <= alpha` broken by real \
                     rendering",
                    px.r, px.g, px.b, px.a
                );

                let mut pm2 = Pixmap::new(w, h);
                ctx.render_with(&mut pm2, &mut resources, settings);
                let unpremul = pm2.take_unpremultiplied();
                let out_px = unpremul[idx];
                println!(
                    "  Pixmap::take_unpremultiplied() -> r={} g={} b={} a={}",
                    out_px.r, out_px.g, out_px.b, out_px.a
                );
            }
        }
    }

    found
}

fn main() {
    print_isolated_math_demo();

    println!(
        "=== Part 2: real end-to-end search via vello_cpu::RenderContext's public API \
         (RenderMode::OptimizeQuality and OptimizeSpeed alternating, random Mix/Compose blend \
         modes, random layer nesting, anti-aliased fractional coverage) ==="
    );
    let iterations: u32 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(2_000_000);
    println!("running {iterations} randomized scenes on an 8x8 canvas ({} pixel checks)...", iterations as u64 * 64);

    let found = run_search(iterations);

    println!();
    println!("tried {iterations} real-rendered scenes, found {found} real invariant violations");
    if found == 0 {
        println!(
            "did NOT reproduce via the real end-to-end rendering path - real rendering never \
             produced a `component > alpha` premultiplied pixel in this search."
        );
        std::process::exit(0);
    } else {
        println!("REPRODUCED via the real end-to-end rendering path.");
        std::process::exit(1);
    }
}
