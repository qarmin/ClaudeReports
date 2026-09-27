# FemtoVG on real hardware: pristine vs this session's changes

Measured by the user on their GPU with `just gpu-report` (medians of 3 runs, 20 frames each,
`rdebug`, window 1280x860). Kept here by hand after a later Xvfb test of the script overwrote the
file; the script now moves an existing report to `GPU_RESULTS_previous.md` instead.

| scene | pristine | session | delta | mimalloc | allocations/frame pristine -> session |
| --- | --- | --- | --- | --- | --- |
| `rects` 3000 | 5.07 ms | **4.35 ms** | **-14.2%** | 4.42 ms | 50 745 -> 33 973 |
| `rects` 3000 --gradient | 5.98 ms | **5.63 ms** | **-5.9%** | 5.94 ms | 52 951 -> 35 076 |
| `list` 100000 | 1.18 ms | **1.15 ms** | -2.5% | 1.42 ms | 3 287 -> 2 015 |
| `scroll` 100000 | 54.71 ms | **48.79 ms** | **-10.8%** | 48.40 ms | 703 099 -> 301 930 |
| `scroll` 100000 --flat-rows | 43.46 ms | **40.57 ms** | **-6.6%** | 38.55 ms | 503 402 -> 302 249 |
| `text` 100000 | 1.96 ms | 1.97 ms | +0.5% | 2.07 ms | 1 903 -> 1 282 |

## How to read it

- **delta** is what candidates 08 (duplicate dependency registrations), 09, 10 (gradient paths)
  and 06+11 (pooled paths with reused tessellation buffers) are worth on this GPU.
- The **mimalloc** column is the same code with a ~3.3x faster allocator, so `session - mimalloc`
  is the allocator-side headroom that is left. It is **gone** for `rects`, `rects --gradient` and
  `list` (mimalloc is slower there now), ~1% for `scroll` and ~5% for `scroll --flat-rows`.
  Further allocation-removing work is therefore not where the next win is.
- Allocation counts are hardware independent and match `FINAL_NUMBERS.md`, which was measured
  under llvmpipe.

## Row variants of the non-virtualized list (candidate 12)

Not yet measured on this GPU - it is part of the same `just gpu-report` run now. Under llvmpipe
(software renderer) the four variants of `scroll` 100 000 came out as:

| row style | items per row | frame time | allocations/frame |
| --- | --- | --- | --- |
| `layout` | 5 | 135.18 ms | 300 744 |
| `flat` | 4 | 116.06 ms | 301 037 |
| `clipped` | 6 | 99.21 ms | 601 151 |
| `bare` | 1 | **25.88 ms** | 200 305 |

`layout` -> `bare` (5.2x) is the prize for pruning the children of culled items; `clipped` shows
how much of that prize today's `clip: true` escape hatch spends on itself. Details and the design
constraint: `12_subtree_pruning_prize/RESULT.md`.
