# Software renderer's radial/conic gradient stop `Vec` allocated every draw

`internal/renderers/software/lib.rs`'s radial-gradient (~1690-1709) and conic-gradient
(~1711-1727) branches of `process_rectangle_impl` build `RadialGradientCommand`/
`ConicGradientCommand` by `g.stops().map(|s| { ...alpha_color... }).collect()`ing into a fresh
`SharedVector<GradientStop>` on every single draw of that primitive - flagged by a femtovg/software
discovery sweep because the linear-gradient branch immediately above handles the identical
per-stop alpha-premultiply job with a lazy `.copied().map(...).peekable()` iterator and no
`collect()`/allocation at all, from `IDEAS.md`'s backlog.

## What was tried

Reading the code first: unlike the linear-gradient branch (which consumes its stops immediately,
within the same function call, and never stores them), the radial/conic branches build a command
struct that one `ProcessScene` implementation (`SceneBuilder`, used by the default `render()`)
pushes into a `Vec` for a later, separate scanline-drawing pass - the borrow from the original
`Brush` cannot survive to that later pass, so *some* owned copy is structurally required there,
unlike the linear case. Mirroring the linear gradient's zero-allocation iterator trick verbatim is
therefore not directly possible for a field typed `SharedVector<GradientStop>`.

The one part of the per-draw cost that *is* avoidable: `alpha_color()` is already a no-op when
`alpha == 255` (the common case - no explicit item alpha and no inherited opacity), so at that
alpha the collected array is byte-for-byte identical to the brush's own already-stored stops
(minus its 3-entry position/radius/angle header). Added `RadialGradientBrush`/
`ConicGradientBrush::raw_stops_with_header_len()` in `i-slint-core`'s `brush.rs`, returning a
*cheap* `SharedVector::clone()` (refcount bump, no allocation) of the whole backing storage plus
the header length. `RadialGradientCommand`/`ConicGradientCommand` gained a `stops_offset: usize`
field and a `stops()` accessor (`&self.stops[self.stops_offset..]`); `draw_functions.rs`'s
`draw_radial_gradient`/`draw_conic_gradient` were changed to go through `g.stops()` instead of
`g.stops` directly. `process_rectangle_impl` now takes the cheap-clone fast path when
`args.alpha == 255` and falls back to the original `collect()` (with `stops_offset = 0`) otherwise.

Full diff: `patch.diff`.

Harness: `perf_test`'s `rects` scene already had a `--gradient` flag that only produced a
`@linear-gradient` (candidate 009's scene). Extended it into `--gradient-kind linear|radial|conic`
(`--gradient` is kept as an alias for `linear`) across `backend.rs`/`bench.rs`/`main.rs` and
`ui/rect-mode.slint`/`ui/main.slint`, so `rects --gradient-kind radial` and `--gradient-kind conic`
now reach the code path this candidate touches. This is a harness extension, not a separate
candidate (see `perf-optimize`'s "documenting the harness" convention) - `perf_test` itself is
untracked scratch tooling at the `slint` repo root, so the change isn't part of the reverted
`slint` diff.

| Variant | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | notes |
|---|---|---|---|
| baseline, radial, `--count 3000 --gradient-kind radial --repaint-buffer reused --frames 20` | 12.10-12.12 ms | 12.15-12.24 ms | 3 paired runs, tight spread |
| patched, same | 12.33-12.37 ms | 12.31 ms | **+1.7% to +2.2%**, 0-1 of 9 pairs faster (9/9, 9/9, 8/9 consistent) |
| baseline, conic, `--count 3000 --gradient-kind conic --repaint-buffer reused --frames 20` | 16.04 ms | 15.92 ms | |
| patched, same | 16.85 ms | 16.68 ms | **+5.0% / +4.8%**, 0 of 9 pairs faster (9/9 consistent both times) |

All via `tools/paired.sh` (`RUNS=9 FRAMES=20`), software renderer's own headless `--bench` (no
GPU/Xvfb involved - this code path is pure CPU rasterization). Machine: 24 cores, `uptime` load
0.7-2.7 at measurement time. A genuine same-binary noise floor could not be produced with
`paired.sh` itself (it refuses to compare two identical binaries), but 6 manual same-binary runs
at these settings showed the scene is itself bimodal (~12.1 ms / ~15.1 ms, matching `paired.sh`'s
own header comment about this workload) - the *paired*, alternating comparison above is what
carries the conclusion, not any single median.

## Result

**A confirmed, consistent regression**, not a win: removing the allocation made both the radial
and conic gradient paths measurably *slower* (+1.3% to +2.2% radial, +4.8% to +5.0% conic),
reproduced 5 times total (3x radial, 2x conic) across pinned and unpinned runs, every time with
8 or 9 of 9 paired runs going the same direction - well past this workspace's "decisive" bar.

Allocation counts confirm the intended effect *did* happen: `--frames 5`, `--count 3000
--gradient-kind radial --repaint-buffer reused`, steady-state frame: 60856 -> 59746 allocations
(-1110, matching the ~1100 visible gradient rectangles - exactly one allocation removed per draw,
as intended). The allocation reduction is real; the wall-clock effect is the opposite sign anyway.

## Does it change behavior/output?

- **Bit-exact?** Yes. `md5sum`-identical `--snapshot` PPMs between baseline and patched binaries
  for: plain `--gradient-kind radial`/`conic` (`--count 3000`, 6 frames); `--item-alpha 0.5`
  (exercises the non-255-alpha fallback path, `--count 500`); `--render-by-line` (exercises the
  other `ProcessScene` implementation - the immediate line-drawing one, not `SceneBuilder`),
  for both gradient kinds. 8 comparisons total, all matched.
- **Is this a breaking change?** No public API change reaches outside the crate: `ProcessScene`,
  `RadialGradientCommand`/`ConicGradientCommand` and the `scene` module that defines them are all
  crate-private to `i-slint-renderer-software` (`mod scene;`, no `pub use`). The one change visible
  outside that crate is `i-slint-core`'s new `#[doc(hidden)] pub fn raw_stops_with_header_len()`
  on `RadialGradientBrush`/`ConicGradientBrush` - purely additive, no existing signature changed.
- Project test suite: `cargo test -p i-slint-core --lib`: 307 passed, 0 failed (unchanged).
  `cargo check -p i-slint-renderer-software`: clean, no warnings introduced.

## Why the result looks the way it does

`SharedVector::clone()` bumps its refcount with `fetch_add(1, Ordering::SeqCst)` - a locked,
fully-fenced atomic RMW - and the corresponding `Drop` pays a matching decrement when the command
is later consumed. That is *two* locked instructions per draw, replacing *one* allocation +
matching `free()` for what is, in this scene, a 3-stop (~24-48 byte) array. Small allocations of a
fixed size, drawn from a hot thread-local free list, are apparently cheaper here than a `SeqCst`
atomic pair on a repeatedly-touched cache line (the same brush's refcount, bumped and dropped
~1100 times per frame for every visible gradient rectangle) - the opposite of what candidate 009's
femtovg-side version of this exact idea found (there, `IntoIterator`-based femtovg gradient APIs
let the allocation be avoided *without* introducing a new atomic operation to replace it, so that
one carried no matching cost). Candidate 059 independently found the same `SharedVector`-Clone
atomic-op cost mattering (in the opposite direction: removing *unneeded* clones there was a small
win); this candidate is evidence that *adding* a clone specifically to dodge an allocation is not
a substitution safe to assume is free.

## What's not done / open questions

- Not investigated: whether a non-atomic (`Relaxed`-ordering, or plain `Cell`-based) refcount
  would close this gap - `SharedVector` is used throughout `i-slint-core` for values that can be
  shared across threads in some configurations, so relaxing its ordering is a much larger, riskier
  change than this single call site and out of scope here.
- Not investigated: whether the effect is specific to this scene's stop count (3) - candidate 059
  found a sign flip between 8 and 64 stops for a related `SharedVector` clone-vs-copy question.
  Higher stop counts might tip the allocation-vs-atomic-op tradeoff back the other way; not
  measured.
- The underlying premise from `IDEAS.md` - "the linear-gradient path proves the allocation is
  avoidable" - turned out to only be true in the narrow sense that *an* allocation-free
  implementation exists (this patch), not that it's *faster*. Left as a documented dead end so a
  future session doesn't re-attempt the same "just clone instead of collect" idea without
  re-deriving this measurement.
