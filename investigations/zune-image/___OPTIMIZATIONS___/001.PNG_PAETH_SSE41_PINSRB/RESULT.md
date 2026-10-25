# PNG Paeth SSE4.1 defilter: avoid per-byte pinsrb/pextrb chain

`crates/zune-png/src/filters/sse4.rs`, `de_filter_paeth_sse41_inner<const SIZE: usize>` (called
with `SIZE` in {3, 4, 6, 8} from `crates/zune-png/src/filters/de_filter.rs:162-171`). This was the
single largest hot spot found profiling PNG decode (`perf record -F 999 --call-graph dwarf`,
`taskset -c 2`/`-c 3`, AMD Ryzen 9 7900, decoding `test-images/png/benchmarks/speed_bench.png` via
`zune_png::PngDecoder::decode_raw()` in a loop): `de_filter_paeth_sse41_inner::<3>` alone was
58.02%/57.44% of total cycles across two repeat runs on different cores. `perf annotate` showed the
cost concentrated in `pinsrb`/`pextrb`/`movd` sequences building/extracting a single SIZE-byte pixel
into/out of a 128-bit register, one byte at a time, per loop iteration - i.e. per-pixel SIMD
domain-crossing overhead, not the Paeth arithmetic itself.

Per-pixel Paeth reconstruction is inherently serial (each pixel's `a`/left neighbor is the previous
iteration's just-reconstructed output), so full row-parallel vectorization wasn't attempted here -
this candidate targets only the per-iteration load/store overhead around the existing one-pixel-
per-iteration SSE4.1 kernel.

rustc 1.90.0 (1159e78c4 2025-09-14), cargo 1.90.0 (840b83a10 2025-08-15).

## What was tried

The original code wrote each SIZE-byte pixel into a 16-byte stack array (`f`/`g`), then did a full
128-bit `_mm_loadu_si128` from that array to widen it to i16 lanes; output went through the same
16-byte array via `_mm_storeu_si128` + `copy_from_slice`. This "store-few-bytes-then-load-whole-
vector" shape is exactly the pattern LLVM tends to reconstruct via per-byte vector inserts
(`pinsrb`) instead of emitting real memory traffic, which matches the profile.

Replaced both load and store with a pair of small `#[inline]` `#[target_feature(enable =
"sse4.1")]` helper functions (`widen_pixel::<SIZE>` / `narrow_pixel::<SIZE>`):

- On `x86_64`: assemble the SIZE bytes into a `u64` via a compile-time-unrolled shift/or loop in a
  general-purpose register (cheap, no SIMD domain-crossing per byte), then move the whole value
  into the low lane with a single `_mm_cvtsi64_si128`/`_mm_cvtsi128_si64` (one `movq` each way)
  instead of SIZE separate `pinsrb`/`pextrb` instructions.
- On 32-bit `x86` (no 64-bit GPR, still compiled/checked by this project's own CI matrix, see
  `.github/workflows/test.yml`): falls back to an 8-byte stack buffer + `_mm_loadl_epi64`/
  `_mm_storel_epi64` (a single `movq` from/to memory) - smaller than the original 16-byte buffer,
  not the GPR-shift fast path, but still a single-instruction memory op rather than a
  reconstructed-vector chain. Verified with `cargo check --target i686-unknown-linux-gnu -p
  zune-png` (compiles clean, not benchmarked - no 32-bit x86 hardware available in this session).

Bench harness: no existing example in `crates/zune-png` reached `decode_raw()` directly with timing
(the project's own `benchmarks/benches/decode_png.rs` criterion bench compares against `png`/`spng`
and adds criterion overhead/statistics this workspace's own paired-comparison discipline doesn't
need) - added `bench/bench_decode.rs` in this candidate's own folder (from
`~/.claude/skills/perf-optimize/templates/bench_template.rs`, adapted for
`zune_png::PngDecoder::decode_raw()`). This is harness, not fix, so it's NOT part of `patch.diff` -
this candidate's `justfile` copies it into `clone/crates/zune-png/examples/bench_decode.rs` as part
of `clone:`, before either build, so it's present identically for both the baseline and patched
binary.

Input: `test-images/png/benchmarks/speed_bench.png` (6.28 MB file, Paeth-filtered RGB8 rows,
decodes to a 99.5 MB raw buffer) - same file used for the original profiling session.

| Variant | single-core p50 (`taskset -c2`) | single-core p50 (`taskset -c3`, repeat) | multi-core p50 (unpinned) | notes |
|---|---|---|---|---|
| baseline | 153.79 ms | 155.54 ms | 158.17 ms | stack-array load/store, per-byte pinsrb/pextrb |
| patched | 116.98 ms | 118.48 ms | 117.12 ms | GPR shift/or + single movq in/out |

Paired alternating comparison (`___OPTIMIZATIONS___/tools/paired.sh`, adapted this session to parse
this project's `bench_decode` output format - `min` line - since no adapted copy existed yet;
`RUNS=9`, `taskset` not applied for this particular run, `iters=8` per invocation, baseline vs.
patched binaries confirmed non-identical via `md5sum` first):

```
A (baseline): 157.012 156.186 157.374 156.966 156.281 156.106 156.385 157.075 154.139
B (patched) : 116.470 117.415 117.315 116.667 116.869 116.912 117.792 117.960 117.836
median 156.385 -> 117.315  (-25.0%)   B faster in 9 of 9 pairs
verdict: consistent
```

## Result

Consistent, large win on wall-clock PNG decode time for Paeth-filtered rows: ~-24% to -25% median
decode time across three independent measurement methods (two single-core `taskset` runs on
different cores, one unpinned multi-core run, and a 9-run paired alternating comparison with 9/9
pairs in the same direction). Noise floor on repeat baseline runs was small (~1-1.5%, single-core
p50 153.79 vs 155.54 ms on two different cores) and far smaller than the observed delta, so this is
not noise. Time only - no memory/allocation-count claim made or measured for this candidate (the
change doesn't touch allocation patterns, only per-iteration register/stack traffic).

## Does it change behavior/output?

- **Bit-exact?** yes. `fnv1a64` hash of the full decoded output (`5ca89c3632c9e076`) is identical
  between baseline and patched across every iteration of every run above (the bench driver asserts
  this itself and aborts on mismatch; it never did).
- **Is this a breaking change?** no. Internal-only change to a private helper function inside
  `crates/zune-png/src/filters/sse4.rs`; no public API, file format, or documented behavior change.
- Project test suite (`crates/zune-png` only - the only crate whose source changed), `cargo test
  --release -p zune-png`, run through the build/test lock, before and after:
  - **lib tests**: 2 passed / 0 failed, both baseline and patched (identical).
  - **integration tests** (`--tests`, all files under `crates/zune-png/tests/`): 35 passed / 0
    failed, both baseline and patched (identical: 2+13+5+4+3+1+5+2 across the 8 test binaries).
  - **doctests** (`--doc`): 9 passed / 2 failed, **both baseline and patched, same two failing
    tests** (`crates/zune-png/src/decoder.rs - decoder` (line 68) and `decoder::PngDecoder<T>::
    more_frames` (line 507), both `error[E0432]: unresolved import
    \`zune_png::post_process_image_apng\`` / a `decoder` name-resolution error in the doc example
    itself) - pre-existing in this project's `dev` branch at `base_commit`, unrelated to this
    patch (reproduced identically with the patch fully reverted). Not fixed by this candidate;
    noted below as incidental.
  - `cargo clippy --release -p zune-png --all-targets`: same 2 pre-existing warnings in
    `apng.rs`/`headers/readers.rs` (unrelated files) both before and after; no new warnings from
    `sse4.rs`.

## Why the result looks the way it does

The arithmetic (min/max/add/compare-select for the Paeth predictor) was already implemented with
SSE4.1 instructions operating on full 128-bit registers - that part wasn't the bottleneck. The
bottleneck, matching the profile, was *getting* a 1-8 byte pixel into a register and *getting* the
1-8 byte result back out, once per pixel: the original code's "write a few bytes into a stack array,
then load the whole array as a vector" shape gives LLVM's SROA/vectorization passes exactly the
pattern that gets lowered to a chain of `pinsrb`/`pextrb` (insert/extract one byte of an XMM
register at a time) rather than real memory traffic, since the array never escapes the function.
Each `pinsrb`/`pextrb` crosses the GPR/SIMD domain, which has nontrivial latency on this
microarchitecture - doing that SIZE times per pixel (up to 8) dominated the loop. Assembling the
same bytes in a general-purpose register first (cheap shifts/ORs, no domain crossing) and doing the
GPR<->XMM move exactly once per direction removes that repeated-domain-crossing cost while keeping
the same 128-bit arithmetic untouched.

## What's not done / open questions

- Not benchmarked on 32-bit `x86` (no hardware available this session) - only confirmed it compiles
  (`cargo check --target i686-unknown-linux-gnu -p zune-png`). The 32-bit fallback path is
  plausible but unmeasured; it may or may not carry the same win there.
- Not tried: a further step of processing multiple *independent* pixels' `b`/upper-row loads
  (which don't have the serial `a`/left dependency the way `d` does) in wider batches - this
  candidate only removed the load/store overhead around the existing one-pixel-per-iteration
  structure, it didn't restructure the iteration itself.
- The two pre-existing doctest failures in `crates/zune-png/src/decoder.rs` are unrelated to this
  candidate (same failures with the patch fully reverted) - not fixed here, flagged in the
  session's final report per `SKILL.md`'s `___BUGHUNT___/` handling rather than folded into this
  performance record.
