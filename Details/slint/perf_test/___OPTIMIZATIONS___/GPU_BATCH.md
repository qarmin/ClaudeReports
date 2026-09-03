# What to run, and how long it takes

| command | time | what it answers |
| --- | --- | --- |
| `just gpu-quick` | **~5 s** + a no-op build | is the current tree still fast (5 scenes), and how big is the remaining structural win (`scroll` vs `scroll --row-style bare`). Appends to `GPU_QUICK.md`. |
| `just bench-all` (in `perf_test`) | ~1 min, **no display needed** | the software-renderer table; deterministic, good for regressions |
| `just gpu-report` | **~10+ min** (it rebuilds a pristine and a mimalloc binary) | the full before/after for a session's worth of changes. Only when that is the question. |



| section | question |
| --- | --- |
| pristine vs session | what this session's applied changes (08, 09, 10, 06, 11) are worth on your GPU |
| mimalloc column | how much allocator-side headroom is left at all - if `mimalloc` is not faster than `session`, further allocation removal is pointless for that scene |
| row variants (new) | the prize for pruning the children of culled items: `layout` -> `bare`, and how much `clip: true` wastes on itself |

## Everything it needs

Only a display and the femtovg prerequisites you already have (`libxkbcommon-x11-0`,
`libxcb-xkb1`). It builds all three binaries itself, stashes and restores the working tree around
the pristine build, and writes results incrementally so a shutdown loses nothing.

## After the run

Nothing to send me - the numbers land in `GPU_RESULTS.md`, and the two tables to look at are:

1. **delta column**: should show the ~-14% / -11% wins measured previously; if a scene regresses,
   that is a bug worth telling me about.
2. **row variants**: `layout` vs `bare` is the size of the remaining structural win. Under llvmpipe
   it was 135 ms vs 26 ms (5.2x); if the ratio holds on the GPU, the traversal - not the
   rasterization - is what bounds non-virtualized lists.
