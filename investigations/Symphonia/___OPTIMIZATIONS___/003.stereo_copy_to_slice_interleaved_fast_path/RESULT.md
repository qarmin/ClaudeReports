# Specialize `copy_to_slice_interleaved` for the 2-channel (stereo) case

Found while investigating the WAV decode profile in session 1: `perf annotate` on
`symphonia_core::audio::util::copy_to_slice_interleaved::<i16, f32, ...>` (7% of the WAV bench's
total profile) showed a near-scalar instruction pattern (`pinsrw`/`psrad`/`cvtdq2ps` processing
what looks like one lane at a time) even though the surrounding code already uses the "copy planes
in pairs" strategy the function's own comments describe as "substantially faster". This function is
the shared, generic implementation behind every decoder's public `copy_to_slice_interleaved` API
(`AudioBuffer`/`GenericAudioBufferRef`/`AudioBufferSlice`, used directly by every format's decode
path including this workspace's own `bench_decode.rs` harness), so any win here applies to every
codec, not just one.

## What was tried

The existing "pairs" loop already specializes for exactly 2 remaining planes as its steady-state
case, but `num_planes` (the destination interleave stride, via `dst.chunks_exact_mut(num_planes)`)
is a runtime value throughout - LLVM cannot assume a fixed chunk size for the *general* N-channel
version of this function, so even though the pair-loop body only ever writes 2 elements per
iteration, the store pattern into `dst.chunks_exact_mut(num_planes)` has to stay generic over an
unknown stride. Added a new, separate branch at the top of the function: when `num_planes == 2`
exactly (checked once, outside any loop), the interleaved write uses `dst.chunks_exact_mut(2)`
with the `2` as a literal, giving LLVM a compile-time-constant stride to vectorize against, then
returns early. The original generic pairs/single-plane loops are otherwise untouched and still
handle mono, 3+ channel, and any future channel count exactly as before.

Single-core (`taskset -c 2`), 20 iterations per run, multiple independent paired
baseline/patched runs (alternating, not batched) per format, `bench_decode`'s own hash check
comparing every iteration:

| format | baseline p50 (4 rounds) | patched p50 (4 rounds) | delta |
|---|---|---|---|
| wav  | 16.22\*/13.36/13.38/13.31ms | 12.83/12.83/12.80/12.73ms | **~-4% to -5%** (excluding the one noisy baseline round) |
| flac | 21.13/21.43/21.15ms | 20.91/20.96/20.92ms | **~-1% to -2%** |
| mp3  | 38.60/38.71ms | 37.86/38.53ms | ~-2% (round 1), ~flat (round 2, within noise) |
| ogg  | 28.47/28.50ms | 28.05/27.45ms | **~-1.5% to -3.5%** |

\*round 1's baseline wav run had a visible noise outlier (p95 21.9ms) - excluded from the summary
range above, full raw numbers are in this candidate's own transcript/justfile output.

A `perf record`+`annotate` re-check after the patch confirmed the mechanism: with the patch
applied, `copy_to_slice_interleaved::<i16, f32, ...>`'s share of the WAV bench's total profile
dropped from 7.09% to 3.33% (same 300-iteration profiling run parameters both times) - the function
itself got measurably cheaper, not just "moved" elsewhere in the profile.

## Result

WAV shows the clearest, most confidently-real win (~4-5%, consistent across 4 independent paired
rounds after discarding one visibly noisy baseline run) since PCM decode's own work is otherwise
minimal and the copy-out step is a proportionally larger share of its total time. FLAC/Ogg show
smaller (~1-3%) but repeatable improvements. MP3 shows a small win in one round and an
inconclusive/noise-level result in the other - unsurprising, since MP3's DSP kernels (candidate
001/002) dominate its profile far more than the copy-out step does. Every format's `bench_decode`
hash matched the pre-patch hash on every iteration of every run. Only wall-clock time was measured;
this change doesn't allocate differently (no allocation/RSS counter relevant here - same buffers,
same sizes, just a different instruction sequence for the copy itself).

## Does it change behavior/output?

- **Bit-exact?** yes - `bench_decode`'s hash matched across all iterations, all 4 formats, and
  separately confirmed via one-off runs against a synthetic mono file and a synthetic 5-channel
  file (both took the *unchanged* generic fallback path, since neither has exactly 2 channels) -
  same hash before and after the patch for both (`d58231f0c24d13d0` mono, `5838202d9dbafab7`
  5-channel), confirming the new branch doesn't accidentally get taken for non-stereo input and
  that the fallback path itself was untouched by the edit.
- **Is this a breaking change?** No - `copy_to_slice_interleaved`'s signature, semantics, and every
  caller are unchanged. This is purely an internal implementation specialization (an early-return
  fast path added before the pre-existing generic loops, which are otherwise byte-for-byte
  identical to before).
- Project test suite: 106 passed / 0 failed both before and after (same counts as
  `___OPTIMIZATIONS___/README.md`'s baseline).

## Why the result looks the way it does

`chunks_exact_mut(n)` with `n` a runtime value forces the auto-vectorizer to treat the destination
stride as unknown, which defeats the kind of wide, batched store LLVM could otherwise emit for a
known-constant stride of exactly 2 (interleaving two `f32` output lanes back-to-back is a
textbook `unpcklps`/wide-store pattern once the stride is a compile-time literal) - the
`perf annotate` disassembly at baseline showed exactly this: single-lane `pinsrw` inserts building
up an `xmm` register piecemeal instead of a batched shuffle+store. Monomorphizing a
`num_planes == 2` literal-stride version removes that ambiguity for the overwhelmingly common
stereo case without touching the correctness-critical generic path at all. The magnitude tracks
"how much of this codec's total decode time is the copy-out step" - large for WAV (little else
happens), small-but-real for FLAC/Ogg, negligible for MP3 (dominated by its own DSP cost, see
candidates 001/002).

## What's not done / open questions

- Only the 2-channel case was specialized, per the function's own existing comment ("may be
  extended to copy planes in triplets... binary will become bloated") - a 1-channel (mono) fast
  path was considered but not added, since the existing "single plane" loop already has a
  compile-time-unknown-but-trivial (single write per iteration) shape that didn't show the same
  disassembly problem in a quick check; not measured separately this round.
- The mirror function `copy_from_slice_interleaved` (the inverse direction - interleaved input to
  planar output) has the exact same shape (generic `num_planes`-stride chunking in its own
  pairs-loop) and very plausibly has the same fix available - not implemented or measured this
  round, left as a backlog item since it's a separate, symmetrical change.
- `copy_bytes_interleaved` (further down the same file, used for byte-level sample export) was not
  inspected for the same pattern.
- MP3's result was inconclusive across the two rounds run (small win / flat) - not chased further
  since MP3's own DSP cost so heavily dominates its profile that this copy-out change is a minor
  factor for that codec specifically; the win is real for WAV/FLAC/Ogg regardless.
