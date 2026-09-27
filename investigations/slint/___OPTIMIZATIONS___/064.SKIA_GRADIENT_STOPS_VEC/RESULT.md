# Skia's `to_skia_stops` allocated two fresh `Vec`s per gradient draw, every frame

`internal/renderers/skia/itemrenderer.rs`'s `brush_to_shader` calls `to_skia_stops(&g.stops)` at
all three gradient call sites (linear/radial/conic, originally lines 255/269/282), each of which
`unzip()`s the brush's stops into a fresh `Vec<skia_safe::Color4f>` and a fresh `Vec<f32>`, on
every single draw of a gradient-filled primitive, every frame - flagged by the 2026-08-23 skia
discovery sweep as the Skia-backend analog of candidate 009 (the same idea for femtovg), which
that sweep confirmed was never addressed by Skia's separate `resolve_brush` refactor (that
refactor made the *brush* allocation-free, but never touched this stop-unzipping helper).

## What was tried

Unlike candidate 009's femtovg fix (whose gradient-stop APIs accept `impl IntoIterator`, so the
`collect()` could be dropped entirely) and candidate 063's software-renderer attempt (where a
`SharedVector::clone()` regressed performance by trading an allocation for two atomic refcount
ops), Skia's `skia_safe::gradient::Colors::new` needs two borrowed slices (`&[Color4f]`,
`&[f32]`) with a matching lifetime, consumed synchronously within the same call that builds the
`Shader` - no `IntoIterator` API exists, and no owned-value type is cheap to clone here (`Vec`s
own their storage, there's no `SharedVector` in this crate). Reused a thread-local scratch buffer
instead: `GRADIENT_STOP_SCRATCH: RefCell<(Vec<Color4f>, Vec<f32>)>`, cleared and refilled on each
call via `with_skia_stops(stops, |colors, pos| { ... })`, which hands the two slices to a closure
that builds and returns the `Shader` before the borrow ends. All three gradient match arms
(`LinearGradient`/`RadialGradient`/`ConicGradient`) now go through this helper instead of calling
`to_skia_stops` and holding onto owned `Vec`s across the `gradient(...)` call.

Harness: `perf_test`'s `rects` scene already had `--gradient-kind linear|radial|conic` (built for
candidate 063). Used unmodified; no harness changes needed for this candidate.

| Variant | single-core p50 (`taskset -c 3`, `--gradient-kind linear`) | multi-core p50 | notes |
|---|---|---|---|
| baseline | 14.15 ms | 15.35 ms | `perf_test --renderer skia --frames`, `xvfb-run` + `LIBGL_ALWAYS_SOFTWARE=1`, `--mode rects --count 3000 --frames 20` |
| patched | 14.16 ms | 14.88 ms | **+0.1%** single-core (5/9 pairs faster), **-3.1%** multi-core (5/9 pairs faster) |
| baseline, `--gradient-kind radial` | 14.59 ms | 15.12 ms | |
| patched, same | 14.25 ms | 15.36 ms | **-2.3%** single-core, **+1.6%** multi-core |

All via `tools/paired_skia_later_frames.sh` (`RUNS=9`), single-core pinned (`TASKSET=3`) and
unpinned multi-core. Machine: 24 cores, `uptime` load 0.4-1.4 at measurement time (well under core
count, no re-baseline needed).

Allocation counter (`--frames 10`, same scene, steady state): baseline 56750 -> patched 54522
allocations/frame (**-2228**, exactly 2 per visible gradient rectangle - confirms the intended
allocation is actually removed at ~1114 visible rects out of 3000 placed).

## Result

The targeted allocation is confirmed removed (allocation counter, above), but wall-clock time
shows no consistent direction: linear gradient single-core is a wash (+0.1%, 5/9 pairs), multi-core
shows -3.1% but only 5/9 pairs favor the patch; radial gradient flips sign again (-2.3%
single-core, +1.6% multi-core). None of the four measurements reach this workspace's decisive bar
of 8-9/9 paired runs agreeing. This matches candidates 009/060/061's precedent for Skia/femtovg
GPU-backed renderers: removing a small (2-8 stop, tens-of-bytes) allocation is real but too small
next to a 14-18ms frame dominated by GPU rasterization/compositing to show up in wall-clock time.

## Does it change behavior/output?

- **Bit-exact?** Yes. `md5sum`-identical `--frames --snapshot` PPM captures for
  `--gradient-kind linear`, `radial`, and `conic` (baseline vs. patched, `--count 3000`, 8
  frames each) - 3 comparisons, all matched.
- **Is this a breaking change?** No - `to_skia_stops` and the new `with_skia_stops`/
  `GRADIENT_STOP_SCRATCH` are private to `i-slint-renderer-skia`'s `itemrenderer.rs`; no public
  API changed.
- Project test suite: `i-slint-renderer-skia` has no `#[test]`s (confirmed via
  `grep -rn '#\[test\]' internal/renderers/skia/*.rs`, same as noted in candidate 056) - relied
  on the pixel-snapshot comparisons above instead.

## Why the result looks the way it does

Skia's gradient shader construction (`linear_gradient`/`radial_gradient`/`sweep_gradient`) already
does its own internal copy of the stop data into the `SkShader` it returns, plus whatever GPU
resource setup and rasterization Skia performs for the actual fill - a 3-stop gradient's ~2
removed heap allocations (tens of nanoseconds combined, per candidate 009's femtovg measurement of
a similarly-sized allocation) are a rounding error next to a frame that already takes 14-18ms on
llvmpipe (software Vulkan/GL via `LIBGL_ALWAYS_SOFTWARE=1`). The allocation-count evidence shows
the fix does what it's supposed to; the timing evidence shows that isn't where this scene's frame
time goes.

## What's not done / open questions

- Not measured on a real GPU (only `LIBGL_ALWAYS_SOFTWARE=1` llvmpipe via Xvfb was available in
  this session) - a hardware-accelerated backend might show a different balance between
  CPU-side allocation cost and GPU fill cost, though candidate 056's experience on this same
  machine suggests CPU-side per-item overhead does show up when it's large enough (that
  candidate's image-texture fix was a real, large win under the same llvmpipe setup).
- Higher stop counts (candidate 059/063 both found sign flips between low and high stop counts
  for adjacent `SharedVector`-related changes) weren't tried here - only the harness's default
  3-stop gradients were measured.
- `draw_glyph_run`'s analogous glyph-id/position `Vec` unzip (also flagged by the same
  2026-08-23 skia sweep, `itemrenderer.rs:1093-1096` at the time) was not touched - left in
  `IDEAS.md`.
