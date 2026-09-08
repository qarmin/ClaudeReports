# What is measured here, and with what

Read this first if picking the work up cold - it explains the harnesses and the environment;
`SUMMARY.md` lists the candidates measured with them, `IDEAS.md` is the backlog.

Project: `png` (image-rs/image-png), a pure-Rust PNG/APNG decoder and encoder. Workspace created
2026-09-02, base commit `05ccdb9c8ae7fc50f4787ba2888a1c515adad944`.

## Harnesses

### The project's own (reuse these first)

| harness | invocation | covers |
|---|---|---|
| `benches/decoder.rs` | `cargo bench --bench decoder` | decoding `tests/benches/*.png`, generated non-compressed images at 4 KiB / 64 KiB / 2 GiB IDAT sizes, and a `row-by-row` group driving `read_row` |
| `benches/unfilter.rs` | `cargo bench --bench unfilter --features benchmarks` | `unfilter` in isolation, per filter type and bpp |
| `benches/adam7.rs` | `cargo bench --bench adam7 --features benchmarks` | `expand_pass` (Adam7 deinterlacing) |
| `benches/expand_paletted.rs` | `cargo bench --bench expand_paletted --features benchmarks` | palette expansion into RGB8/RGBA8 |
| `examples/corpus-bench.rs` | `cargo run --release --example corpus-bench -- <dir>` | decoding a directory of real files |

**Forgetting `--features benchmarks` on the last three silently leaves them unbuilt** - they go
through `src/benchable_apis.rs`, which only exists under that feature.

### `harness/encoder_bench` (built by this workspace, 2026-09-02)

The repo ships nothing at all for the encoder, and most of this workspace's backlog is
encoder-side, so this harness fills that gap. It is one binary with modes - extend it with a new
mode or flag when a new question comes up rather than starting a second encoder bench.

```
cd ___OPTIMIZATIONS___/harness/encoder_bench
cargo build --release
./target/release/encoder_bench --help

# everything, synthetic 512x512 RGBA source
./target/release/encoder_bench --size 512 --iters 20

# one mode against a real file
./target/release/encoder_bench --mode filter --input ../../../tests/benches/kodim02.png

# every real file in the repo's own bench corpus
./target/release/encoder_bench --mode corpus --corpus ../../../tests/benches
```

Modes: `filter` (one run per `Filter` variant), `compress` (one per `DeflateCompression`), `color`
(one per color type/bit depth), `apng` (`--frames` frames via `write_image_data`), `stream`
(`StreamWriter` at several buffer sizes), `decode-apng` (the one decode path `benches/decoder.rs`
does not cover), `corpus` (re-encode every `*.png` in a directory), `memory` (one per
`DeflateCompression`, encoding into `std::io::sink()` instead of a `Vec` - added for candidate 017;
see "Measuring the encoder's *own* peak RSS" below before using this one).

What each line reports: wall time (min/p50/mean/p95/max, first 10% of iterations discarded as
warmup), encoded size and ratio, throughput, **allocation count, reallocation count and total
allocated bytes**, and process peak RSS. Output lines are `RESULT ` + `key=value` throughout, so
`tools/*.sh` can parse them without touching the program.

Things to know before trusting a number from it:

- **Correctness is checked on every iteration**, not once: the encoded bytes must hash identically
  across iterations (determinism), and the output is decoded back and compared byte-for-byte with
  the source pixels. A configuration that fails prints `verify=FAIL(...)`; treat its timing as
  meaningless. All 15 files in `tests/benches/` currently round-trip `verify=OK`, including the
  indexed and grayscale ones.
- **Allocation counting is always on** (a `GlobalAlloc` wrapper around `System`, two relaxed
  atomics per allocation). For an encoder doing tens of allocations per image this is far below
  the noise floor, but it is not free - if a future candidate is timing something allocation-heavy,
  say so rather than assuming the counter is invisible.
- **`peak_rss_MiB` is a process-wide high-water mark**, so it only distinguishes configurations
  when the harness runs one at a time - use `--only <substring>` for that.
- **Measuring the encoder's *own* peak RSS needs `--mode memory`, not `compress`/`apng`.** Every
  other mode encodes into a `Vec<u8>` (needed for the hash/roundtrip check), so the harness's own
  output sink ends up holding the whole encoded PNG in memory regardless of whether `png::Writer`
  itself streamed its compressed output or buffered it internally - this fully masked candidate
  017's own encoder-internal buffering change (`compress --only Level6` reported the *same*
  `peak_rss_MiB` before and after a patch that measurably changed the encoder's own peak RSS by
  ~40% at 4096x4096). `memory` mode encodes into `std::io::sink()` instead, so its `peak_rss_MiB`
  reflects only what the encoder itself holds. Always pass `--skip-verify` with it: `memory`
  mode's own correctness check normally does one `Vec`-sink encode first, which - since RSS is a
  monotonic high-water mark for the whole process - permanently sets the peak before the
  sink-based measurement loop even runs. Verify correctness separately, in its own process, via
  `compress`/`apng` mode on the same configuration instead.
- **The synthetic `photo` pattern is not a stand-in for a real image.** On it, flate2 `Level1`
  produced a *smaller* file than `Level3` (717,652 vs 745,843 B) - an artifact of that pattern's
  structure, not a `png` bug; the same comparison on `kodim02.png` is monotonic as expected. Use
  `--input`/`--corpus` for anything where the ratio matters.
- The harness pins nothing: use `taskset -c N` for ranking runs, and check `uptime` first, per
  `MEASUREMENT.md`.
- **`tests/benches/` is narrower than it looks**: all 15 files are 8-bit and non-interlaced (color
  types 0/2/3/6 only) - no 16-bit, no Adam7, no sub-byte depths, no APNG (checked 2026-09-02 by
  reading every IHDR). So `benches/decoder.rs` and `--mode corpus` both leave the interlacing,
  bit-depth-expansion and animation paths completely unmeasured; use the harness's synthetic
  `color`/`apng`/`decode-apng` modes (or `benches/adam7.rs --features benchmarks`) for those.

Copying it into a candidate's `clone/` requires rewriting one path (its `Cargo.toml` points at
`../../..`, the main checkout):

```
cp -r ../harness/encoder_bench clone/encoder_bench
sed -i 's|path = "../../.."|path = ".."|' clone/encoder_bench/Cargo.toml
```

## Baseline numbers (2026-09-02)

Machine: AMD Ryzen 9 7900 (12 cores / 24 threads), Linux, rustc 1.98.0, release build with
`debug = true`, unpinned, load average ~0.6. Not a ranking-quality measurement - a calibration
snapshot, so a future session knows roughly where the time goes before it starts.

**Encoding `tests/benches/kodim02.png` (768x512 RGB8, 1.18 MB raw), p50:**

| configuration | time | output | note |
|---|---|---|---|
| `NoCompression` | 0.77 ms | 1,180,318 B | filters forced off |
| `FdeflateUltraFast` + Adaptive | 1.88 ms | 663,722 B | what `Compression::Fastest`/`Fast` select |
| `Level(1)` | 8.41 ms | 681,335 B | |
| `Level(3)` | 26.8 ms | 645,471 B | |
| `Level(6)` | 69.0 ms | 631,220 B | **the default** (`Compression::Balanced`) |
| `Level(9)` | 97.5 ms | 628,941 B | `Compression::High` |

**The single most important calibration fact:** `Encoder`'s default is `Compression::Balanced` ->
`DeflateCompression::Level(6)`, which is **~37x slower than `FdeflateUltraFast`** for ~5% smaller
output. On the default path, deflate is ~97% of encode time and *everything else in the encoder is
rounding error*. Any candidate about filtering, buffer growth or per-row copies is therefore a
candidate about the **fast** paths (`Fastest`/`Fast`/`FdeflateUltraFast`), and its `RESULT` lines
must say which compression setting they were measured at.

**Filtering cost, same image, all at `FdeflateUltraFast`, p50:**

| filter | time | output |
|---|---|---|
| `Avg` | 1.185 ms | 685,096 B |
| `Up` | 1.209 ms | 730,707 B |
| `Sub` | 1.211 ms | 713,665 B |
| `Paeth` | 1.326 ms | 674,872 B |
| `Adaptive` (default) | 1.541 ms | 663,722 B |
| `MinEntropy` | 2.905 ms | 666,988 B |
| `NoFilter` | 1.869 ms | 1,180,318 B |

So adaptive filtering costs ~0.33 ms over a single fixed filter here - roughly **20% of encode
time in the fast path** - to buy 1.6% smaller output. `MinEntropy` costs 88% more than `Adaptive`
and produced a *larger* file on both this image and the synthetic one; nothing measured so far
justifies its cost.

**Allocation behavior (this is where the clearest wins are):** every buffered encode grows its
output `Vec` from zero. Measured reallocation counts per single encode: 19 for a 1 MB RGBA image,
23 for the 11 MB `Lohengrin` grayscale file (which churned **32.25 MiB of allocation to produce a
9.96 MB output**, ~3.2x amplification), 30-31 when the output is incompressible. Eight-frame APNG
encoding: 25 allocations and **120 reallocations**, 13.08 MiB allocated for an 8 MB source. The
`StreamWriter` path allocates far less (8-14 reallocations) because it never holds the whole
compressed stream.

## Ground rules used in this workspace

1. Every number in a `RESULT.md` says which harness, which compression setting, and which source
   image produced it - `Level(6)` and `FdeflateUltraFast` numbers are not comparable.
2. Bit-exact output verification on every candidate (the harness does it per iteration), plus the
   project's own `cargo test` before and after.
3. The main working tree is always reverted; `harness/encoder_bench/` is the one thing in this
   workspace that is *meant* to be edited and kept.
4. Crashes or correctness bugs noticed while measuring go to `___BUGHUNT___/`, not here.
