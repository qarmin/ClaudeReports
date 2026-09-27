// Standalone reproduction for: a zero-length subpath stroked with `Cap::Round` (or `Cap::Square`)
// does not render the dot/square the SVG spec requires (vello issue #1450, and independently
// reported as #1055 against an earlier renderer).
//
// This drives vello_cpu's real public API end-to-end (`RenderContext::stroke_path` ->
// `RenderContext::render_with` -> `Pixmap`) - the same path any real caller of the crate uses.
// This is a single-participant bug (a caller handing a degenerate path straight to the renderer's
// genuine public entry point), so no second participant needs to be faked here (see
// CONFIRMATION.md section 11 in the bug-hunt skill this was found by).
//
// Exit code convention: 0 = did not reproduce (bug absent, i.e. fix is present and working),
// non-zero = reproduced (bug present).

use vello_cpu::color::palette::css::LIME;
use vello_cpu::kurbo::{BezPath, Cap, Stroke};
use vello_cpu::{Pixmap, RasterizerSettings, RenderContext, RenderSettings, Resources};

/// Render `path` stroked with the given cap style into a 100x100 pixmap and return the alpha
/// value of the pixel at (50, 50) - the center of the degenerate point used by every case below.
fn center_alpha(path: &BezPath, cap: Cap) -> u8 {
    let mut ctx = RenderContext::new_with(100, 100, RenderSettings::default());
    let mut resources = Resources::new();

    ctx.set_stroke(Stroke::new(3.0).with_caps(cap));
    ctx.set_paint(LIME);
    ctx.stroke_path(path);
    ctx.flush();

    let mut pixmap = Pixmap::new(100, 100);
    ctx.render_with(&mut pixmap, &mut resources, RasterizerSettings::default());

    let stride = 100usize * 4;
    let idx = 50 * stride + 50 * 4;
    pixmap.data_as_u8_slice()[idx + 3]
}

fn zero_length_closed_path() -> BezPath {
    // The exact repro from vello issue #1450.
    let mut path = BezPath::new();
    path.move_to((50.0, 50.0));
    path.line_to((50.0, 50.0));
    path.close_path();
    path
}

fn zero_length_open_path() -> BezPath {
    // The exact repro from vello issue #1055 (no close_path).
    let mut path = BezPath::new();
    path.move_to((50.0, 50.0));
    path.line_to((50.0, 50.0));
    path
}

fn main() {
    let mut failures = Vec::new();

    let round_closed = center_alpha(&zero_length_closed_path(), Cap::Round);
    println!("#1450 repro (closed subpath, Cap::Round): center alpha = {round_closed}");
    if round_closed == 0 {
        failures.push("closed zero-length subpath with Cap::Round rendered nothing (expected an opaque dot)");
    }

    let round_open = center_alpha(&zero_length_open_path(), Cap::Round);
    println!("#1055 repro (open subpath, Cap::Round):   center alpha = {round_open}");
    if round_open == 0 {
        failures.push("open zero-length subpath with Cap::Round rendered nothing (expected an opaque dot)");
    }

    let square_closed = center_alpha(&zero_length_closed_path(), Cap::Square);
    println!("closed subpath, Cap::Square:              center alpha = {square_closed}");
    if square_closed == 0 {
        failures.push("closed zero-length subpath with Cap::Square rendered nothing (expected an opaque square)");
    }

    // Sanity check: Cap::Butt must still render nothing for a zero-length subpath (per spec) -
    // a fix that draws a dot unconditionally would be just as wrong as drawing nothing always.
    let butt_closed = center_alpha(&zero_length_closed_path(), Cap::Butt);
    println!("closed subpath, Cap::Butt (must stay 0):  center alpha = {butt_closed}");
    if butt_closed != 0 {
        failures.push("closed zero-length subpath with Cap::Butt rendered something (expected nothing)");
    }

    if failures.is_empty() {
        println!("did not reproduce - all cases match the SVG zero-length-subpath cap spec");
    } else {
        eprintln!("REPRODUCED:");
        for f in &failures {
            eprintln!("  - {f}");
        }
        std::process::exit(1);
    }
}
