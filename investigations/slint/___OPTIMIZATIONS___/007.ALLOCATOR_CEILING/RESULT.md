# 07 - How much of a frame is the allocator? (and a correction)

Ported from `perf_test/___OPTIMIZATIONS___/07_allocator_ceiling/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

Measured. It corrects candidate `001.SCROLLVIEW_ROW_STRUCTURE`'s estimate of what a
`DependencyNode` arena could be worth - the earlier number was too optimistic because the
micro-benchmark behind it used the wrong allocation pattern.

## What was wrong before

Candidate 001's `FINDINGS.md` section 2 estimated the arena (upstream #11292 / #12255) at
**1.6-1.8x** on FemtoVG, from "703 101 allocations/frame x 34 ns". The 34 ns came from
`tools/allocbench.rs`, which allocated N boxes and *then* freed them all - a growing-cache
pattern. Per-frame `DependencyNode` traffic is the opposite: the list is rebuilt every frame, so
the allocator's free list for that size class stays hot. `tools/allocbench.rs` now measures both:

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

## What was tried

`perf_test --features mimalloc` swaps glibc malloc for mimalloc (mutually exclusive with the
counting allocator, so only times are comparable). Medians of 3 runs, `--frames 6/8`:

| scene | renderer | glibc | mimalloc | delta |
| --- | --- | --- | --- | --- |
| `scroll` 100 000 (701 k allocations/frame) | software | 148.17 ms | **130.22 ms** | **-12.1%** |
| `rects` 3 000 (52 k allocations/frame) | software | 2.17 ms | 2.45 ms | +12.9% |
| `scroll` 100 000 (703 k allocations/frame) | FemtoVG (llvmpipe) | 59.69 ms | 62.59 ms | +4.9% |
| `text` 100 000, show+layout (1.5 M allocations, one-off) | software | 2.02 s | 2.04 s | +1% |

## Result

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

Candidate `001.SCROLLVIEW_ROW_STRUCTURE` measured 51 ns per *removed tracked read* in situ (making
two child geometry properties constant removed exactly 2 allocations/row and 10.2 ms/frame at
100 k rows on FemtoVG). If only ~13 ns of that is the allocator and the allocator's total share is
single-digit percent on FemtoVG, then most of the 51 ns is the rest of the read: the
thread-local lookup of the current binding, the two linked-list insertions
(`SingleLinkedListPinHead::push_front` + `DependencyListHead::append`), and the cache misses
those cause - each property's list head lives in a different cache line, and there are ~700 000
of them per frame.

## Does it change behavior/output?

N/A - measured/quantified only, no patch applied to real source in the original session. This
candidate compares two build configurations (`--features mimalloc` vs default glibc allocator);
no framework source was patched.

## Why the result looks the way it does

The correction exists because the original arena estimate used a "batch" allocation
micro-benchmark (allocate N, then free all N) as a stand-in for a "churn" workload (allocate one,
free one, repeat, with a hot free list) - and the two differ by 2.6x (35.6 ns vs 13.4 ns) for
glibc. Per-frame `DependencyNode` traffic is a churn pattern, so the churn number is the correct
one to reason from.

That reframes the upstream discussion: making the *allocation* cheaper (arena) is worth less
than reducing the *number of tracked reads* or making the registration itself cheaper. Candidate
`008.DUPLICATE_DEPENDENCY_REGISTRATIONS` measures whether any of those reads are redundant.

## What's not done / open questions

No allocator swap or arena was applied to the tree - this candidate only measures the ceiling.
The `dep-stats` feature and the `allocbench_mimalloc` control crate remain as measurement tooling.

## Reproduce

```sh
just run           # this folder: glibc vs mimalloc across the scenes
just allocbench    # in perf_test: both allocation patterns
cd allocbench_mimalloc && cargo run --release -- 700000     # the control
```

Fully reproducing the `perf_test`-based bench additionally needs `perf_test/` copied in manually,
since it is untracked in the main repo's git history. `allocbench_mimalloc/` is self-contained and
copied into this folder verbatim.
