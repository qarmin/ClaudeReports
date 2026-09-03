# 46 - what the layout solver costs, with nothing to shape

Candidate 44 found that having a layout in a row is what causes every row's text to be shaped at
startup. That raises a question it could not answer, because its rows contained `Text`: how much of
that is the **solver**, and how much is what the solver asks for?

New scene: `--mode layouts`. N containers, each with M plain `Rectangle` children and no text at all,
so nothing can shape. `--layout-kind h|v|grid|none` picks `HorizontalLayout`, `VerticalLayout`,
`GridLayout`, or the same children positioned explicitly - the control.

## Per frame, a layout is free

300 containers, `--repaint-buffer new` so every frame is a full repaint, `RUNS=7`:

| kind | 4 children | 16 children |
| --- | --- | --- |
| none (explicit positions) | 1.35 ms | 1.92 ms |
| `HorizontalLayout` | 1.37 ms | 1.95 ms |
| `VerticalLayout` | 1.36 ms | 1.92 ms |
| `GridLayout` | 1.41 ms | 2.08 ms |
| `HorizontalLayout` + `horizontal-stretch` | 1.37 ms | 1.94 ms |

**+1.5% for a layout, +8% for a grid, and stretching is free.** The solver does not run per frame -
the geometry it produced is cached and only recomputed when an input to it changes.

Child count scales linearly and gently: 300 containers at 1/2/4/8/16/32 children cost
1.20 / 1.28 / 1.36 / 1.59 / 1.95 / 2.61 ms, which is **~160-180 ns per child** including drawing it.
No super-linear term anywhere.

## At layout time it is half a microsecond per child, and a grid is twice that

First frame, 300 containers x 16 children = 4 800 children, medians of 9 runs:

| kind | first frame | over the control | per child |
| --- | --- | --- | --- |
| none (explicit) | **5.73 ms** | - | - |
| `VerticalLayout` | 7.94 ms | +39% | **460 ns** |
| `HorizontalLayout` | 8.68 ms | +51% | **615 ns** |
| `GridLayout` | **11.16 ms** | **+95%** | **1 131 ns** |

A `GridLayout` costs roughly **twice** what the same children cost with no layout, and about 1.8x a
`HorizontalLayout`. For a one-row grid, which is what this scene builds - a real grid with rows and
columns has more to solve.

## What this says about candidate 44

Candidate 44's rows cost **~30 us per row** in startup shaping (60 081 shapes for 20 000 rows in
576 ms). The solver for a three-child layout costs about **1.8 us**.

**The layout solver is not the problem; what the layout asks for is.** A layout is a ~0.5 us per
child event that happens once, and the reason it dominates candidate 44's startup is that asking a
`Text` how wide it would like to be costs 17-30 us, not that the solving is slow.

That also means the fix direction for 44 is not "make layouts faster". It is either to not ask
off-screen items (subtree bounds, candidate 12), or to make the answer cheaper for text that has
never been shaped.

## What did not measure

Forcing a relayout every frame with `--resize-step 3` was meant to isolate the solver in steady
state. The spreads came out at 2.09-3.74 ms against differences of under 2 ms - the resize changes
what is visible and therefore the dirty region as well, so the measurement is not of the solver.
Recorded so it is not attempted again the same way; isolating a repeated relayout needs a knob that
changes a layout input without changing the viewport.

## Reproduce

```sh
RUNS=7 tools/matrix.sh \
  "none|--mode layouts --count 300 --layout-children 16 --layout-kind none --repaint-buffer new" \
  "grid|--mode layouts --count 300 --layout-children 16 --layout-kind grid --repaint-buffer new"

for k in none h v grid; do
  for i in $(seq 1 9); do
    perf_test --bench --mode layouts --count 300 --layout-children 16 --layout-kind $k --frames 3
  done
done   # read the `first frame` line
```
