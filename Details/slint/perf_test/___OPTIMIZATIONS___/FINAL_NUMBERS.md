# End-of-session measurement: pristine tree vs everything applied

Two binaries built from the same `perf_test`, differing only in the Slint-side changes this
session applied (candidates 08, 09, 10 - see `SESSION.md` for the exact file list):

- **pristine** - `internal/core/properties.rs`, `internal/core/graphics/brush.rs`,
  `internal/renderers/femtovg/itemrenderer.rs` at git HEAD (dep-stats instrumentation compiled
  out as well).
- **session** - the working tree.

Medians of 3 runs, `rdebug`, 1280x900, scrolling 200 px per frame. Allocation counts are from a
separate 4-frame run (they are stable to +-2 per frame).

## FemtoVG (window under Xvfb with Mesa llvmpipe)

Two columns for "after": candidates **08-10** (core + gradient paths) and then **06+11** on top
of them (pooled paths with reused tessellation buffers).

| scene | pristine | 08-10 | +06+11 | allocations/frame pristine -> final |
| --- | --- | --- | --- | --- |
| `scroll` 100 000 (non-virtualized rows) | 56.19 ms | 53.31 ms | **50.73 ms (-9.7%)** | 703 200 -> **301 988 (-57%)** |
| `rects` 3 000 `--gradient` | 51.38 ms | 51.68 ms | 47.14 ms | 59 194 -> **36 146 (-38.9%)** |
| `rects` 3 000 (solid) | 45.98 ms | 46.77 ms | 46.63 ms | 51 423 -> **35 035 (-31.9%)** |
| `list` 100 000 (ListView) | 4.94 ms | 4.98 ms | 5.26 ms | 3 977 -> **2 064 (-48.1%)** |
| `text` 100 000 lines | - | - | 4.40 ms | - -> 1 395 |

Frame times here are llvmpipe-bound (candidate 07), so only the `scroll` column is above the
noise; the allocation column is the reliable one. On real hardware the same `rects` scene runs at
**5.99 ms with 50 359 allocations** (measured by the user before 06+11), where 15 000 fewer
allocations is ~200 us, i.e. **~3.4% of the frame** - `just gpu-report` produces the real table.

## Software renderer (`--bench`, headless, deterministic)

| scene | pristine | session | time | allocations/frame pristine -> session |
| --- | --- | --- | --- | --- |
| `scroll` 100 000 | 142.97 ms | **129.93 ms** | **-9.1%** | 701 088 -> **300 743 (-57%)** |
| `scroll` 100 000 `--flat-rows` | 114.11 ms | **105.13 ms** | **-7.9%** | 501 380 -> 301 036 (-40%) |
| `text` 100 000 lines | 3.69 ms | 3.62 ms | -2% | 441 -> 219 (-50%) |
| `list` 100 000 | 2.17 ms | 2.17 ms | - | 2 165 -> 1 394 (-36%) |
| `rects` 3 000 | 2.12 ms | 2.15 ms | - | 18 501 -> 18 414 |
| `rects` 3 000 `--gradient` | 2.98 ms | 2.98 ms | - | 18 501 -> 18 414 |

(The gradient fixes are FemtoVG-only, which is why the software renderer shows no difference
between the solid and gradient variants' allocation counts.)

### Candidate 03 landed after this table

The tables above are the state before candidate 03 (lazy per-paragraph shaping), so the `text` row
does not include it. Measured separately, against a binary built from the pristine
`internal/core/textlayout` on the tree that already has everything else - median of three runs of
`--bench --mode text --count 100000 --frames 8`:

| | pristine | session | |
| --- | --- | --- | --- |
| `show + layout` | 2.13 s | **1.77 s** | -17% |
| resident memory | 540.8 MB | **38.5 MB** | **-93%** |
| later frames | 3.89 ms | 4.29 ms | +10% (13 paragraphs shaped per frame instead of 0) |

Other scenes on the same pair of binaries: `list` 100k 2.51 -> 2.47 ms, `scroll` 100k
152.9 -> 140.6 ms, `rects` 3 000 2.18 -> 2.30 ms. Pixels identical on `text` (also 12 frames deep
into a 100 000-line document), `list` and `scroll`. Details in `03_text_document_shaping/RESULT.md`.
None of this is confirmed on the GPU yet - it is software-renderer only, so `just gpu-quick` is the
next thing to run.

## How to read this

- **Allocation counts are hardware independent** and reproduce exactly; they are the primary
  metric here.
- **Frame times under llvmpipe are rasterization-dominated.** Candidate 07 established that with
  a 3.3x faster allocator: the software `scroll` frame moved -12.1% while the FemtoVG frame did
  not move at all. So the sub-2% time deltas above (`rects`, `list`) are below this environment's
  resolution, and the two clear wins (`scroll`: -5.1% FemtoVG, -9.1% software) are the ones that
  rise above it. On a real GPU, where a `rects` frame is a few milliseconds instead of 46, the
  per-primitive allocation savings (candidates 06/09/10) should be a larger *relative* share -
  that measurement needs your hardware: `just frames-mode rects 3000 8`.
- The `scroll` scene is the pathological one on purpose (100 000 live rows). The realistic
  application advice is still candidate 01: use `ListView`, or keep layouts out of repeated rows.

## Real hardware (user's GPU, `just gpu-report`, medians of 3 x 20 frames)

| scene | pristine | session | delta | mimalloc | allocations/frame |
| --- | --- | --- | --- | --- | --- |
| `rects` 3 000 | 5.07 ms | **4.35 ms** | **-14.2%** | 4.42 ms | 50 745 -> 33 973 |
| `rects` 3 000 `--gradient` | 5.98 ms | **5.63 ms** | **-5.9%** | 5.94 ms | 52 951 -> 35 076 |
| `scroll` 100 000 | 54.71 ms | **48.79 ms** | **-10.8%** | 48.40 ms | 703 099 -> 301 930 |
| `scroll` 100 000 `--flat-rows` | 43.46 ms | **40.57 ms** | **-6.6%** | 38.55 ms | 503 402 -> 302 249 |
| `list` 100 000 | 1.18 ms | **1.15 ms** | -2.5% | 1.42 ms | 3 287 -> 2 015 |
| `text` 100 000 | 1.96 ms | 1.97 ms | +0.5% | 2.07 ms | 1 903 -> 1 282 |

This is the measurement llvmpipe could not give: with rasterization no longer dominating, the
allocation and bookkeeping savings show up as **-14.2% on a rectangle-heavy frame** and
**-10.8% on the non-virtualized list**, against +-2% noise under Xvfb.

### What the mimalloc column says about what is left

`session` vs `mimalloc` is the same code with a 3.3x faster allocator, so the gap is the
remaining allocator-side headroom:

| scene | session | mimalloc | headroom left |
| --- | --- | --- | --- |
| `rects` 3 000 | 4.35 ms | 4.42 ms | **none** - mimalloc is slower now; the allocator is no longer a bottleneck here |
| `rects --gradient` | 5.63 ms | 5.94 ms | **none** |
| `list` 100 000 | 1.15 ms | 1.42 ms | **none** (mimalloc's own overhead dominates a 1 ms frame) |
| `scroll` 100 000 | 48.79 ms | 48.40 ms | ~1% |
| `scroll --flat-rows` | 40.57 ms | 38.55 ms | ~5% |

**Conclusion for the next round:** further allocation removal (femtovg's `Vertex`/`Drawable`
buffers, `Rc<[GradientStop]>`, a `DependencyNode` arena) is now worth ~0 on rect- and
ListView-shaped scenes and at most a few percent on the pathological non-virtualized list. The
levers that remain are structural, not allocation-related:

1. the O(items) traversal of culled items (candidate 01's subtree pruning) - `scroll` is still
   48.79 ms because 100 000 rows are visited every frame,
2. `TextInput` shaping and retaining whole documents (candidate 03),
3. per-item property reads (8 per visible primitive, 4 per culled one) - fewer reads, not cheaper
   registrations.

## Visual validation of the whole session

FemtoVG, 5 frames of content moving 29 px/frame, comparing the pixels above the status line
(which prints per-run timings and therefore always differs):

| scene | pristine vs session |
| --- | --- |
| `rects --gradient` 2 000 | **0 bytes differ** |
| `list` 5 000 | **0 bytes differ** |
| `scroll` 5 000 | **0 bytes differ** |
| `text` 5 000 | **0 bytes differ** |

Control: two runs of the same binary also differ in 0 bytes, after `src/frames.rs` was fixed to
derive the scroll offset from the frame count instead of the timer tick count (see PIPELINE.md).

Test suites on the final tree: `i-slint-core` 361, `slint` 77, `tests/` interpreter driver 818,
Rust driver 745, `perf_test` 5 - **2 006 tests, 0 failures**. `cargo clippy` clean for
`i-slint-core`, `i-slint-renderer-femtovg` and `perf_test`.

## Reproduce

```sh
cd perf_test/___OPTIMIZATIONS___ && just final-numbers
```
