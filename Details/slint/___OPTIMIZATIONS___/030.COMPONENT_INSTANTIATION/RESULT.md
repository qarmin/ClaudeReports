# 30 - What creating a component costs, and a median that lied

Ported from `perf_test/___OPTIMIZATIONS___/30_component_instantiation/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

Tabs, expanders, dialogs and `if` blocks destroy and re-create subtrees constantly. Every scene here
built its item tree once at startup and never touched it again, so component *instantiation* had
only ever been measured as part of the first frame.

`--toggle-visibility N` puts N items behind an `if` and flips it every frame.

## Measured

`--mode rects --count 3000`, the toggled subtree on top:

| items toggled | allocations on a creating frame | median frame |
| --- | --- | --- |
| 500 | 5 533 | 3.95 ms |
| 1 000 | 11 035 | 3.44 ms |
| 2 000 | 19 478 | 3.55 ms |
| 4 000 | 35 480 | 4.93 ms |
| (static baseline) | 14 | 2.58 ms |

From differences, so the fixed part cancels:

| | allocations per component |
| --- | --- |
| 500 -> 1 000 | 11.00 |
| 1 000 -> 2 000 | 8.44 |
| 2 000 -> 4 000 | **8.00** |

**~8 allocations to instantiate one component**, converging as the batch grows. Destroying them is
close to free: the frames that hide the subtree allocate 25 and run ~1 ms faster than the ones that
show it.

Perspective: after candidate 014 a *static* frame of 3 000 rectangles allocates **14** in total.
Creating 2 000 components allocates 19 478. Component instantiation is now, by a wide margin, the
most allocation-heavy thing a frame can do.

## The median hid this completely

The first measurement of this used `tools/matrix.sh`, which reports medians. It said **25
allocations per frame** for all of 100, 500 and 2 000 toggled items - a flat, unremarkable row.

The workload alternates: 19 491 allocations on the frames that create the subtree, 25 on the frames
that destroy it. The median of an alternating series is one of the two sides, and here it picked
the empty one. Only the raw per-frame table showed it:

```
frame   time    allocations
    1   3.54ms           25
    2   5.54ms        19440     <- created
    3   3.47ms           25
    4   5.43ms        19491     <- created
```

Candidate 025's report already switched the summary lines to medians *because* one outlying frame
could contaminate a mean (`README.md`). Both are true, and they pull in opposite directions: a
median is right for noise and wrong for a bimodal workload. The harness's own `spread min..max`
line covers it - `allocations 25..19491` - which is why that line exists.

`tools/matrix.sh` now carries a warning to read the raw run whenever the workload is not the same
on every frame.

## Reproduce

```sh
# note: read the per-frame table, not the median
../../target/rdebug/perf_test --bench --mode rects --count 3000 --toggle-visibility 2000 --frames 8
```

## Does it change behavior/output?

N/A - measured/quantified only, no patch applied to real source in the original session.
