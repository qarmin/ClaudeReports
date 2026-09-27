# 25, corrected - the cost *does* grow with the corner radius

Candidate 25 measured a corner radius at **+15%** and recorded:

> A "cost grows with radius" reading did not reproduce and is written up as not answered.

It reproduces. It was not visible because the comparison was unpaired.

## Paired measurement

`rects` 3 000, `--border 0` so the border cost cannot leak in, `--repaint-buffer new` so every frame
is a full repaint, 9 alternating pairs against radius 0 each time:

| radius | vs radius 0 | pairs where it is slower |
| --- | --- | --- |
| 1 px | **+11.4%** | 8/9 |
| 4 px | **+23.3%** | 9/9 |
| 8 px | **+42.9%** | 9/9 |
| 12 px | **+56.7%** | 9/9 |
| 13 px (half the 26px cell) | **+59.3%** | 9/9 |

Monotonic, and 9 of 9 at every step above the first.

An unpaired `matrix.sh` sweep of the same configurations put radius 0 at 1.99 ms and radius 1 at
1.86 - reading as "radius 1 is *faster* than radius 0", which is how a real dependency gets written
off as noise. Paired, radius 0 sits at 1.62-1.67 ms; the 1.99 was a slow-mode sample of the
per-process bimodality from candidate 35. **That is the third time in this session that unpaired
medians produced a wrong conclusion** (candidates 35, 39 and now 25).

## Mechanism

`draw_rounded_rectangle_line` (`software/draw_functions.rs:~318-430`) works per display line. For a
line inside the corner band it calls `calculate_xxxx`, which computes **four integer square roots**
(`(r*r - y*y).sqrt()` and three more for the inner border edge), then anti-aliases the resulting
runs pixel by pixel. Outside the band it takes a constant path.

The number of lines in the band is `2 x radius`, capped by the item's height. For the 26px cells here:

| radius | lines in the band | measured |
| --- | --- | --- |
| 1 | 8% | +11.4% |
| 4 | 31% | +23.3% |
| 8 | 62% | +42.9% |
| 12 | 92% | +56.7% |
| 13 | 100% | +59.3% |

So the model is a **fixed cost for taking the rounded path at all** (~10%, visible at radius 1 where
almost nothing is curved) **plus a per-line cost across the corner band**, saturating at +59% once
every line is in it.

## What this means in practice

A corner radius is not a rounding error in a software-rendered UI. On small controls - where the
radius is a large fraction of the height, which is exactly how modern styles look - it is worth up to
**60% of the item's draw cost**. Fluent's default control radius against a 26px row is already in the
+40% region.

Two fix directions, neither implemented here:

1. The four square roots per line are recomputed for every rectangle of every size. For a given
   (radius, border) pair the sequence of `x1..x4` per line is fixed - a small per-radius table,
   computed once and reused, would remove the arithmetic entirely. Radii in a UI come from a handful
   of style constants, so the table count is small.
2. A radius of 0 should not enter this path at all. It appears not to (radius 0 is the cheapest
   point), but radius 1 already costs +11%, so the threshold for "worth the rounded path" is above 1.

## Reproduce

```sh
for r in 1 4 8 12 13; do
  for i in $(seq 1 9); do
    perf_test --bench --mode rects --count 3000 --radius 0  --border 0 --repaint-buffer new --frames 25
    perf_test --bench --mode rects --count 3000 --radius $r --border 0 --repaint-buffer new --frames 25
  done
done
```
