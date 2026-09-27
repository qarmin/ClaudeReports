# A sweep across every axis the harness has

Run in one batch to find what else is worth a candidate of its own. Medians of `RUNS=5`. Most of
this is confirmation or null - which is the point of running it: the two entries that were *not*
null (candidate 43, and the nine-slice path below) would not have been found by looking harder at
the things already known to be slow.

## Not null

### Nine-slice costs +81% over a plain image of the same size

| 30 items at 200px, full repaint | frame |
| --- | --- |
| `medium.png` | 1.46 ms |
| `nine_slice.png` (same drawn size) | **2.64 ms** |

A 9-slice draw is nine sub-draws with their own scaling, so some cost is expected; +81% for a border
decoration that most styles apply to every button and panel is worth knowing. Not investigated
further.

### Wrapped text costs 10x the memory of unwrapped

| `text` 50 000 lines | frame | RSS |
| --- | --- | --- |
| `no-wrap` (Slint's default) | 3.96 ms | **26.1 MB** |
| `--word-wrap` | 3.64 ms | **280.8 MB** |

Frame time is the same; memory is 10.7x. This is candidate 03's deliberate exclusion made visible:
lazy per-paragraph shaping is disabled for wrapped text, so every paragraph keeps its glyphs. The
frame-time parity says the exclusion costs nothing *per frame* - it costs 255 MB.

### A model prepend, and a scale-factor change

Both re-shape every text in a non-virtualized tree - **617 ms and 603 ms at 20 000 rows**. Written up
separately as candidate 43.

## Confirmations

| axis | result |
| --- | --- |
| `--row-style clipped` | **13.02 ms** against 21.84 for `layout` - `clip: true` is **-40%**, as candidate 01 found |
| `--row-style touch` (a `TouchArea` per row) | 32.89 ms, **+51%** over `layout` |
| `--row-style flat` | 19.54 ms, -11% |
| `--gradient` | 2.92 ms against 2.05 solid, **+42%** |
| `--opacity 0.5` | 2.49 ms, +21% - matches candidate 25's +18% |
| `--border 4` | 2.01 ms against 2.05 - **free**, as candidate 25 found |
| image rescaling | 1.51 ms against 1.46 at 1:1 - **null**, as candidate 16 found |
| `--line-length 1000` vs 100 | 8.46 vs 4.00 ms - long lines cost about 2x |

## Null

| axis | result |
| --- | --- |
| **`Model::row_data` per frame** (plan item E1) | **not called per frame.** Instrumented `VecModel::row_data`: `list` 20 000 over 20 frames makes **fewer than 500 calls** total; `scroll` makes exactly 20 000, once per row at construction. The model plumbing is not a per-frame cost |
| **Many timers** (plan item E2) | **completely flat**: 0.079 us per tick with no timers, **0.077 us with 10 000**. `maybe_activate_timers` keeps the list sorted by expiry, so a tick that fires nothing looks at the head and stops. `src/bin/timer_probe.rs` |
| `--toggle-global` | 22.55 vs 21.51 ms - nothing, as candidate 17 found |
| `--model-adapter filter/sort/map` | 19.75 / 21.84 / 22.16 vs 21.51 ms - all within the spread |
| `--toggle-visibility 2000` | 2.97 ms - creating and destroying 2 000 items per frame is cheap |
| `--scale-factor 1.5 / 2.0` (static) | 22.18 / 19.82 vs 22.80 ms - a higher scale factor costs nothing by itself. Only *changing* it does |
| upscaling an image 8x from a 64px source | 1.20 ms, *cheaper* than 1:1 from a 512px source - the cost is destination pixels and source cache locality, not the ratio |

## Nesting

| `--nest-depth` | frame |
| --- | --- |
| 1 | 0.43 ms |
| 4 | 4.56 ms |
| 8 | 6.42 ms |
| 8, `--nest-plain` | 6.59 ms |

Layout wrappers cost the same as plain ones (6.42 vs 6.59), which is candidate 18's conclusion. The
growth is item count, not depth as such.

---

# Second sweep

## Not null

### The row's structure costs more at startup than anything else measured

50 000 rows, `scroll`, by row style:

| row style | RSS | first frame |
| --- | --- | --- |
| `bare` (one `Rectangle`) | 75.4 MB | **16.8 ms** |
| `flat` (no layout inside) | 165.1 MB | 62.4 ms |
| `clipped` | 199.1 MB | 53.4 ms |
| `layout` (the realistic row) | 257.0 MB | **1.46 s** |
| `touch` (a `TouchArea` too) | 282.3 MB | **1.50 s** |

**87x on first frame, 3.7x on memory**, from the same 50 000 records. Candidate 01 measured the
memory side; the startup side is larger and is dominated by running the layout solver once per row.
A `TouchArea` per row adds 25 MB and nothing to startup, but costs **+51% per frame** (first sweep).

### Accessibility costs ~6% on a large non-virtualized list

Built with `--features accessibility` (the harness disables it by default, with a comment saying it
"costs a lot on large lists"):

| scene | off | on | |
| --- | --- | --- | --- |
| `scroll` 20 000 | 22.83 ms | 24.19 ms | **+6.0%**, 0 of 5 pairs faster |
| `list` 20 000 (`ListView`) | 2.26 ms | 2.26 ms | 0.0% |
| `scroll` 20 000 `touch` rows | 36.37 ms | 36.35 ms | -0.1% |

First frame (588 vs 586 ms) and RSS (114.5 vs 115.1 MB) are unchanged. So "costs a lot" overstates
it - 6%, only where the tree is large and unvirtualized.

**Caveat that matters**: the headless benchmark has no assistive client attached, so this is the
*idle* cost of the feature being compiled in. What it costs while a screen reader is walking the
tree is a different measurement and is not made here.

## Confirmed

### Candidate 20 stands: typing re-shapes the whole document, linearly

| `text` lines | per keystroke | paragraph shapes |
| --- | --- | --- |
| 5 000 | 91.02 ms | 5 052 |
| 20 000 | 350.11 ms | 20 052 |
| 50 000 | **867.93 ms** | 50 052 |

One shape per line, every keystroke - candidate 20's 882 ms reproduces at 868 ms. About **17 us per
paragraph**.

**Cursor movement is fine**: `--key-input` costs 4.61 ms against 3.94 for a static document, and 47
shapes. So candidate 03's lazy shaping does its job for *navigating* a large document; only editing
falls off the cliff. That is a sharper statement of the problem than candidate 20 made - the fix
needs to target the edit path specifically, not shaping in general.

`--wheel` (4.03 ms) is likewise free.

## Null

| axis | result |
| --- | --- |
| **The `NewBuffer` cache purge** (plan item C2) | **no-op.** Removing it changes nothing, RSS is flat over 200 frames either way, and the cache holds **0 entries** at the moment it is purged. Written up in `37_.../C2_PURGE.md` |
| window size at fixed item count | 1.20 ms at 640x480 up to 6.57 ms at 3840x2160 - 27x the pixels for 5.5x the time, because a fixed 3 000 items fill less of a large window. Not a clean measurement of either axis; recorded so nobody re-runs it expecting one |
