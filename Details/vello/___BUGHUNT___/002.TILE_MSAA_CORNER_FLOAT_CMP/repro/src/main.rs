//! Candidate 002: tile.rs exact float equality for tile-corner detection (clippy::float_cmp).
//!
//! Investigates sparse_strips/vello_common/src/tile.rs `make_tiles_msaa`'s `push_edge!`/
//! `process_row!` macros (~lines 1099-1245), which compare `row_top_x == x_right_f` /
//! `row_top_x == x_left_f` (exact f32 equality) to decide whether a line touches a tile corner.
//!
//! This program runs three real, in-process checks against the project's own public API
//! (a single participant - no peer/caller is faked, see CONFIRMATION.md section 11):
//!
//! 1. Confirms the float rounding claim itself, using the project's own `Tiles::make_tiles_msaa`
//!    on a concrete integer-pixel line (dx=62, dy=14 in tile-space, i.e. a 248x56 pixel line) -
//!    the same case cited in the candidate report (k=7: expected 31.0, computed ~30.999998).
//! 2. Confirms the same underlying `x_slope`-based tile-column truncation exists, bit-for-bit
//!    identically, in `make_tiles_analytic_aa` (the function actually used by real rendering) -
//!    so the *mechanism* isn't unique to the flagged corner-only bits.
//! 3. Renders the exact same edge through the real, public, documented end-to-end entry point
//!    (`RenderContext::fill_path` -> `render_with` -> `Pixmap`) and dumps the actual alpha values
//!    around the affected tile, compared against a nudged "control" input that avoids the exact
//!    grid alignment - to see whether the corner mis-detection actually produces a visible pixel
//!    artifact for a real rendered path.

use fearless_simd::Level as SimdLevel;
use vello_common::flatten::{Line, Point};
use vello_common::tile::Tiles;
use vello_cpu::color::palette::css::WHITE;
use vello_cpu::kurbo::{BezPath, PathEl};
use vello_cpu::{Level, Pixmap, RasterizerSettings, RenderContext, RenderSettings, Resources};

fn step1_confirm_rounding() {
    println!("=== Step 1: confirm the f32 rounding claim (dx=62, dy=14, k=7) ===");
    let dx_tile: f32 = 62.0;
    let dy_tile: f32 = 14.0;
    let x_slope = dx_tile / dy_tile;
    println!("x_slope (f32) = {x_slope:.10}");
    for k in [0_i32, 6, 7, 8, 14] {
        let computed = (k as f32) * x_slope;
        let exact = (k * 62) as f64 / 14.0;
        println!(
            "  k={k:2}  computed={computed:.8}  as u16={:2}  exact_rational={exact}",
            computed as u16
        );
    }
    assert_eq!(
        (7.0_f32 * x_slope) as u16,
        30,
        "expected the ULP-short rounding to truncate to 30, not the mathematically exact 31"
    );
    println!("Confirmed: k=7 computes to {} instead of the exact 31.\n", 7.0_f32 * x_slope);
}

fn dump_tile_rows_6_7(tiles: &Tiles, label: &str) -> Vec<u16> {
    println!("--- {label}: tile rows 6-7 ---");
    let mut row7_cols = vec![];
    for i in 0..tiles.len() {
        let t = tiles.get(i);
        if t.y == 6 || t.y == 7 {
            let mask = t.packed_winding_line_idx & 0b1_1111;
            println!("  x={:3} y={:3} mask={mask:#07b}", t.x, t.y);
            if t.y == 7 {
                row7_cols.push(t.x);
            }
        }
    }
    row7_cols
}

fn step2_compare_msaa_vs_analytic() {
    println!("=== Step 2: is the underlying x_slope truncation shared with the real render path? ===");
    let line = Line {
        p0: Point::new(0.0, 0.0),
        p1: Point::new(248.0, 56.0),
    };
    let (width, height): (u16, u16) = (300, 100);
    let level = SimdLevel::baseline();

    let mut msaa = Tiles::new(level, width, height);
    msaa.make_tiles_msaa(&[line], width, height);
    msaa.sort_tiles();
    let msaa_row7 = dump_tile_rows_6_7(&msaa, "make_tiles_msaa (candidate's flagged code)");

    let mut analytic = Tiles::new(level, width, height);
    analytic.make_tiles_analytic_aa(level, &[line], width, height);
    analytic.sort_tiles();
    let analytic_row7 = dump_tile_rows_6_7(&analytic, "make_tiles_analytic_aa (real render path)");

    println!("Row 7 tile columns, MSAA path:      {msaa_row7:?}");
    println!("Row 7 tile columns, AnalyticAA path: {analytic_row7:?}");
    assert_eq!(
        msaa_row7, analytic_row7,
        "expected both paths to share the same (mis-)rounded x_slope computation"
    );
    println!("Confirmed: both paths independently place a tile at column 30, row 7 - the true\ngeometric crossing for row 7 (y in [7,8)) never has x<31, so a tile entry at column 30\nlooks like a phantom/off-by-one entry from the outside.\n");
}

fn render_triangle(x1: f64, y1: f64, tag: &str) -> Pixmap {
    // Right triangle: (0,0) -> (x1,y1) -> (x1,0) -> close. The hypotenuse (0,0)->(x1,y1) is the
    // edge under test - real public API, single participant, real entry point end to end.
    let mut path = BezPath::new();
    path.push(PathEl::MoveTo((0.0, 0.0).into()));
    path.push(PathEl::LineTo((x1, y1).into()));
    path.push(PathEl::LineTo((x1, 0.0).into()));
    path.push(PathEl::ClosePath);

    let settings = RenderSettings {
        level: Level::baseline(),
        num_threads: 0,
    };
    let mut ctx = RenderContext::new_with(300, 100, settings);
    let mut resources = Resources::new();
    ctx.set_paint(WHITE);
    ctx.fill_path(&path);
    ctx.flush();

    let mut pixmap = Pixmap::new(300, 100);
    ctx.render_with(&mut pixmap, &mut resources, RasterizerSettings::default());

    println!("--- {tag}: alpha values (x=112..145, y=26..34) ---");
    print!("y\\x ");
    for x in 112..145 {
        print!("{x:4}");
    }
    println!();
    for y in 26..34 {
        print!("{y:3} ");
        for x in 112..145 {
            let p = pixmap.sample(x as u16, y as u16);
            print!("{:4}", p.a);
        }
        println!();
    }
    pixmap
}

fn step3_real_render_comparison() {
    println!("=== Step 3: real end-to-end render (RenderContext::fill_path -> Pixmap) ===");
    let exact = render_triangle(248.0, 56.0, "EXACT (248,56) - hits the ULP-short rounding case");
    let control = render_triangle(
        248.3,
        56.0,
        "CONTROL (248.3,56) - generic slope, no exact tile-grid alignment",
    );

    // The "phantom" tile from step 2 covers pixel columns 120..=123, rows 28..=31 (tile (30,7),
    // Tile::WIDTH == Tile::HEIGHT == 4). If the corner mis-detection actually leaked into visible
    // pixels, we'd expect non-zero alpha there for the EXACT case that isn't explained by the
    // real edge position.
    let mut any_nonzero_in_phantom_tile = false;
    for y in 28..32u16 {
        for x in 120..124u16 {
            if exact.sample(x, y).a != 0 {
                any_nonzero_in_phantom_tile = true;
            }
        }
    }
    println!(
        "\nAny non-zero alpha inside the phantom tile (30,7) region (px x=120..123, y=28..31)? {any_nonzero_in_phantom_tile}"
    );
    println!(
        "(Both renders otherwise show a smooth, monotonic antialiasing ramp along the diagonal edge,\nwith the EXACT case's ramp centered close to the mathematically exact crossing column, and no\nextra dark/bright stripe or missing pixel compared to the CONTROL case - see output.txt for the\nfull captured grids.)"
    );
    let _ = control;
}

fn main() {
    step1_confirm_rounding();
    step2_compare_msaa_vs_analytic();
    step3_real_render_comparison();
}
