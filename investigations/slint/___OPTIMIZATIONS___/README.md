# What is under test, and how it is measured

Ported on 2026-08-23 from `perf_test/___OPTIMIZATIONS___/PIPELINE.md` - see `SUMMARY.md`'s header
for provenance. The harness itself (`perf_test/`, a separate Cargo project at the repo root) was
**not** touched, moved, or duplicated - this file only restates how it works for readers of this
workspace. For the fuller original, including every trap discovered while building it, read
`perf_test/___OPTIMIZATIONS___/PIPELINE.md` directly.

## Workload

`perf_test` has several modes over the same generated data (see `perf_test/README.md`):

| mode | what it stresses |
|---|---|
| `text` | one `TextInput` inside a `ScrollView` (hand-rolled `TextEdit`), N lines of ~110 unique lorem characters |
| `list` | `ListView` (virtualized) over N records, row = `Rectangle` + `HorizontalLayout` + 3 `Text` |
| `scroll` | the same rows in a plain `for` inside a `ScrollView` (no virtualization) |
| `rects` | a grid of rounded, bordered rectangles, all visible, moving every frame - dominated by per-primitive renderer cost rather than the item tree |
| `nest` | plain rectangles at a configurable nesting depth, no text - isolates layout-solver cost from shaping cost |
| `images` | `Image` items - count, source sharing, source resolution, real decoded assets under `perf_test/assets/` |

Modifiers reaching paths no static scene does: `--mutate <n> --mutate-mode changed|changed-offscreen|push|insert-front|rotate|reset`
(live model data), `--animate [--animate-count <n>]` (time-driven, **not pixel-comparable**),
`--word-wrap`, `--resize-step <px>`, `--pointer-move`, `--key-input`, `--select-all`/`--type-text`,
`--toggle-global`, `--scale-factor`/`--scale-toggle`, `--repaint-buffer <new|reused|swapped>`,
`--row-style layout|flat|clipped|bare|touch`, `--font-sizes`, `--opacity`/`--radius`/`--border`,
`--path`, `--line-length`. Content is generated to defeat caching (index prefix + rotated word
window + hash suffix).

## Measurement harness

Two entry points, both fill the mode with content, render N frames, print a per-frame table:

| command | renderer | needs a display |
|---|---|---|
| `perf_test --bench` | software (own `MinimalSoftwareWindow` platform) | no |
| `perf_test --frames N` | the selected real renderer (FemtoVG default) | yes, or `xvfb-run` (`just frames-xvfb` inside `perf_test/`) |

Per frame: wall time, allocations, text shapes, paragraph shapes, line breaks; plus RSS after the
last frame. Summary lines are **medians with the spread printed alongside**, not means.

- **allocations** - wrapping global allocator, feature `alloc-stats`.
- **paragraph shapes** (`TextLayoutCache::paragraph_shape_count()`) is the ground truth for "how
  much shaping happened" since candidate 003 made shaping lazy - `text shapes`/`cache_miss_count()`
  only counts cache *entries* rebuilt, not work (see MEASUREMENT.md section 9).
- **allocation attribution** - `--alloc-profile <n>` samples 1 in n allocations, ranks allocation
  *sites*. Needed because `perf`/`valgrind`/`heaptrack` are unavailable in this environment
  (`perf_event_open` blocked even with `perf_event_paranoid=-1`).
- **pixel comparison** - `--snapshot <file>.ppm`. Prefer `--bench` (headless): no display/GL/
  `libxkbcommon` needed, and the scroll offset is derived from the frame count so two runs are
  byte-identical. Compare only above the status line (rows 0..820 at 1280x900) - it prints
  per-run timings, so those bytes always differ. Use `tools/pixelcheck.sh`, not a raw `cmp`.
- **paired comparison** - use `tools/paired.sh` for every before/after (see MEASUREMENT.md
  section 2b) - this machine has a per-process performance mode that survives `taskset` pinning,
  so unpaired medians compare two lotteries.
- **`BenchPlatform`'s clock is a deterministic virtual clock** (candidate `102`), advanced by a
  fixed synthetic 16ms per frame instead of reading the real system clock. This makes any scene's
  `animate` *property* transition (e.g. std-widgets' `ScrollBar` thumb, `animate width, height:
  150ms`) sample the same state on every run, closing the run-to-run pixel jitter candidate `099`
  root-caused (visible only once a scene's own per-frame cost drops below such a transition's
  duration - see `099`/`102`'s `RESULT.md`). `--animate`'s own `animation-tick()`-driven scenes
  read the same clock and so also get deterministic tick *values* now, but candidate `102` found
  `--mode rects --animate` snapshots still differ run to run at some frame counts even so (0 diff
  at `--frames 1`/`2`, thousands of differing px at `--frames 5`/`10`/`20`/`30`, non-monotonically)
  - an unrelated, unidentified source of non-determinism in that scene specifically. `--animate`
  therefore keeps its **not pixel-comparable** label above; that label's reason is different now
  (used to be "real clock", now "a different, uninvestigated source"), not resolved.

## Environment

- FemtoVG under Xvfb needs `libxkbcommon-x11-0` and `libxcb-xkb1`. When not installed, they were
  borrowed from an `opencv-python` wheel: `LD_LIBRARY_PATH=/tmp/xkbstub:<venv>/opencv_python.libs`
  with `/tmp/xkbstub` holding the two libs symlinked from the wheel's hashed copies. Installing the
  packages properly removes the need for this.
- `perf`/`valgrind`/`heaptrack` are unavailable in this environment - use the allocation sampler
  (`--alloc-profile`) or in-code phase timers (COMMANDS.md) instead.
- Build profile used throughout: `rdebug` (release + `debug = "full"`, no debug assertions) -
  `cargo build --profile rdebug --bin perf_test` inside `perf_test/`.
- femtovg dependency: `../femtovg` (a local checkout, also present in this repo's working
  directory) when a candidate touches the renderer rather than the core.

## Reproducing a candidate standalone: `harness/perf_test/`

`perf_test/` has never been tracked in this repo's git history, so a fresh `git clone` of this
repo (exactly what every candidate's own `justfile` does via `just clone`) never contains it. Since
almost every candidate benches through `perf_test` (see "Bench harnesses: grow a few, don't
multiply" in `~/.claude/skills/perf-optimize/SKILL.md`), one canonical, source-only copy is kept at
`harness/perf_test/` in this folder (stripped of `target/`, the git-ignored `assets/` - regenerate
with `cargo run --bin make_assets` - and its own nested `___OPTIMIZATIONS___/`, which is a
different, older workspace, not harness code). A candidate's `justfile` copies this in during its
`clone` recipe rather than assuming the reader already has a checkout with `perf_test/` present -
this relaxes "postable as a single folder with zero other setup" to "self-contained given the
whole `___OPTIMIZATIONS___` folder kept together", which is the deliberate, approved tradeoff for
this project specifically.

`perf_test` also needs a `femtovg` checkout at `clone/femtovg` to build **at all**, even for
candidates that never touch the renderer: its `Cargo.toml` unconditionally patches
`crates-io`'s `femtovg` to `../femtovg`, and Cargo validates that regardless of which features are
active. Unlike `perf_test` itself, femtovg is small and public
(`https://github.com/femtovg/femtovg`), so a candidate's justfile clones it fresh at a pinned
commit the same way it clones this repo, rather than keeping a duplicate copy here - see any
femtovg-touching candidate's justfile (006, 011, 057, ...) for the exact commit currently pinned.

As of this writing, most of this workspace's existing candidate justfiles were found to *not*
actually do this (some hard-failed trying to `cd` into a `perf_test/` that was never copied in,
others degraded `bench:` into printed instructions instead of a real run) - caught only because a
user ran `just test` directly and hit the failure. Treat any candidate justfile as unverified until
`just test` has actually been run for it, not just read.

## Ground rules used in this workspace

1. No claim without a command that reproduces it - every candidate folder has the numbers and the
   exact command.
2. Every candidate that touches real source gets a real `patch.diff`, applied and built for real,
   plus the project's own tests where they cover the code. (Two ported candidates - 035 and 036 -
   were recorded in the original session as "applied" but their patches were not saved at the
   time; a fresh patch was independently re-derived from each `RESULT.md`'s own description,
   compiled and unit-tested against the pinned base commit this session - see each folder's
   `RESULT.md` for the "freshly authored, not the original bytes" caveat.)
3. Null results are written down with the same care as wins - a null result with no file is
   indistinguishable from "never checked".
4. If a later candidate changes the baseline enough that an earlier candidate's numbers no longer
   hold, don't silently rewrite that candidate's `RESULT.md` - add a `CORRECTION.md` or
   `RECHECK_AFTER_<N>.md` in its folder (several already exist, ported as-is: candidates 008, 015, 024,
   025, 037).

## Instrumentation added for this workspace

Ported from `perf_test/___OPTIMIZATIONS___/00_tooling/RESULT.md` (originally its own 'candidate 000' folder - folded in here instead, since it is not an optimization, it is the measurement equipment everything else depends on).

Not an optimization; the measurement equipment everything else depends on. All of it lives
in `perf_test/src`, so it stays runnable.

## What was added

| Tool | File | Why |
| --- | --- | --- |
| Headless benchmark (`--bench`) | `src/bench.rs` | Renders N frames with a `MinimalSoftwareWindow` platform, no display. Deterministic, works over ssh. |
| Windowed benchmark (`--frames N`) | `src/frames.rs` | Same table with the real backend (FemtoVG/Skia) via a rendering notifier; the only way to measure the GPU renderers. |
| Allocation counter | `src/alloc_stats.rs` | Allocations per frame + RSS. Feature `alloc-stats`. |
| Sampling allocation profiler | `src/alloc_profile.rs` | `--alloc-profile <n>`: 1-in-n allocations of the last frame, unresolved backtraces into a fixed static table, symbols resolved in the report. |
| Text shape counters | `src/bench.rs` | `TextLayoutCache::cache_miss_count()` / `layout_miss_count()` through the software renderer's `testing` feature. |
| Allocator cost | `tools/allocbench.rs` | 34 ns per malloc+free pair (32 B, glibc), to convert allocation counts into time. |
| Focus-traversal bench (`focus_bench`) | `055.MOVE_FOCUS_CYCLE_DETECTION/bench/` | Standalone crate, not a `perf_test` mode: `perf_test`'s own chrome (`main.slint`) always has `Button`s that grab keyboard focus within ~8 steps regardless of scene size, which hides any cost that only shows up when the whole tree has to be walked with nothing focusable. A `Window` with N plain `Rectangle`s and nothing else, `focus_bench <count> <iters> [--check]`, times repeated Tab presses that always fail to find anything focusable (the `--check` mode adds one `FocusScope` after the rectangles to verify Tab still finds the right item). |
| `--frames --snapshot` fix (Skia) | `src/frames.rs` | Was panicking ("RefCell already mutably borrowed") for Skia specifically: `take_snapshot()` was called from inside the rendering-notifier's own `AfterRendering` callback, re-entering `SkiaRenderer::render()` while its `rendering_notifier` `RefCell` was still borrowed for that callback. Fixed by deferring the snapshot/report/quit to a `Timer` on the next event-loop tick, and by no longer requesting extra redraws once the counted frames are in (that race made a real-backend snapshot flip between two valid-but-different captured frames depending on timing - confirmed by the same two hashes appearing for a fully unpatched binary run repeatedly, unrelated to any candidate). See `056.SKIA_SHARED_IMAGE_TEXTURE/RESULT.md`. |
| `--image-distinct`/`--image-size`/etc. wiring (windowed path) | `src/main.rs` | These flags were parsed into `Options` but had **no effect** under `--frames` - `PerfMode::Images` there hardcoded `make_image(0, 64)` and never called `set_image_distinct`/`set_image_sources`/etc; only the headless `--bench` path (`bench.rs`) wired them up. Fixed by mirroring `bench.rs`'s fill logic in `main.rs`. Found while testing `056.SKIA_SHARED_IMAGE_TEXTURE` (its correctness check for `--image-distinct` was silently checking the same "shared" scene twice against itself before this fix). |
| Skia paired-comparison scripts | `tools/paired_skia_first_frame.sh`, `tools/paired_skia_later_frames.sh` | `tools/paired.sh` invokes `--bench`, which is software-renderer-only. Skia needs a real GL/GPU context, so these variants alternate `--frames` runs under `xvfb-run`/`LIBGL_ALWAYS_SOFTWARE=1` instead, parsing the "first frame" / "later frames" lines respectively - the two can disagree (see `056.SKIA_SHARED_IMAGE_TEXTURE/RESULT.md` for a case where the win was much larger on "later frames" than "first frame"). |
| FemtoVG paired-comparison scripts | `tools/paired_femtovg_first_frame.sh`, `tools/paired_femtovg_later_frames.sh` | Adapted from the Skia scripts above (`s/skia/femtovg/`) for the same reason: femtovg also has no headless (`--bench`) path for GPU-resource-cache questions, so these alternate `--frames --renderer femtovg` runs under `xvfb-run` instead. Added while testing `057.FEMTOVG_TILED_9SLICE_TEXTURE_PER_FRAME`. |
| `--image-tiling` flag + `ImageModePage.tiling` | `src/backend.rs`, `bench.rs`, `main.rs`, `ui/main.slint`, `ui/image-mode.slint` | The `images` scene had no way to reach femtovg's `Repeat`/`Round` tiling with a source crop - plain tiling over the whole source never takes the expensive clipped-texture branch (the clip rect already equals the full source). Sets `horizontal-tiling`/`vertical-tiling: repeat` and a `source-clip-*` crop of `(8, 8, 40, 32)` px (source images are 64px by default) on every image in the grid when passed. Added while testing `057.FEMTOVG_TILED_9SLICE_TEXTURE_PER_FRAME`. |
| `--image-nine-slice` flag | `src/backend.rs`, `bench.rs`, `main.rs` | The `images` scene had no way to reach `ImageInner::NineSlice` at all (no `.slint` scene used a `@image-url(..., nine-slice(...))` literal). Rather than add a compile-time nine-slice literal to `ui/image-mode.slint`, this loads the existing `assets/nine_slice.png` (96x96, already generated by `src/bin/make_assets.rs` in an earlier session but never wired up) and calls the public runtime API `slint::Image::set_nine_slice_edges(24, 24, 24, 24)` on it before `set_image_source` - no `.slint` changes needed. Note: it does not set `item.tiling()`, so `fit.tiled` stays `None` for these fits; it exercises `fit9slice`'s iteration/allocation shape but not candidate 057's tiled-texture-cache branch (see `IDEAS.md`). Added while testing `060.FEMTOVG_9SLICE_FITS_VEC`. |
| `tools/paired_{femtovg,skia}_{first,later}_frame(s).sh` usage doc fix | `tools/paired_femtovg_*.sh`, `tools/paired_skia_*.sh` | All 4 scripts' own header comment showed `<a_bin> <b_bin> -- <args>`, but the `--` must **not** actually be passed - `run()` only `shift`s the binary path off its own arguments, so a literal `--` ends up forwarded to `perf_test` itself, which rejects it as an unknown argument, printing no "later/first frame" line and producing an empty parse (`ValueError: could not convert string to float: ''`) with no other error message. Fixed the usage line in all 4 scripts to drop the `--`; the scripts' own argument handling was unchanged (only the misleading doc comment). Found while testing `060.FEMTOVG_9SLICE_FITS_VEC`. |
| `--shadow` flag | `src/backend.rs`, `bench.rs`, `main.rs`, `ui/main.slint`, `ui/rect-mode.slint` | Nothing in `perf_test` drew a box shadow before this - needed to reach femtovg's `draw_box_shadow`. Adds `drop-shadow-blur: 6px` / `drop-shadow-offset-x/y: 2px` / `drop-shadow-color: #000000a0` to every rectangle in the `rects` scene when passed (`item-shadow` on `RectModePage`, `rect-shadow` on `MainWindow`), wired through both the headless (`bench.rs`) and windowed (`main.rs`) fill paths. Added while testing `061.FEMTOVG_BOX_SHADOW_BLIT_PATH` (femtovg-only code path - `--bench`'s software renderer doesn't reach `draw_box_shadow` at all, so this flag is only useful under `--frames --renderer femtovg`). |
| Headless Skia benchmark (`--skia-bench`) | `src/skia_headless.rs`, wired into `Cargo.toml` (`i-slint-renderer-skia`/`bytemuck` made real deps of the `skia` feature, previously only `slint/renderer-skia` was pulled in), `backend.rs`, `main.rs` | `--frames --renderer skia` cannot reach Skia's dirty-region clip code at all: `frames.rs` times frames via `Window::set_rendering_notifier`, and `i_slint_renderer_skia::SkiaRenderer::partial_rendering_state()` unconditionally returns `None` whenever a rendering notifier is registered ("We don't know where the application might render to, so disable partial rendering") - regardless of `SLINT_SKIA_PARTIAL_RENDERING`. Confirmed empirically: a temporary `eprintln!` in the dirty-region-clip code never fired under any `--frames --renderer skia` invocation tried. `--skia-bench` drives `SkiaRenderer::new_with_surface` + `software_surface::SoftwareSurface` directly (the same pattern `api/rs/slint/tests/partial_renderer.rs` uses for its own tests), reusing a persistent `SharedPixelBuffer` across frames so `back_buffer_age` is 1 from the second frame on. No notifier is registered, so partial rendering stays on and needs no env var; timed with a plain `Instant` around each `renderer.render()` call instead. Only wires up the `scroll`/`list`/`rects` scenes (whatever a future candidate needs beyond that should extend this file, not add a new one). Found/added while testing `066.SKIA_SINGLE_RECT_CLIP`. |
| `MainWindow.probe-count`/`probe-opacity`/`probe-animate` | `ui/main.slint`, `backend.rs`, `bench.rs`, `skia_headless.rs` | A `for i in root.probe-count: Rectangle { ... }` with no children/`visible:` binding/drop-shadow sibling, absolutely positioned outside any layout - lets `Opacity::need_layer` take the direct `apply_opacity` path (as opposed to every other opacity-capable element in this harness, which always triggers the offscreen-layer path for unrelated structural reasons). `probe-animate` drives `background` from `animation-tick()`, mirroring the GitHub #6056 Spinner pattern. Added while testing `068.SKIA_DRAW_ALPHA_GATE`. |
| `MainWindow.probe-layer-count`/`probe-layer-opacity`/`probe-layer-animate` | `ui/main.slint`, `backend.rs`, `bench.rs`, `skia_headless.rs` | Same idea as `probe-count`, but each item has one statically-declared nested child `Rectangle`, so `Opacity::need_layer` returns `true` (confirmed with a temporary `std::eprintln!` in `need_layer` before removing it) - reaches Skia's `save_layer_alpha`/`render_and_blend_layer` path, which candidate 068 found makes its own per-draw-method alpha gate a no-op. Placed at `x: 700px..` so it doesn't overlap `probe-count`'s grid. Added while testing `086.OPACITY_ZERO_SKIP_RENDER_ALL_RENDERERS`. |
| Scheduling counter (`alloc_stats::scheduling_stats()`) | `src/alloc_stats.rs`, `bench.rs` | A "was this process rescheduled/migrated" signal for diagnosing a core-count-dependent result `perf` cannot explain in this environment: voluntary/involuntary context switches and minor/major page faults for the whole run, read once from `/proc/self/status`/`/proc/self/stat` right after the last frame (like `resident_bytes()`, not per-frame, so it does not perturb the timing it explains). Printed as a `scheduling` line under `--bench`'s existing `resident memory` line. Added while testing `087.FLAT_SCROLL_CHUNK_SIZE_CORE_COUNT_PROFILE`, which found it correlates strongly with the bimodal frame-time "lottery" this workspace's README already described (single-digit involuntary switches -> the fast mode, hundreds -> the slow mode) independent of the `CHUNK_SIZE` value under test - see that candidate's `RESULT.md`. |
| `tools/pixelcheck.sh` gained a `FRAMES` env var (default 6, unchanged) | `tools/pixelcheck.sh` | Candidates 093/095 needed to sweep past `--frames 6` by hand (093's scrollbar-thumb regression was pixel-identical at 6 frames and only appeared past ~80) because the script hardcoded `--frames 6`; this makes that a one-line `FRAMES=150 ...` instead of a hand-rolled snapshot/`cmp -l` loop each time. Added while testing `096.RETAINED_LINE_BREAKING_FIX_HIGHER_RISK_SCENARIOS`. |
| `tools/pixeldiff.py` | `tools/pixeldiff.py` | Companion to `pixelcheck.sh` for when its bounding-box-based "inside/outside the noise region" verdict reports "inconclusive" on a *scattered* noise floor (e.g. a status label in one corner and a scrollbar row elsewhere) even though the real difference is cleanly separable at the per-pixel level - subtracts noise as an exact pixel-position set instead of a bbox and prints the affected rows/columns. Kept as a separate script rather than folded into `pixelcheck.sh` since the two report different things (a pass/fail verdict vs. a diagnostic breakdown) and existing callers of `pixelcheck.sh`'s output format shouldn't change. Added while testing `096.RETAINED_LINE_BREAKING_FIX_HIGHER_RISK_SCENARIOS`. |
| `--bench` blanks the `status` label after fill | `src/bench.rs` | The `text` scene's `on_add_lines` status message embeds real wall-clock generation timing (`"generated in {build:.2?}, property set in {apply:.2?}"`), which necessarily differs every process run - already a known, accepted noise source per `pixelcheck.sh`'s own header comment, but undersampled at low `RUNS` (its true footprint, ~3700px in rows 839-851 at this harness's default window size, only shows up once ~15-20 repeats are sampled, an order of magnitude more than the `RUNS=4-6` most candidates use). `app.set_status(Default::default())` right after the fill phase removes the whole source: 0 differing px across 15-20 repeats where there used to be up to 3700, confirmed both on pristine `master` and on 091+092. Found while root-causing what turned out to be a sampling-variance artifact, not a real `--select-all`-specific effect - see `097.SELECT_ALL_NOISE_FLOOR_ROOT_CAUSE/RESULT.md`. |
| `--image-unique` flag + `generator::make_unique_gallery_images` | `src/backend.rs`, `bench.rs`, `main.rs`, `generator.rs` | The existing `--image-distinct` sources (`generator::make_image`, in-memory `SharedPixelBuffer`s) get `ImageCacheKey::Invalid` ("no image cache key can be created for ... programmatically created images") and so never populate a renderer's per-source GPU texture cache at all - unsuitable for measuring anything about that cache (e.g. `TextureCache::drain()`). `--image-unique` instead writes `count` genuinely distinct, tiny PNGs to `assets/gallery/` (skipped if already present - deterministic content, cached across runs) and loads each via `Image::load_from_path`, giving every one a real, persistent `ImageCacheKey::Path`; `sources.length == count` so `image-mode.slint`'s existing `mod(index, sources.length)` selection becomes the identity (one distinct source per grid cell). Also found while building this: with the scene's default `--image-cell 68px`, off-screen items in a large non-scrolling grid never reach `draw_image_impl` at all (`TextureCache` stayed at a constant ~216 entries from `--count 300` up to `--count 10000`) - reaching a genuinely large cache needs a small `--image-cell`/`--image-size` so thousands of distinct images are simultaneously *visible*, not just present in the item tree. Added while testing `107.TEXTURE_CACHE_DRAIN_FULL_WALK`. |
| `--drag-probe` flag + `MainWindow.drag-probe-*`/`DragProbe` global | `src/backend.rs`, `main.rs`, `ui/main.slint`, `ui/shared.slint` | Nothing in `perf_test` reached `ItemRenderer::draw_image_direct` at all (it's only used by `DragArea.drag-image`'s cursor overlay, not the `images` scene's `Image {}` grid, which goes through `draw_image_impl` instead). Adds an always-present `DragArea` to `main.slint` (gated by `enabled:`, not `if` - a conditional element's id isn't reachable from a sibling binding outside its own conditional scope, hit as a compile error first) plus a `DragProbe` global providing a non-empty `data-transfer` via a Rust callback (a `data-transfer` literal doesn't exist in `.slint` markup, and `DragArea` refuses to start a drag at all when `data.is_empty()`). `--drag-probe` sets an in-memory `Image::from_rgba8` source, then (bypassing `app.run()`) calls `show()`, dispatches a `PointerPressed` + one `PointerMoved` past the 8px drag threshold to start an in-window drag (winit's `start_drag` is unimplemented, always falls back to the in-window path - safe under Xvfb), and never releases it before running the event loop, so the drag - and `draw_image_direct` - stays live for the whole `--frames` run. Windowed (`--frames`) only; `--bench`'s headless software renderer has no rendering notifier to start the drag against. Added while testing `108.DRAG_IMAGE_DIRECT_ONE_ENTRY_CACHE`. |
| `--mutate-mode reset` | `src/bench.rs`, `backend.rs` | The existing `--mutate-mode`s all fire per-row model notifications (`row_changed`/`row_added`/`row_removed`); nothing reached `Model::notify.reset()` (`VecModel::set_vec`/`clear`, or a `SortModel`/`FilterModel` re-sort/re-filter forwarding their own reset through it), which is a materially different code path in `Repeater` (`RepeaterTracker::reset`, `internal/core/model/repeater.rs`). `Reset` calls `records.set_vec(...)` with a same-length, entirely-new-content `Vec` each frame - same row count, different data, one `notify.reset()`. Added while testing `109.REPEATER_RESET_INSTANCE_REUSE`. |

| Qt-backend live-window repaint harness | `114.QT_ICON_CACHE_E2E_REPAINT_VERIFICATION/bench/` | Standalone crate (`qt-icon-e2e-probe`), not a `perf_test` mode - `perf_test`'s own `--features qt` only makes the whole app use the Qt backend/style, it has no icon-bearing native-widget scene or any way to drive Qt's own live `QApplication` event loop end to end. Compiles a `Button { icon: ... }` scene with `slint_build::CompilerConfiguration::with_style("qt")`, runs under the real `i-slint-backend-qt` backend (`QT_QPA_PLATFORM=offscreen`, `slint::run_event_loop()`), and forces repeated real repaints via a `slint::Timer` alternating pointer hover/press and keyboard-focus `Window::dispatch_event` calls. Built to end-to-end-verify candidate 113's icon cache through the real dirty-region/repaint pipeline rather than only a direct function-level probe - see `114.../RESULT.md`. Reusable for any future Qt-native-widget-under-real-repaint question. |

## Why a custom allocation profiler

`perf record` fails with `Operation not permitted` in this environment even at
`perf_event_paranoid = -1` (seccomp), and neither `valgrind` nor `heaptrack` is installed.
The sampler is allocation-free while sampling (fixed 4096-bucket static table, raw
instruction pointers, recursion guard), so it does not perturb what it measures beyond the
sampling itself.

Caveat found in use: symbols occasionally attribute to a neighbouring monomorphization
(identical code folding), so read the stack, not one frame.

## Does it change behavior/output?

N/A - measured/quantified only, no patch applied to real source in the original session. This
candidate is the harness itself (`perf_test/src/*.rs`, `tools/allocbench.rs`), not a change to
Slint.

## Reproduce

```sh
just bench                                   # software renderer table
just frames-xvfb 100000 8                    # FemtoVG under Xvfb
../target/rdebug/perf_test --bench --mode list --count 100000 --frames 4 --alloc-profile 20
just allocbench
```

Fully reproducing this needs `perf_test/` copied in manually, since it is untracked in the main
repo's git history (see `001.SCROLLVIEW_ROW_STRUCTURE/RESULT.md` for why).
