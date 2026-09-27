# Ratio-aware initial capacity for the encoder's compressed-output buffer

Follow-up to `001.ENCODER_OUTPUT_VEC_CAPACITY`. That candidate reserved the full stored-block
upper bound (`fdeflate::StoredOnlyCompressor::compressed_size(raw_len)`) up front for all three
`write_image_data` compression paths, which cut reallocation count but over-reserved badly on
highly-compressible images (up to +624% `alloc_MiB`). This candidate instead reserves
`raw_len / 4` for the two paths whose eventual size is unknown in advance
(`FdeflateUltraFast`'s main compressor and `Level(level)`'s `ZlibEncoder`) - matching fdeflate's
own `compress_to_vec` heuristic (`fdeflate-0.3.7/src/compress.rs:265`,
`Vec::with_capacity(input.len() / 4)`) - and leaves `NoCompression` (and `FdeflateUltraFast`'s
own stored-fallback branch, taken only when compression didn't help) reserving the exact
`stored_size`, since that path's output size is known to be close to that bound already, not a
guess.

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`. Base commit
`05ccdb9c8ae7fc50f4787ba2888a1c515adad944`.

## What was tried

Measured with `harness/encoder_bench --mode corpus --corpus tests/benches` (all 15 real files in
the repo's own bench corpus, default `fdeflate` compression, 15 iterations, 10% warmup discarded),
plus paired alternating-binary runs (`tools/paired.sh`, `RUNS=15-21`, both `taskset -c 0` and
unpinned) on 5 representative files spanning the compression-ratio range.

Allocation counters (exact atomic counters, one process per file, not subject to timing noise):

| File | ratio | reallocs: baseline / 001 (stored_size) / 014 (ratio-aware) | alloc_MiB: baseline / 001 / 014 |
|---|---|---|---|
| Exoplanet_Phase_Curve_(Diagram).png | 25.51x | 20 / 5 / 5 | 3.54 / 25.62 / 7.82 |
| Exoplanet...indexed_gimp.png | 14.45x | 21 / 7 / 7 | 1.93 / 9.02 / 3.08 |
| Fantasy_Digital_Painting.png | 2.60x | 21 / 5 / 6 | 8.19 / 11.21 / 8.04 |
| Lohengrin...png (grayscale, ~no compression) | 1.12x | 23 / 5 / 7 | 32.25 / 29.69 / 29.69 |
| Transparency.png | 76.34x | 12 / 5 / 5 | 0.02 / 0.35 / 0.10 |
| kodim02.png | 1.78x | 19 / 5 / 7 | 2.10 / 2.40 / 2.40 |
| kodim07.png | 1.90x | 19 / 5 / 7 | 2.02 / 2.31 / 2.31 |
| kodim17.png | 1.70x | 19 / 5 / 7 | 2.15 / 2.45 / 2.45 |
| kodim23.png | 1.97x | 19 / 5 / 7 | 1.97 / 2.27 / 2.27 |
| lorem_ipsum_oxipng.png (indexed) | 4.15x | 19 / 7 / 7 | 0.44 / 0.71 / 0.35 |
| lorem_ipsum_screenshot.png | 5.70x | 18 / 5 / 5 | 1.09 / 2.58 / 1.15 |
| paletted-zune.png (indexed) | 2.54x | 23 / 7 / 8 | 6.78 / 7.88 / 5.67 |
| tango-...-128-rsvg-convert.png | 5.02x | 13 / 5 / 5 | 0.04 / 0.09 / 0.04 |
| tango-...-16.png | 1.45x | 9 / 5 / 7 | 0.00 / 0.00 / 0.00 |
| tango-...-32.png | 2.12x | 11 / 5 / 6 | 0.01 / 0.01 / 0.01 |

`001`'s and baseline's numbers above are re-quoted from `001.ENCODER_OUTPUT_VEC_CAPACITY/RESULT.md`
for comparison (re-verified: this session's own baseline run reproduced them exactly, e.g.
Exoplanet 20 reallocs / 3.54 MiB, Lohengrin 23 / 32.25). `peak_rss_MiB` was flat across baseline/001/014
in this session's runs too (~51-52 MiB), same "large virtual reservation, not resident" explanation
as candidate 001.

Paired wall-clock timing (`tools/paired.sh`, alternating baseline/014-patched, both single-core
`taskset -c 0` and unpinned multi-core), **first build** (binaries built directly in the worktree
used for the allocation-counter runs above, `/tmp/bin_baseline_014` / `/tmp/bin_patched_014`):

| File | ratio | single-core (`taskset -c 0`), RUNS | multi-core (unpinned), RUNS |
|---|---|---|---|
| Exoplanet_Phase_Curve_(Diagram).png | 25.51x | 8.063ms -> 8.613ms (+6.8%), 21/21 consistent | 8.461ms -> 9.065ms (+7.1%), 15/15 consistent |
| Exoplanet...indexed_gimp.png | 14.45x | 3.480ms -> 3.627ms (+4.2%), 15/15 consistent | not run |
| Transparency.png | 76.34x | 0.072ms -> 0.078ms (+8.3%), 15/15 consistent (absolute: +6us) | not run |
| paletted-zune.png (indexed) | 2.54x | 6.336ms -> 6.328ms (-0.1%), 12/15 - NOT consistent | not run |
| Lohengrin...png (grayscale, ~no compression) | 1.12x | 23.645ms -> 23.558ms (-0.4%), 11/15 - NOT consistent | not run |
| kodim02.png | 1.78x | 1.821ms -> 1.823ms (+0.1%), 5/15 - NOT consistent | 1.863ms -> 1.866ms (+0.2%), 5/15 - NOT consistent |

**This did not replicate.** Running `just test` (a fully independent `git clone` + fresh
`cargo build --release` of both binaries, different `md5sum` from the pair above) and re-running
the identical paired protocol on the identical file, `taskset -c 0`, gave the opposite read:
`RUNS=15`: 8.253ms -> 8.286ms (+0.4%), 8/15 - **NOT consistent**; `RUNS=21`: 8.417ms -> 8.441ms
(+0.3%), 9/21 - **NOT consistent**. Immediately re-running the *original* `/tmp/bin_baseline_014`/
`/tmp/bin_patched_014` pair again, same file, `taskset -c 0`, `RUNS=21`, reproduced the original
read: 8.262ms -> 8.810ms (+6.6%), 0/21 - consistent. So, exactly as candidate 001 documented on
this same machine (and, by coincidence, the same file - `Exoplanet_Phase_Curve_(Diagram).png`):
two independently-built pairs of binaries from the identical source+patch give opposite
consistency verdicts, and each pair's own verdict is individually stable when re-run. This is
build-to-build codegen variance, not a patch-attributable effect.

## Result

**Allocation count: real win, matches candidate 001 closely.** `reallocs` drops from 18-23
(baseline) to 5-8 (this patch) - the same 60-70% reduction candidate 001 achieved, with 1-2 more
reallocations than 001 on several files (e.g. Fantasy 5->6, paletted-zune 7->8, kodim* 5->7)
because `raw_len/4` sometimes undershoots the final size by more than one doubling, unlike 001's
exact-upper-bound guess.

**Allocated bytes: substantially improved over candidate 001, the goal this candidate targeted.**
On the two highest-ratio files, `alloc_MiB` overshoot drops from 001's +624%/+1650% (Exoplanet
3.54->25.62, Transparency 0.02->0.35) to +121%/+400% here (3.54->7.82, 0.02->0.10) - both still
over baseline, but far smaller in absolute and relative terms, and Transparency's absolute
overshoot is 0.08 MiB either way. On moderate-ratio files the new guess tracks the final size more
closely than 001's fixed upper bound and in three cases (Fantasy 11.21->8.04, lorem_ipsum_screenshot
2.58->1.15, paletted-zune 7.88->5.67) `alloc_MiB` is now *lower* than 001's, in one case
(paletted-zune 6.78->5.67) even lower than baseline. Low-ratio Lohengrin is identical to 001
(29.69, both approaches converge once the guess and the stored bound are both close to the actual
output).

**Time: not reliably determined - reproduces candidate 001's own finding that wall-clock timing on
this machine is not reproducible session-to-session, this time on the exact same file.** The first
build's binaries showed an internally consistent (single- and multi-core, 15-21/21 pairs) +4% to
+8% slowdown on the three highest-ratio files (Exoplanet 25.51x, Exoplanet-indexed 14.45x,
Transparency 76.34x - the last only ~6 microseconds in absolute terms) and no consistent
difference on moderate/low-ratio files (paletted-zune 2.54x, Lohengrin 1.12x, kodim02 1.78x). A
second, independently-built pair of binaries from the identical source and identical patch (via
`just test`) showed **no consistent difference at all** on the same file that showed the strongest
first-build signal (Exoplanet, 8/15 and 9/21 - both NOT consistent), while the original pair,
re-tested immediately after, reproduced its original consistent +6.6% read. Like candidate 001,
this candidate cannot responsibly claim a real timing delta in either direction - the two
independently-built pairs disagree with each other, and each is individually stable when re-run.
Unlike candidate 001 (where the *inconsistent* pair was the norm and one *consistent* reading was
the outlier), here the first-built pair was the one that read consistently in both directions
tested (single- and multi-core) and only the fresh justfile-built pair broke that consistency -
underscoring that "internally consistent across several protocol variants within one build" is not
by itself sufficient evidence on this machine; a second independent build is required before
trusting any timing number here, for any candidate.

## Does it change behavior/output?

- **Bit-exact?** yes - `verify=OK` on every iteration, every file, both before and after (the
  harness re-decodes and byte-compares against the source pixels, and checks output-hash
  determinism across iterations). Encoded output bytes are byte-identical to baseline on every
  corpus file (only capacity, not content, changed).
- **Is this a breaking change?** No - `write_image_data`'s signature, return type and encoded
  output bytes are unchanged; only the growth strategy of an internal buffer changed.
- Project test suite (`cargo test --release`): **107 passed, 1 ignored, 0 failed**, identical
  before and after (91+1+4+3+8 across the 5 test binaries/doctests) - same counts as candidate 001.

## Why the result looks the way it does

The allocation-count and allocated-bytes mechanism works exactly as intended: `raw_len/4` is a much
closer guess to the final compressed size than the full stored-block upper bound for most real
images (compression ratios in this corpus range 1.1x-76x, so `raw_len/4` undershoots badly only
above ~4x, and even then by less than 001's bound overshoots), while still cutting most of the
`Vec` doubling-reallocation chain that the unpatched baseline pays.

The wall-clock question could not be settled either way: one build's binaries showed an internally
consistent (single- and multi-core, matching direction and magnitude across protocol variants)
regression on the highest-ratio files, but a second, independently-built pair of binaries from the
identical source and patch showed no such thing on the same file, while the first pair's own
reading reproduced when re-tested immediately after. This is the same build-to-build codegen
variance candidate 001 already documented on this machine (coincidentally, on the same file). No
`perf`/`strace` instrumentation was used to look for a mechanism (e.g. whether `raw_len/4`'s
~6 MiB allocation for Exoplanet crosses glibc's mmap threshold while baseline's doubling growth
mostly doesn't) precisely because the effect itself was not established as real - there is nothing
confirmed yet to explain. This reinforces candidate 001's own conclusion (see that candidate's
`RESULT.md`) that no wall-clock number on this project should be trusted without an independent
second build, and extends it: even a result that looks solid *within* one build (agreeing across
single-core, multi-core, and 15-21 run repeats) is not sufficient - the disagreement here is
between builds, not between runs.

## What's not done / open questions

- **The apparent timing regression was not confirmed as real, and therefore its mechanism was not
  investigated.** A future session wanting to settle this would need several more independent
  `cargo build --release` + paired-run cycles (as candidate 001's own "what's not done" already
  proposed, e.g. `CARGO_INCREMENTAL=0`, pinned `-C codegen-units=1`) to tell apart "real effect
  sometimes masked by codegen variance" from "no effect, purely codegen variance in both
  directions." If a real effect is eventually confirmed, the mmap-threshold hypothesis above is a
  reasonable starting point (test a range of `raw_len/N` divisors, or `Vec::with_capacity` sizes
  straddling glibc's default 128 KiB mmap threshold on a synthetic image sized to land exactly
  there) but is currently speculation, not a finding.
- This candidate's allocation-count/allocated-bytes evidence is unaffected by the above - those are
  exact atomic counters from a single process run, not wall-clock measurements, and were
  additionally cross-checked (baseline counts reproduced exactly against candidate 001's own
  recorded numbers) in this session.
- Not measured: multi-frame APNG (`write_image_data` is also the per-frame encode path, same as
  noted in candidate 001) and multi-core timing for files other than Exoplanet (kodim02 was
  checked as a not-consistent control; the other three representative files were only measured
  single-core).
- A different, untried heuristic: scale the initial guess by color type/bit depth (e.g. a lower
  fixed ratio for palette/grayscale images, which tend to compress better, per this corpus's own
  ratio spread) rather than one constant `raw_len/4` divisor for every color type - left as an
  IDEAS.md follow-up rather than bundled into this candidate.
