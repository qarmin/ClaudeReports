# What is under test, and how it is measured

Read this first if picking the work up cold - it explains the harness, `SUMMARY.md` lists the
candidates measured with it.

## Workload

The project (image-rs/jpeg-decoder) is a pure-Rust baseline+progressive JPEG decoder. The hot
path under test is `jpeg_decoder::Decoder::decode()` - parsing + Huffman entropy decode + IDCT +
upsampling + color conversion, ending in a flat `Vec<u8>` of decoded pixel data.

Workload images (all already present in the main repo, reused as-is - no new test data was
added):

| image | what it stresses |
|---|---|
| `benches/tower.jpg` (512x512, baseline, ~70KB) | small baseline decode, fixed-cost-dominated |
| `benches/tower_progressive.jpg` (512x512, progressive) | progressive scan re-parsing overhead |
| `benches/tower_grayscale.jpg` (512x512, grayscale) | single-component path, no upsampling/color-convert |
| `benches/large_image.jpg` (2268x1512, baseline RGB, ~500KB) | mid-size realistic photo decode |
| `tests/reftest/images/lossless/1/jpeg_lossless_sel1-rgb.jpg` (3072x2048, ~4.5MB) | large image, dominates wall time, best signal-to-noise for ranking |

## Measurement harness

`___OPTIMIZATIONS___/harness/bench_decode/` - a standalone Cargo crate (own `Cargo.toml`, own
`[workspace]` so it never gets pulled into the main crate's workspace), depending on jpeg-decoder
via a path dependency. It is **not** part of the tracked jpeg-decoder repo; kept as a single
canonical copy here and reused by every candidate (see `SKILL.md` "Bench harnesses: grow a few,
don't multiply" and "When the project's own benchmark harness is untracked in the main repo").

```
bench_decode <input_file> [iters=30] [expected_hash_hex] [--info]
```

- Decodes (or, with `--info`, just reads header info) `iters` times, timing each iteration with
  `Instant::now()`.
- Hashes the output (FNV-1a) every iteration; aborts immediately if any iteration's hash differs
  from iteration 0, or from `expected_hash_hex` if given - this is the correctness check baked
  into every single bench run, not a separate step.
- Discards the first ~10% of iterations as warmup, then prints `min`/`p50`/`mean`/`p95`/`max` in
  ms plus every raw per-iteration timing in microseconds (`raw_us = [...]`).
- Uses `std::hint::black_box` on both the input reference and the decoded output so the compiler
  can't eliminate the timed work.

### Why not the existing `benches/decoding_benchmark.rs` / `benches/large_image.rs` (criterion)?

These exist and do reach the right code paths, but criterion's own harness (statistical
resampling, its own warmup/measurement-time budgeting, `--save-baseline`/`--baseline` diffing)
makes running dozens of quick A/B candidate comparisons slower and harder to script against than
a plain "print min/p50/raw" binary. `bench_decode` is deliberately simpler and pairs directly with
`tools/paired.sh` (alternating A/B invocation counting, per `MEASUREMENT.md` section 2b) instead.
The criterion benches remain available and untouched in the main repo if a candidate ever wants a
second opinion with a different statistical method.

### Instrumentation added for this workspace

`bench_decode` itself (see above) is the only instrumentation added. No changes were made to the
jpeg-decoder crate's own source to build it - it only depends on the crate via its public API
(`jpeg_decoder::Decoder`).

**For a candidate's own `justfile` (clone-based, standalone-postable)**: copy the canonical
harness in and fix up its path dependency, since the harness's `Cargo.toml` points at the main
project via `path = "../../.."` (correct only when run from
`___OPTIMIZATIONS___/harness/bench_decode/` directly against the live checkout):

```
clone:
    rm -rf clone
    git clone {{repo_url}} clone
    cd clone && git checkout {{base_commit}}
    cp -r "{{justfile_directory()}}/../harness/bench_decode" clone/bench_decode
    sed -i 's#path = "\.\./\.\./\.\."#path = ".."#' clone/bench_decode/Cargo.toml

build-baseline:
    cd clone/bench_decode && cargo build --release
    cp clone/bench_decode/target/release/bench_decode ./bin_baseline

# apply patch.diff to clone/ (the jpeg-decoder checkout, NOT clone/bench_decode) here, then:
build-patched:
    cd clone/bench_decode && cargo build --release
    cp clone/bench_decode/target/release/bench_decode ./bin_patched

bench:
    ./bin_baseline clone/benches/large_image.jpg 30
    ./bin_patched  clone/benches/large_image.jpg 30
```

### Characterized-difference tooling

`bench_decode` currently aborts on any hash mismatch - by design, since almost every candidate is
expected to be bit-exact. If a candidate intentionally changes output (e.g. a faster approximate
IDCT), add a temporary `--dump <path>` flag to `bench_decode` that writes the decoded bytes to a
file (in addition to hashing), run it once per side, then use `tools/diffbytes.py` to get a
max-delta / affected-fraction characterization. Revert the `--dump` addition afterward - it's
scratch instrumentation for that one candidate, not a permanent harness feature, unless a later
session finds it's needed repeatedly.

## Environment

- CPU: AMD Ryzen 9 7900 (12 cores / 24 threads), Linux 7.0.0-30-generic.
- `perf` is **not installed** on this machine (`perf_event_paranoid` is -1, i.e. would be usable,
  but the binary itself is missing) - discovery relies on code reading against `HEURISTICS.md`
  plus in-code phase timers (`std::time::Instant` spans wrapped around suspect sections,
  temporarily, reverted before the candidate's tree is reverted) rather than `perf record`.
- The project pins `rust-toolchain` to exactly `1.61.0` (its MSRV) - that toolchain is what
  `cargo build` inside the main project's own directory uses by default (rustup override).
  For benchmarking, build with `cargo +stable` instead (a modern stable toolchain, already
  installed via rustup) to get realistic release-optimized numbers representative of how most
  consumers of this crate actually build it - MSRV compliance of a candidate's own diff is a
  separate, secondary concern (don't introduce syntax/APIs newer than 1.61 in `patch.diff` if it
  can be avoided, but don't let it block measuring the performance question).
- Default build uses the `rayon` feature (on by default in `Cargo.toml`) - i.e. the harness as
  built above measures the parallel-decode path unless a candidate explicitly builds with
  `--no-default-features` to isolate the single-threaded path.

## Ground rules used in this workspace

1. No claim without a command that reproduces it - every candidate folder has the numbers and the
   exact command.
2. Every candidate that touches real source gets a real `patch.diff`, applied and built for real,
   plus the project's own tests where they cover the code.
3. Null results are written down with the same care as wins - a null result with no file is
   indistinguishable from "never checked".
4. If a later candidate changes the baseline enough that an earlier candidate's numbers no longer
   hold, don't silently rewrite that candidate's `RESULT.md` - add a short `RECHECK_<reason>.md`
   or `CORRECTION.md` in its folder explaining what changed and why, and update its `SUMMARY.md`
   row to point at it. History stays readable that way.
