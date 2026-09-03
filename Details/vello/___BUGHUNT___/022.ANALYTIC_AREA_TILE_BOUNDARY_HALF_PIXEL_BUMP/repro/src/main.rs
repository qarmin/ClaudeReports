use vello_cpu::color::palette::css;
use vello_cpu::kurbo::{BezPath, Rect};
use vello_cpu::peniko::Fill;
use vello_cpu::{Pixmap, RenderContext, Resources};

// Candidate 022 hypothesis under test: "a per-tile-boundary discontinuity in the analytic
// coverage computation reused across adjacent tiles" causes the half-pixel wall bumps seen in
// https://github.com/linebender/vello/issues/1556 under Analytic Area AA.
//
// This program compares two ways of rendering the same visual "T-junction" shape (a horizontal
// wall segment with a vertical wall segment hanging off its bottom edge, exactly as two touching-
// but-not-merged maze wall pieces would be authored) under vello_cpu's Analytic Area AA:
//
//   1. `single_path` (ground truth): the union rendered as ONE closed path - what the
//      geometrically-correct anti-aliased coverage should look like.
//   2. `two_rects`: the same union rendered as TWO separate `fill_path` calls with the same
//      opaque color, composited with normal alpha-over blending - how a scene made of
//      independently-drawn wall segments (the actual maze demo) would be built.
//
// It diffs the two renders while sweeping the T-junction's position both across sub-pixel offsets
// AND across every offset (0..=12) relative to the 4px tile grid. If candidate 022's hypothesis
// were correct, the size of the discrepancy should depend on whether the junction lands on a tile
// boundary vs. mid-tile. If instead it's an ordinary "conflation artifact" (compositing two
// independently-antialiased shapes at a concave corner - a well-known, inherent limitation of
// analytic-coverage AA, not specific to this project's tiling), the discrepancy should be exactly
// the same magnitude regardless of tile alignment, and should instead appear/disappear based only
// on whether the corner is a concave (reflex) junction between two SEPARATE shapes.
//
// A second check (`single_shape_edge_is_tile_boundary_clean`) rules out a *plain* per-tile
// discontinuity independently of any conflation: a single path (no compositing of separate shapes
// at all) with a fractional vertical edge spanning many tile rows must have byte-identical
// coverage on every row.

fn rect_path(r: &Rect) -> BezPath {
    let mut p = BezPath::new();
    p.move_to((r.x0, r.y0));
    p.line_to((r.x1, r.y0));
    p.line_to((r.x1, r.y1));
    p.line_to((r.x0, r.y1));
    p.close_path();
    p
}

fn render_path(path: &BezPath, w: u16, h: u16) -> Vec<u8> {
    let mut ctx = RenderContext::new(w, h);
    let mut resources = Resources::new();
    ctx.set_paint(css::BLACK);
    ctx.set_fill_rule(Fill::NonZero);
    ctx.fill_path(path);
    ctx.flush();
    let mut target = Pixmap::new(w, h);
    ctx.render(target.as_mut(), &mut resources);
    target.data_as_u8_slice().to_vec()
}

fn render_rects_composited(rects: &[Rect], w: u16, h: u16) -> Vec<u8> {
    let mut ctx = RenderContext::new(w, h);
    let mut resources = Resources::new();
    ctx.set_paint(css::BLACK);
    ctx.set_fill_rule(Fill::NonZero);
    for r in rects {
        // Go through fill_path (not fill_rect's axis-aligned fast path) so each shape is really
        // composited as an independent path, the way separately drawn wall segments would be.
        ctx.fill_path(&rect_path(r));
    }
    ctx.flush();
    let mut target = Pixmap::new(w, h);
    ctx.render(target.as_mut(), &mut resources);
    target.data_as_u8_slice().to_vec()
}

/// Hand-built union of two touching (not overlapping) rects: `r0` is a wide, short horizontal
/// wall; `r1` is a narrower vertical wall hanging off `r0`'s bottom edge (`r1.y0 == r0.y1`). This
/// produces two concave (reflex) corners in the union, at `(r1.x0, r0.y1)` and `(r1.x1, r0.y1)` -
/// the kind of corner where conflation artifacts between independently-antialiased shapes show up.
fn union_path(r0: Rect, r1: Rect) -> BezPath {
    assert!(r1.x0 >= r0.x0 && r1.x1 <= r0.x1, "r1 must be inset in x from r0");
    assert!((r1.y0 - r0.y1).abs() < 1e-9, "r1 must start exactly where r0 ends");

    let mut p = BezPath::new();
    p.move_to((r0.x0, r0.y0));
    p.line_to((r0.x1, r0.y0));
    p.line_to((r0.x1, r0.y1));
    p.line_to((r1.x1, r0.y1));
    p.line_to((r1.x1, r1.y1));
    p.line_to((r1.x0, r1.y1));
    p.line_to((r1.x0, r0.y1));
    p.line_to((r0.x0, r0.y1));
    p.close_path();
    p
}

fn max_byte_diff(a: &[u8], b: &[u8], w: u16, h: u16) -> (u8, usize, usize) {
    let mut max_diff = 0_u8;
    let mut at = (0, 0);
    for y in 0..h as usize {
        for x in 0..w as usize {
            let idx = (y * w as usize + x) * 4;
            for c in 0..4 {
                let d = a[idx + c].abs_diff(b[idx + c]);
                if d > max_diff {
                    max_diff = d;
                    at = (x, y);
                }
            }
        }
    }
    (max_diff, at.0, at.1)
}

fn conflation_vs_tile_alignment(w: u16, h: u16) -> bool {
    println!("=== Test 1: conflation-artifact magnitude vs. tile-grid alignment ===");
    println!("frac_x  base_y  tile_shift(0..12, tile height=4)  max_byte_diff  at");

    let mut diffs = Vec::new();
    for &frac_x in &[0.0_f64, 0.1, 0.3, 0.5, 0.7, 0.9] {
        for &tile_shift in &[0_u16, 1, 2, 3, 4, 5, 6, 7, 8, 12] {
            let base_y = 10.0 + tile_shift as f64;
            let r0 = Rect::new(2.0 + frac_x, base_y, 30.0 + frac_x, base_y + 6.3);
            let r1 = Rect::new(12.0 + frac_x, base_y + 6.3, 20.0 + frac_x, 36.0);

            let composited = render_rects_composited(&[r0, r1], w, h);
            let single = render_path(&union_path(r0, r1), w, h);

            let (diff, x, y) = max_byte_diff(&composited, &single, w, h);
            println!("{frac_x:.1}     {base_y:.1}    shift={tile_shift:<3}                        {diff:<4}          ({x},{y})");
            diffs.push((tile_shift, diff));
        }
    }

    // The hypothesis under test predicts the diff should vary with tile_shift (bigger right at
    // tile boundaries, e.g. shift % 4 == 0, than mid-tile). Check whether all diffs across all
    // tile_shift values (for a fixed frac_x) are identical - if so, tile alignment has NO effect.
    let all_same_regardless_of_tile_shift = diffs.windows(2).all(|w| w[0].1 == w[1].1);
    println!(
        "diff is real (conflation artifact present): {}",
        diffs.iter().any(|(_, d)| *d > 0)
    );
    println!(
        "diff magnitude is IDENTICAL regardless of tile-grid alignment (refutes the \
         per-tile-boundary hypothesis): {all_same_regardless_of_tile_shift}"
    );
    all_same_regardless_of_tile_shift
}

fn single_shape_edge_is_tile_boundary_clean(w: u16, h: u16) -> bool {
    println!();
    println!("=== Test 2: single-path straight edge across many tile-row boundaries ===");
    let mut p = BezPath::new();
    let r = Rect::new(5.37, 0.0, 30.0, h as f64);
    p.move_to((r.x0, r.y0));
    p.line_to((r.x1, r.y0));
    p.line_to((r.x1, r.y1));
    p.line_to((r.x0, r.y1));
    p.close_path();

    let buf = render_path(&p, w, h);
    let x = 5_usize;
    let values: Vec<u8> = (0..h as usize)
        .map(|y| buf[(y * w as usize + x) * 4 + 3])
        .collect();
    println!("alpha at x={x} for rows 0..{h}: {values:?}");
    let all_identical = values.windows(2).all(|v| v[0] == v[1]);
    println!("coverage identical on every row (no per-tile discontinuity): {all_identical}");
    all_identical
}

fn main() {
    let w = 40_u16;
    let h = 40_u16;

    let refutes_tile_hypothesis = conflation_vs_tile_alignment(w, h);
    let no_single_shape_discontinuity = single_shape_edge_is_tile_boundary_clean(w, h);

    println!();
    if refutes_tile_hypothesis && no_single_shape_discontinuity {
        println!(
            "RESULT: NOT REPRODUCIBLE as a per-tile-boundary bug. A real coverage artifact exists \
             at concave junctions between independently-composited shapes, but its magnitude is \
             independent of tile-grid alignment, and a single path's own coverage across tile \
             rows is perfectly consistent. This matches a plain conflation artifact, not a \
             tile-boundary-reuse defect."
        );
    } else {
        println!(
            "RESULT: signal seen that is NOT explained by conflation alone - would need further \
             investigation of tile-boundary-specific state."
        );
    }
}
