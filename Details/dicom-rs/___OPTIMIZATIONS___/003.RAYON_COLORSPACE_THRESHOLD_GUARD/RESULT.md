# convert_colorspace_u8/u16: pixel-count threshold + core-availability guard before rayon dispatch

`convert_colorspace_u8`/`convert_colorspace_u16` (`pixeldata/src/lib.rs`) unconditionally dispatch
to rayon's `par_chunks_mut` whenever the `rayon` feature is compiled in, regardless of how many
pixels are being converted or how many cores are actually available. For small images, rayon's own
per-call dispatch/work-stealing setup can cost more than the sequential per-pixel work it would
save. Carried over as a well-specified lead from a prior session's notes (candidates 004/008/009 in
`~/.claude/skills/perf-optimize/projects/dicom-rs.md`), re-implemented and re-measured fresh here
(base commit `df2b9e4236298efa3fa37a1f360fc4bf284d586a`) as a single combined fix rather than
replicating each historical intermediate step separately, since threshold-only and
threshold+guard-combined were reported as producing the same result for this function pair in the
prior notes.

## What was tried

Added `COLORSPACE_RAYON_PIXEL_THRESHOLD: usize = 16_384` (pixel count, i.e. `i.len() / 3`) and a
`rayon_worth_it()` helper (`std::thread::available_parallelism().map(|n| n.get() > 1)`, cached in a
`OnceLock<bool>` so the syscall only happens once per process) as a joint gate before dispatching
to `par_chunks_mut` - below the threshold, or on a single-available-core host, both functions now
use the plain sequential `chunks_mut` even when the `rayon` feature is enabled. Extracted each
function's per-pixel closure body into a named `convert_pixel_u8`/`convert_pixel_u16` function so
both the `par_chunks_mut(3).for_each(...)` and `chunks_mut(3).for_each(...)` branches can share the
exact same code (required because `ParallelIterator`/`Iterator` are different types that can't
share a single `let iter = ...; iter.for_each(...)` binding the way the original `#[cfg]`-only
dispatch did).

Bench: `___OPTIMIZATIONS___/harness/bench_pipeline`'s `colorspace` mode, against two new synthetic
fixtures generated via the existing `gen-ybr-random` mode from candidate 002 (using its
`row-multiplier` argument to hit exact pixel counts): `ybr_below_threshold.dcm` (100x100 = 10,000
pixels, below the 16,384 threshold) and `ybr_above_threshold.dcm` (3,000x100 = 300,000 pixels,
above it). `paired.sh`, RUNS=9, per this workspace's paired-comparison discipline.

| Variant | below-threshold (10,000px) multi-core p50 | above-threshold (300,000px) multi-core p50 | single-core (no-rayon build) |
|---|---|---|---|
| baseline (unconditional rayon) | 0.119 ms / 0.118 ms (2 rounds) | 1.038 ms / 0.832 ms (2 rounds) | 2.336 ms |
| patched (threshold+guard) | 0.080 ms / 0.079 ms (2 rounds) | 1.161 ms / 0.831 ms (2 rounds, not consistent) | 2.336 ms (not consistent) |

## Result

**Below the 16,384px threshold: -32.5% multi-core (unpinned, default rayon-enabled build), 9/9
paired wins, consistent across both rounds** - a real, reproducible small win for small-image
colorspace conversion. **Above the threshold: no consistent change** (round 1: +11.8%, 3/9 - not
consistent; round 2: -0.1%, 5/9 - not consistent), exactly as expected since the code path there is
identical to baseline (same `par_chunks_mut` dispatch). **Single-core (`--no-default-features
--features image,native` build, no rayon linked, confirmed via `nm | grep -c rayon_core` = 0 both
builds): no consistent change** (+0.0%, 5/9 - not consistent), also as expected since the entire
threshold/guard block is behind `#[cfg(feature = "rayon")]` and compiles out completely without
that feature - the no-rayon code path is textually identical to baseline (`i.chunks_mut(3).for_each(...)`,
just now calling the extracted named function instead of an inline closure with the exact same
body).

## Does it change behavior/output?

- **Bit-exact?** Yes, in every configuration tested (below-threshold, above-threshold,
  `ybr_random.dcm`, `ybr_big.dcm` from candidate 002's fixtures) - `sha256sum` of the decoded RGB
  output is identical between baseline and patched builds in all four cases. Only the dispatch
  mechanism changed, never the arithmetic.
- **Is this a breaking change?** No public API/type changes. `COLORSPACE_RAYON_PIXEL_THRESHOLD`
  and `rayon_worth_it()` are private items in `pixeldata/src/lib.rs`.
- Project test suite: **537 passed / 0 failed / 6 ignored, both before and after** (full
  `cargo test --workspace`, including doctests) - no new failures.

## Why the result looks the way it does

At 10,000 pixels the sequential per-pixel arithmetic (a handful of `f32` multiplies plus a
`floorf` call) is cheap enough in aggregate that rayon's fixed per-call dispatch cost (spawning
work-stealing tasks across `par_chunks_mut`'s chunks) is the larger term - removing that dispatch
removes proportionally more total work at this size. At 300,000 pixels the aggregate per-pixel
work is large enough that rayon's dispatch cost is a small fraction of the total either way, so the
threshold correctly leaves that path untouched (measured as noise-level, not a real change) rather
than needing to change anything - the two pixel counts were deliberately chosen to sit on either
side of the ~16,384 threshold, and each other's baseline confirms it does the right thing at both
ends. This matches this workspace's own candidate-002 finding that rayon's fixed per-call cost can
dominate over cheap per-element arithmetic at moderate sizes.

## What's not done / open questions

- The precise crossover point (prior notes report "sharp, between 10,000 and 20,000 pixels" on
  their box) was not independently re-measured pixel-count-by-pixel-count this session - only the
  two chosen endpoints (10,000 below, 300,000 well above) were tested, which is sufficient to
  confirm the threshold value works correctly at both ends but doesn't re-derive the exact
  crossover on this box. If the crossover here differs meaningfully from 16,384, the constant could
  be off in either direction without this candidate's own measurements catching it.
- The same guard was not applied to `interleave()` (`PlanarConfiguration::PixelFirst` path,
  separate backlog item - a much higher pixel threshold is expected there since its per-element
  work is only byte copies, not float arithmetic) or to the remaining unconditional rayon sites in
  `build_monochrome_image`/`convert_pixel_slice` - left as open backlog items in `IDEAS.md`.
- Whether `rayon_worth_it()`'s core-availability check adds any measurable benefit *beyond* the
  pixel threshold alone (e.g. under `taskset -c 0` with a pixel count *above* the threshold, where
  the threshold alone would still dispatch to rayon with a 1-thread pool) was not isolated
  separately this session - prior notes report this specific scenario shows no further measurable
  benefit from the guard for this function pair (unlike `interleave()`, where it reportedly did
  matter) - not independently re-verified here.
- A separate re-derivation of this exact candidate found the harness's `colorspace-fast` mode
  (length-only checksum) gives a *reproducibly backwards* direction from `colorspace` (real FNV-1a
  hash) when timing `convert_colorspace_u8` at 10,000px: patched (threshold-gated, sequential
  dispatch) measured ~10-13% *slower* via `colorspace-fast` but ~6-8.5% *faster* via `colorspace`,
  both highly reproducible. Leading hypothesis (not confirmed with a profiler): the baseline's
  unconditional `par_chunks_mut` writes the converted buffer from a rayon worker thread possibly on
  a different core than the one that re-reads it; `colorspace`'s hash step re-reads the whole buffer
  right after (paying any cross-core cache-line migration cost), while `colorspace-fast` never
  touches the buffer again, hiding that cost entirely. This is why `README.md` flags
  `colorspace-fast` as unsafe to use below ~300k pixels - use `colorspace` to re-verify any
  small-pixel-count result from this candidate.
