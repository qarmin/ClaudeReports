# 37 - the repaint mode nobody measured, and it is the default

Ported from `perf_test/___OPTIMIZATIONS___/37_repaint_buffer_mode/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

`RepaintBufferType::NewBuffer` is `#[default]` (`core/partial_renderer.rs:337-340`). The harness has
always used `ReusedBuffer` (`perf_test/src/bench.rs:293`). **Every number in this workspace came from
the non-default mode**, so the first question was whether any of it generalizes.

In `NewBuffer` mode `prepare_scene` sets the dirty region to the whole screen and calls
`clear_cache()`, skipping `apply_dirty_region` entirely (`software/lib.rs:1518-1527`). That is
*exactly* candidate 032's ablation - the one whose -59% was written up as unreachable headroom - only
it is a shipped, supported mode.

Added `--repaint-buffer <new|reused|swapped>` to the harness.

## The default is much faster on large trees, and much slower on small ones

Medians of `RUNS=5`. Scrolling:

| scene | `reused` | `new` | |
| --- | --- | --- | --- |
| `scroll` 20 000 | 21.27 ms | **12.37 ms** | new is **1.7x** faster |
| `scroll` 100 000 `bare` | 17.78 ms | **11.14 ms** | new is **1.6x** faster |
| `rects` 3 000 | 2.35 ms | **2.09 ms** | new |
| `list` 20 000 (ListView) | 2.22 ms | 2.23 ms | tie |

Static (`--scroll-step 0`), where partial rendering is supposed to earn its keep:

| scene | `reused` | `new` | |
| --- | --- | --- | --- |
| `rects` 300 | **0.09 ms** | 0.72 ms | reused is **8x** faster |
| `rects` 3 000 | **0.76 ms** | 1.93 ms | reused is 2.5x |
| `scroll` 2 000 | **0.84 ms** | 2.46 ms | reused is 2.9x |
| `scroll` 20 000 | 18.30 ms | **11.43 ms** | new is 1.6x |
| `scroll` 100 000 `bare` | 14.78 ms | **11.26 ms** | new is 1.3x |

The crossover sits between 2 000 and 20 000 items, matching candidate 032's estimate from the
ablation. `SwappedBuffers` tracks `reused` within noise everywhere (22.46 / 17.69 / 2.37 / 2.21).

All three modes render the same image: compared with a noise floor taken from four runs of each
mode, `scroll` 20 000 and `text` 20 000 both show **0 px outside the noise**.

`clear_cache()` every frame turns out **not** to cost allocations - 22/frame in `new` against
27 in `reused` on `scroll` 100 000 `bare`. Candidate 014's node pooling absorbs it: the dropped
trackers return their dependency nodes to the pool instead of freeing them. Two fixes interacting in
the right direction, which is worth recording because the reverse is more common.

## What this does to candidate 032

Candidate 032 measured the dirty-region walk at 59% of a scrolling frame and concluded the walk
could not be skipped, because it also registers the geometry dependencies that schedule the next
frame for culled items (candidate 033). That reasoning stands - but it describes `ReusedBuffer`.
`NewBuffer` does not need those dependencies, because it repaints unconditionally.

So the headroom is **already reachable today, by configuration**, for any application that repaints
every frame anyway. What is missing is not code but a decision: Slint picks the mode once, from the
buffer strategy, and never from the size of the tree - even though the data says that is what
decides which mode wins.

## What this does to candidates 035 and 036 - one of them is nearly inert by default

Measured per fix, in each mode, on `scroll` 100 000 `bare`, 9 paired runs each:

| fix | `reused` | `new` |
| --- | --- | --- |
| **35** cull geometry read | **-8.8%** (9/9) | **-1.7%** (6/9) |
| **36** conditional state stack | **-9.8%** (8/9) | **-5.2%** (9/9) |
| both | -11.3% (9/9) | -4.4% (9/9) |

**Fix 35 barely does anything in the default mode**, and the reason is mechanical: it serves a culled
item's origin out of the partial-rendering cache, and `NewBuffer` clears that cache every frame, so
the entry is almost never there and the old path is taken. Fix 36 touches no cache and carries over.

This is the qualification candidate 035 was missing. It is still correct and still a clear win in
`ReusedBuffer`; it is close to noise in the mode most applications get by default.

## Recommendations

1. **Choose the mode from the tree size, not only the buffer strategy.** `ReusedBuffer` below a few
   thousand items, `NewBuffer` above. The harness re-derives the crossover on any machine.
2. At minimum, document it. "Only redraw the parts that have changed" reads like a strict
   improvement, and above ~20 000 items it is a 1.6x pessimization.
3. Re-check candidates 014 and 026 in `NewBuffer` too. 14 is now known to help it; 26 is untested there
   and its sweep interacts with a cache that mode already clears.

## Does it change behavior/output?

N/A - measured/quantified only, no patch applied to real source in the original session. Both
`RepaintBufferType` modes are existing, shipped configurations; this candidate only compared them.
Pixel comparison against a same-mode noise floor showed 0 px outside the noise on `scroll` 20 000 and
`text` 20 000, i.e. the two modes render the same image.

## Reproduce

```sh
for m in reused new swapped; do
  RUNS=5 tools/matrix.sh \
    "scroll 20000|--mode scroll --count 20000 --repaint-buffer $m" \
    "rects 300 static|--mode rects --count 300 --scroll-step 0 --repaint-buffer $m"
done
```
