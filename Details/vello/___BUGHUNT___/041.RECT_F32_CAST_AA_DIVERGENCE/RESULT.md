# RECT_F32_CAST_AA_DIVERGENCE

`sparse_strips/vello_common/src/rect.rs` lines 49-52 downcast `rect.x0`/`y0`/`x1`/`y1` from `f64`
to `f32` in vello_cpu's fast axis-aligned `fill_rect` strip-generation path
(`render_impl`/`vello_common::rect::render`). The candidate claimed this cast is (a) flagged by
`clippy::cast_precision_loss`, and (b) causes a real alpha-coverage divergence of exactly 1/255
(e.g. 26 vs 25) between `fill_rect(&rect)` and `fill_path(&rect.to_path(eps))` for identical
sub-pixel-positioned rects, allegedly because the general `fill_path` pipeline "keeps rect
coordinates in f64 through flattening" while the fast path doesn't.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `flatten.rs` to check the premise that `fill_path` keeps coordinates in f64 through flattening | Static read of `vello_common::flatten::Point` and its `impl From<kurbo::Point> for Point` | False premise: `flatten.rs` converts every `kurbo::Point` (f64) to its own `Point` (f32) via `value.x as f32` / `value.y as f32` immediately when a path enters the flattening pipeline - the exact same downcast expression `rect.rs:49-52` uses, applied to the same corner value (for an axis-aligned rect with the identity transform used by both `fill_rect`'s fast path and `fill_path`, `Rect::to_path`'s corners pass through unmodified). Both paths downcast to f32 at essentially the same point on the same value, so a divergence caused specifically by "one path staying in f64 longer" is not structurally possible here. |
| `cargo clippy -p vello_common --lib -- -W clippy::cast_precision_loss` | debug build, whole crate | Only flags `rect.rs:138` (an unrelated `usize -> f32` cast inside the `coverage()` helper). Does **not** flag lines 49-52 at all - `clippy::cast_precision_loss` only covers integer-to-float casts, not `f64 -> f32` narrowing casts. The candidate's specific clippy claim does not hold in this checkout. |
| Differential equivalence test #1 (structured/curated sweep): render the same `Rect` via `RenderContext::fill_rect` and via `RenderContext::fill_path(&rect.to_path(0.1))`, compare every output pixel's premultiplied alpha, over sub-pixel x0/y0 offsets x widths/heights straddling the 4x4 tile grid x 4 coordinate-magnitude bases (0, 3, 1000, 65000, i.e. up to just below `u16::MAX` where the f64->f32 downcast loses the most relative precision) | Release build, 7056 rect configurations checked (`repro/src/main.rs` step 2) | 0 mismatched pixels found across all 7056 configurations. |
| Differential equivalence test #2 (seeded random fuzz, fixed PRNG seed for repeatability): random sub-pixel positions in [-2,40), random widths/heights in [0.001,20), same fill_rect vs fill_path comparison, intersected against a 48x48 canvas | Release build, 4981 non-degenerate random rects (`repro/src/main.rs` step 3) | 0 mismatched pixels found across all 4981 random rects. |
| Same two differential tests, run first as inline `#[test]`s directly inside `sparse_strips/vello_cpu/src/render.rs` in an isolated worktree (before extracting the standalone `repro/`), including a `multi_threaded` dispatcher check | debug + release builds, `cargo test -p vello_cpu --lib` | Also 0 mismatches. Additionally noticed `vello_cpu`'s multi-threaded dispatcher (`dispatch/multi_threaded.rs::fill_rect_fast`) doesn't even use `rect.rs`'s fast algorithm at all - it falls back to building a 4-point `PathEl` path and rendering it through the ordinary `FillPath` task, so this candidate's suspected code path is single-threaded-dispatcher-only (the default `RenderContext::new` dispatcher, which is what all the above tests exercise). |

## Status

**Not reproducible.** Covered: sub-pixel rect positions from ~0.0001 to ~4.0 past 4 magnitude
bases (0, 3, 1000, 65000), rect widths/heights from 0.5 to ~8.0 straddling the 4-pixel tile grid
in both directions, near-`u16::MAX` coordinates (where `f64->f32` relative precision loss is
largest), and 4981 uniformly-random sub-pixel rects intersected against a canvas - a total of over
12,000 real end-to-end renders compared pixel-by-pixel. No divergence of any magnitude (not just
1/255) was found between `fill_rect` and `fill_path(rect.to_path(eps))` in any of them. Re-run
twice with an identical (deterministic, fixed-seed) result both times.

**Fully reproducible**: N/A (no bug found - this is a negative result; see Status). The
differential checks themselves ran the real, public, end-to-end `vello_cpu::RenderContext` API
(`fill_rect`/`fill_path` -> `render_with` -> `Pixmap`) directly, with no faked participant.

## Class

N/A (not Confirmed - see Status).

## Root cause

Not a bug, for two independent reasons:

1. The candidate's premise that `fill_path`'s flattening pipeline "keeps rect coordinates in f64
   through flattening" (implying `fill_rect`'s fast path uniquely loses precision) is false as of
   this checkout: `vello_common::flatten::Point`'s `From<kurbo::Point>` impl casts every point's
   `x`/`y` to `f32` immediately (`flatten.rs:43-46`), using the identical `as f32` expression
   `rect.rs:49-52` uses. For an axis-aligned rect rendered with the identity transform (as used in
   both call sites in these tests), both paths reduce to casting the exact same `f64` corner value
   to `f32` the same way - there is no extra f64-precision window on the `fill_path` side for a
   divergence to come from.
2. Even setting the premise aside and just testing empirically: both paths ultimately quantize
   fractional coverage to a `u8` alpha with the identical `(coverage * 255.0 + 0.5) as u8`-shaped
   formula (`rect.rs`'s `combined_tile_alpha`/`alpha_mask_from_x_coverage` vs. `strip.rs`'s
   `coverage.mul_add(255.0, 0.5)` in the general winding-accumulation code), and across >12,000
   real rendered comparisons (curated boundary sweep + random fuzz), the two paths never produced
   a different quantized alpha for the same rect - not even by 1/255.

## Introduced by

N/A - not applicable, since Status is Not reproducible (no bug found to trace to an introducing
commit).

## The fix

No fix attempted - there is nothing to fix (see Status/Root cause). No source files in the main
project were modified during this investigation; all rendering comparisons were run from a
separate git worktree (`/tmp/vello-wt-041`, deleted after this session) with a few temporary
`#[test]`s added to `render.rs` purely for exploration, never committed or left behind, plus the
standalone `repro/` crate built against a fresh clone.

- Project test suite: N/A, no change was made to the main project. Sanity baseline (pristine
  worktree, no patch): `cargo test -p vello_cpu --lib` -> 81 passed; 0 failed; 0 ignored (before
  adding the temporary differential tests); 83 passed; 0 failed; 0 ignored (with the two temporary
  differential tests added, both passing).

## What's not done / open questions

- Did not test non-identity, non-axis-aligned-but-still-supported transforms combined with
  `fill_rect` (a skew/rotation forces `fill_rect` itself to fall back to `rect_to_temp_path` +
  `fill_path`, so both code paths already converge there by construction - not a distinct case for
  this candidate's suspected divergence).
- Did not test with a non-default `aliasing_threshold` (this also forces `fill_rect`'s own
  fallback to path-based rendering, so `rect.rs`'s fast algorithm is never invoked in that case
  either).
- Did not test the multi-threaded dispatcher's `fill_rect_fast` any further, since it was found to
  not use `rect.rs`'s algorithm at all in this checkout (see "What was tried" table) - it already
  goes through `fill_path` internally, so it cannot exhibit this candidate's suspected divergence
  by construction.
- If a future session wants to push this further: `vello_common/src/rect.rs`'s own module tests
  (`render_edge_row_at_u16_right_edge`/`render_edge_row_at_u16_bottom_edge`) already probe
  `u16::MAX`-adjacent coordinates directly against `rect.rs`'s strip/alpha output (not a
  `fill_path` comparison) - could be extended into a proper differential test living permanently
  in the project's own test suite, rather than only in this candidate's throwaway `repro/`.
