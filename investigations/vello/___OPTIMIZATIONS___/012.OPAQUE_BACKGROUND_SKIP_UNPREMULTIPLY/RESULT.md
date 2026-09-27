# vello_cpu: detect opaque, viewport-covering backgrounds to skip `take_unpremultiplied`'s per-pixel scan

`Pixmap::take_unpremultiplied()` (`sparse_strips/vello_common/src/pixmap.rs`) already has a fast
path: it only runs its per-pixel `unpremultiply_rgba8` SIMD scan when `self.may_have_transparency`
is `true`. The problem this candidate (upstream PR #1850) addresses is that `RenderContext::render`
(`sparse_strips/vello_cpu/src/render.rs`) never actually sets that flag - a freshly-created
`Pixmap` starts with `may_have_transparency: true` and nothing in the render path ever downgrades
it - so `take_unpremultiplied` always pays the full per-pixel scan, even for the extremely common
case of a scene that draws a fully opaque, viewport-covering background first (a solid page/canvas
background) and is therefore actually 100% opaque end to end. The patch adds a `RootBackground`
detector (`sparse_strips/vello_common/src/record.rs`) that recognizes this pattern - a `fill_rect`
covering the whole viewport with an opaque solid paint, drawn before anything else, with the
background not later invalidated by a non-default-blend-mode draw touching the root target - and,
when it holds through to the end of the render, sets `may_have_transparency = false` on the output
`Pixmap`, and also uses the extracted background color to fill now-uncovered fine-buffer regions
directly (instead of unpacking the destination or zero-filling) since the background's own
`fill_rect` command is elided entirely rather than rasterized normally.

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`. CPU: x86_64 (`fearless_simd`
picks AVX2 on this machine, exercised by both `unpremultiply_rgba8` and the render pipeline's own
SIMD fine kernels).

## What was tried

Applied PR #1850 as-is (one hunk in `render.rs` needed a 1-line manual rebase past an unrelated,
already-landed opacity-clamping commit; everything else applied cleanly - see `patch.diff`).

No existing benchmark reached the `RenderContext::render()` -> `Pixmap::take_unpremultiplied()`
call sequence this candidate targets (the existing `pixmap/unpremultiply` bench in
`vello_bench::pixmap` builds its `Pixmap` via `Pixmap::from_parts` with a from-parts opacity
rescan, which *already* downgrades a fully-opaque "opaque" test input to `may_have_transparency =
false` before baseline, so it doesn't exercise the gap this PR closes). Extended
`sparse_strips/vello_bench/src/integration.rs` (already the project's home for full-pipeline
`RenderContext` benchmarks, extending its existing `images` bench module rather than adding a new
bench crate) with a new `integration::background` bench group, gated the same way as `images`
(`#[cfg(feature = "extended")]` via `crate::EXTENDED`) - see `patch.diff`. Scene: a 1920x1080
viewport with a viewport-covering opaque white `fill_rect` drawn first, followed by 40 smaller,
semi-transparent (alpha 200) colored rects on top (representative of an opaque page background
with translucent overlay content - eligible for `RootBackground` detection, but not fully
re-covered by later draws). Three bench functions:

- `render` - just `RenderContext::render()`, to check whether the fine-buffer-fill side of the
  patch (background color written directly instead of unpacking/zeroing) moves the render time
  itself.
- `render_and_take_unpremultiplied` - the full realistic call sequence: render, then
  `pixmap.take_unpremultiplied()`.
- `take_unpremultiplied_only` - isolates just the mechanism this candidate targets:
  `take_unpremultiplied()` on a `Pixmap.clone()` of an already-rendered pixmap (built once outside
  the timed loop), so the reported number is purely the effect of the `may_have_transparency` flag.

Measured with two independently-built binaries (separate `CARGO_TARGET_DIR`s: one from an
unmodified checkout + bench-only diff = baseline, one with the full patch = patched), same
Criterion binary flags (`--warm-up-time 1 --measurement-time 2`), baseline and patched run
**back-to-back** (not literally alternated sample-by-sample - Criterion's own per-benchmark
warm-up/collection already runs each side for 2+ seconds, and both runs landed within Criterion's
own tight confidence intervals across repeats - see below), 2 full repeats each, both pinned
single-core (`taskset -c 0`) and unpinned (whole 24-core, moderately loaded, `uptime` load average
~8-13 during this session - other sibling optimization candidates were running concurrently, per
this workspace's shared-build-lock convention) multi-core:

| Variant | `render` (single-core) | `render` (multi-core) | `render_and_take_unpremultiplied` (single-core) | `render_and_take_unpremultiplied` (multi-core) | `take_unpremultiplied_only` (single-core) | `take_unpremultiplied_only` (multi-core) |
|---|---|---|---|---|---|---|
| baseline, run 1 | 345.4 µs | 339.0 µs | 1.6265 ms | 1.2474 ms | 869.9 µs | 844.1 µs |
| patched, run 1 | 342.4 µs | 344.7 µs | 391.4 µs | 412.7 µs | 26.16 ns | 25.69 ns |
| baseline, run 2 | 381.9 µs | 341.9 µs | 1.3937 ms | 1.2638 ms | 900.4 µs | 859.8 µs |
| patched, run 2 | 346.1 µs | 329.1 µs | 480.5 µs | 391.7 µs | 29.81 ns | 26.33 ns |

(All figures are Criterion's own point estimate, i.e. median of its bootstrap distribution, from
100-sample runs; each cell's own 95% CI width was under ~3% of the point estimate except `render`
run 2 baseline, single-core, which had a wider spread - consistent with ordinary scheduler noise
on a loaded machine, not a directional signal, since it doesn't change the conclusion below.)

A separate, standalone (non-Criterion) check confirmed the same effect with a plain
`std::time::Instant`-timed loop (50 iterations, clone + `take_unpremultiplied` per iteration):
baseline 869.7 µs average, patched 35 ns average - consistent with the Criterion numbers above.

## Result

**Time.** Two of the three measured quantities show a large, consistent, reproducible effect
across all 4 single-core/multi-core x run-1/run-2 combinations; the third (`render` alone) shows
no reliable effect:

- **`take_unpremultiplied_only`** (the mechanism this candidate specifically targets): baseline
  ~844-900 µs -> patched ~26-30 ns, i.e. **the per-pixel unpremultiply scan is skipped almost
  entirely** (residual patched time is just the `Vec<PremulRgba8>` -> `Vec<Rgba8>` cast, which
  `bytemuck::cast_vec` on a matching layout does at effectively zero cost) - roughly a
  30,000-35,000x reduction in this specific call's own cost, for a 1920x1080 image whose whole
  background was detected as opaque. This scales with image pixel count (it's a full O(pixels)
  scan being skipped), so the absolute saving grows with resolution.
- **`render_and_take_unpremultiplied`** (the realistic end-to-end sequence a caller experiences
  when fetching straight (unpremultiplied) RGBA8 bytes, e.g. for PNG encoding or handing pixels to
  a non-premultiplied-alpha consumer): baseline ~1.25-1.63 ms -> patched ~0.39-0.48 ms, i.e.
  **roughly 65-75% lower** total time for this scene shape (viewport-covering opaque background +
  moderate translucent overlay content) on a 1920x1080 target.
- **`render` alone**: baseline 339-382 µs vs patched 329-346 µs across the 4 combinations - the
  two ranges overlap and neither side is consistently ahead; a separate, later `just test` run of
  this candidate's own justfile (non-interleaved, single build each) showed the opposite direction
  again (~360 µs baseline vs ~394-449 µs patched for `render` alone), reinforcing that whatever
  small effect the fine-buffer-fill side of this patch has on `render()` in isolation is within
  this environment's noise floor, not a reliably measurable win or loss for the tested scene.

No memory/allocation-count claim is made beyond what's implied by the time result: the patched
`take_unpremultiplied` still allocates and returns the same `Vec<Rgba8>` (via `bytemuck::cast_vec`,
which reuses the existing `Vec<PremulRgba8>` allocation rather than allocating a new one, on both
baseline and patched) - this candidate removes CPU work (the scan), not allocations.

## Does it change behavior/output?

- **Bit-exact?** Yes, for the scene this candidate's bench exercises. Verified with a dedicated,
  standalone check (not part of `patch.diff`): rendered the same scene (opaque viewport-covering
  background + 40 translucent overlay rects) on both baseline and patched builds, called
  `take_unpremultiplied()` on each, and hashed the resulting byte buffer
  (`std::hash::DefaultHasher` over the raw RGBA8 bytes) - **identical hash
  (`e1d4cd76e63acc36`), identical length (8,294,400 bytes), identical first/last 16 bytes** on
  both builds, despite `pixmap.may_have_transparency()` differing (`false` on patched, `true` on
  baseline) - confirming the flag change doesn't alter the actual pixel values for this scene, only
  whether `take_unpremultiplied` bothers to touch them (correct, since the scene actually is fully
  opaque end to end: every pixel starts at the opaque background color and any translucent overlay
  composited via `SrcOver` onto an already-opaque destination is still opaque, by definition of
  premultiplied-alpha `SrcOver` compositing).
- The PR's own added unit tests (9 new tests in `vello_cpu/src/render.rs`, 5 new tests in
  `vello_common/src/record.rs`) specifically probe the cases where the heuristic must *not* fire
  (an active clip, an active mask, inside a layer/opacity group, a non-default blend mode touching
  the root target after the background is recorded, a second draw that doesn't cover the full
  viewport) - all pass (see test counts below), which is the project's own evidence that the
  heuristic's guard conditions are exercised, not just the happy path.
- **Is this a breaking change?** No - internal implementation detail of `RenderContext::render`
  and `Pixmap::take_unpremultiplied`. No public API removed; the PR does add new public items
  (`PremulColor::as_opaque_color`, `PixmapMut::set_may_have_transparency`,
  `vello_common::record::RootBackground`) but doesn't change the signature or behavior of any
  existing public API's documented contract.
- Project test suite (`cargo test --release`, run through the shared build lock, before = base
  commit + bench-only diff, after = base commit + full `patch.diff`):
  - `-p vello_common --lib`: baseline 159 passed / 0 failed / 1 ignored -> patched **164 passed** /
    0 failed / 1 ignored (+5 new `RootBackground` unit tests in `record.rs`).
  - `-p vello_cpu --lib`: baseline 81 passed / 0 failed -> patched **90 passed** / 0 failed (+9 new
    background-detection tests in `render.rs`).
  - `-p vello_cpu --doc`: both baseline and patched hit the same **pre-existing, unrelated**
    doctest failure (`error[E0432]: unresolved import 'glifo::atlas::GlyphCacheStats'`), confirmed
    present on a completely unmodified checkout - not caused by this patch (same finding as
    candidate 006's `RESULT.md`).
  - `-p vello_sparse_tests` (the crate with the most direct visual/snapshot coverage): **not
    usable for a real pass/fail comparison in this environment** - `git-lfs` is not installed here,
    so this checkout's reference PNG snapshots are unresolved LFS pointer files, and essentially
    every snapshot-comparison test fails identically regardless of this patch (same environment
    limitation as candidate 006). Baseline: 89 passed / 3538 failed / 1230 ignored. Patched: 89
    passed / 3569 failed / 1239 ignored - the **same 89 tests pass in both**, and the +31
    failed/+9 ignored delta is exactly accounted for by the 9 new `#[vello_test]` functions this
    PR adds (`clip_prevents_background`, `compose_clear_after_background`,
    `layer_prevents_background`, `mask_prevents_background`), each expanded by the project's own
    test macro into multiple backend variants (cpu_f32/cpu_u8/hybrid/multithreaded x several SIMD
    levels) - all of which hit the same LFS-missing-reference-image failure as every other
    snapshot test in this environment, not a patch-caused regression. No pre-existing test in this
    suite newly failed or newly passed.

## Why the result looks the way it does

`take_unpremultiplied`'s existing `may_have_transparency` fast path was already there and already
correct - the gap was purely that nothing in the render path ever populated it accurately for the
common "opaque background" case, so real callers (e.g. anyone calling `take_unpremultiplied` to
get straight alpha bytes for PNG encoding, screenshotting, or handing off to a non-premultiplied
consumer) were paying a full-image SIMD scan for information the renderer could have known for
free. The scan itself isn't slow *per pixel* (it's already vectorized, 64 bytes/iteration via
`fearless_simd`), but it's still O(pixel count) work with a real reciprocal-multiply per pixel, so
at 1920x1080 (~2.07M pixels) it adds up to the ~850-900 µs measured here - which the patch reduces
to a fixed, tiny cost by proving up front (via cheap, O(draw-command-count) bookkeeping during
recording, not a pixel scan) that the scan can be skipped altogether. The `render()`-time side of
the patch (filling now-uncovered fine-buffer spans with the extracted background color directly,
instead of unpacking the destination pixmap or zero-filling) is a much smaller effect in this
scene: the background's own `fill_rect` command still had to be classified as "solid, fully
covering" and processed somehow either way (either as a normal `fill_rect_fast` covering-solid
draw command on baseline, or as a background-color scratch-buffer fill on patched), so unlike
`take_unpremultiplied` there's no full pass being newly skipped here - just one cheap
representation being swapped for another comparably cheap one, which is consistent with this
measurement landing inside the noise floor rather than showing a clear directional win.

## What's not done / open questions

- Not benchmarked on non-x86_64 (NEON/scalar-fallback) targets - `unpremultiply_rgba8`'s existing
  `dispatch!` machinery is unmodified by this patch, and the new `RootBackground` bookkeeping is
  plain scalar Rust with no SIMD dependency, so risk of a platform-specific regression is low, but
  no ARM measurement was taken here.
- Only one scene shape was benchmarked (opaque full-viewport background + moderate translucent
  overlay). The heuristic's own guard conditions (clip/mask/layer/non-default-blend invalidation)
  are covered by the PR's own unit tests (see above) but weren't independently re-measured for
  performance here - a scene that repeatedly invalidates and re-establishes the background, or one
  where the background is covered by opaque content before the end of the scene (in which case
  `may_have_transparency` still ends up `false` correctly, just via a different code path), would
  be worth a follow-up bench if a reader wants tighter bounds on the render-time side of this
  patch specifically.
- The environment's git-lfs gap means the project's visual snapshot suite could not be used as an
  additional correctness signal beyond the dedicated hash-based check and the PR's own unit tests;
  a reader with `git-lfs` available could re-run `cargo test -p vello_sparse_tests` for the new
  `*_prevents_background`/`compose_clear_after_background` snapshot tests specifically.
- This candidate did not investigate whether the same `RootBackground` mechanism could be
  extended to `vello_hybrid` (the PR's own description mentions this as a considered, not yet
  implemented, follow-up, blocked partly on a separate PR #1783).
