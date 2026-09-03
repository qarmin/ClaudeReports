# SSE4.1 horizontal convolution kernel for CbCr `u16` (bit_depth <= 12)

`src/factory/cbcr16.rs`'s `bit_depth <= 12` branch only special-cased `aarch64`+`neon`
(`src/neon/cbcr_u16_lb.rs`); on x86_64 it fell straight through to the scalar
`fixed_point_horizontal::convolve_row_handler_fixed_point_4::<u16, i32, 2>` handler, with no
SSE4.1/AVX2 kernel at all - unlike the 8-bit CbCr path (`src/sse/cbcr8.rs`), which already has
one. Callgrind on `resize_cbcr` (Lanczos 3) showed that scalar fallback at 20.0% of total Ir
(~806M of 4.03B instructions), the largest resize-specific hotspot found across this project's
four bench suites - see `___OPTIMIZATIONS___/IDEAS.md`'s (now-removed) top backlog entry. This
candidate adds `src/sse/cbcr_u16_lb.rs`, an SSE4.1 kernel for this exact case, and wires it into
`src/factory/cbcr16.rs` ahead of the scalar fallback (same dispatch position/pattern already
used by `src/factory/rgba_u16.rs`'s SSE4.1 low-bit-depth branch).

## What was tried

Only one variant: a direct SSE4.1 port of `src/sse/cbcr8.rs`'s "reorder taps into per-channel
groups via `shuffle_epi8`, then `madd_epi16`, fold once at the end" trick, lifted from 8-bit to
16-bit lanes. Because `bit_depth <= 12` values fit in an `i16` without widening, the batched
(2/4/8-tap) paths need no zero-extend step - `idx_deinterleave` is a pure byte-pair permutation
(not a zero-fill widen like `src/sse/rgba_u16_lb.rs`'s CN=4 kernel needs). The weight-shuffle
tables (`weights_lo`/`weights_hi`) are byte-for-byte identical to `cbcr8.rs`'s, since weights are
`i16` in both kernels. No AVX2 tier was added in this candidate (see "What's not done" below) -
scope was kept to "give x86_64 an SSE4.1 kernel where it currently has none at all", matching the
IDEA text's framing.

Bench: `app/benches/resize_cbcr/main.rs`, `"Pic scale CbCr10: Lanczos 3"` case
(`ResamplingFunction::Lanczos3`, `WorkloadStrategy::PreferSpeed`, `ThreadingPolicy::Single`,
`bit_depth = 10` i.e. `<= 12`, so it exercises exactly the patched path; `bit_depth = 10` was
chosen over the file's other case, `"Pic scale CbCr16: Lanczos 3"` (`bit_depth = 16`), because
16-bit CbCr takes the separate `bit_depth > 12` floating-point path, untouched by this patch).
`app/Cargo.toml`'s `pic-scale` feature list needed `"sse"` added to reach any x86 SIMD tier at
all (see `README.md`); this was done on both binaries being compared, not committed.

Two prebuilt binaries (`cargo bench -p app --bench resize_cbcr --no-run`, saved out before
reverting between builds) were compared via `___OPTIMIZATIONS___/tools/paired.sh`, alternating
A(baseline)/B(patched), 9 pairs per run, point estimate = criterion's reported mean (middle of
`[low mean high]`). `uptime` before the session showed load ~6-12 on a 24-core box (this box's
host-visible load average is noisy - no runaway process visible in `ps` inside this container,
but per-pair spread on the pinned run was wide, up to ~2x between the fastest and slowest A
sample within the same run - see raw pairs below).

| Variant | single-core (`taskset -c 3`), median of 9 (paired.sh's own consistency check) | multi-core (unpinned), median of 9 | notes |
|---|---|---|---|
| baseline (today's code - scalar `fixed_point_horizontal` fallback) | 43.156 / 39.211 / 41.412 ms across 3 repeated pinned runs | 21.714 ms | A in all runs |
| patched (new SSE4.1 kernel active) | 31.982 / 32.377 / 31.497 ms across the same 3 runs | 16.354 ms | B in all runs |

Three repeated pinned runs (not just the usual two) were done because the first two landed
outside `paired.sh`'s own "decisive" band (needs `wins <= 1` or `wins >= n-1` out of 9):

- Pinned run 1: `-25.9%` (43.156 -> 31.982 ms), B faster in 7/9 pairs - **NOT consistent** per
  the script's own threshold.
  `A: 43.156 44.073 44.096 24.602 51.657 25.205 25.63 23.199 50.833`
  `B: 32.625 32.203 31.982 37.108 19.968 19.545 17.707 28.149 37.931`
- Pinned run 2: `-17.4%` (39.211 -> 32.377 ms), B faster in 7/9 pairs - **NOT consistent**.
  `A: 38.142 43.53 21.166 43.829 43.373 32.987 43.552 39.211 36.172`
  `B: 31.915 32.803 33.002 32.527 19.116 34.255 32.377 28.082 32.375`
- Pinned run 3: `-23.9%` (41.412 -> 31.497 ms), B faster in **9/9** pairs - consistent.
  `A: 43.208 42.757 43.529 41.406 41.412 42.836 24.04 22.033 22.382`
  `B: 31.497 31.784 31.919 31.407 32.725 31.971 16.91 16.225 17.481`
- Unpinned (multi-core) run: `-24.7%` (21.714 -> 16.354 ms), B faster in 8/9 pairs - consistent.
  `A: 30.839 21.799 21.114 29.381 31.393 21.714 21.313 21.517 21.648`
  `B: 20.504 16.16 23.295 20.84 18.374 15.856 16.026 16.354 16.333`

The recurring bimodal pattern in A's own samples (e.g. run 3's A alternates between a ~42-44ms
cluster and a ~22-24ms cluster within the same 9-run sequence) points at host-level noise
(thermal/turbo state or a noisy neighbor invisible to `ps` inside this container) rather than
anything caused by the patch - B's samples show the same two-cluster pattern, just both clusters
shifted down.

## Result

A real, substantial improvement, consistent in direction across all 4 runs (2 pinned, 1
"not-consistent"-by-the-strict-rule pinned re-run, 1 unpinned) and consistent by `paired.sh`'s
own decisiveness rule in the last pinned run and the unpinned run: roughly **-18% to -26%**
(median ~-23%) wall-clock for the `"Pic scale CbCr10: Lanczos 3"` case, both pinned and unpinned.
The two runs that didn't clear the strict 8/9-or-better bar still had B faster in 7/9 pairs each
and a similar-magnitude median improvement - read as "real effect, noisy box" rather than "no
effect," given the third repeat and the unpinned run both cleared the bar cleanly at a matching
magnitude.

## Does it change behavior/output?

- **Bit-exact?** Yes. `bit_exact_check/` (candidate-local standalone crate) resamples 11
  synthetic CbCr `u16` cases - sizes/scale-ratios chosen to sweep the SSE kernel's 1/2/4/8-tap
  batch sizes and every remainder combination, `bit_depth` in `{1, 8, 10, 12}`, both
  `WorkloadStrategy::PreferSpeed` and `PreferQuality` - and hashes the concatenated output with
  `DefaultHasher`. Built once against baseline source and once against patched source (both with
  the `sse` feature enabled): both print the identical `FINAL_HASH=4078897055118658543` and
  identical per-case `first8=[...]` samples.
- **Is this a breaking change?** No public API change either way.
- Project test suite (`cargo test -p pic-scale --features sse --lib`): 66 passed, 0 failed,
  identical before and after (the existing suite has no dedicated CbCr16 low-bit-depth test, so
  this candidate's own `bit_exact_check` was written to cover that gap directly - see "What's not
  done" below).

## Why the result looks the way it does

Same shape as the already-shipping `src/sse/cbcr8.rs` (8-bit CbCr) and `src/sse/rgba_u16_lb.rs`
(16-bit RGBA) kernels: batching several filter taps per SIMD call amortizes the per-tap load and
weight-broadcast overhead that dominates the scalar handler's tight, small (`CN=2`) inner loop.
Unlike candidate 001 (an AVX-512VNNI dot-product kernel that turned out to *regress* vs. the
existing AVX2 fallback on this Zen4 box), this candidate isn't competing against an existing,
already-decent SIMD path - it's filling a genuine gap (scalar-only), so the win is much less
surprising and much larger in relative terms.

## What's not done / open questions

- No AVX2 tier was added - only SSE4.1 (matching this candidate's scoped goal of "give x86_64 an
  SSE4.1 kernel where there was none," not "match RGBA u16's full AVX2+SSE4.1 tiering"). Adding
  one is a natural, likely-smaller follow-up now that the shuffle/reorder trick is proven
  correct at 16-bit granularity - see `IDEAS.md`.
- The host box's load/noise made single-core numbers noisier than typical for this workspace
  (see raw pairs above) - if re-measuring, consider a longer quiet window or more repeats.
- `bit_exact_check`'s 11 synthetic cases are not the real `nasa-4928x3279-rgba.png` asset (unlike
  candidate 001's correctness check) - chosen instead to directly control width/height/scale
  ratio so every tap-count remainder branch is exercised deliberately; this seemed more
  targeted than hoping a single real-image resize happens to hit all of them.
