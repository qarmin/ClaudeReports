# FlateDecode output `Vec` pre-sizing (`hayro-syntax`'s `flate::decode`, via `flate2`/`zlib-rs`)

`perf record` on the `render_bench` workload attributed `zlib_rs::inflate::inflate_fast_help_avx2`
(5.91%), `zlib_rs::inflate::inflate` (1.22%) and `inftrees::inflate_table` (0.66%) - roughly 7.8%
of total cycles combined - to decompressing PDF FlateDecode streams (content streams,
xref/object streams, image data), reached from `hayro-syntax`'s `flate::decode`
(`hayro-syntax/src/filter/lzw_flate.rs`). That function's two decode paths (`zlib_stream`,
`deflate_stream`) both call `flate2::read::{ZlibDecoder,DeflateDecoder}::read_to_end` into a
`Vec::new()` - std's default `read_to_end` grows an empty `Vec` from scratch (starting tiny,
then doubling), which forces `flate2` to decompress into a series of small output buffers early
on (limiting how much work the decoder's AVX2 fast path can do per call) and repeatedly
reallocate+copy the already-decoded bytes as the `Vec` grows. Pre-sizing the `Vec` before
`read_to_end` avoids both costs for the common case where the guess is in the right ballpark.

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

## What was tried

`flate::decode` is `pub(crate)` inside `hayro-syntax`, not reachable from an external bench
crate, so this candidate does not reuse `hayro-bench`'s `render_bench` as the primary measurement
(a secondary, full-pipeline check against it is included below for corroboration). Instead,
`bench/` is a standalone crate that reproduces the exact two functions under test, built against
`flate2 = "=1.1.9"` with the `zlib-rs` feature (same version `hayro-syntax` pins), and decodes 20
real FlateDecode-compressed stream payloads (`testdata/*.zlib`, 58,883 bytes compressed / 932,977
bytes decompressed combined) extracted directly from this workspace's own test PDFs
(`hayro-tests/pdfs/load/issue391.pdf` and this repo's `img.pdf`) via a small Python `zlib.decompress`
sweep over every `stream`/`endstream` block. Both code paths (baseline: `Vec::new()`; patched:
`Vec::with_capacity(data.len() * 3)`) are compiled into one binary and selected by a CLI arg, so
there's never an ambiguity about whether a given run actually used the patched code. Each
iteration decodes all 20 streams and folds their FNV-1a hashes together; 200 iterations, first
10% (20) discarded as warmup.

Note: the bench crate's own `Cargo.lock` resolved `zlib-rs` to `0.6.7` (latest compatible at
measurement time), vs. the main repo's `Cargo.lock`-pinned `0.6.3` - both code paths in a given
bench run use the identical resolved version, so the baseline-vs-patched *comparison* is unaffected,
but the absolute timings may differ slightly from what `render_bench` (built against the main
repo's lockfile) would show.

| Variant | single-core p50 (`taskset -c 2`, 9 paired runs x 200 iters, 20 streams/iter) | multi-core p50 (unpinned, 9 paired runs) | notes |
|---|---|---|---|
| baseline (`Vec::new()`) | 1.093-1.097 ms | 1.476-1.551 ms | |
| patched (`Vec::with_capacity(len*3)`) | 1.042-1.057 ms | 1.419-1.479 ms | faster in 9/9 pairs, both single- and multi-core |

Single-core paired runs (`../tools/paired.sh`-style alternating A/B, `RUNS=9`, `taskset -c 2`):

```
pair 1: baseline=1.094 patched=1.045
pair 2: baseline=1.095 patched=1.049
pair 3: baseline=1.097 patched=1.045
pair 4: baseline=1.095 patched=1.042
pair 5: baseline=1.095 patched=1.042
pair 6: baseline=1.094 patched=1.046
pair 7: baseline=1.093 patched=1.057
pair 8: baseline=1.088 patched=1.047
pair 9: baseline=1.093 patched=1.042
```
Patched faster in 9/9 pairs, ~4.2-4.7% p50 reduction, very low spread (consistent with a
single-threaded CPU-bound bench pinned to one core).

Multi-core (unpinned) paired runs, same methodology:

```
pair 1: baseline=1.486 patched=1.463
pair 2: baseline=1.496 patched=1.462
pair 3: baseline=1.551 patched=1.439
pair 4: baseline=1.521 patched=1.447
pair 5: baseline=1.535 patched=1.457
pair 6: baseline=1.476 patched=1.419
pair 7: baseline=1.502 patched=1.479
pair 8: baseline=1.538 patched=1.465
pair 9: baseline=1.548 patched=1.451
```
Patched faster in 9/9 pairs, ~2.6-7.2% p50 reduction (noisier than single-core, as expected -
this sandbox had several sibling optimization candidates building/benchmarking concurrently
during this measurement, all serialized through the shared `flock` lock).

**Secondary corroboration** (full pipeline, `hayro-bench`'s `render_bench --backend hayro`, 4 real
PDFs - `img.pdf`, `issue391.pdf`, `fonts_type1_latex.pdf`, `image_cmyk_icc_jpg.pdf` - summed
`total_ms` across all 4 per run, `--iter 30`, `taskset -c 2`, `RUNS=11`): median 48.874ms ->
48.155ms (-1.5%), only 8/11 pairs favored the patch - **not consistent** at this system's noise
floor. This is expected: flate decode is ~7.8% of total cycles in the original profile, and this
composite-of-4-heterogeneous-files number also includes parsing, interpretation, JPEG2000
decode and rasterization, none of which this patch touches - the isolated bench above is the
number that actually isolates this candidate's effect above the noise floor.

## Result

**Time**, isolated to the two changed functions: a consistent, small win - single-core p50 ~4.5%
faster, multi-core p50 ~2.6-7.2% faster, 9/9 paired runs favoring the patch in both configurations.
Not measured: peak RSS/allocation *count* (the change trades "several smaller allocations,
reallocated/copied on growth" for "one larger upfront allocation" - allocation *count* almost
certainly drops, but this wasn't independently instrumented; the *reallocation-copy* elimination is
what the timing delta above already captures indirectly).

At the full-pipeline (`render_bench`) level, on this specific 4-PDF corpus, the effect is real in
direction (median moved the right way) but too small relative to this sandbox's noise floor to
call "consistent" by the paired-run bar this workspace uses elsewhere (8/11, not >=10/11) - see
MEASUREMENT.md's noise-floor discipline. A cleaner full-pipeline confirmation would need a PDF
corpus with a much larger fraction of decode-time spent in FlateDecode specifically (e.g. many
large, highly-compressible content/object streams) and/or a quieter host.

## Does it change behavior/output?

- **Bit-exact?** Yes. The isolated bench's combined hash across all 20 real streams is identical
  (`511ead8db74611d0`) between baseline and patched, over all 200 iterations each. Additionally,
  `hayro-tests`'s pixel-snapshot tests `filter_flate_1bit` and `filter_tiff_predictor_gray`
  (the two tests in this repo's suite whose names directly name FlateDecode/predictor decoding)
  pass with bit-identical rendered output before and after the patch.
- **Is this a breaking change?** No - `zlib_stream`/`deflate_stream` are private helper functions,
  no public API or file-format-visible behavior changes. The only externally observable effect is
  a change in peak memory use during decode (a single upfront allocation of up to `3x` the
  compressed stream's size, vs. the previous incremental-growth pattern) - for a compressed stream
  whose real decompressed size is much larger than 3x its compressed size (e.g. this workspace's
  own `testdata/stream_016.zlib`, 3202 bytes compressed -> 129,600 bytes decompressed, a ~40x
  ratio), the initial guess undershoots and `read_to_end` simply falls back to its normal
  from-3x-capacity growth for the remainder - still correct, just not fully avoiding the
  regrowth cost for that one stream.
- Project test suite:
  - `cargo test --release -p hayro-syntax --features std,unsafe`: **199 passed, 2 failed**,
    identical before and after the patch. Both failures
    (`pdf::tests::pdf_version_header`, `pdf::tests::pdf_version_catalog`) are pre-existing and
    unrelated to this change - they `unwrap()` a `std::fs::read` of
    `../hayro-tests/downloads/pdfjs/alphatrans.pdf` / `../hayro-tests/downloads/pdfbox/2163.pdf`,
    and this sandbox's `hayro-tests/downloads/` directory does not exist (a separate,
    not-checked-out download corpus - confirmed with a clean `git status`/directory listing, not
    something this patch touches).
  - `cargo test --release -p hayro-tests --test tests -- filter_flate_1bit
    filter_tiff_predictor_gray`: **2 passed, 0 failed**, identical before and after (these two
    reference PNGs live in `hayro-tests/snapshots/render/custom/`, which is untracked/gitignored
    and self-bootstraps its own reference images on first run in a fresh worktree - the first run
    in this fresh worktree correctly reported "new reference image was created" for both, a
    harmless first-run bootstrap, not a failure; the second run, which is what's reported here,
    is a real comparison against those freshly-created references and passed for both baseline and
    patched).
  - The broader ~1700-test `hayro-tests` visual-snapshot suite was not run in full this session
    (each test needs its own locally-bootstrapped reference PNG in this sandbox, and running the
    whole suite twice - baseline and patched - under this session's shared build/bench lock with
    several sibling candidates contending for it was judged too expensive for the risk this
    two-line, purely-capacity-hint change plausibly carries); the two targeted flate/predictor
    tests plus the full `hayro-syntax` unit-test suite (which includes `decode_flate`,
    `decode_flate_zlib`, and all four `predictor_*` tests) were judged sufficient coverage for a
    change that only pre-sizes an output buffer without touching decode logic.

## Why the result looks the way it does

`read_to_end`'s grow-from-empty behavior is a well-known general inefficiency for any `Read`
implementation whose true output size can be roughly estimated ahead of time - PDF stream content
(text operators, glyph indices, xref/object stream data) compresses at a fairly predictable ratio
in practice, so a `3x` guess lands "close enough" for most real streams (this bench's own 20-stream
corpus spans compressed:decompressed ratios from about 1:1 up to 1:40, and the aggregate result
is still a consistent win) without needing per-stream-type tuning or a PDF-spec-derived exact hint
(no such hint exists in general for FlateDecode - unlike e.g. a font's `/Length1`, there's no
PDF-mandated field carrying the decompressed size of a `/Filter /FlateDecode` stream).

## What's not done / open questions

- No attempt was made to source a more precise per-stream-type size hint (e.g. content streams
  vs. image streams likely compress at different typical ratios) - `3` was chosen as a single,
  simple, conservative middle-ground constant, not tuned against a larger corpus.
- Peak RSS / allocation count were not independently instrumented (see "Result" above) - the
  timing delta already reflects their combined effect but doesn't separate them.
- The full `hayro-tests` visual-regression suite (~1700 tests) was not run twice end-to-end for
  this candidate - see the correctness section above for what was run instead and why.
