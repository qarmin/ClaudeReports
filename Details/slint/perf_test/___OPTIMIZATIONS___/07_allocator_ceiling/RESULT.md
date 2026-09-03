# 07 - How much of a frame is the allocator? (and a correction)

**Status: measured. It corrects candidate 01's estimate of what a `DependencyNode` arena
could be worth - the earlier number was too optimistic because the micro-benchmark behind it
used the wrong allocation pattern.**

## What was wrong before

`../../FINDINGS.md` section 2 estimated the arena (#11292 / #12255) at **1.6-1.8x** on FemtoVG,
from "703 101 allocations/frame x 34 ns". The 34 ns came from `tools/allocbench.rs`, which
allocated N boxes and *then* freed them all - a growing-cache pattern. Per-frame
`DependencyNode` traffic is the opposite: the list is rebuilt every frame, so the allocator's
free list for that size class stays hot. `tools/allocbench.rs` now measures both:

```
$ just allocbench                      # glibc
batch  700000 x (alloc 32B) then free all: 24.91ms per round = 35.6 ns per pair
churn  700000 x (alloc 32B + free, hot free list): 9.41ms per round = 13.4 ns per pair

$ cd allocbench_mimalloc && cargo run --release -- 700000     # control, mimalloc
batch  700000 x (alloc 32B) then free all:  4.10ms per round =  5.9 ns per pair
churn  700000 x (alloc 32B + free, hot free list): 2.84ms per round =  4.1 ns per pair
```

So the realistic figure is **13.4 ns**, not 35.6 ns, and mimalloc is a real 3.3x faster on that
pattern - which makes an allocator swap a valid way to bound allocator-side work.

## The experiment

`perf_test --features mimalloc` swaps glibc malloc for mimalloc (mutually exclusive with the
counting allocator, so only times are comparable). Medians of 3 runs, `--frames 6/8`:

| scene | renderer | glibc | mimalloc | delta |
| --- | --- | --- | --- | --- |
| `scroll` 100 000 (701 k allocations/frame) | software | 148.17 ms | **130.22 ms** | **-12.1%** |
| `rects` 3 000 (52 k allocations/frame) | software | 2.17 ms | 2.45 ms | +12.9% |
| `scroll` 100 000 (703 k allocations/frame) | FemtoVG (llvmpipe) | 59.69 ms | 62.59 ms | +4.9% |
| `text` 100 000, show+layout (1.5 M allocations, one-off) | software | 2.02 s | 2.04 s | +1% |

## What it means

- On the allocation-heaviest scene with the **software** renderer, a 3.3x faster allocator buys
  12.1%. Scaling that back to "allocation is free" (mimalloc removes ~69% of the per-pair cost)
  puts the *whole* allocator cost at **~17% of that frame**. That is the ceiling for any
  allocator-side change there, arena included.
- On **FemtoVG**, the same swap makes the frame slightly *worse* (+4.9%, outside the ~2%
  run-to-run spread). Allocation is not a bottleneck on that path, and a faster allocator can
  lose more to locality than it gains: this scene retains 1.5 GB, so heap layout matters.
  **The arena's ceiling on the FemtoVG path is small - single digits at best, not 1.6-1.8x.**
- The small `rects` scene gets worse with mimalloc: with a 2 ms frame and partial rendering,
  the allocator is noise and mimalloc's own overhead shows.

## So where do the ~51 ns per tracked property read go?

Candidate 01 measured 51 ns per *removed tracked read* in situ (making two child geometry
properties constant removed exactly 2 allocations/row and 10.2 ms/frame at 100 k rows on
FemtoVG). If only ~13 ns of that is the allocator and the allocator's total share is
single-digit percent on FemtoVG, then most of the 51 ns is the rest of the read: the
thread-local lookup of the current binding, the two linked-list insertions
(`SingleLinkedListPinHead::push_front` + `DependencyListHead::append`), and the cache misses
those cause - each property's list head lives in a different cache line, and there are ~700 000
of them per frame.

That reframes the upstream discussion: making the *allocation* cheaper (arena) is worth less
than reducing the *number of tracked reads* or making the registration itself cheaper. Candidate
08 measures whether any of those reads are redundant.

## Reproduce

```sh
just run           # this folder: glibc vs mimalloc across the scenes
just allocbench    # in perf_test: both allocation patterns
cd allocbench_mimalloc && cargo run --release -- 700000     # the control
```
