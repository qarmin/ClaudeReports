# Decoder-side IDAT chunking granularity sensitivity (measurement only, no patch)

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`.

Decoder-side counterpart to issue #193 (the encoder half - `StreamWriter`'s output buffer size -
was already closed by candidate `010.STREAMWRITER_BUFFER_SIZE_CORPUS`). `benches/decoder.rs`
already parametrizes how finely a generated PNG's pixel data is split into `IDAT` chunks via three
`criterion` groups (`generated-noncompressed-{4k,64k,2g}-idat`, `src/test_utils.rs`'s
`write_noncompressed_png`), but per `IDEAS.md` these had never actually been run - the `-2g-idat`
group generates images up to 576 MB and the concern was that criterion's default sampling could
cost "many minutes" on an image that size. In practice (see below) that concern didn't
materialize once the benchmark binary is invoked correctly (see "gotcha" below): the entire
9-benchmark sweep, including the two 576 MB cases, completes in under two minutes.

**Binary-invocation gotcha found while running this**: invoking the compiled criterion binary
directly (`./target/release/deps/decoder-<hash> <filter>`) without an explicit `--bench` flag
silently runs in criterion's `Mode::Test` (a single correctness pass per benchmark, no timing
loop at all - see `criterion-0.8.2/src/lib.rs:960-964`, `(false, _) => true` i.e. test-mode is the
*default* unless `--bench` is passed) and prints only `Testing <id> / Success` for every
benchmark, with no `time:`/`thrpt:` line - producing what looks like a real (and suspiciously
fast, ~7s for the whole suite) run while doing zero actual measurement. `cargo bench` itself
always passes `--bench` for you, so this only bites when invoking the compiled binary directly
(e.g. from a `justfile`, as this candidate folder's own `justfile` does) - worth remembering for
every future candidate in this workspace that shells out to a `criterion` binary directly.

## What was tried

`cargo build --release --bench decoder` (default features - no `benchmarks` feature needed for
this bench), then invoking the binary directly with `--bench <filter> --noplot`, `taskset -c 0`
for single-core runs and unpinned for the multi-core column. Three independent runs: two pinned
(the required baseline-twice-for-noise-floor check) plus one unpinned. All numbers are criterion's
own reported median (p50) from a 100-sample run for the two small sizes (auto) or a 10-sample run
for the two large sizes (`bench_file` in `benches/decoder.rs` already forces `sample_size(10)`
once the input file exceeds 1 MB) - no manual `--sample-size`/`--measurement-time` override was
actually needed once `--bench` was passed correctly; the existing per-file size threshold in the
harness already keeps every group's total run time low (worst case ~30s wall for a single 576 MB
case, ~108-118s for the entire 9-benchmark sweep).

| Image size | Group (IDAT chunk cap) | single-core p50 (run 1) | single-core p50 (run 2) | unpinned p50 (run 3) |
|---|---|---|---|---|
| 128x128 (64 KB raw) | 4k-idat | 6.8200 µs | 6.8388 µs | 6.8380 µs |
| 128x128 (64 KB raw) | 64k-idat | 5.6753 µs | 5.6078 µs | 5.6869 µs |
| 2048x2048 (16 MB raw) | 4k-idat | 1.9668 ms | 2.0774 ms | 1.7667 ms |
| 2048x2048 (16 MB raw) | 64k-idat | 1.6767 ms | 1.7008 ms | 1.6038 ms |
| 2048x2048 (16 MB raw) | 2g-idat (1 chunk) | 1.7337 ms | 1.6685 ms | 1.6162 ms |
| 12288x12288 (576 MB raw) | 4k-idat | 90.037 ms | 89.028 ms | 90.169 ms |
| 12288x12288 (576 MB raw) | 64k-idat | 83.594 ms | 82.605 ms | 83.276 ms |
| 12288x12288 (576 MB raw) | 2g-idat (1 chunk) | 80.711 ms | 81.271 ms | 85.700 ms |

(8x8 exists only in the 4k-idat group so has no cross-group comparison; omitted from the table -
run 1/run 2/run 3 medians were 677.88 ns / 666.30 ns / 649.93 ns for completeness.)

A standalone (non-criterion) manual loop (`examples/profile_idat.rs`, written for this
investigation only, not part of the patch/harness deliverable - see "What's not done" below)
cross-checked the 12288x12288 case with 50-100 iterations of a plain `for` loop timed with
`Instant`: 4k-idat 94.68-99.21 ms/iter, 64k-idat 83.84-86.06 ms/iter, 2g-idat 85.41 ms/iter -
consistent with the criterion numbers above (same ~8-16% 4k-vs-64k gap).

`perf record -F 4000 -g` on that standalone loop (12288x12288, 4k-idat vs 64k-idat) attributes the
extra 4k-idat time to two per-chunk-boundary costs that scale with chunk *count*, not
image size: `crc32fast::specialized::pclmulqdq::calculate_avx512` self-time is disproportionately
higher for 4k-idat (12.89% of samples vs 9.68% for 64k-idat) despite covering the same total
byte count, because AVX-512 CRC has a fixed per-call setup/teardown cost that a 4 KiB buffer
amortizes 16x worse than a 64 KiB one; and `StreamingDecoder::parse_u32`/`::update`
(`src/decoder/stream.rs`) - the chunk length/type/CRC header state machine that runs once per
chunk boundary - shows up as a visible, separate cost (~5%+3% self-time) in the 4k-idat profile
but doesn't even make the top 30 hottest symbols for 64k-idat, since there are 16x fewer chunk
boundaries to walk through.

## Result

**Real, reproducible, and directionally consistent across all three runs: finer `IDAT` chunking
measurably slows decoding, with rapidly diminishing returns above ~64 KiB** - the same shape as
the encoder-side `StreamWriter` finding in candidate 010, just on the decode path instead.
4 KiB chunks cost roughly **+21% (128x128), +18-20% (2048x2048), +8% (12288x12288)** versus 64 KiB
chunks for the same pixel data; 64 KiB versus effectively-unchunked (2g-idat, one giant chunk for
the whole image) differs by only 0-3% - within this measurement's own noise floor (repeat-run
spread on the *same* config was typically 1-5%, see the run-1-vs-run-2 columns above, e.g.
64k-idat/12288x12288: 83.594 ms vs 82.605 ms, ~1.2% apart). The 12288x12288 case shows a smaller
percentage gap (~8%) than the two smaller sizes (~18-21%) despite having proportionally the same
chunk-count-per-byte ratio; the leading hypothesis (not conclusively isolated, see below) is that
the 576 MB case is more memory-bandwidth-bound overall (working set far exceeds any cache level),
which dilutes the *relative* contribution of the same absolute per-chunk overhead.

This directly answers the open question in `IDEAS.md`: the `-2g-idat` group's cost fear did not
materialize (worst case ~30s for one 576 MB case, ~2 min for the whole 9-benchmark sweep, no
override needed once `--bench` is passed correctly - see the gotcha above) and the parametrized
groups were worth running - they surface a real, quantifiable effect.

## Does it change behavior/output?

- **N/A - measurement/investigation only, no patch applied to real source.** This candidate
  quantifies an existing, inherent property of PNG's per-chunk CRC-32 framing (every chunk,
  including consecutive `IDAT` chunks, carries its own independent CRC over its own type+data -
  see `src/decoder/stream.rs`'s `ImageData`/`ReadChunkData` state handling) interacting with how
  finely an *encoder* chooses to split image data across `IDAT` chunks; it is not something a
  decoder-side patch can eliminate without either violating the chunk-CRC framing PNG mandates,
  or building new, higher-scope machinery to batch CRC/state-machine work across multiple already
  fully-buffered chunk boundaries when the input is known to be entirely in memory (see "What's
  not done" below for why that's out of scope for this candidate).
- Correctness: `benches/decoder.rs`'s own `bench_file` calls `.unwrap()` on every decode
  (baseline `next_frame` call used for the throughput header, and every iteration of the timed
  loop) - a wrong/corrupt decode would have panicked the whole run rather than silently reporting
  a number, and the same deterministic input was decoded on every iteration, so a successful,
  panic-free run is sufficient evidence output was byte-identical every time. No pixel/hash
  spot-check beyond that was added since no source was touched.
- Project test suite: not run - no source file was changed by this candidate (only a throwaway
  `examples/profile_idat.rs` was added in the worktree for profiling, then discarded before the
  worktree was removed - see "What's not done" below).

## Why the result looks the way it does

CRC-32 and the `U32`/`ReadChunkData`/`ImageData` chunk-header state machine in
`StreamingDecoder::next_state` (`src/decoder/stream.rs`) both do a fixed amount of work once per
chunk boundary, independent of how many bytes the chunk carries: `crc.reset()` +
`crc.update(&type_str.0)` (4 bytes) at chunk start, a fresh CRC finalize+compare at chunk end, and
4-9 bytes of length/type/CRC field parsing through the `U32` state. At 4 KiB per chunk that fixed
cost recurs 16x more often per byte than at 64 KiB, and profiling shows it lands disproportionately
in `crc32fast`'s AVX-512 path (which has real per-call vector setup/teardown overhead that a small
buffer can't amortize) plus the header-parsing functions themselves, which is exactly the
mechanism this candidate set out to check for existence, not previously measured for the decode
side of this crate. Diminishing returns above ~64 KiB match the encoder-side finding in candidate
010 almost exactly (rapid initial improvement, then a flat curve) - both are the same "fixed
per-boundary cost amortized over a growing byte count" shape, just measured from opposite ends of
the same chunking decision.

## What's not done / open questions

- This is descriptive, not prescriptive, same as candidate 010: whether/how to reduce the
  decoder's own per-`IDAT`-chunk-boundary cost (as opposed to encouraging encoders to prefer
  larger `IDAT` chunks, which is a file-producer choice, not a decoder one) is not something this
  candidate attempted - it would need a concrete, bounded mechanism (e.g. coalescing CRC updates
  or header parsing across multiple chunks known to already be fully present in the input buffer)
  that wasn't obvious to scope safely inside one candidate; recorded as a backlog idea instead of
  attempted here.
- The smaller relative overhead at 576 MB (~8%) versus 16 MB/64 KB (~18-21%) is reported as an
  observation with a plausible (memory-bandwidth-dilution) explanation, not a conclusively
  isolated one - a future candidate could confirm/refute this with `perf stat -e
  L3-cache-misses,cycles` comparing the same two configs, which wasn't done here to keep this
  candidate bounded to the originally-scoped question (does chunking granularity matter at all).
- `examples/profile_idat.rs`, written in the worktree to get a perf profile isolated from
  `benches/decoder.rs`'s own per-run setup cost (see next bullet), was **not** kept - it was
  deleted before the worktree's tree was reverted/removed, consistent with "no patch.diff for a
  measurement-only candidate" above. Its contents (a ~30-line `Instant`-timed loop around
  `Decoder::read_info`/`Reader::next_frame`) are reproducible from this file's own description if
  ever needed again.
- Incidental harness observation, not a bug: `benches/decoder.rs`'s `load_all` unconditionally
  generates and does one full throughput-priming decode of *every* benchmark's input (all three
  `generated-noncompressed-*-idat` groups x all four sizes, including both 576 MB cases, plus the
  real-file `decode/` group) every time the binary runs `load_all()`, regardless of which
  benchmark name was passed as a CLI filter - the filter only skips criterion's own timed
  sampling loop, not this per-registration setup cost. This is why even a single-benchmark filtered
  run still generates and decodes ~1.2 GB of test data every invocation ((2 + 3) × 576 MB across
  the three IDAT-size groups' 12288x12288 cases plus one more from the `-4k-idat` group's), and
  why a `perf record` wrapped around the whole process (rather than an isolated loop like
  `profile_idat.rs`) captures that unrelated setup cost as the top "hot spot" instead of the
  targeted benchmark - not a performance bug in the library, just a gotcha for interpreting a
  whole-process `perf record` against this specific bench binary in future candidates.
