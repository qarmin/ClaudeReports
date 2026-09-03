# Skia per-frame dirty-region clip: skip the `PathBuilder` for the common single-rect case

`internal/renderers/skia/lib.rs`'s partial-rendering path (`render_components_to_canvas`, around
line 676) built the per-frame clip by always constructing a fresh `skia_safe::PathBuilder`, adding
one rectangle per entry in `partial_renderer.dirty_region` (at most 3, see
`i_slint_core::partial_renderer::DirtyRegion::MAX_COUNT`), `detach()`ing it into a `Path`, and
calling `clip_path()` - even when the region is exactly one rectangle, where `clip_rect()` does
the identical job with no `Path`/`PathBuilder` object built at all. Flagged by the 2026-08-23
"skia sweep" entry in `IDEAS.md`.

## What was tried

Branch on `dirty_region.iter()` yielding exactly one rect (checked without allocating, by pulling
at most two items off the iterator) and call `skia_canvas.clip_rect(...)` directly in that case;
fall back to the original `PathBuilder`/`clip_path()` code for 0, 2, or 3 rects. The `Visualize`
debug mode (`SLINT_SKIA_PARTIAL_RENDERING=visualize`) still needs an actual `Path` to draw an
outline with afterwards, so it always takes the general path regardless of rect count - only the
default (no debug mode) and `Log` modes take the new single-rect shortcut.

**Harness note (real finding, not just plumbing):** `perf_test`'s existing `--frames
--renderer skia` path cannot reach this code at all. `frames.rs` times frames via
`Window::set_rendering_notifier`, and `i_slint_renderer_skia::SkiaRenderer::partial_rendering_state()`
unconditionally returns `None` whenever a rendering notifier is registered ("We don't know where
the application might render to, so disable partial rendering" - see `lib.rs`), regardless of
`SLINT_SKIA_PARTIAL_RENDERING`. Confirmed by temporarily adding `eprintln!("RECT_COUNT {}",
partial_renderer.dirty_region.iter().count())` right at the clip-building code: it never printed
under any `--frames --renderer skia` invocation tried (`--mode scroll`, various counts/frames),
with or without `SLINT_SKIA_PARTIAL_RENDERING` set. This means the premise in `IDEAS.md`'s
original entry ("existing perf_test --renderer skia scenes already exercise partial rendering
with single dirty rects") was wrong - a new harness mode was needed. Added `--skia-bench`
(`perf_test/src/skia_headless.rs`), which drives `SkiaRenderer::new_with_surface` +
`software_surface::SoftwareSurface` directly - the same pattern
`api/rs/slint/tests/partial_renderer.rs` uses for its own tests - reusing one `SharedPixelBuffer`
across frames so `back_buffer_age` is 1 from the second frame on. No rendering notifier is
involved, so partial rendering stays on unconditionally and frame time is measured with a plain
`Instant` around each `renderer.render()` call. See `../README.md`'s "Instrumentation added"
section for the full file list; the harness lives in `perf_test/` (untracked in the main repo),
not in this candidate's `patch.diff`.

With `--skia-bench --mode list --count 5000 --frames 60`, the dirty region was confirmed (via the
same temporary `eprintln!`, removed before the final patch) to be exactly 1 rect on every frame
after the first - `list` uses `ListView`'s virtualization, so each frame's redraw is one
contiguous band of newly-visible rows. `--mode scroll` (non-virtualized) instead produced 3 rects
per frame (the multi-rect path, unaffected by this fix) - a useful data point for future
candidates that specifically want the multi-rect branch.

| Variant | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | notes |
|---|---|---|---|
| baseline vs patched, run 1 | 1.545 ms | 1.527 ms | `RUNS=9`, `--skia-bench --mode list --count 5000 --frames 60` |
| baseline vs patched, run 1 (patched) | 1.527 ms | 1.538 ms | -1.2% single-core, +0.7% multi-core |
| baseline vs patched, run 2 (repeat) | 1.557 ms → 1.523 ms | 1.541 ms → 1.547 ms | -2.2% single-core, +0.4% multi-core |
| baseline vs patched, run 3 (`tools/paired_skia_headless.sh`, `RUNS=9`) | 1.559 ms → 1.519 ms | 1.547 ms → 1.531 ms | -2.6% single-core (6/9 pairs faster), -1.0% multi-core (6/9 pairs faster) |
| baseline vs patched, run 4 (`RUNS=15`) | 1.569 ms → 1.548 ms | 1.534 ms → 1.512 ms | -1.3% single-core (10/15 pairs faster), -1.4% multi-core (9/15 pairs faster) |

All four rounds via `___OPTIMIZATIONS___/tools/paired_skia_headless.sh` (added for this
candidate - see `../README.md`), same two binaries throughout (one `cargo build` per variant, not
rebuilt between rounds). Machine: 24 cores, `uptime` load 1.8-2.3 at measurement time (well under
core count, no re-baseline needed).

## Result

Small and consistent, not decisive: across 4 independent measurement rounds (2 single-core,
2 multi-core, `RUNS` 9 and 15), the patched binary was faster by roughly 1-2.6% in every round,
with 6-10 of 9-15 paired runs favoring it each time (60-67%). None of the rounds reach this
workspace's stated decisive bar of 8-9/9 paired runs agreeing, and the very first exploratory
pair of runs (done with an ad hoc script before `paired_skia_headless.sh` existed) showed a wash
on multi-core (+0.7%, +0.4%) - but every round using the final script agrees on direction and
magnitude. Total frame time in this scene is only ~1.5 ms (a `ListView` redrawing one newly
visible band of rows), so a ~1-2% win is consistent with removing one small `PathBuilder`
allocation (on Skia's C++ heap, not the Rust one - see below) plus a few fewer virtual/FFI calls
per frame, not a structural change.

## Does it change behavior/output?

- **Bit-exact?** Yes. `--skia-bench --mode list --count 5000 --frames 10 --snapshot out.ppm`
  produces byte-identical PPM output before and after the patch (`sha256sum` match,
  `91cd72a1cbcd242e8ae7289e66a8f5533273008c629bf32b3d93c79a27817c16` both times). Expected:
  `Canvas::clip_rect(rect, op, aa)` and `Canvas::clip_path(single-rect-path, op, aa)` apply the
  identical clip geometry for a single, non-antialiased axis-aligned rectangle.
- **Is this a breaking change?** No - internal renderer code, no public API or documented
  behavior touched.
- Rust-side allocation counter (`perf_test`'s `alloc_stats`, counts the Rust global allocator)
  showed **no difference** (2531 allocations/frame both variants) - `skia_safe::PathBuilder`'s
  internal storage is a C++ `SkPath` allocated through Skia's own (non-Rust) heap, so the
  allocation this candidate removes is invisible to this counter. This does not contradict the
  timing result; it just means allocation count is the wrong instrument for this particular fix
  (unlike e.g. candidate 064's Rust-side `Vec`s, which the same counter did detect).
- Project test suite: not run - no existing test exercises this exact code path (see harness note
  above; `api/rs/slint/tests/partial_renderer.rs` tests dirty-region *computation*, not this
  clip-building code), and the change is behaviorally a no-op by construction (single-rectangle
  clip via two equivalent Skia APIs).

## Why the result looks the way it does

The removed work (one `PathBuilder::new()`, one `add_rect()`, one `detach()`) is genuinely small
relative to everything else a frame does (layout read-back, item traversal, Skia's own rasterizer
work for the visible rows) - the scene picked specifically to hit the single-rect branch consistently
(`list`, virtualized) also has one of the smallest total frame times in this workspace's scene set
(~1.5 ms), which is exactly why the ~1-2% signal is visible at all: on a scene with more per-frame
work the same fixed saving would be a smaller fraction and likely disappear into noise entirely,
similar to how candidate 064's analogous Skia allocation fix measured a mix of signs across
gradient kinds.

## What's not done / open questions

- The multi-rect branch (2-3 rects, e.g. `--mode scroll`) is unchanged and unmeasured here by
  design - this candidate only targeted the single-rect case per the original `IDEAS.md` entry.
- The related `.collect::<Vec<_>>()` of damage rects in `software_surface.rs`'s
  `SoftbufferRenderBuffer::with_buffer` (softbuffer-backed windowed presentation, not the
  `SoftwareSurface`/`RenderBuffer` custom-platform path this candidate's harness uses) is a
  separate, untried candidate - see `IDEAS.md`.
- Only `list` was measured for the win; `--skia-bench` also wires up `scroll` and `rects` (see
  `perf_test/src/skia_headless.rs`) for future candidates that need the multi-rect path or a
  higher-per-frame-work scene.
