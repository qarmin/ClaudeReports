# 07 - How much of a frame is the allocator? (plan, before measuring)

## Why

`register_as_dependency_to_current_binding` is the top allocation site in every scene measured
so far: 39.5% of allocations in `list`, 52.3% in `rects`, and ~7 of 17 allocations per
rectangle. The fix upstream is discussing (#11292 / #12255) is an arena/slab for
`DependencyNode`. Candidate 001 estimated its value indirectly (1.6-1.8x on the pathological
`ScrollView`) from a 34 ns malloc+free micro-benchmark. That estimate assumes the allocator
cost is the *only* thing an arena removes and that 34 ns is representative in-situ.

A cheaper and more honest bound: **make allocation itself faster and see what the frame does.**
If swapping glibc malloc for mimalloc moves a frame by X%, then no allocator-side change
(arena included) can be worth more than roughly X% on that scene, because the arena's whole
job is to make these allocations cheaper.

## Plan

1. Add mimalloc behind a cargo feature in `perf_test` (`--features mimalloc`, replaces the
   counting allocator; allocation counts are then unavailable, times are what matters).
2. Measure every scene, glibc vs mimalloc, software renderer (`--bench`, deterministic) and
   FemtoVG under Xvfb where it matters:
   - `scroll` 100k (the allocation-heaviest: 701 k/frame)
   - `rects` 3000 (per-primitive: 52 k/frame)
   - `list` 100k (well-behaved: 2 k/frame)
   - `text` 100k show+layout (1.5 M allocations, one-off)
3. Report the delta as the ceiling for allocator-side work, and cross-check against the 34 ns
   estimate.

## Expectation

`scroll` 100k software: 149 ms/frame with 701 k allocations. At 34 ns/pair that is 24 ms
(16%). mimalloc is typically 2-3x faster than glibc malloc on small short-lived allocations,
so the expected move is ~8-16 ms (5-11%) if the estimate is right. A much larger move would
mean the allocator matters more than the micro-benchmark suggests (fragmentation, cache
effects); a much smaller one would mean glibc's fast path is already good and the arena is
not worth much.
