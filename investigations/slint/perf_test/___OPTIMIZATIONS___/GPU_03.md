# Candidate 03 on real hardware

Produced by `just gpu-03` (`tools/gpu_before_after.sh`): builds a binary from the **pristine**
`internal/core/textlayout` and runs it against the current tree on the same display in the same
session, each binary `RUNS` times so the noise floor is visible.

**Never compare against a previous run of this file** - see "allocation counts do not survive
across runs" below.

## Run 2026-08-01 (fifth), RUNS=2, FRAMES=10, FemtoVG, 1280x860 - the shapes column works

| scene | run | show + layout | later frames | allocations/frame | shapes/frame | resident |
| --- | --- | --- | --- | --- | --- | --- |
| `text 100000 no-wrap` | pristine | 7.88 / 7.85s | 2.23 / 2.10ms | 1282 | 0 (stub) | 638.5 / 638.7 MB |
| `text 100000 no-wrap` | session | 7.53 / 7.48s | 2.68 / 2.80ms | 1474 | **11** | **122.5 / 122.6 MB** |
| `text 100000 word-wrap` | pristine | 7.82 / 7.76s | 2.18 / 2.26ms | 1292 | 0 (stub) | 638.4 / 638.3 MB |
| `text 100000 word-wrap` | session | 7.70 / 7.71s | 2.06 / 2.18ms | 1308 | **0** | 634.9 / 635.1 MB |
| `text 10000 no-wrap` | pristine | 883.24 / 907.82ms | 2.14 / 2.13ms | 1282 | 0 (stub) | 156.3 / 153.2 MB |
| `text 10000 no-wrap` | session | 865.40 / 859.17ms | 2.48 / 2.57ms | 1474 | **11** | 103.7 / 103.9 MB |
| `rects 3000` | pristine | 73.24 / 82.82ms | 4.13 / 4.35ms | 34 106 | 0 (stub) | 105.4 / 102.1 MB |
| `rects 3000` | session | 69.55 / 70.01ms | 3.93 / 4.08ms | 34 113 | 0 | 101.9 / 102.7 MB |
| `rects 3000 --gradient` | pristine | 69.01 / 74.85ms | 5.42 / 5.80ms | 35 217 | 0 (stub) | 102.1 / 102.7 MB |
| `rects 3000 --gradient` | session | 71.83 / 71.25ms | 6.41 / 5.68ms | 35 224 | 0 | 105.9 / 105.8 MB |
| `list 100000` | pristine | 97.02 / 94.65ms | 1.37 / 1.31ms | 2 023 | 0 (stub) | 121.8 / 121.3 MB |
| `list 100000` | session | 98.89 / 95.53ms | 0.98 / 1.14ms | 2 106 | 0 | 122.2 / 121.5 MB |
| `scroll 100000` | pristine | 5.92 / 5.83s | 49.44 / *56.61*ms | 301 938 / *301 977* | 0 (stub) | 1495.4 / *1531.2* MB |
| `scroll 100000` | session | 6.14 / 6.00s | 49.63 / 48.87ms | 302 022 | 0 | 1517.0 / 1517.1 MB |

The pristine binary's `0` is a stub: the counter does not exist on that tree, and the script's
sed replaces the call with `0`. *Italics* mark the one disturbed run (`scroll` pristine 2, which
also moved its allocation count, so `scroll` is the one scene that is not perfectly reproducible).

### The open question is answered: 11 shapes per frame, exactly as designed

The windowed run shapes **11** paragraphs per frame, not more than the headless 13 - so the
per-frame cost is not hiding extra work. 11 is the number of lines entering a 1280x860 viewport at
200 px/frame with a ~19 px line height. The remaining hypothesis for "why does a shape look like
~50 us here and ~27 us headless" is therefore just CPU state: the windowed run has a live
compositor and GL driver threads competing for cache. At +0.6 ms per frame in absolute terms, that
is not worth chasing further.

**`word-wrap` shapes 0 per frame.** That is the exclusion working exactly as intended, measured
rather than argued: a wrapped document keeps every paragraph's glyphs and never re-shapes.

### Why `list` and `scroll` report 0 shapes, and why that is correct

The windowed report measures `BeforeRendering` to `AfterRendering` - **rendering only**. The gap
between two renders is mostly vsync idle, so it cannot be included. `list` does all of its text
shaping in the *layout* pass, which falls outside that window; headless, where a frame is charged
from before the property writes, the same scene reports 18 shapes per frame:

| scene | headless (layout + render) | windowed (render only) |
| --- | --- | --- |
| `text` 100 000 `no-wrap` | 13 | 11 |
| `list` 100 000 | 18 | 0 |
| `scroll` 100 000 | 0 | 0 |

Neither number is wrong; they answer different questions. The windowed report now says so on its
own output line, and `PIPELINE.md` carries the rule.

### Confirmed, fifth run running

| | pristine | session | delta |
| --- | --- | --- | --- |
| resident, `text 100000 no-wrap` | 638.6 MB | **122.6 MB** | **-81%** |
| resident, `text 10000 no-wrap` | 154.8 MB | 103.8 MB | -33% |
| `show + layout` | 7.87 s | 7.51 s | -0.36 s |
| per frame, `text 100000 no-wrap` | 2.17 ms | 2.74 ms | +0.57 ms |
| per frame, `text 10000 no-wrap` | 2.14 ms | 2.53 ms | +0.39 ms |
| per frame, `word-wrap` | 2.22 ms | 2.12 ms | -0.10 ms (unchanged) |

Two document sizes at +0.57 and +0.39 ms - the third consecutive run with no document-size scaling.
Allocations reproduce exactly between repeats on every scene but the disturbed `scroll` run.

## Run 2026-08-01 (fourth), RUNS=2, FRAMES=10, FemtoVG, 1280x860

| scene | run | show + layout | later frames | allocations/frame | shapes/frame | resident |
| --- | --- | --- | --- | --- | --- | --- |
| `text 100000 no-wrap` | pristine 1 | 8.09s | 2.38ms | 1282 | n/a | 636.0 MB |
| `text 100000 no-wrap` | pristine 2 | 7.85s | 2.31ms | 1282 | n/a | 638.6 MB |
| `text 100000 no-wrap` | session 1 | 7.53s | 2.89ms | 1474 | n/a | **122.3 MB** |
| `text 100000 no-wrap` | session 2 | 7.42s | 3.02ms | 1474 | n/a | **122.5 MB** |
| `text 100000 word-wrap` | pristine 1 | 7.76s | 1.92ms | 1292 | n/a | 637.7 MB |
| `text 100000 word-wrap` | pristine 2 | 7.94s | 2.28ms | 1292 | n/a | 638.5 MB |
| `text 100000 word-wrap` | session 1 | 7.93s | 2.11ms | 1308 | n/a | 634.9 MB |
| `text 100000 word-wrap` | session 2 | 7.94s | 2.30ms | 1308 | n/a | 634.8 MB |
| `text 10000 no-wrap` | pristine 1 | 917.22ms | 2.15ms | 1282 | n/a | 156.4 MB |
| `text 10000 no-wrap` | pristine 2 | 898.51ms | 2.47ms | 1282 | n/a | 156.7 MB |
| `text 10000 no-wrap` | session 1 | 847.89ms | 2.58ms | 1474 | n/a | 103.4 MB |
| `text 10000 no-wrap` | session 2 | 868.31ms | 2.78ms | 1474 | n/a | 103.8 MB |
| `rects 3000` | pristine | 82.97 / 80.16ms | 4.00 / 4.94ms | 34 106 | n/a | 105.4 MB |
| `rects 3000` | session | 82.54 / 75.88ms | 4.46 / 3.93ms | 34 113 | n/a | 105.5 MB |
| `rects 3000 --gradient` | pristine | 84.24 / 81.78ms | 6.76 / 6.63ms | 35 217 | n/a | 105.9 MB |
| `rects 3000 --gradient` | session | 82.50 / 78.03ms | 6.20 / 6.05ms | 35 224 | n/a | 105.3 MB |
| `list 100000` | pristine | 107.63 / 104.66ms | 1.35 / 1.41ms | 2 023 | n/a | 121.6 MB |
| `list 100000` | session | 105.18 / 115.04ms | 1.17 / 1.37ms | 2 106 | n/a | 121.5 MB |
| `scroll 100000` | pristine | 5.95 / 5.86s | 48.09 / 47.39ms | 301 938 | n/a | 1495.6 MB |
| `scroll 100000` | session | 5.98 / 5.98s | 48.48 / 50.46ms | 302 022 | n/a | 1516.9 MB |

### The shapes column is `n/a`: a parser bug, not a missing counter

The harness prints it; the script's `awk` never read it. The edit that added `shapes = $9` to the
report parsers matched `gpu_quick.sh` and missed `gpu_before_after.sh`, whose `awk` block is
indented differently - so the column was printed from a variable that was never assigned, and awk
renders that as empty rather than complaining.

Fixed, and this time the parser was run against the exact line the harness emits rather than
assumed to work:

```
later frames   2.89ms median, 346.0 fps (1474 allocations, 13 paragraph shapes per frame)
  -> time=2.89ms | allocs=1474 | shapes=13
```

The headless report now prints the same wording, so one parser reads either. The next run fills the
column in.

### What this run does confirm

| | pristine | session | delta |
| --- | --- | --- | --- |
| resident, `text 100000 no-wrap` | 637.3 MB | **122.4 MB** | **-81%** |
| resident, `text 10000 no-wrap` | 156.6 MB | 103.6 MB | -34% |
| `show + layout`, `text 100000` | 7.97 s | 7.48 s | **-0.50 s** (spreads 0.24 / 0.11) |
| per frame, `text 100000 no-wrap` | 2.35 ms | 2.96 ms | +0.61 ms (spreads 0.07 / 0.13) |
| per frame, `text 10000 no-wrap` | 2.31 ms | 2.68 ms | +0.37 ms (spreads 0.32 / 0.20) |
| per frame, `text 100000 word-wrap` | 2.10 ms | 2.21 ms | +0.11 ms (spreads 0.36 / 0.19) |

Two document sizes, +0.61 ms and +0.37 ms - a gap of 0.24 ms against spreads of 0.13 and 0.32. **No
document-size scaling**, holding across both runs since the fix (+1.06/+0.92 then +0.61/+0.37; the
absolute level moves with the machine, the relationship does not).

Allocations reproduce exactly between repeats on every scene: `rects` +7, `list` +83, `scroll` +84,
`text` +192 or +182, all of it the extra `Box` per shaped paragraph. `scroll` resident +21.3 MB
(+1.4%), the same figure for the third run in a row.

## Run 2026-08-01 (third), RUNS=2, FRAMES=10, FemtoVG, 1280x860

After the O(document) scans were removed and both reports switched to medians.

| scene | run | show + layout | later frames | allocations/frame | resident |
| --- | --- | --- | --- | --- | --- |
| `text 100000 no-wrap` | pristine 1 | 8.07s | 1.91ms | 1282 | 645.9 MB |
| `text 100000 no-wrap` | pristine 2 | 7.84s | 1.84ms | 1286 | 646.6 MB |
| `text 100000 no-wrap` | session 1 | 7.36s | 3.10ms | 1474 | **130.8 MB** |
| `text 100000 no-wrap` | session 2 | 7.57s | 2.77ms | 1474 | **130.8 MB** |
| `text 100000 word-wrap` | pristine 1 | 7.77s | 1.92ms | 1292 | 646.8 MB |
| `text 100000 word-wrap` | pristine 2 | 7.85s | 2.22ms | 1292 | 646.7 MB |
| `text 100000 word-wrap` | session 1 | 7.95s | 2.32ms | 1308 | 643.4 MB |
| `text 100000 word-wrap` | session 2 | 7.97s | 2.08ms | 1308 | 639.2 MB |
| `text 10000 no-wrap` | pristine 1 | 899.54ms | 2.05ms | 1282 | 161.2 MB |
| `text 10000 no-wrap` | pristine 2 | 902.44ms | 2.28ms | 1282 | 164.5 MB |
| `text 10000 no-wrap` | session 1 | 863.11ms | 2.92ms | 1474 | 111.7 MB |
| `text 10000 no-wrap` | session 2 | 894.19ms | 3.24ms | 1474 | 112.0 MB |
| `rects 3000` | pristine 1 | 80.99ms | 4.94ms | 34 106 | 113.6 MB |
| `rects 3000` | pristine 2 | 76.30ms | 4.89ms | 34 106 | 113.4 MB |
| `rects 3000` | session 1 | 78.82ms | 4.78ms | 34 113 | 114.3 MB |
| `rects 3000` | session 2 | 82.72ms | 4.62ms | 34 113 | 114.2 MB |
| `rects 3000 --gradient` | pristine 1 | 78.40ms | 6.57ms | 35 217 | 114.1 MB |
| `rects 3000 --gradient` | pristine 2 | 91.64ms | 6.75ms | 35 217 | 113.5 MB |
| `rects 3000 --gradient` | session 1 | 86.05ms | 6.65ms | 35 224 | 114.2 MB |
| `rects 3000 --gradient` | session 2 | 85.40ms | 6.29ms | 35 224 | 114.3 MB |
| `list 100000` | pristine 1 | 103.74ms | 1.58ms | 2 023 | 130.3 MB |
| `list 100000` | pristine 2 | 106.43ms | 1.24ms | 2 023 | 130.2 MB |
| `list 100000` | session 1 | 107.50ms | 1.37ms | 2 106 | 130.3 MB |
| `list 100000` | session 2 | 109.50ms | 1.34ms | 2 106 | 130.2 MB |
| `scroll 100000` | pristine 1 | 5.93s | 48.96ms | 301 938 | 1503.5 MB |
| `scroll 100000` | pristine 2 | 5.91s | 49.10ms | 301 938 | 1503.4 MB |
| `scroll 100000` | session 1 | 5.96s | 47.18ms | 302 022 | 1524.7 MB |
| `scroll 100000` | session 2 | 6.02s | 48.52ms | 302 022 | 1525.1 MB |

**The medians did their job**: every scene now reports the same allocation count in both runs of the
same binary (only `text 100000 no-wrap` pristine differs, by 4). Compare with the run below, where
three rows were contaminated by a single frame each. No row here needs an asterisk.

### The O(document) fix is confirmed

| | before the fix | after |
| --- | --- | --- |
| `text` 100 000, per frame | +1.32 ms | **+1.06 ms** |
| `text` 10 000, per frame | +0.30 ms | **+0.92 ms** |

Two document sizes doing the same work now cost the same, within a spread of 0.33 ms. Before, they
were 4.4x apart. (The 10 000 figure rose rather than the 100 000 one falling because the machine was
in a different state - `text 10000`'s startup went from 500 ms to 900 ms between runs - which is
exactly why only within-run pristine-vs-session pairs are compared.)

### Confirmed

- **Memory 646.3 -> 130.8 MB (-80%)**, both runs of each binary within 0.7 MB. `text` 10 000:
  162.9 -> 111.9 MB.
- **Startup -0.49 s** (7.96 s -> 7.47 s), spreads 0.23 and 0.21 s. Headless the same work is
  2.01 -> 1.73 s; the windowed figure is diluted by window creation, GL init and font loading.
- **Wrapped text unchanged**: +0.13 ms against a 0.30 ms spread, 646.8 -> 641.3 MB.
- **Allocations +192 on `text`, +83 on `list`, +84 on `scroll`, +7 on `rects`** - the extra `Box`
  per shaped paragraph.
- `scroll` 100 000 resident +21 MB (+1.4%), spread 0.1 MB on both sides: ~32 bytes per retained
  paragraph, real and consistent across all three runs.

### Still open: the per-frame cost is ~4x the headless one

+1.0 ms here against +0.2 ms headless, on the same machine, for what should be the same 13
paragraphs per frame. The FemtoVG glyph cache is keyed on font, glyph id and size (`draw_glyph_run`
in `internal/renderers/femtovg/itemrenderer.rs`), so re-shaping cannot be invalidating rasterized
glyphs - that hypothesis is dead. The remaining candidate is simply that the windowed run shapes
*more* paragraphs per frame than the headless one, which nothing has measured because the windowed
report had no shape counter.

It has one now: `paragraph_shape_count()` is exposed as a free function (the FemtoVG renderer has no
reachable `TextLayoutCache`) and the windowed report prints paragraph shapes per frame, in the table
and in the summary line. Both GPU scripts carry the column. The next run says whether 13 is really
13.

## Run 2026-08-01, RUNS=2, FRAMES=10, FemtoVG, 1280x860

| scene | run | show + layout | later frames | allocations/frame | resident |
| --- | --- | --- | --- | --- | --- |
| `text 100000 no-wrap` | pristine 1 | 7.57s | 2.47ms | 1291 | 646.4 MB |
| `text 100000 no-wrap` | pristine 2 | 7.66s | 2.53ms | 1291 | 646.4 MB |
| `text 100000 no-wrap` | session 1 | 7.31s | 3.92ms | 1487 | **130.6 MB** |
| `text 100000 no-wrap` | session 2 | 7.41s | 3.72ms | 1487 | **130.4 MB** |
| `text 100000 word-wrap` | pristine 1 | 7.86s | 2.30ms | 1301 | 646.7 MB |
| `text 100000 word-wrap` | pristine 2 | 7.59s | 2.33ms | 1301 | 646.3 MB |
| `text 100000 word-wrap` | session 1 | 7.54s | 2.42ms | 1317 | 642.8 MB |
| `text 100000 word-wrap` | session 2 | 7.63s | *8.80ms* | *1499* | 642.0 MB |
| `text 10000 no-wrap` | pristine 1 | 505.84ms | 2.26ms | 1276 | 155.5 MB |
| `text 10000 no-wrap` | pristine 2 | 511.99ms | 2.26ms | 1276 | 155.2 MB |
| `text 10000 no-wrap` | session 1 | 484.77ms | 2.56ms | 1472 | 104.7 MB |
| `text 10000 no-wrap` | session 2 | 482.66ms | 2.55ms | 1472 | 104.6 MB |
| `rects 3000` | pristine 1 | 68.61ms | 4.52ms | *33 164* | 109.5 MB |
| `rects 3000` | pristine 2 | 86.72ms | 5.05ms | 36 164 | 109.8 MB |
| `rects 3000` | session 1 | 68.65ms | 4.63ms | 36 171 | 109.7 MB |
| `rects 3000` | session 2 | 65.65ms | 4.44ms | 36 171 | 109.4 MB |
| `rects 3000 --gradient` | pristine 1 | 67.51ms | 5.73ms | 37 267 | 109.8 MB |
| `rects 3000 --gradient` | pristine 2 | 69.41ms | 5.89ms | *36 267* | 109.3 MB |
| `rects 3000 --gradient` | session 1 | 70.50ms | 5.86ms | 37 274 | 110.0 MB |
| `rects 3000 --gradient` | session 2 | 68.78ms | 5.76ms | 37 274 | 109.7 MB |
| `list 100000` | pristine 1 | 93.23ms | 1.50ms | 2 049 | 124.6 MB |
| `list 100000` | pristine 2 | 91.33ms | 1.35ms | 2 040 | 124.2 MB |
| `list 100000` | session 1 | 92.41ms | 1.42ms | 2 121 | 123.9 MB |
| `list 100000` | session 2 | 94.47ms | 1.52ms | 2 122 | 124.8 MB |
| `scroll 100000` | pristine 1 | 5.82s | 47.74ms | 301 961 | 1497.7 MB |
| `scroll 100000` | pristine 2 | 5.77s | 48.09ms | 301 961 | 1497.8 MB |
| `scroll 100000` | session 1 | 5.83s | 50.19ms | 302 044 | 1512.7 MB |
| `scroll 100000` | session 2 | 5.87s | 48.60ms | 302 009 | 1518.8 MB |

*Italics* mark rows contaminated by a single outlying frame; see "what this run exposed".

## Confirmed

**Memory: 646.4 -> 130.5 MB (-80%)** on `text` 100 000 `no-wrap`, both runs of each binary within
0.2 MB. The FemtoVG baseline - window, GL context, glyph atlases - is ~109 MB (the `rects` rows), so
the *document's* own share falls from ~537 MB to ~21 MB, matching the software renderer's
543 -> 38 MB over its own ~15 MB baseline. `text` 10 000: 155.4 -> 104.7 MB.

**Startup, finally distinguishable at RUNS=2**: 7.57/7.66 s pristine against 7.31/7.41 s session, a
**-0.25 s against a 0.10 s spread**. Small in relative terms (-3.3%) because the windowed startup is
dominated by window creation, GL init and font loading - the same work is 2.02 -> 1.71 s (-15%)
when measured headless, where none of that is in the number.

**Wrapped text unchanged**, as designed: 646.5 -> 642.4 MB and 2.32 -> 2.42 ms. (An earlier run
suggested wrapped documents pay ~4 MB for the boxing; across both runs the sign flips, so that was
noise. `scroll` 100 000 is the case where it is real and consistent: 1497.75 -> 1515.75 MB, +1.2%,
with a 0.1 MB spread on the pristine side. That is ~32 bytes per retained paragraph.)

**Allocations: +196 per frame on `text`, +82 on `list`, +83 on `scroll`, +7 on `rects`** - the extra
`Box` per shaped paragraph, and nothing else.

## What this run exposed, and what was fixed because of it

### 1. A per-frame cost that scaled with the document (fixed)

`text` 10 000 and `text` 100 000 shape the *same* 13 paragraphs per frame, so they should cost the
same. They didn't: **+0.30 ms against +1.32 ms**. The difference was two scans in the new code that
walked the whole paragraph list on every `with_text_layout` call - the retention window in
`Layout::dismantle` and the width-change check in `Layout::glyphs`. Two calls per frame over
100 000 `RefCell`s is ~1 ms, which was *four times* the cost of the shaping this candidate actually
introduced, and invisible on the software renderer where a frame is 3.6 ms rather than 2.5 ms.

`RetainedLineBreaking` now carries a `shaped` range bounding where glyphs can be, so both scans are
O(retention window) instead of O(document). Headless afterwards, 10 000 and 100 000 cost the same:
+0.34 ms and +0.36 ms. Needs a re-run here to confirm the GPU figure follows.

### 2. Single-frame outliers were being averaged into the headline (fixed)

Three rows above are contaminated by exactly one frame:

- `rects 3000` pristine 1: 33 164 against 36 164 - a difference of **exactly 3 000**, one
  allocation per rectangle, i.e. one frame that rendered before the window was fully mapped.
- `rects --gradient` pristine 2: 36 267 against 37 267 - exactly 1 000.
- `text word-wrap` session 2: 8.80 ms against 2.42 ms and 1 499 allocations against 1 317. One
  frame of nine at ~60 ms accounts for it exactly.

The reports averaged the frames, so one bad frame moved the row by 3.6x. Both the windowed and the
headless report now print the **median** plus a `spread <min>..<max>` line, so an outlier is visible
instead of contaminating the number. That is also the likely explanation of the earlier
"-3 994 allocations on `rects`" that this file spent a run chasing.

### 3. Allocation counts do not survive across runs

The same session binary produced 29 972 and 36 171 for `rects 3000` on different invocations while
being exactly reproducible within each. Only pristine-vs-session rows measured in one invocation
attribute anything; the reference tables in `GPU_QUICK.md` are history, not a baseline. Now stated
in `PIPELINE.md` and in this script's own output.

## Status

Nothing open. The memory win, the exclusion of wrapped text, the absence of document-size scaling
and the per-frame shape count are all measured on hardware across five runs. `just gpu-03` is the
command that reproduces any of it; `just gpu-quick` carries the same `text` rows for a one-minute
regression check.
