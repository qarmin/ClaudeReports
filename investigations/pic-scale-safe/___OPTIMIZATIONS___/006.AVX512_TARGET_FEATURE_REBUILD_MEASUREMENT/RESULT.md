# Building this crate with -C target-feature=+avx512f,+avx512bw,+avx512vl widens the vertical convolution tap loop further (measurement only)

**Ported from `image` repo's `___OPTIMIZATIONS___/052.PIC_SCALE_SAFE_AVX512_TARGET_FEATURE_REBUILD/`.**
Same method as candidate 005, with `+avx512f,+avx512bw,+avx512vl` instead of `+avx2`. **Not
re-measured in this workspace.** **No `patch.diff` in this folder, correctly** - pure
build-configuration measurement, no source change.

## What was tried

Three builds of the unmodified registry crate: baseline, `+avx512f,+avx512bw,+avx512vl`, and a
freshly-built `+avx2` for a direct same-session comparison. Disassembly confirmed the AVX-512 build
emits 32 `zmm`-operand instructions in `column_handler_fixed_point::<u8, i32>` (vs. 62 `ymm`/0
`zmm` for AVX2, 0 `ymm`/`zmm` for baseline).

| Variant | single-core p50 | multi-core p50 | scenario |
|---|---|---|---|
| baseline | 18.808 ms | 18.739 ms | Lanczos3, 4032x3024->400x300 |
| AVX-512 | 10.418 ms (**-44.6%**, 5/5) | 10.400 ms (**-44.5%**, 5/5) | same |
| baseline | 7.384 ms | n/a | Triangle |
| AVX-512 | 5.154 ms (**-30.2%**, 5/5) | n/a | same |
| baseline | 219.66 us | n/a | Lanczos3, small image |
| AVX-512 | 147.66 us (**-32.8%**, 5/5) | n/a | same |
| AVX2 (this session) | 11.091 ms | n/a | Lanczos3, same as above |
| AVX-512 vs. AVX2 | 10.422 ms (**-6.0%**, 5/5) | n/a | direct same-session comparison |

## Result

Confirms a further win beyond AVX2: **-44.6%/-44.5%** (single/multi-core) vs. baseline on the large
Lanczos3 scenario, **-30.2%** on Triangle, **-32.8%** on the small-image case - and AVX-512 beats a
freshly-built AVX2 by a further, smaller **-6.0%** on the same host. No downclocking regression was
observed on the measuring host (AMD Ryzen 9 7900) for this workload.

## Does it change behavior/output?

- **Bit-exact?** Yes, confirmed directly across all three configurations (55 hashes each, matched).
- **Is this a breaking change?** N/A - no source change; correctly no `patch.diff`, for the same
  portability reason as candidate 005 (forcing AVX-512 as a default would crash on the large
  population of x86_64 CPUs without it).
- Project test suite: original measurement quoted 622 passed / 1 pre-existing unrelated failure,
  identical in both builds. Not re-run against this crate's own suite in this session (no change
  made here).

## Why the result looks the way it does

Same mechanism as candidate 005, one register-width step further (512-bit `zmm`, 16x `i32` lanes,
vs. AVX2's 256-bit `ymm`, 8x lanes). The marginal AVX2-to-AVX-512 gain (-6.0%) is much smaller than
the baseline-to-AVX2 gain (~41-45%, candidate 005) because non-vectorized parts of the pipeline
(row setup, dispatch, memory traffic) don't share in a further width doubling and become a larger
share of what's left to speed up - a standard diminishing-returns shape for successive SIMD-width
doublings.

## What's not done / open questions

- **No source-level change is proposed here**, same portability reasoning as candidate 005 - see
  candidates 009/010 in this workspace for the safe runtime-dispatched alternative.
- AVX-512 downclocking was not directly instrumented (wall-clock only, one host's CPU) - a
  different AVX-512-capable CPU model could show a different tradeoff.
- Whether downstream consumers already building with `-C target-cpu=native` on an AVX-512 host
  are already getting this speedup was not verified beyond this measurement.
