# 28 - What an item costs, split into "visited" and "drawn"

Ported from `perf_test/___OPTIMIZATIONS___/28_drawn_vs_visited/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

Candidates 015 and 018 both landed on ~90 ns as the cost of *visiting* an item. This separates that
from the cost of actually *drawing* one, which is the number that says how much of candidate 012's
prize comes from skipping the draw and how much from skipping the visit.

The `nest` scene makes the split possible: its wrapper items are transparent and draw nothing,
while its leaves draw a small rounded, bordered rectangle. Comparing configurations at the same
*leaf* count isolates the wrappers; comparing at the same *depth* isolates the leaves.

## Measured

Medians of 7 runs on an idle machine:

| configuration | items | frame time |
| --- | --- | --- |
| 500 leaves, depth 0 | 500 drawn | 1.01 ms |
| 500 leaves, depth 16 | 500 drawn + 8 000 transparent | 2.00 ms |
| 8 500 leaves, depth 0 | 8 500 drawn | 3.87 ms |

Both costs come out of *differences*, so the fixed per-frame overhead cancels:

| | cost per item |
| --- | --- |
| visited, draws nothing | (2.00 - 1.01) / 8 000 = **124 ns** |
| visited **and** drawn | (3.87 - 1.01) / 8 000 = **358 ns** |

**Drawing an item costs ~2.9x visiting it.**

## The absolute numbers move, the ratio does not

An earlier run of the same three configurations, taken while a test suite was running in the
background, gave **172 ns** and **753 ns** - both inflated, ratio **4.4x**. The run before that
gave 96 ns and 405 ns, ratio **4.2x**. The absolute figures track machine load; the ratio is what
survives, and it sits between 2.9 and 4.4.

Recorded this way deliberately: a single number here would have been wrong three times over. What
the measurement supports is "drawing costs a few times what visiting costs", not a constant.

## What it says about candidate 012

Culling an item (skipping its draw but still visiting it) - which is what `clip: true` achieves
today - saves roughly **two thirds to three quarters** of its cost. Pruning the subtree entirely,
so it is never visited, saves the rest.

That matches the measured scene: `scroll` 100 000 goes 135 ms (`layout` rows) -> 99 ms (`clipped`)
-> 26 ms (`bare`, i.e. a fifth of the items). Clipping recovers part of it, and the remaining gap
to `bare` is the visits.

So candidate 012's subtree bounds are worth having *on top of* clipping, not instead of it - and the
part clipping already gets is the larger share per item.

## Reproduce

```sh
RUNS=7 tools/matrix.sh \
  "500 drawn|--mode nest --count 500 --nest-depth 0" \
  "500 drawn + 8000 transparent|--mode nest --count 500 --nest-depth 16" \
  "8500 drawn|--mode nest --count 8500 --nest-depth 0"
```

## Does it change behavior/output?

N/A - measured/quantified only, no patch applied to real source in the original session.
