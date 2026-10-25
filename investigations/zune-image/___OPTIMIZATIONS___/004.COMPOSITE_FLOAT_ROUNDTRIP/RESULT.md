# Composite (Over/Multiply/Screen, u8): replace per-pixel f32 round-trip with premul_alpha's fastdiv-style integer math

`crates/zune-imageprocs/src/composite.rs`, `composite_region` (`CompositeMethod::Over` at line
316, `Multiply` at 383, `Screen` at 394), called per-pixel inside `process_overlap!` from the
`par_process_regions` hot loop (`composite_two_images`, `BitType::U8` branch). For `u8` images
these three branches convert every color/alpha sample to `f32` via `f32::from(T)`, multiply by
`inv_max`/`max_val`, `clamp`, `.round()`, then convert back with `T::from_f32(...)` - per pixel,
per channel. `crates/zune-imageprocs/src/premul_alpha.rs` already solves exactly this "associated
alpha" integer-scaling problem for `u8`/`u16` (its own module doc: "division is slow ... badly
vectorized", fixed there via Lemire fastmod reciprocal multiplication), but `composite.rs` never
reused that technique for its `u8` path.

rustc 1.98.1 (48a229cea 2026-09-01), cargo 1.98.1 (797e8a9bc 2026-08-05).

## What was tried

All three float formulas (`Over`'s color blend `a*c_src + (1-a)*c_dst`, `Over`'s alpha blend
`a_src + a_dst*(1-a_src)`, `Multiply`'s `s*d`, `Screen`'s `1-(1-s)*(1-d)`) are, for `u8`, each
either a single product divided by 255 or a weighted sum of two products divided by 255 - the same
shape `premultiply_u8`/`unpremultiply_u8` already handle. Added a `div255(x: u32) -> u32` helper
using the standard bit-trick reciprocal-multiply approximation (`y = x + 128; (y + (y>>8)) >> 8`,
the same style as `premultiply_u8`'s `temp = a*b + 0x80; (temp + (temp>>8)) >> 8`), and a new
`composite_region_u8_fast` function implementing `Over`/`Multiply`/`Screen` entirely in `u32`
integer arithmetic, no `f32` anywhere. `composite_two_images`'s `BitType::U8` branch now calls
`composite_region_u8_fast` for these three methods and keeps calling the existing generic
`composite_region::<u8>` for every other method (`Src`, `Dst`, `DstIn`, `DstOut`, `SrcIn`,
`SrcOut`, `Xor`) - those are untouched, no change to `u16`/`f32` bit depths either.

Bench harness: no existing example/bench in the project reached `Composite::execute_multiple()`
directly - added `bench/bench_composite.rs` (this candidate's own folder, copied into
`clone/crates/zune-imageprocs/examples/bench_composite.rs` by the justfile's `clone:`, same
pattern as candidate 002). It builds two 1920x1080 RGBA images from a deterministic xorshift PRNG
once, then repeatedly runs `Composite::new(method, (0,0)).execute_multiple(...)` on fresh copies of
both images, hashing (fnv1a64) the resulting pixel buffer every iteration to confirm bit-stability
across iterations and between baseline/patched binaries.

Machine: AMD Ryzen (24 logical cores), `uptime` load average ~2.0-2.6 throughout (well under core
count) at measurement time. `par_process_regions` uses rayon internally, so single-core
(`taskset -c2`) numbers below reflect rayon effectively running single-threaded under CPU
affinity, not an unparallelized code path - both baseline and patched measurements are affected
identically, so the comparison is still fair (see "Why the result looks the way it does").

Time only - no memory/allocation-count claim made; the change doesn't add or remove allocations
(the old and new code both write into the existing destination buffer in place), only replaces the
per-element arithmetic.

| Variant | single-core p50 (`taskset -c2`) | single-core p50 (repeat) | multi-core p50 (unpinned) | multi-core p50 (repeat) |
|---|---|---|---|---|
| baseline, Over | 25.454 ms | 25.518 ms | 2.589 ms | 2.656 ms |
| patched, Over | 9.171 ms | - | 1.127 ms | - |
| baseline, Multiply | 15.690 ms | - | 1.801 ms | - |
| patched, Multiply | 5.385 ms | - | 0.859 ms | - |
| baseline, Screen | 19.310 ms | - | 2.044 ms | - |
| patched, Screen | 6.367 ms | - | 0.904 ms | - |

(1920x1080 RGBA, 60 iterations per run, warmup discarded, through
`___OPTIMIZATIONS___/.locks/build.lock`.)

Paired alternating comparison (`tools/paired.sh`, `RUNS=9`, `over` method, unpinned, baseline vs.
patched binaries confirmed non-identical via `md5sum` first):

```
A (baseline): 2.335 2.386 2.428 2.257 2.551 2.574 2.387 2.346 2.342
B (patched) : 0.976 0.953 0.893 1.018 1.022 1.063 0.969 0.909 0.935
median 2.386 -> 0.969  (-59.4%)   B faster in 9 of 9 pairs
verdict: consistent
```

## Result

Real, large speedup, far outside the measured noise floor (baseline's own repeat runs for `Over`
varied by <0.3% single-core; the patched binary is consistently 2.3-2.8x faster single-core and
2.0-2.3x faster multi-core/unpinned across all three methods). The paired alternating comparison
is decisive: patched binary faster in 9 of 9 pairs, median -59.4%. Time only, as stated above - no
RSS/allocation-count claim.

## Does it change behavior/output?

- **Bit-exact?** Yes, two independent checks:
  - The bench's own fnv1a64 hash of the composited output is identical between baseline and
    patched binaries for all three methods (`Over`: `3535a2590a993513`, `Multiply`:
    `caffaea90a4108bf`, `Screen`: `4aab9194df4b4080`), stable across all iterations of both
    binaries, on a 1920x1080 pseudo-random RGBA image pair.
  - A standalone, dependency-free exhaustive check (`rustc -O`, no zune dependency - just the
    extracted float and integer formulas side by side) compared every possible `u8` input
    combination: `Multiply`/`Screen`/`Over`-alpha each over the full 65,536 `(s, d)` pairs, and
    `Over`'s color blend over the full 16,777,216 `(a_src, c_src, c_dst)` triples. **Zero
    mismatches** in all four cases (`max_diff=0` for the color-blend case). This is expected: the
    `div255` bit-trick is a well-known exact reproduction of `round(x/255)` (round-half-up, same
    direction as `f32::round`'s round-half-away-from-zero for positive inputs) for `x` in
    `0..=65025`, and every formula here reduces to that shape.
- **Is this a breaking change?** No. Internal-only change inside a private function
  (`composite_region_u8_fast`, called only from `composite_two_images`'s `u8` branch for 3 of the
  10 `CompositeMethod` variants); no public API, file format, or documented behavior change. `u16`
  and `f32` bit depths, and the other 7 `CompositeMethod` variants, are completely untouched.
- Project test suite (`cargo test --release --workspace --no-fail-fast --lib --tests`, doctests
  excluded - see "What's not done" below), through the build/test lock, before and after:
  **404 passed / 32 failed, identical set of passing/failing tests before and after** (line-by-line
  diff of every `test result:` line between the two runs showed zero differences beyond timing).
  The 32 pre-existing failures are entirely in `zune-inflate` (30), `zune-jpeg` (1) and
  `zune-tests` (1) - none in `zune-imageprocs`, none touching `composite.rs`; reproduced identically
  with the patch fully reverted, so unrelated to this candidate.
  `zune-imageprocs`'s own lib test suite (which includes `composite.rs`'s existing
  `Over`/`Multiply`/`Screen` unit tests): **77 passed / 0 failed, identical before and after**.

## Why the result looks the way it does

The float path pays, per pixel per channel: two `f32::from(u8)` conversions, one or two
multiplies, one `clamp`, one `.round()` (a real instruction, not free), and one `T::from_f32`
convert-back - each of these is individually cheap but the chain has real latency and, per
`premul_alpha.rs`'s own module docs, division/rounding-heavy per-element f32 work of this shape is
known to vectorize poorly compared to pure integer multiply-shift. The integer path replaces the
whole chain with 1-2 `u32` multiplies, an add, and two shifts - no conversions, no `clamp`, no
`round` call, everything in registers the whole way, which both shortens the per-element critical
path and is far more amenable to auto-vectorization (no float rounding-mode/NaN-handling baggage
for the compiler to preserve). The 2.3-2.9x single-core speedup and roughly proportional
multi-core speedup (parallelism via rayon in `par_process_regions` is orthogonal to this - both
baseline and patched use the same region-splitting, so the per-region work just got proportionally
cheaper) are consistent with removing a real, per-element bottleneck rather than something masked
by cache/memory-bandwidth effects (the working set here, two 1920x1080x4-byte images, is small
enough to be memory-bandwidth-light already).

## What's not done / open questions

- `u16` and `f32` bit-depth paths still use the original float `composite_region` for all methods,
  including `Over`/`Multiply`/`Screen` - `premul_alpha.rs` already has a `u16` fastdiv table
  (`create_unpremul_table_u16`/`unpremultiply_u16`) that a `u16` version of this fix could reuse,
  but that's a separate, unmeasured candidate (noted in `IDEAS.md`).
  `f32` has no natural fixed-point analog and is out of scope for this technique entirely.
- The other 7 `CompositeMethod` variants (`Src`, `Dst`, `DstIn`, `DstOut`, `SrcIn`, `SrcOut`,
  `Xor`) are the same "associated alpha" shape for `u8` (each is a weighted combination of alpha
  and color factors divided by 255) and are plausible candidates for the same `div255` treatment -
  not attempted here to keep this candidate's patch scoped to the three methods the task named;
  noted in `IDEAS.md`.
- Doctests were excluded from the correctness run (`--lib --tests` only, not `--doc`) because the
  environment ran out of disk space (`/tmp` at 100%, "No space left on device") partway through an
  earlier `cargo test --workspace` attempt that did include doctests - an environment resource
  constraint unrelated to this patch (the same failure occurred identically whether or not the
  patch was applied, and doctests don't exercise `composite.rs`, which has no doc-tested code
  examples). Flagged here for transparency, not folded into the test-suite pass/fail counts above.
