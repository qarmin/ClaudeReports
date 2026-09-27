# AVX-512VNNI dispatch branch for RGBA u8 horizontal convolution

`src/factory/rgba_u8.rs` only ever activates the AVX-512-feature-gated dot-product horizontal
kernel (`src/avx512/rgba_u8_dot_vnni.rs`) when `is_x86_feature_detected!("avxvnni")` is true.
`avxvnni` is the narrow VEX-encoded extension found on Alder Lake+/Sierra Forest; it is absent on
far more common real AVX-512 machines (Skylake-X/Cascade Lake/Ice Lake Xeon, and this dev/bench
box - AMD Ryzen 9 7900, Zen4 - which has `avx512vnni`+`avx512bw`+`avx512vl` but not `avxvnni`,
confirmed via `/proc/cpuinfo` and `is_x86_feature_detected!`). This candidate adds a second
dispatch branch that activates the same kernel on `avx512vnni`+`avx512vl` hardware, using the
EVEX-encoded `_mm256_dpwssd_epi32`/`_mm_dpwssd_epi32` intrinsics instead of the VEX-encoded
`_mm256_dpwssd_avx_epi32`/`_mm_dpwssd_avx_epi32` the existing `avxvnni` path uses - same
computation, different required CPU feature/encoding. It looked worth trying because it is a
scoped feature-detection/intrinsic-selection fix to an already-written, already-tested kernel,
directly measurable on this machine via the existing `resize_rgba` bench (`PreferQuality`
Lanczos3, "Pic scale RGBA with alpha: Lanczos 3") with no new harness needed.

## What was tried

Only one variant: add an `avx512vnni`+`avx512vl` branch in
`HorizontalFilterPass<u8, f32, 4>::horizontal_plan` (checked before the `avxvnni` branch) that
calls new `convolve_horizontal_rgba_avx512vnni_row_1`/`_row_4` entry points. Those call the same
`_impl` functions as the existing `avxvnni` path, now generic over a `const USE_AVX512VNNI: bool`
that swaps `_mm256/_mm_dpwssd_avx_epi32` (VEX, `avxvnni`) for `_mm256/_mm_dpwssd_epi32` (EVEX,
`avx512vnni,avx512vl`) at every one of the 13 call sites in the two impl functions, via two small
`#[inline(always)]` dispatch helpers. No separate avx512vnni-only variant of the kernel logic was
written - it's the exact same shuffle/accumulate structure, just the final dot-product instruction
differs.

Baseline in the table below is the code as shipped today on this machine: since `avxvnni` is
false here, `is_x86_feature_detected!("avxvnni")` never fires, and RGBA u8 horizontal convolution
silently uses the plain AVX2 kernel (`convolve_horizontal_rgba_row_4`/`convolve_horizontal_rgba_avx_row_1`
from `src/avx2/rgba_u8.rs`, which computes the same dot product as two instructions,
`_mm256_madd_epi16` + `_mm256_add_epi32`, instead of one fused `vpdpwssd`).

Bench: `app/benches/resize_rgba/main.rs`, `"Pic scale RGBA with alpha: Lanczos 3"` case
(`ResamplingFunction::Lanczos3`, `WorkloadStrategy::PreferQuality`, `ThreadingPolicy::Single`,
4928x3279 -> 1232x819, `../assets/nasa-4928x3279-rgba.png`). This bench is single-threaded by
construction (the harness always sets `ThreadingPolicy::Single` for every case in this file), so
"single-core" below is that same process `taskset -c 3`-pinned, and "multi-core" is the same
unpinned - see README.md's "Instrumentation added for this workspace" for why the app crate needed
a features change to reach this code path at all, and for the harness's own paired-comparison
adaptation.

Paired via `___OPTIMIZATIONS___/tools/paired.sh` (alternating A/B, 9 pairs each), point estimate =
criterion's reported mean (middle of `[low mean high]`).

| Variant | single-core (taskset -c 3), median of 9 | multi-core (unpinned), median of 9 | notes |
|---|---|---|---|
| baseline (today's code - AVX2 fallback, `avxvnni` gate never fires) | 25.03 ms | 20.57 ms | A in both runs |
| patched (new `avx512vnni`+`avx512vl` branch active) | 31.51 ms (+25.9%, B faster in 1/9 pairs) | 27.54 ms (+33.9%, B faster in 0/9 pairs) | B in both runs |

Raw pairs, single-core: `A: 25.028 23.236 26.717 35.55 21.667 19.851 34.955 21.606 27.006` /
`B: 30.965 32.51 31.505 34.272 32.12 27.412 74.61 26.86 27.821`.

Raw pairs, multi-core (unpinned): `A: 20.997 20.57 20.602 21.504 19.411 20.228 19.262 20.048 21.747`
/ `B: 28.556 27.224 27.558 27.539 26.406 26.679 25.977 28.544 27.912`.

## Result

Consistent regression in both pinned and unpinned runs (`paired.sh`'s own consistency check:
B faster in 1/9 pairs pinned, 0/9 unpinned - both far outside the "not consistent" 2-8 band). The
new `avx512vnni`-gated dot-product kernel is reliably ~26-34% *slower* than the AVX2 fallback
that already ships and runs on this machine today, not faster. This is the opposite of what the
candidate hypothesized (that the existing dot-product kernel, once reachable, would beat AVX2).

## Does it change behavior/output?

- **Bit-exact?** Yes. A standalone correctness-check crate (`bit_exact_check/` in this folder)
  resizes the same `nasa-4928x3279-rgba.png` asset with `Lanczos3`/`PreferQuality`/alpha,
  built once against baseline source and once against patched source: both produce
  `len=4036032 hash=a9cf045dafca2f6a` (`DefaultHasher` over the output byte buffer).
- **Is this a breaking change?** No public API change either way.
- Project test suite: not run for this candidate - the change only adds a new dispatch branch
  reachable when `avx512vnni`+`avx512vl` are both runtime-detected, which no CI/test machine used
  by this project's own suite has been confirmed to have; behavior on `avxvnni`-only hardware
  (the pre-existing branch) is untouched by this patch.

## Why the result looks the way it does

Not fully root-caused, but the most likely explanation given a bounded look: this isn't just an
encoding swap (VEX vs EVEX of the same instruction) - the *baseline* on this machine is a
different, simpler kernel (`src/avx2/rgba_u8.rs`'s plain AVX2 path, 2 instructions per
accumulate: `vpmaddwd` + `vpaddd`) while the *patched* path is the dot-product kernel's
shuffle-heavy reformatting (splitting each 4-channel pixel into interleaved 16-bit lanes via
several `vpshufb`s) feeding a single `vpdpwssd`. On this Zen4 core, the extra shuffle overhead
plus `vpdpwssd`'s own latency apparently costs more than it saves versus the straightforward
`vpmaddwd`/`vpaddd` pair - i.e. the dot-product kernel design itself may not be a net win over
plain AVX2 on Zen4, independent of which feature gates it. This candidate cannot distinguish
"EVEX dpwssd is slow on Zen4" from "the whole dot-product kernel shape is slow on Zen4 regardless
of avxvnni-vs-avx512vnni" - see IDEAS.md for a follow-up that would need real `avxvnni` hardware
(Alder Lake+) to separate those.

## What's not done / open questions

- Not tested on real `avxvnni` hardware (Alder Lake+/Sierra Forest) - can't rule out that the
  existing, shipping `avxvnni` branch is itself a similar regression there; this candidate only
  establishes that the *kernel*, when reached via the EVEX/avx512vnni path on Zen4, loses to the
  AVX2 fallback already in place.
- The `rgb_u16.rs`/`rgba_u16.rs`/`plane_u16.rs` occurrences of the same `avxvnni`-only gating
  pattern (noted in the original IDEAS.md entry) were not touched or measured - given this
  result, wiring those up the same way is not expected to be a win on this hardware either
  without first checking whether their kernels have the same shuffle-heavy shape.
- `PreferSpeed` workload strategy explicitly excludes this dispatch branch already (unchanged by
  this patch) - not re-evaluated here.
