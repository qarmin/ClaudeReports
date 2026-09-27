# 00 - Tooling

Not an optimization; the measurement equipment everything else depends on. All of it lives
in `perf_test/src`, so it stays runnable.

## What was added

| Tool | File | Why |
| --- | --- | --- |
| Headless benchmark (`--bench`) | `src/bench.rs` | Renders N frames with a `MinimalSoftwareWindow` platform, no display. Deterministic, works over ssh. |
| Windowed benchmark (`--frames N`) | `src/frames.rs` | Same table with the real backend (FemtoVG/Skia) via a rendering notifier; the only way to measure the GPU renderers. |
| Allocation counter | `src/alloc_stats.rs` | Allocations per frame + RSS. Feature `alloc-stats`. |
| Sampling allocation profiler | `src/alloc_profile.rs` | `--alloc-profile <n>`: 1-in-n allocations of the last frame, unresolved backtraces into a fixed static table, symbols resolved in the report. |
| Text shape counters | `src/bench.rs` | `TextLayoutCache::cache_miss_count()` / `layout_miss_count()` through the software renderer's `testing` feature. |
| Allocator cost | `tools/allocbench.rs` | 34 ns per malloc+free pair (32 B, glibc), to convert allocation counts into time. |

## Why a custom allocation profiler

`perf record` fails with `Operation not permitted` in this environment even at
`perf_event_paranoid = -1` (seccomp), and neither `valgrind` nor `heaptrack` is installed.
The sampler is allocation-free while sampling (fixed 4096-bucket static table, raw
instruction pointers, recursion guard), so it does not perturb what it measures beyond the
sampling itself.

Caveat found in use: symbols occasionally attribute to a neighbouring monomorphization
(identical code folding), so read the stack, not one frame.

## Reproduce

```sh
just bench                                   # software renderer table
just frames-xvfb 100000 8                    # FemtoVG under Xvfb
../target/rdebug/perf_test --bench --mode list --count 100000 --frames 4 --alloc-profile 20
just allocbench
```
