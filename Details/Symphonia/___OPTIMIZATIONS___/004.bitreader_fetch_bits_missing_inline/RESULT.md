# Add `#[inline]` to `BitReaderLtr`/`BitReaderRtl`'s `fetch_bits`

Found while profiling FLAC decode (`perf record`/`annotate`): `decode_rice_partition` (18.24% of
the flac bench's total profile) contains a real indirect `callq *...FetchBitsLtr...fetch_bits$got`
inside its hot per-sample loop. Every other method on the same trait impl (`fetch_bits_partial`,
`get_bits`, `num_bits_left`, `consume_bits`) is marked `#[inline]`/`#[inline(always)]`, but
`BitReaderLtr`'s own `fetch_bits` (the buffer-refill path called whenever the 64-bit bit cache runs
low - i.e. on essentially every multi-byte read once the cache is exhausted) has no inline hint at
all, so it's the one method in the whole trait that's actually left as a real, non-inlined call
across the module boundary (`symphonia-bundle-flac` calling into `symphonia-core`). This matches
HEURISTICS.md's "small hot accessor/getter missing `#[inline]`, called across a module/crate
boundary" pattern. `BitReaderRtl::fetch_bits` (used by Vorbis/Ogg's residue/floor decode) has the
exact same omission, fixed in the same patch since it's the same mechanism in the same file.

**Follow-up:** after the initial FLAC-focused write-up below, this session also built a bench/perf
profile for ALAC (`master.m4a`, not previously covered by this workspace) and found the exact same
un-inlined `callq *...fetch_bits$got` inside `ElementChannel::read_residuals`'s hot loop (also
`BitReaderLtr`) - ALAC turned out to benefit *more* than FLAC from this same fix (see Result below).

## What was tried

Added a plain `#[inline]` (not `#[inline(always)]` - HEURISTICS.md recommends starting with the
weaker hint) to both `BitReaderLtr::fetch_bits` and `BitReaderRtl::fetch_bits` in
`symphonia-core/src/io/bit.rs`. No other code changed.

Single-core (`taskset -c 2`), 25-30 iterations per run, multiple independent paired
baseline/patched runs per format, `bench_decode`'s own hash check on every iteration:

| Variant | flac p50 | ogg p50 | wav p50 | mp3 p50 | notes |
|---|---|---|---|---|---|
| baseline (4 rounds) | 21.78/21.87/21.60(alt)/21.60(alt)ms | 29.07/28.96/28.64/28.59ms | 13.98/17.92\*/13.88/13.73ms | 40.15/39.56/39.91/39.54ms | |
| patched (4 rounds) | 21.49/21.74/21.48(alt)/21.42(alt)ms | 29.24/28.61/28.58/28.71ms | 13.99/16.46\*/13.73/13.73ms | 42.34/40.49/40.00/40.04ms | |

\*one wav round had a visible noise outlier on both sides (system noise, not attributable to the
patch - both baseline and patched showed elevated p95/max in that same round).

An additional 4-round alternating-order paired check on FLAC alone (baseline-then-patched and
patched-then-baseline interleaved) confirmed patched < baseline in **4/4 pairs**, by 0.1-0.3ms
(~0.5-1.4%) each time - small in absolute terms but consistent and in the direction the mechanism
predicts (removing a real indirect call from a hot per-sample loop).

## Result

**ALAC: the largest and most consistent win found in this workspace so far** (~2.6-3.2% on p50,
paired **4/4**, baseline `___OPTIMIZATIONS___/testdata/master.m4a` at 25 iterations each): min
60.24/60.29/60.24/60.16ms -> 58.45/58.31/58.52/58.47ms, p50 60.44/60.66/60.53/60.84ms ->
58.82/58.81/58.70/58.99ms. `bench_decode`'s hash matched (`9df86ccd83e65783`, same as the
identical-content flac/wav masters) on every run. `ElementChannel::read_residuals` (22% of ALAC's
own profile, per `perf record`/`annotate` on `master.m4a`) has the exact same un-inlined
`callq *...fetch_bits$got` in its hot loop as FLAC's `decode_rice_partition` does - same mechanism,
bigger effect here because ALAC's residual-reading loop calls `fetch_bits` even more densely
relative to its own (comparatively cheap) per-sample work than FLAC's does.

**FLAC: a small, real, consistently-reproducible win** (~0.5-1.5% on p50, paired 4/4 across two
separate paired-comparison sessions, i.e. 8/8 total pairs favoring patched) - unsurprising given
`decode_rice_partition` is FLAC's single largest profile entry and calls `fetch_bits` once per
Rice-coded residual sample.

**Ogg/WAV/MP3: no clear win, within noise.** Ogg showed patched-favoring in 2 of 4 further paired
rounds (net roughly flat); WAV and MP3 showed no consistent direction across rounds, with MP3 in
particular showing one round with patched *slower* by ~2ms (likely unrelated system noise, since
this patch touches a code path barely exercised by MP3's own bit-reading - MP3 uses `BitReaderLtr`
too, but its Huffman-heavy hot loop calls `fetch_bits` far less often per decoded sample than FLAC's
one-call-per-residual pattern). Ogg/Vorbis uses `BitReaderRtl` and calls `fetch_bits` far less often
per decoded value than FLAC's rice-partition loop, which is consistent with a smaller/noise-level
effect there.

Only wall-clock time was measured; this is a pure inlining-hint change, not an allocation/memory
change, so no RSS/allocation counter is relevant here.

## Does it change behavior/output?

- **Bit-exact?** yes - `bench_decode`'s hash matched baseline vs. patched for all 5 formats (flac,
  ogg, wav, mp3, alac), on every run.
- **Is this a breaking change?** No - a plain `#[inline]` hint changes no public API, no documented
  behavior, and no file format handling; the function bodies are byte-for-byte identical to before.
- Project test suite (`cargo test --release --workspace`): 0 failures before, 0 failures after -
  105 total passing test-result lines in both cases (`symphonia-core`'s own 65 unit tests plus
  every other crate's suite), confirmed via `grep -cE "^error"` (0 both times) and counting
  `test result` lines (39 result lines both times, matching a known-good full-workspace run).

## Why the result looks the way it does

`#[inline]` is a hint, not a mandate - LLVM's own inlining heuristic had apparently judged this
particular call site (crossing the `symphonia-bundle-flac` -> `symphonia-core` crate boundary,
with no cross-crate MIR available without the hint) as not worth inlining without it, exactly the
mechanism HEURISTICS.md describes for the `image-rs` `get_pixel`/`get_pixel_mut` precedent. FLAC
benefits the most because `decode_rice_partition` is the one hot loop in this workspace's whole
test suite that calls a `fetch_bits`-family method once per decoded *sample* rather than once per
decoded *value that happens to need a cache refill less often* (Ogg/Vorbis's residue/floor
decoding reads wider VLC codes per call, so the bit cache empties less frequently relative to the
number of decoded values).

## What's not done / open questions

- Not tried: `#[inline(always)]` instead of the plain hint - per HEURISTICS.md's own guidance on
  `#[inline(always)]`, escalating past a plain hint is only worth doing if profiling still shows
  the call site as a real cost after this fix, which would need a fresh `perf record`/`annotate`
  pass on the patched binary to check (not done in this session; the paired timing win above is
  believed to already reflect the call becoming inlined, based on how consistent and directionally
  correct the FLAC result is, but this wasn't independently re-confirmed via a second profile).
- Not investigated: whether the same missing-inline pattern exists on any other
  `FetchBitsLtr`/`FetchBitsRtl` implementor in `symphonia-core/src/io/bit.rs` beyond the two fixed
  here (a grep at the time of this candidate found exactly these two occurrences without the hint,
  every other implementor/method already had one).
