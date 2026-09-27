# FEMTOVG_BOX_SHADOW_BLIT_PATH

`internal/renderers/femtovg/itemrenderer.rs`'s `draw_box_shadow` builds the blurred shadow
texture inside a `BoxShadowCache::get_box_shadow` closure that only runs on a cache miss, but the
`femtovg::Path` used afterwards to blit that texture onto the canvas (`shadow_image_rect`) is
built with a fresh `femtovg::Path::new()` **outside** the closure, so it allocates on every frame
the shadow is visible, cache hit or not - unlike `rect_to_path`/`rect_with_radius_to_path`, which
candidate `006.FEMTOVG_PATH_REUSE` already pools. This extends that same `ScratchPath`/thread-
local-pool mechanism to this one remaining call site, per `IDEAS.md` row 62.

## What was tried

Added a `--shadow` flag to `perf_test`'s `rects` scene (nothing in `perf_test` drew a box shadow
before this - see `../README.md`/this candidate's `perf_test_fixes/` for the harness change),
then pooled `shadow_image_rect` the same way candidate 006 pools `rect_to_path`: a thread-local
`Vec<femtovg::Path>` behind an RAII `ScratchPath` guard (`Deref`/`DerefMut` to `femtovg::Path`,
cleared and returned to the pool on drop, capped at 8). Needs the same `femtovg::Path::clear()`
addition (`patch_femtovg.diff`) candidate 006 added to the vendored femtovg checkout - additive,
already validated there (`cargo test --lib`: 76 passed).

Scoped to only this call site (not `rect_to_path`/`rect_with_radius_to_path`, which 006 already
measures separately, one fix per candidate folder).

| Variant | single-core p50 (`taskset -c 3`) | multi-core p50 | notes |
|---|---|---|---|
| baseline vs baseline (noise floor), `--count 2000 --shadow --frames 12` | 112.46 ms | 47.52 ms | same binary both sides |
| baseline, `--count 2000 --shadow --frames 12` | 111.45 ms | 47.43 ms | |
| patched, same | 111.97 ms | 47.42 ms | +0.5% / -0.0% |
| baseline, `--count 8000 --shadow --frames 10` | 119.61 ms | 57.43 ms | |
| patched, same | 119.20 ms | 56.92 ms | -0.3% / -0.9% |

All numbers: `RUNS=9`, `tools/paired_femtovg_later_frames.sh`, median of 9 alternating runs each
side, `--renderer femtovg`, `LIBGL_ALWAYS_SOFTWARE=1 xvfb-run -s "-screen 0 1280x900x24"` (femtovg
has no headless `--bench` path). Machine: 24 cores, `uptime` load 0.6-0.8 at measurement time.
Per-pair win counts (not just medians, per MEASUREMENT.md): 5/9, 6/9, 4/9, 6/9 across the four
rows above - none reach the >=8/9 bar this workspace treats as decisive.

Confirmed the allocation removal is real via the harness's own allocation counter
(`--frames 6 --count 500 --shadow`, later-frame steady state): baseline 31406 allocations/frame ->
patched 30406 allocations/frame, a drop of exactly 1000 = 500 items x 2 (the pooled path's verb
and coordinate vectors, same 2-allocations-per-`Path::new()` mechanism candidate 006 found).

## Result

No measurable wall-clock change. The allocation saving is real and exactly the size predicted
(-2 allocations/item), but every paired comparison (2000 and 8000 items, single- and multi-core)
lands inside the noise floor, with win counts (4-6 of 9) far short of the >=8/9 this workspace's
tooling treats as a real effect.

## Does it change behavior/output?

- **Bit-exact?** Yes. `md5sum`-identical PPM snapshots between baseline and patched binaries,
  3 runs each side, at `--mode rects --count 500 --shadow --frames 6` (plain shadow), `--shadow
  --gradient` (shadow + gradient fill), and no `--shadow` at all (unaffected code path) - all
  three variants byte-identical baseline vs. patched.
- **Is this a breaking change?** No. `femtovg::Path::clear()` is additive femtovg API (already
  shipped as part of candidate 006's still-unapplied patch); the renderer-side pool is an internal
  implementation detail with no observable behavior change.
- `cargo test --lib` in the femtovg checkout with `patch_femtovg.diff` applied: 76 passed, 0
  failed (same as candidate 006's rebase measured - this candidate reuses that exact femtovg-side
  patch unmodified). `i-slint-renderer-femtovg` has no unit tests of its own; `cargo build --bin
  perf_test --profile rdebug` (default features, femtovg + software) compiles cleanly with both
  patches applied together.

## Why the result looks the way it does

Same mechanism, same conclusion as candidate `060.FEMTOVG_9SLICE_FITS_VEC`: the allocation this
candidate removes (2 small `Vec`s/item/frame) is real but tiny next to what actually dominates
this scene's frame time under `LIBGL_ALWAYS_SOFTWARE=1` (llvmpipe) - femtovg draw calls, path
tessellation, and (for the box-shadow scene specifically) a `SourceIn`-composited texture blit per
shadowed item. The blit and its surrounding GPU/software-rasterization state changes cost far more
per item than the two `malloc`/`free` pairs the pool removes, so removing them doesn't move the
needle at either 2000 or 8000 items. Candidate 006's own original finding matches this exactly:
"Frame time is unchanged within noise here... because llvmpipe's CPU rasterization dominates that
frame" - this call site is smaller still (one path per *item*, not per rectangle-with-border-and-
background), so if anything the effect should be less visible than 006's already-marginal one, and
that's what was measured.

## What's not done / open questions

- Not measured on a real (non-`llvmpipe`) GPU driver, where per-item draw-call cost is lower and a
  fixed per-item CPU allocation cost might show up proportionally more - same caveat candidate 006
  and 060 both left open, still unaddressed (added back to `IDEAS.md` as a standing "re-measure the
  small femtovg allocation candidates on real hardware" item, not re-added per-candidate).
- Only the `rects` scene's box shadow was exercised (`--shadow`, fixed 6px blur / 2px offset). Not
  tested with a much larger blur radius (bigger shadow texture, same blit-path cost) or with many
  distinct blur/radius combinations defeating `BoxShadowCache` (a cache-miss-heavy scenario), which
  would additionally exercise the *other*, still-cache-miss-only `femtovg::Path::new()` inside the
  same function's `get_box_shadow` closure - out of this candidate's scope by design (that one isn't
  a per-frame cost, see the IDEAS.md entry this candidate closes out).
- This candidate's patch is standalone (only this call site); combining it with candidate 006's own
  patch (which pools `rect_to_path`/`rect_with_radius_to_path`, also drawn by every shadowed
  rectangle's background/border) was not measured together - plausible the combined effect is more
  visible than either alone, not verified.
