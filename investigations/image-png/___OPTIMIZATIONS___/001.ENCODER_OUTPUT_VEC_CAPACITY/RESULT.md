# Reserve the compressed-output Vec's capacity instead of growing it from zero

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`.

`Writer::write_image_data` (`src/encoder.rs:835-890`) starts the zlib/fdeflate output sink at
`Vec::new()` for all three compression paths (`NoCompression`, `FdeflateUltraFast`, `Level(level)`)
and lets it grow via `Vec`'s doubling reallocation as compressed bytes are written - every
reallocation copies everything written so far. A stored (uncompressed) DEFLATE block can never
exceed `fdeflate::StoredOnlyCompressor::compressed_size(raw_len)` (the code already computes this
exact bound at line ~859 to decide whether to fall back to the stored encoding), so that bound is a
safe upper-bound capacity to reserve up front for every path, including the `flate2::ZlibEncoder`
path which had no such existing computation. This is the same technique open PR #713 applied only
to the stored-fallback path; this candidate applies it to all four `Vec::new()` call sites
(`src/encoder.rs:838, 846, 866, 880`).

## What was tried

Measured with `harness/encoder_bench --mode corpus --corpus tests/benches` (all 15 real files in
the repo's own bench corpus, default `fdeflate` compression, 15 iterations, 10% warmup discarded),
plus paired alternating-binary runs (`tools/paired.sh`, `RUNS=13`, `taskset -c 0`) on 4
representative files spanning the compression-ratio range.

| Variant | single-core p50 (paired, taskset -c 0) | notes |
|---|---|---|
| baseline (`Vec::new()`) | see paired medians below | |
| patched (`Vec::with_capacity(stored_size / raw_len)`) | see paired medians below | |

Paired timing (`RUNS=13`, alternating A=baseline/B=patched, `taskset -c 0`):

| File | ratio | median A -> B | consistency |
|---|---|---|---|
| kodim02.png (768x512, low compressibility) | 1.78x | 1.834ms -> 1.828ms (-0.3%) | NOT consistent (B faster 2/13) - noise |
| Exoplanet_Phase_Curve_(Diagram).png (3840x2160, high compressibility) | 25.51x | 8.053ms -> 8.099ms (+0.6%) | NOT consistent (B faster 3/13) - noise |
| Lohengrin...png (2812x3982 grayscale, ~no compressibility) | 1.12x | 23.340ms -> 23.422ms (+0.4%) | NOT consistent (B faster 2/13) - noise |
| Exoplanet...indexed_gimp.png (3840x2160 indexed, high compressibility) | 14.45x | 3.399ms -> 3.409ms (+0.3%) | NOT consistent (B faster 3/13) - noise |

An initial unpaired single-shot comparison (baseline corpus run, then separately a patched corpus
run, both `taskset -c 0`) showed an apparent 15-18% speedup on the two largest/most-compressible
files - this did **not** replicate under the proper paired/alternating protocol above, and is
recorded here as a cautionary example of exactly the confound `paired.sh`'s own header warns
about (drift between two separate, non-interleaved process launches), not as a real effect.

**This candidate's own timing signal turned out to be unstable across separate measurement
sessions, and that instability is itself the most trustworthy finding here.** After writing this
candidate's `justfile` and running `just test` (a completely independent build: fresh `git clone`,
fresh `cargo build` of both binaries in `clone/encoder_bench/target/`, not the same binaries used
for the table above), the same paired protocol on the same file (`Exoplanet_Phase_Curve_(Diagram).png`,
`RUNS=13` then `RUNS=21`, `taskset -c 0`) gave the *opposite* read: **consistent, B faster 21/21,
median 8.674ms -> 8.064ms (-7.0%)**. Re-running the paired comparison against the *original*
`/tmp/bin_baseline`/`/tmp/bin_patched` pair immediately afterward, at similar system load
(`uptime` ~1.0-1.25), reproduced the original "NOT consistent" (8/21) result. So: two
independently-built pairs of binaries from the identical source (baseline) and identical patch
(patched), measured with the identical paired/taskset protocol on the identical machine, gave
opposite consistency verdicts for the same file. The most likely explanation is build-to-build
codegen non-determinism (parallel codegen-unit scheduling, or something in how `rustc`/LLVM lays
out this hot loop) rather than filesystem/page-cache warm-up (both binaries in each pair were built
and measured back-to-back the same way). This was not chased further (out of scope for a single
bounded candidate - see "What's not done" below), but it means **no wall-clock claim in either
direction for this candidate should be trusted without independently re-measuring on the exact
binaries in hand**, and it's a caution worth carrying into every other candidate in this
workspace: a single paired-run session, however disciplined, may not be reproducible session-to-session
on this machine.

Allocation counters (from the harness's `GlobalAlloc` wrapper, one process per corpus file, not
subject to timing noise - `reallocs` and `alloc_MiB` are exact per-run counts):

| File | ratio | reallocs before -> after | alloc_MiB before -> after |
|---|---|---|---|
| Exoplanet_Phase_Curve_(Diagram).png | 25.51x | 20 -> 5 | 3.54 -> 25.62 |
| Exoplanet...indexed_gimp.png | 14.45x | 21 -> 7 | 1.93 -> 9.02 |
| Fantasy_Digital_Painting.png | 2.60x | 21 -> 5 | 8.19 -> 11.21 |
| Lohengrin...png (grayscale) | 1.12x | 23 -> 5 | 32.25 -> 29.69 |
| Transparency.png | 76.34x | 12 -> 5 | 0.02 -> 0.35 |
| kodim02.png | 1.78x | 19 -> 5 | 2.10 -> 2.40 |
| kodim07.png | 1.90x | 19 -> 5 | 2.02 -> 2.31 |
| kodim17.png | 1.70x | 19 -> 5 | 2.15 -> 2.45 |
| kodim23.png | 1.97x | 19 -> 5 | 1.97 -> 2.27 |
| lorem_ipsum_oxipng.png (indexed) | 4.15x | 19 -> 7 | 0.44 -> 0.71 |
| lorem_ipsum_screenshot.png | 5.70x | 18 -> 5 | 1.09 -> 2.58 |
| paletted-zune.png (indexed) | 2.54x | 23 -> 7 | 6.78 -> 7.88 |
| tango-...-128-rsvg-convert.png | 5.02x | 13 -> 5 | 0.04 -> 0.09 |
| tango-...-16.png | 1.45x | 9 -> 5 | 0.00 -> 0.00 |
| tango-...-32.png | 2.12x | 11 -> 5 | 0.01 -> 0.01 |

`peak_rss_MiB` was flat (52.0 -> 52.2, within measurement noise) across every file in both runs -
the larger `alloc_MiB` figures on highly-compressible files reflect a large *virtual* reservation
(most of which is never written to, since actual output is far smaller than the stored-block upper
bound) rather than resident memory growth; glibc's malloc routes allocations of this size through
`mmap`, which doesn't commit physical pages until touched.

## Result

**Time: not reliably determined - the signal itself was unreproducible across measurement
sessions on this machine, see above.** Three of four representative files, and the first
measurement of the fourth (Exoplanet), read as "NOT consistent" under the paired/taskset protocol
(true delta within noise). A later, independently-built pair of binaries for that same fourth file
read as a strong, internally-consistent -7.0% (21/21 pairs). Re-testing the original binaries
immediately after reproduced the original "NOT consistent" read. This candidate cannot responsibly
claim either "no time change" or "a real time win" - the honest statement is that wall-clock timing
for this patch was not reproducible session-to-session here, most likely due to build-to-build
codegen variance rather than a real, patch-attributable effect. An earlier *unpaired* single-shot
comparison suggesting a 15-18% win (see above) is a separate, weaker artifact and should be
disregarded entirely.

**Allocation count: real, consistent win.** `reallocs` drops from 18-23 (baseline, every file) to a
flat 5 (7 for indexed images, which allocate one extra buffer) - a 65-78% reduction in reallocation
calls, and therefore in the number of times the whole output-so-far gets copied. This is exact
(process-internal atomic counters), not subject to timing noise.

**Allocated bytes: mixed, and this is the real trade-off of this patch.** For low-compressibility
images (ratio close to 1x, e.g. Lohengrin) `alloc_MiB` drops slightly (32.25 -> 29.69 MiB, -8%) -
the upfront reservation is close to what would eventually be needed anyway, without doubling
overshoot. For highly-compressible images (ratio > ~10x, e.g. the two Exoplanet files, Transparency)
`alloc_MiB` increases substantially (3.54 -> 25.62 MiB on the largest file, +624%) because the
patch reserves the full stored-block upper bound regardless of how well the image is expected to
compress - it has no way to know ahead of time. `peak_rss_MiB` does not reflect this (stayed flat),
because the excess capacity is reserved but never written, so it's not resident - but a
memory-constrained embedder measuring virtual address space or accounting allocator-reported bytes
(not RSS) would see this as a real regression on highly-compressible inputs.

## Does it change behavior/output?

- **Bit-exact?** yes - `verify=OK` on every iteration, every file, both before and after (the
  harness re-decodes and byte-compares against the source pixels, and checks output-hash
  determinism across iterations).
- **Is this a breaking change?** No - `write_image_data`'s signature, return type and encoded
  output bytes are unchanged; only the growth strategy of an internal buffer changed.
- Project test suite (`cargo test --release`): **107 passed, 1 ignored, 0 failed**, identical
  before and after (91+1+4+3+8 across the 5 test binaries/doctests).

## Why the result looks the way it does

The mechanism this patch targets - eliminating `Vec`'s doubling-reallocation chain - is real and
measured directly (the allocation counters, which are exact and not subject to timing noise). What
it buys in wall-clock terms could not be pinned down here: DEFLATE/fdeflate compression work likely
dominates total encode time enough that saving ~15-18 `memcpy`s of a buffer that tops out well
under a few tens of MB is close to this machine's measurement floor - close enough that build-to-
build codegen variance (see above) swamps whatever real effect exists, in either direction. The
allocation-count win is real and unconditional regardless. The capacity strategy chosen (the
stored-block upper bound) also introduces a real, measured cost of its own in allocated-bytes terms
on highly-compressible images, independent of the timing question - see "What's not done" below.

## What's not done / open questions

- **The build-to-build timing non-reproducibility noted above was not chased down.** It would need
  several more independent `cargo build --release` + paired-run cycles (ideally with
  `CARGO_INCREMENTAL=0`, a pinned `-C codegen-units=1`, and checking whether `RUSTFLAGS` or system
  load correlates with which way it flips) to tell apart "real but tiny effect drowned by codegen
  variance" from "no effect at all, purely codegen variance both times." Worth doing before trusting
  a wall-clock number from *any* candidate in this workspace to better than a few percent - added to
  IDEAS.md as its own investigation.
- The capacity heuristic used (`stored_size`, i.e. an upper bound assuming near-zero compression)
  is deliberately simple and matches the code's existing fallback-decision computation, but it is
  the worst case for the common case of photos/graphics that compress well. A tighter estimate
  (e.g. `raw_len / 4` with one guaranteed extra doubling in reserve, or scaling by a rough
  per-color-type ratio) would likely keep most of the reallocation-count win while avoiding the
  large `alloc_MiB` increase seen on Transparency.png/Exoplanet - not attempted here, left as a
  follow-up idea in IDEAS.md rather than bundled into this candidate (one fix per candidate).
- Not measured: multi-frame APNG (`write_image_data` is also the per-frame encode path) - the
  existing IDEAS.md backlog has a separate entry for per-frame scratch-buffer reuse that likely
  interacts with this change; re-check both together if that candidate is picked up later.
- Not measured on a multi-core/unpinned run - single-core paired comparisons were sufficient to
  establish "no consistent time change," and this patch does not touch any parallelism-relevant
  code.
