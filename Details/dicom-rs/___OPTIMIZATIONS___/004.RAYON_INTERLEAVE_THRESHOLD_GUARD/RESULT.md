# interleave(): add a rayon-parallel path, gated by pixel-count threshold + core guard

`interleave()` (`pixeldata/src/lib.rs`) converts `PlanarConfiguration::PixelFirst` (color-by-plane)
pixel data into color-by-pixel, via a plain sequential `zip`/`flat_map`/`collect`. Unlike
`convert_colorspace_u8`/`u16`, it has no rayon path at all. Carried over as a well-specified lead
from a prior session's notes (candidates 005/006/008 in
`~/.claude/skills/perf-optimize/projects/dicom-rs.md`), re-implemented fresh here (base commit
`df2b9e4236298efa3fa37a1f360fc4bf284d586a`) - **this re-verification found a real, different
result from what the prior notes described**, including a genuine implementation bug caught and
fixed during this session (see below), so treat the numbers here as authoritative for this box,
not the prior notes' remembered ones.

## What was tried

Added a `#[cfg(feature = "rayon")]` parallel path gated by `rayon_worth_it()` (core-availability
check, `OnceLock`-cached) and a `component_len >= 3_000_000` pixel threshold - below the threshold,
or without the feature/on a single-core host, behavior is unchanged (the original sequential
`zip`/`flat_map`/`collect`).

**First implementation attempt (caught and fixed this session, not carried into the final
patch):** `let mut out = vec![T::default(); data.len()]; out.par_chunks_mut(3).enumerate().for_each(...)`.
This compiled, passed correctness, and *regressed* multi-core performance at 10,000,000 pixels by
a consistent **+12.4% (9/9 paired, `colorspace-fast` mode)** - the opposite of the expected
direction. Root cause: `vec![T::default(); n]` performs a full zero-initialization write pass over
the entire output buffer *before* the parallel loop even starts writing real values - a full extra
memory pass (30MB for this test size) that the original sequential `.collect()` never pays (a
`FromIterator`-based collect doesn't need to pre-zero). **Fixed** by collecting into
`Vec<[T; 3]>` via `(0..component_len).into_par_iter().map(|i| [r[i], g[i], b[i]]).collect()`
(rayon's indexed collect writes each element exactly once, no pre-zeroing) and reinterpreting the
result as `Vec<T>` via `Vec::from_raw_parts` (safe for `T: Copy` - `[T; 3]` has the same
size/alignment as three consecutive `T`s, no padding). This is the version in `patch.diff`.

Bench: `___OPTIMIZATIONS___/harness/bench_pipeline`'s existing `colorspace-fast` mode (same
`to_dynamic_image` call as `colorspace`, but a length-only checksum instead of a full hash - safe
to use here per this workspace's own candidate-003-adjacent finding that `colorspace-fast` can
disagree with `colorspace` at *small* pixel counts, which doesn't apply at the multi-million-pixel
sizes tested here) against three new synthetic `PhotometricInterpretation=RGB`,
`PlanarConfiguration=1` fixtures built by a new harness mode, `gen-planar-rgb` (pseudo-random plane
content, since `interleave()`'s pure-copy work doesn't need photographically realistic data to
exercise correctly - unlike the colorspace-conversion candidates, which needed real/random content
to characterize a *rounding* difference).

| Variant | 2.5M px (below threshold) | 10M px | 20M px |
|---|---|---|---|
| baseline (sequential only) | 4.822 ms | 3.034-3.071 ms (2 rounds) | 22.552 ms |
| patched (buggy `vec![default]` version) | - (not tested below threshold) | 3.358 ms (+12.4% regression) | - (not tested) |
| patched (final, `Vec<[T;3]>` collect) | 4.826 ms (no change) | 2.792-2.825 ms (2 rounds) | 22.559 ms (no change) |

## Result

**Below the 3,000,000px threshold: no consistent change** (+0.1%, 6/9 - not consistent), as
expected since the code path is untouched there. **At 10,000,000 pixels: a real, reproducible
-8.0% multi-core win, 8/9 and 9/9 paired wins across two rounds, consistent both times** - real,
but far smaller than the prior session's remembered "-26.6% at 3.0M scaling to -66.0% at 10.0M".
**At 20,000,000 pixels: no consistent change** (+0.0%, 6/9 - not consistent) - the win does *not*
grow with size as the prior notes described; it appears to be a narrow window rather than a
scaling trend on this box. Single-core (`--no-default-features --features image,native` build, no
rayon linked, `taskset -c 0`): no consistent change at 10M pixels (-0.8%, 5/9 - not consistent), as
expected since the entire threshold/guard block compiles out without the `rayon` feature.

## Does it change behavior/output?

- **Bit-exact?** Yes, in every configuration tested (2.5M, 10M, and 20M-pixel fixtures) -
  `sha256sum` of the decoded RGB output is identical between baseline and patched builds in all
  cases. Only the dispatch/collection mechanism changed, never the values written.
- **Is this a breaking change?** No public API/type changes. The added items
  (`rayon_worth_it`, `INTERLEAVE_RAYON_PIXEL_THRESHOLD`) are private, and `interleave()`'s generic
  bound grew from `T: Copy` to `T: Copy + Default + Send + Sync` - `Default` is no longer actually
  needed by the final implementation (only the first, reverted attempt needed it for
  `vec![T::default(); n]`) but was left in place since both call sites (`u8`, `u16`) satisfy it
  trivially and removing it is a one-line cleanup with no measured effect either way; `Send + Sync`
  are required for the `rayon` parallel path and are likewise trivially satisfied by `u8`/`u16`.
  `interleave` itself is a private function, so none of this is externally observable either way.
- Project test suite: **537 passed / 0 failed / 6 ignored, both before and after** (full
  `cargo test --workspace`, including doctests) - no new failures. The existing `test_interleave`
  unit test (small, sequential-path-only input) passes unchanged.

## Why the result looks the way it does

`interleave()`'s per-element work is a pure 3-byte copy with no arithmetic, so it is almost
entirely memory-bandwidth-bound rather than compute-bound - this is exactly the SKILL.md-flagged
scenario ("a single-core win can vanish if the workload is memory-bandwidth- or lock-bound") that
`taskset`-based single-core numbers can't reveal and that only shows up by testing multiple sizes.
The plausible (not independently profiled this session) explanation for the narrow window: at
2.5M pixels (7.5MB per plane), the sequential version is likely still cache/prefetcher-friendly
enough that rayon's dispatch overhead isn't worth it; at 10M pixels (30MB per plane, larger than
L3 on most consumer/server chips), splitting the *reads* across cores lets more of the total
working set be served by per-core caches simultaneously than one core reading alone; at 20M pixels
(60MB per plane), the working set is large enough relative to total memory bandwidth that multiple
cores reading from three separate large offsets concurrently no longer helps - the operation
becomes bandwidth-saturated regardless of core count, so the fixed dispatch overhead has nothing
left to win against. This is a plausible mechanism, not confirmed with a profiler (no
`perf`/`strace` available on this box, and `valgrind --tool=cachegrind` was not attempted this
session due to bounded time).

## What's not done / open questions

- The exact shape of the win between 3M and 20M pixels (does it hold flat around -8%, peak
  somewhere in between, or something else) was not mapped out - only three points (2.5M, 10M, 20M)
  were tested, enough to establish "narrow window, not a monotonic scaling trend" but not the
  precise boundaries.
- Not independently profiled: the memory-bandwidth-bound hypothesis above is the leading
  explanation for the narrow window, but was not confirmed with `cachegrind`/`massif` or any other
  tool this session - a future session with more time budget could firm this up.
- Given how narrow and modest the measured win is (a single-digit percentage in a window that may
  not even correspond to realistic file sizes - `PlanarConfiguration=1` is a legacy encoding mostly
  seen on smaller secondary-capture images per this workspace's own prior candidate notes), whether
  this complexity (the `unsafe` `Vec::from_raw_parts` reinterpretation, the two extra private
  items) is worth taking for the actual measured benefit is left entirely to the reader - this file
  states only the facts.
- The `Default` bound on `interleave<T>` is vestigial after the final fix (see above) - a one-line
  cleanup (`T: Copy + Send + Sync` would suffice) that was not applied, to keep `patch.diff` an
  exact record of what was actually measured.

## Gotcha

A separate re-derivation of this same candidate (independent session) first tried
`par_iter().zip().zip().flat_map_iter().collect()` (mirroring the original sequential code's
`zip`/`flat_map` shape) instead of the `Vec<[T; 3]>` indexed collect above, and measured it as
8x-36x *slower* than the sequential baseline at 20,000-300,000 pixels, still ~2-3x slower at
1,000,000 pixels. This is not ordinary rayon dispatch overhead - `flat_map_iter`'s parallel
iterator has no known output length, which forces rayon's `collect()` into an unindexed
linked-list-of-`Vec`s strategy instead of a plain indexed collect. General lesson for any future
rayon work on this codebase: prefer an indexed collect (`par_chunks_mut` on a preallocated output,
or `Vec<[T; N]>` via `into_par_iter().map().collect()` as done above) over `flat_map_iter().collect()`
for any fixed-output-size-per-item parallel transform - the two are not performance-equivalent
even though they're semantically similar.
