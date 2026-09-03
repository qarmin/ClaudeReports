# Autonomous session log - state and how to resume

Written continuously so the work survives a shutdown. Read `SUMMARY.md` for the candidate
index, `PIPELINE.md` for the harness.

## What is applied to the working tree right now

| Change | Files | Status |
| --- | --- | --- |
| Candidate 03: lazy per-paragraph shaping + glyph retention window | `internal/core/textlayout/sharedparley{,/shaping,/layout,/cache,/draw,/selection,/tests}.rs` | **applied**, validated (2025 tests, pixels identical) |
| Candidate 03 tooling: `paragraph_shape_count`, headless `--bench --snapshot`, `--word-wrap`, `--resize-step` | same + `perf_test/src/{bench,backend}.rs`, `perf_test/ui/{main,text-mode}.slint` | **applied** |
| Candidate 14: pool dependency nodes instead of freeing them each evaluation | `internal/core/properties.rs` (thread-local `NodePool` + `recycle_into`), `properties/change_tracker.rs` | **applied**, validated (tests + pixels, `--features ffi` and `no_std` checked) |
| Candidate 08: skip duplicate dependency registrations | `internal/core/properties.rs` (`front_is` + the skip) | **applied**, validated (2000+ tests) |
| Candidate 08 instrumentation: `dep-stats` feature (counters) | `internal/core/properties.rs`, `internal/core/Cargo.toml` | **applied**, off by default, zero cost when off |
| Candidate 09: gradient stops without a `Vec` | `internal/renderers/femtovg/itemrenderer.rs`, `internal/core/graphics/brush.rs` (`stops()` returns `+ Clone`) | **applied**, validated |
| Candidate 10: gradient bounds without re-tessellating | `internal/renderers/femtovg/itemrenderer.rs` | **applied**, validated |
| Candidate 06: pooled scratch paths in the renderer | `internal/renderers/femtovg/itemrenderer.rs` | **applied**, validated |
| Candidate 11: `Path::clear()` + `PathCache::rebuild()` (tessellation buffers survive) | `femtovg/src/path.rs`, `femtovg/src/path/cache.rs` | **applied**, femtovg's 61 tests pass |
| Local femtovg wired into every workspace | `Cargo.toml`, `examples/`, `demos/`, `tests/` `Cargo.toml` (+ lockfiles) | **applied** - required by candidate 06; revert with `git checkout` those files |
| Root workspace `exclude` for `femtovg`/`perf_test` | `Cargo.toml` | applied (needed so both directories can be their own workspaces) |

Not applied, kept as a patch: candidate 01's `clip_rows.diff` (an application-level illustration).

**Candidate 03 is confirmed on real hardware** (`GPU_03.md`, five runs): `text` 100 000 `no-wrap`
**638.6 -> 122.6 MB** on FemtoVG, wrapped text untouched as designed and measured shaping **0**
paragraphs per frame, `no-wrap` shaping **11** - exactly the lines entering the viewport. Two things the GPU run found that headless could not:
an **O(document) scan running twice per frame** in the new code (the retention window and the
width-change check), which cost ~1 ms/frame at 100 000 paragraphs - four times the lazy shaping
itself - now bounded by a `shaped` range and confirmed fixed on hardware (two document sizes doing
the same work now cost +1.06 and +0.92 ms, against 4.4x apart before); and that **single outlying
frames were being averaged into every headline** (one 60 ms frame turned a 2.42 ms row into
8.80 ms), so both reports now print medians with the spread, after which every allocation column
reproduces exactly between repeats. The `rects` allocation delta that looked like a regression is **+7, not
-3 994**. Numbers here do not survive across runs - `PIPELINE.md` now says so.

**The one command to run on real hardware:** `cd perf_test/___OPTIMIZATIONS___ && just gpu-report`
- builds pristine / session / mimalloc binaries, runs every scene on the real display, and writes
`GPU_RESULTS.md`. `FRAMES`, `RUNS` and `SCENE_FILTER` are env overrides.

To see everything this session changed:

```sh
git -C <repo> diff internal/core internal/renderers
git -C <repo>/femtovg diff
```

## How to re-run everything

```sh
cd perf_test && just bench-all                 # the scene table, software renderer
cd perf_test/___OPTIMIZATIONS___ && just run   # every candidate that has headless numbers
just run-xvfb                                   # the FemtoVG ones through Xvfb
```

FemtoVG needs `libxkbcommon-x11-0` and `libxcb-xkb1` installed (see PIPELINE.md for the
wheel-borrowing workaround used here).

## Confirmed on real hardware

`GPU_RESULTS.md` (and the table in `FINAL_NUMBERS.md`): **-14.2%** on `rects` 3 000, **-10.8%** on
`scroll` 100 000, -6.6% on `scroll --flat-rows`, -5.9% on the gradient scene, -2.5% on `list`,
`text` neutral. The mimalloc control in the same run shows the allocator headroom is now
**exhausted** for rect/ListView scenes (mimalloc is *slower* than the current tree there), so the
next round has to be structural: subtree culling (01), text document shaping (03), fewer property
reads per item.

## Measured wins so far, in one place

| Change | Scene | Effect |
| --- | --- | --- |
| 08 duplicate registrations | `scroll` 100k, FemtoVG | 57.5 -> 53.0 ms (-7.8%), allocations -57% |
| 08 | `scroll` 100k, software | 148.2 -> 142.9 ms (-3.5%), allocations -57% |
| 09 + 10 gradients | `rects --gradient` 3 000, FemtoVG | 58 002 -> 51 304 allocations/frame (-11.5%), 52.1 -> 50.7 ms |
| 06+11 pooled paths and tessellation buffers | `rects` 3 000, FemtoVG | allocations 50 193 -> 35 035 (-30%), `list` -35% |
| 01 row structure (application level) | `scroll` 100k | first frame 3.10 s -> 0.13 s, RSS 1542 -> 318 MB |
| 03 lazy paragraph shaping | `text` 100k `no-wrap`, software | RSS 540.5 -> 38.6 MB (-93%), `show + layout` 2.01 -> 1.73 s, resize 76 -> 5.6 ms, +0.2 ms/scrolling frame |
| 03 lazy paragraph shaping | `text` 100k `no-wrap`, FemtoVG | RSS 646.4 -> 130.5 MB (-80%), startup -0.25 s |
| 14 dependency node pooling | `scroll` 100k, software | 129.9 -> 119.5 ms (-8.0%), allocations 300 820 -> **172** |
| 14 dependency node pooling | `scroll` 100k `bare`, software | 26.2 -> 20.0 ms (-23.7%), allocations 200 313 -> **28** |

## Final validation state (end of session)

- Candidate 03 revalidated separately: **2 025 tests, 0 failures** (`i-slint-core --all-features`,
  `slint`, the two build-time drivers), pixels identical on ten scenes including `--word-wrap` and
  `--resize-step`.
- Before candidate 03: 2 006 tests pass, 0 failures: `i-slint-core` 361, `slint` 77, interpreter driver 818, Rust
  driver 745, perf_test 5. `cargo clippy` clean on the crates touched.
- All four scenes render **pixel-identically** to the pristine tree on FemtoVG with moving
  content (`FINAL_NUMBERS.md`).
- `FINAL_NUMBERS.md` holds the end-to-end before/after table for both renderers, reproducible
  with `just final-numbers`.
- Harness fix worth knowing about: the windowed benchmark used to scroll by timer ticks, which
  made snapshots from different builds incomparable. It now scrolls by frame count.

## Where the remaining allocations are (rects 3 000, after everything applied)

35 035 per frame, ~1 100 visible rectangles of 3 000:

| roughly | what | note |
| --- | --- | --- |
| ~17 000 | tracked property reads registering dependencies | 8 per visible rect (geometry + brushes) and 4 per culled rect; candidate 08 already removed the duplicates, the rest are genuine reads |
| ~4 400 | femtovg `Vertex` / `Drawable` per-draw-call buffers | `Command::drawables` is a fresh `Vec` per command; the canvas vertex buffer regrows per frame. Same pooling idea as candidate 11, inside femtovg's renderer |
| ~2 200 | `GradientColors::from_stops` -> `Rc<[GradientStop]>` (gradient scenes only) | an inline small-stop representation would remove it |
| rest | text shaping for newly appeared rows, image/texture bookkeeping, model updates | |

## Reference baseline on the user's GPU (window pinned, deterministic)

`rects` 3 000 5.49 ms / 35 164 allocations, `rects --gradient` 7.44 ms / 36 267,
`list` 100 000 1.46 ms / 2 039, `scroll` 100 000 50.38 ms / 301 961,
`scroll --row-style bare` 10.70 ms / 201 014. All at 1280x860 physical, scale factor 1
(`GPU_QUICK.md`). The `scroll` -> `bare` ratio of **4.7x** is the remaining structural win.

## Measurement trap found in the second GPU run (fixed)

The windowed benchmark let the window manager pick the window size, so two runs of the same
command could differ by 2x in visible rows and therefore in work. It now pins the window to
`--size` (default 1280x860 physical) and prints the size and scale factor it measured with.
Any table row where the allocation counts differ between two binaries is not comparable - check
that column first.

## The one thing to run when the machine is free

`cd perf_test/___OPTIMIZATIONS___ && just gpu-quick` - **~5 seconds** of measuring (plus a
no-op cargo build): the current tree on 5 scenes plus the size of the remaining structural win,
appended to `GPU_QUICK.md`. `just gpu-report` is the thorough one that rebuilds a pristine and a
mimalloc binary; that is the one that took an hour, so only run it when a full before/after is
actually needed. See `GPU_BATCH.md`. It now also measures
the four row variants from candidate 12, so one run answers both "what did the session buy" and
"how big is the remaining structural win".

## Autonomous block: probing areas no candidate had touched

Everything before this point came out of scenes that are **static** - content is filled once and
then scrolled. Two things that real UIs do constantly had never been measured, and both produced a
finding:

- **Live model data** (`--mutate`): candidate 13. Changing an off-screen row in a non-virtualized
  list re-shapes its text, 3 shapes per row, whether or not it is anywhere near the viewport.
  `ListView` is unaffected, and explicitly positioned row children make it exactly zero.
- **Per-item animation** (`--animate`): a binding on the animation clock costs 6 allocations per
  item per frame - which is what sent the allocation profiler at the property engine and produced
  candidate 14, the largest single win of the whole session.

- **Input** (`--pointer-move`): candidate 15. One mouse move over a 100 000-row list costs
  **+38 ms** - hit-testing visits every item in the tree, ~98 ns each, because an item whose
  geometry misses the cursor still forwards the event to all its children. `ListView` +0.01 ms.
- **Cursor movement** (`--key-input`): not a new candidate but the interaction candidate 03 was
  never tested with. A PageDown costs 47 shapes - exactly one screenful - whether the document is
  1 000 or 100 000 lines. Written up under 03.
- **Images** (`--mode images`): candidate 16, a null result. Item count, source sharing and source
  resolution are all flat; 11 allocations/frame for the whole scene.

The lesson generalises past Slint: the scene list was measuring one axis (how much content) and
none of the others (how much of it changes, how it is animated, what the user is doing to it).
Every new axis was one flag on an existing scene, not a new scene - and three of the four found
something.

Second round of the same approach, four more axes:

- **Global property invalidation** (`--toggle-global`, a theme switch): candidate 17, **null** -
  bindings are lazy, so dirtying 100 000 of them costs a flag walk and ~25 re-evaluations. The
  property system is not where the O(items) problem lives.
- **Nesting depth** (`--mode nest --nest-depth`): candidate 18. A level costs exactly what an item
  costs (**~86 ns/frame**) and a layout level costs no more than a plain one. With `--features
  dep-stats`: 8 000 extra transparent wrapper items = exactly **8 000 extra dependency
  registrations**, one per item.
- **Images** (`--mode images`): candidate 16, null.
- **Input and cursor movement**: candidates 15 and the 03 addendum, above.

Third round, interaction-shaped:

- **Selection** (`--select-all`): candidate 19, **fixed**. Resolving a selection scanned every
  paragraph (~2 ns each); 400 000 lines cost +1.09 ms/frame in skipped iterations. The draw loop
  already binary-searched the visible band, selection did not. Now +0.43 ms and flat,
  pixel-identical.
- **Typing** (`--type-text`): candidate 20, and the most user-visible number in this workspace.
  **882 ms per keystroke** at 50 000 lines - one shape per paragraph, because a text change drops
  the whole cache entry. Pre-existing and slightly *worse* before candidate 03. That candidate's
  split into `{range, y, metrics, glyphs}` is what makes an incremental path tractable; the blocker
  is now that the entry is destroyed before it can be diffed against the new text.

**Typing is the ceiling on candidate 03.** Loading an 11 MB document is cheap now (38 MB, 1.7 s);
*editing* it is not, and that is what a `TextEdit` is for. Anyone reading the memory numbers needs
that caveat next to them.

**~90 ns per item visit is the engine's unit of currency.** It shows up as ~98 ns in hit-testing
(15) and ~86 ns in nesting (18), and it is one tracked property read plus the traversal machinery -
not rendering (the nest wrappers draw nothing) and not layout (settled and cached). Every
O(total items) pass is spending that.

**The same gap, on three different passes.** Candidates 12 (rendering), 13 (layout) and 15 (input)
are all the same missing thing: no notion of what a subtree covers, so all three traversals are
O(total items) where they should be O(visible). A cached subtree bounding box next to the existing
per-item rendering cache would answer all three at once, which makes it the single highest-value
change left in the engine.

Also added: `tools/matrix.sh`, which runs a set of configurations N times each and reports medians
with the spread - the shape almost every question here wants, and the thing that stops a single
contaminated run from becoming a finding.

## Fourth round: a crash, and the aggregate shape of the text cache

- **A long text line aborts the process** (candidate 23). Glyphs were never culled *horizontally*,
  so a `no-wrap` line wider than the viewport hands the software renderer glyphs whose device
  coordinates exceed `i16::MAX`, and `euclid`'s `cast()` unwraps. ~5 000 characters on one line at
  13 px. Pre-existing. Fixed by a per-glyph cull - culling whole *runs* is not enough, because a
  no-wrap line of uniform style is a single run. Also **-35%** frame time on wide lines.
- **The shaped-text cache never evicts** (candidate 26). **4.68 KB per `Text` item**, exactly
  linear from 6 000 to 60 000 items, released only when the component dies. Candidate 03's
  128-paragraph gate is correct for one item and wrong in aggregate.
- **Font size variety** (24): +57% per frame at 128 distinct sizes, plateauing - the cost is per
  distinct size *visible at once*.
- **Styling costs** (25): opacity +18% with no offscreen layer, a corner radius +15%, a border ~+4%.
  One reading in that candidate ("cost grows with radius") did not reproduce and is written up as
  not answered rather than quietly dropped.

## Fifth round: model adapters, and the per-item cost split

- **`SortModel` makes appending O(model)** (candidate 29, **fixed**). +6.82 ms/frame appending 200
  rows to a sorted 200 000-row model, where a bare `push` is flat. Half of it was an
  index-adjustment scan in `row_added` that provably cannot do anything when rows are appended;
  skipping it is **-59%**. The rest is `Vec::insert` per row and needs a batched `row_added`.
- **The per-item cost splits ~1:3** (candidate 28): visiting an item ~124 ns, drawing one ~358 ns.
  So `clip: true` (skip the draw, still visit) already recovers the larger share per item, and
  candidate 12's subtree bounds are worth having *on top of* it, not instead. The absolute numbers
  move with machine load - the ratio (2.9-4.4x across three runs) is what survives, and is reported
  as a range for that reason.
- **`TouchArea` per row** (27): no special penalty, but it doubles the per-mouse-move hit-test cost
  because it makes candidate 15's per-item body heavier.

## Open threads, in the order I would pick them up

0. **Subtree pruning of culled items (candidate 12)** - the measured prize is 5.2x on the
   non-virtualized list and it is the only remaining change of that size. The design constraint is
   now known: the pruning must cost nothing per item (a compiler flag saying "children stay inside
   the root's geometry", or subtree bounds cached next to the existing per-item rendering cache),
   because doing it with a `Clip` item doubles the allocations and spends half the win.
1. **Windowed FemtoVG numbers on real hardware.** Everything here is llvmpipe, so frame *times*
   are rasterization-dominated; allocation counts are hardware independent but the time wins
   (08: -7.8%, 10: -3.7%) should be re-measured on a GPU. `just frames-mode rects 3000 8`.
2. **Candidate 03's remainder**, two parts. (a) `preferred-width` is still O(document) in *time* -
   it is the widest line, so every paragraph is shaped once (1.71 s for 11 MB); removing that means
   making it progressive, which changes what `preferred-width` means. (b) **Wrapped** documents keep
   the old cost entirely (543 MB), because under wrapping the line breaking depends on the width and
   re-shaping on every resize frame measured 18.7x worse than retaining. Both need the same
   upstream thing: a compact shaped-but-unbroken representation in parley. Numbers in
   `03_text_document_shaping/IMPLEMENTATION_NOTES.md`.
3. **Candidate 01's core fix**: prune subtrees of culled items in `render_item_children`. The only
   change that would remove the O(items) traversal for non-virtualized views.
4. **femtovg per-draw buffers**: `Vertex`/`Drawable` vectors are reallocated per primitive
   (~4 allocations each in the `rects` profile). Pooling them inside femtovg's `Canvas` is the
   next femtovg-side win; `MultiStopGradient`'s `Rc<[GradientStop]>` is another.
5. **Candidate 02's loose end**: newly shaped text is line-broken twice per frame.
