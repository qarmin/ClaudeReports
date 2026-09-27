// Standalone reproduction for candidate 001: `Tiles::make_tiles_msaa`'s `needs_clamping` check
// (sparse_strips/vello_common/src/tile.rs) is a mathematical tautology for every finite (non-NaN)
// `f32` line, so the "unclamped fast path" it's supposed to gate is dead code. This uses only
// vello_common's public API (`vello_common::tile::Tiles`, `vello_common::flatten::{Line, Point}`).
//
// Two independent checks, both against the real public entry point / the real published formula:
//
// 1. A property check over many random finite (non-NaN) `f32` pairs, recomputing the exact
//    `needs_clamping` formula quoted from the source at the base commit. This proves the
//    tautology itself: it should never find a single finite input for which the formula is
//    `false`.
//
// 2. A real call to the public `Tiles::make_tiles_msaa` with a `NaN` line coordinate (the one
//    input class that *does* make the source's `needs_clamping` formula evaluate to `false`,
//    since comparisons against NaN are always false). NaN is genuine external input here (e.g. a
//    degenerate upstream transform producing 0/0) - this is a single-participant call through the
//    real public API, not a hand-built stand-in for a second participant. Before the fix, this
//    collapses the whole line to a single degenerate tile column (x == 0) for every row; after
//    the fix (which deletes the dead branch entirely, always taking what used to be the "clamped"
//    path), the same call produces a sane, multi-column result instead.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced.

use vello_common::fearless_simd::Level;
use vello_common::flatten::{Line, Point};
use vello_common::tile::Tiles;

/// The exact formula from `make_tiles_msaa` (tile.rs, base commit) that decides whether the
/// "unclamped" fast path is taken. Returns `true` when the (buggy, tautological) check says
/// clamping is needed - i.e. `false` is the only case in which the dead branch would run.
fn needs_clamping_per_source_formula(p0_x: f32, p1_x: f32) -> bool {
    let (line_left_x, line_right_x) = if p0_x < p1_x {
        (p0_x, p1_x)
    } else {
        (p1_x, p0_x)
    };
    let min_x = p0_x.min(p1_x);
    let max_x = p0_x.max(p1_x);
    min_x < line_left_x || max_x >= line_right_x
}

/// Small deterministic PRNG (splitmix64) so this repro needs no extra dependency and is fully
/// reproducible without a fixed external seed file.
struct SplitMix64(u64);
impl SplitMix64 {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn next_f32(&mut self, lo: f32, hi: f32) -> f32 {
        let u = (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32; // in [0, 1)
        lo + u * (hi - lo)
    }
}

fn check_tautology() -> bool {
    let mut rng = SplitMix64(0xC0FFEE_1234_5678);
    const N: usize = 2_000_000;
    let mut counterexamples = 0usize;
    for _ in 0..N {
        // Realistic range: tile-space coordinates for viewports up to ~65535px wide.
        let p0_x = rng.next_f32(-20_000.0, 20_000.0);
        let p1_x = rng.next_f32(-20_000.0, 20_000.0);
        if !needs_clamping_per_source_formula(p0_x, p1_x) {
            counterexamples += 1;
            if counterexamples <= 5 {
                eprintln!("  counterexample: p0_x={p0_x}, p1_x={p1_x} -> needs_clamping = false");
            }
        }
    }
    println!(
        "Tautology check: {N} random finite (p0_x, p1_x) pairs, {counterexamples} counterexamples \
         (cases where needs_clamping was false)."
    );
    counterexamples == 0
}

fn nan_case_tile_xs() -> Vec<u16> {
    // A line from (NaN, 5.0) to (50.0, 90.0) in a 100x100 viewport (25x25 tiles of size 4).
    // p1 alone spans tile column 12 (50.0 / 4.0), so a sane result should touch more than one
    // tile column across its rows.
    let line = Line {
        p0: Point { x: f32::NAN, y: 5.0 },
        p1: Point { x: 50.0, y: 90.0 },
    };

    let mut tiles = Tiles::new(Level::baseline(), 100, 100);
    tiles.make_tiles_msaa(&[line], 100, 100);

    tiles.sort_tiles();
    tiles.iter().map(|t| t.x).collect()
}

fn main() {
    println!("== Check 1 (documentary, not a before/after signal): is `needs_clamping` a");
    println!("   tautology for finite input, per the formula quoted from the source? ==");
    let tautology_holds = check_tautology();
    println!(
        "-> {}",
        if tautology_holds {
            "the formula was true for every finite sample; the \"unclamped\" branch it gates is \
             unreachable for any of these inputs. Note: this check reimplements the formula \
             standalone, so it holds identically whether run against the pristine or the patched \
             tree (the patch deletes the check/branch rather than changing this arithmetic fact) \
             - it's supporting evidence for the root cause, not the before/after signal."
        } else {
            "found finite input(s) where the formula is false without NaN involved - the \
             tautology claim does not hold as stated."
        }
    );

    println!();
    println!("== Check 2 (the actual before/after signal): real public API call with a NaN");
    println!("   line coordinate ==");
    let xs = nan_case_tile_xs();
    println!("Tile x-coordinates produced (sorted): {xs:?}");
    let all_zero = !xs.is_empty() && xs.iter().all(|&x| x == 0);
    println!(
        "-> {}",
        if all_zero {
            "REPRODUCED: every emitted tile collapsed to column x=0, even though the line's \
             finite endpoint (50.0, tile column 12) means a correct/expected result should span \
             more than one column - a phantom column of geometry at the wrong location. This is \
             the NaN-reachable manifestation of the dead \"unclamped\" branch's missing \
             NaN-rescuing clamp (before the fix)."
        } else {
            "did not reproduce: no phantom x=0 column artifact (after the fix, this line's \
             tiles are computed by what used to be the \"clamped\" path only, which discards the \
             NaN via f32::min/max instead of propagating it)."
        }
    );

    if all_zero {
        std::process::exit(1);
    }
}
