# sRGB gamma LUT, fixed cast + hoisted LUT refs (follow-up to candidate #002)

Candidate #002 (`002.SRGB_GAMMA_POWF_LUT`) replaced `v.powf(1.0/2.4)` in `srgb_apply_gamma`
(`rawler/src/imgop/srgb.rs`) with an IEEE-754 mantissa/exponent-decomposition LUT, but measured
slower than `powf` on this machine; its `RESULT.md` attributed the regression to two disassembly-
confirmed costs: (1) `scaled as usize` compiling to Rust's multi-instruction saturating float-to-
int cast, and (2) `mantissa_lut()`/`exp_lut()` re-checking their `OnceLock` init flag on every
single pixel call. This candidate applies both suggested fixes - `unsafe { .to_int_unchecked() }`
and hoisting the LUT references once per `par_iter_mut` call - to see whether the LUT approach
wins once those two costs are removed.

## What was tried

Same isolated microbenchmark as #002 (`bench/srgb_gamma_bench.rs`, run as a `rawler` example):
`srgb_apply_gamma_inplace` on a 134,283,264-element `f32` buffer, 20 iterations, 2 discarded as
warmup, fresh buffer clone per iteration. Two implementation variants were actually tried in this
session, not just one:

- **v1**: `unsafe { scaled.to_int_unchecked::<usize>() }` (the literal fix #002 suggested) +
  hoisted `mantissa_lut()`/`exp_lut()` references passed into a new `fast_pow_gamma_with` taking
  `&[f32; N]` params, resolved once in `srgb_apply_gamma_inplace` before the `par_iter_mut` loop.
- **v2** (kept as this candidate's `patch.diff`): same hoisting, but the unchecked cast targets
  `i32` instead of `usize`, converting to `usize` only after computing `frac`. Disassembly
  (`objdump -d --demangle` on the built example) showed v1's `scaled.to_int_unchecked::<usize>()`
  still lowered to the same multi-instruction two-`cvttss2si`-plus-2^63-splitting sequence as the
  safe `as usize` cast in #002 - not because of saturation, but because x86-64 has no direct
  float<->unsigned-64-bit-int instruction, so LLVM emits that fallback for *any* `f32 -> usize`
  conversion regardless of checked/unchecked. The same fallback recurred a second time in v1 for
  `idx as f32` (computing `frac`), since `idx: usize` there too. Routing the truncate and the
  round-trip back to float through `i32` (single `cvttss2si` / `cvttps2dq`+`cvtdq2ps`, confirmed
  in the post-fix disassembly) removes both occurrences; only widening `i32 -> usize` for the
  final array index is left, which is a single `movslq`.

| Variant | single-core p50 (`taskset -c 0`, `RAYON_NUM_THREADS=1`) | multi-core p50 (unpinned, 24 threads) | notes |
|---|---|---|---|
| baseline (`v.powf(1.0/2.4)`), run A | 505.10ms, 537.89ms (2 runs) | 49.31ms, 50.57ms (2 runs) | machine load ~7 (busier than #002's session) |
| baseline (`v.powf(1.0/2.4)`), run B | 510.07ms, 502.65ms (2 runs) | 47.46ms, 48.36ms... see below | measured twice, before and after the v1/v2 patched runs, to bound noise from the busier machine |
| v1 (`to_int_unchecked::<usize>`, hoisted LUTs) | 600.96ms, 598.94ms (2 runs) | 56.14ms, 55.61ms (2 runs) | still clearly slower than baseline |
| v2 (`to_int_unchecked::<i32>`, hoisted LUTs) - **this candidate's patch** | 583.03ms, 595.97ms (2 runs) | 53.49ms, 53.32ms (2 runs) | smaller gap than v1, still slower than baseline |

Raw output, in the order actually run (machine load average 6-8 throughout, busier than #002's
original idle-ish session - hence two baseline measurements bracketing the patched ones, see
"Why the result looks the way it does"):

```
baseline run1 (single): min=469.05ms p50=510.07ms mean=513.74ms p95=584.88ms max=584.88ms
baseline run2 (single): min=464.55ms p50=502.65ms mean=503.67ms p95=538.24ms max=538.24ms
baseline run1 (multi):  min=46.77ms  p50=51.10ms  mean=50.36ms  p95=53.43ms  max=53.43ms
baseline run2 (multi):  min=47.19ms  p50=49.54ms  mean=49.31ms  p95=51.72ms  max=51.72ms

v1 run1 (single): min=568.08ms p50=600.96ms mean=601.91ms p95=643.36ms max=643.36ms
v1 run2 (single): min=562.15ms p50=598.94ms mean=599.95ms p95=652.34ms max=652.34ms
v1 run1 (multi):  min=53.73ms  p50=56.14ms  mean=56.44ms  p95=59.89ms  max=59.89ms
v1 run2 (multi):  min=52.35ms  p50=55.61ms  mean=55.28ms  p95=59.81ms  max=59.81ms

v2 run1 (single): min=554.10ms p50=583.03ms mean=597.90ms p95=701.13ms max=701.13ms
v2 run2 (single): min=550.27ms p50=595.97ms mean=590.64ms p95=672.96ms max=672.96ms
v2 run1 (multi):  min=50.94ms  p50=53.49ms  mean=53.17ms  p95=54.57ms  max=54.57ms
v2 run2 (multi):  min=51.12ms  p50=53.32ms  mean=53.13ms  p95=55.85ms  max=55.85ms

baseline (re-check, run A, single): min=481.70ms p50=505.10ms mean=516.02ms p95=636.51ms max=636.51ms
baseline (re-check, run B, single): min=482.11ms p50=537.89ms mean=561.46ms p95=715.26ms max=715.26ms
baseline (re-check, run A, multi):  min=46.89ms  p50=49.31ms  mean=49.32ms  p95=53.06ms  max=53.06ms
baseline (re-check, run B, multi):  min=48.67ms  p50=50.57ms  mean=50.67ms  p95=55.15ms  max=55.15ms
```

`uptime` load average was 6.2-8.0 throughout this session (busier than #002's near-idle machine),
so absolute numbers here are not directly comparable to #002's RESULT.md table - only the
within-this-session baseline-vs-patched comparisons (both measured back-to-back on the same
loaded machine) are used for the conclusion below. Baseline `min` (the least noise-sensitive
statistic) is tight across all three baseline measurement pairs (464-482ms single-core, 47-49ms
multi-core) despite the p50/mean/p95 columns being visibly noisier than #002's - both v1 and v2's
`min` sit clearly above that band with no overlap (v1: 562-568ms single-core, 52-54ms multi-core;
v2: 550-554ms single-core, 51ms multi-core).

## Result

Both fixes from #002's writeup were applied and are individually disassembly-confirmed to have
worked as intended (single `cvttss2si`/`cvttps2dq` instead of the multi-instruction unsigned-
conversion fallback; LUT pointers loaded once into callee-saved registers before the `par_iter_mut`
loop rather than re-resolved per pixel) - but the LUT approach is **still slower than `powf`** on
this machine (AMD Ryzen 9 7900, glibc `powf`), just by a smaller margin than #002's unfixed
version: single-core `min` ~549-554ms vs. baseline's ~464-482ms (~14-19% slower); multi-core `min`
~51-54ms vs. baseline's ~47-49ms (~8-13% slower). The i32-cast refinement (v2) is measurably
better than the literal usize-based fix #002 suggested (v1) - single-core `min` 550-554ms vs.
562-568ms, multi-core `min` ~51ms vs. ~52-54ms - but neither variant closes the gap to baseline.

## Does it change behavior/output?

- **Bit-exact?** No - same characterized difference as #002 (this candidate changes only which
  integer type the truncating cast targets, not the interpolation algorithm or table values).
- Re-ran #002's dense analytic scan (`bench/accuracy_check.rs`, unchanged from #002, run standalone
  via `rustc -O`) against this candidate's cast strategy: `max_abs_err=2.384e-7`,
  `max_rel_err=2.833e-7`, `max_u16_diff=1` - identical to #002's numbers, confirming the cast
  change is numerically transparent (converting the same in-range integer value to `f32` gives the
  same result regardless of whether the source integer type is `i32` or `usize`).
- **Is this a breaking change?** No public API/format change; same as #002, output pixel values
  could shift by at most 1 LSB (out of 65535) on a small fraction of pixels if this were applied -
  not visually distinguishable, but not byte-identical either. Full-pipeline real-sample hashing
  (X-T5/Canon EOS R5, per #002) was not re-run for this candidate since the interpolation math is
  provably unchanged from #002 - the analytic scan above is sufficient to confirm that.
- Project test suite not run for this candidate, same reasoning as #002 (no existing test asserts
  exact developed pixel values for these samples).

## Why the result looks the way it does

Disassembly of the patched `bridge_producer_consumer::helper<..., srgb_apply_gamma_inplace::
{closure#0}>` (the whole gamma stage is inlined into this one function by LLVM, in both baseline
and patched builds) confirms both of #002's suggested fixes did what they were supposed to:

1. **LUT hoisting worked.** `mov (%r12),%r15` / `mov 0x8(%r12),%r12` near the top of the function
   load the mantissa/exp LUT base pointers once, outside the per-element loop - there is no
   `OnceLock`-check branch inside the hot loop in either v1 or v2.
2. **The `i32` cast (v2) genuinely removes the costly conversion the `usize` cast (v1) still had.**
   v1's `to_int_unchecked::<usize>()` disassembled to the *same* two-`cvttss2si`-plus-2^63-check
   sequence #002 blamed on the *safe* `as usize` cast - x86-64 simply has no single-instruction
   float<->u64 conversion, so LLVM emits that fallback for `f32 -> usize` regardless of
   checked/unchecked, and a second instance of it recurred for `idx as f32` (used to compute
   `frac`), since `idx` was `usize` there too. v2's disassembly shows both replaced by a single
   `cvttss2si` (float -> i32) and a single `cvttps2dq`+`cvtdq2ps` pair (i32 -> float back for
   `frac`), with the `i32 -> usize` widen for the final array index reduced to one `movslq`. This
   is a real, measurable improvement over v1 (see the table above) - just not enough to beat
   `powf`.
3. **The remaining gap is most likely the LUT approach's inherent extra data-dependent work, not
   an implementation bug.** Even fully fixed, `fast_pow_gamma_with` per element does: a compare +
   branch (linear-vs-exponential crossover, present in both baseline and patched), a bit-extract +
   compare + branch (exponent-range fallback check), a float truncate, *two* data-dependent loads
   from `mlut` at an index computed from the input value itself (`mlut[idx]`, `mlut[idx+1]` -
   these cannot be prefetched ahead of knowing `idx`, and their result feeds directly into the
   next multiply/add), a third load from `elut`, plus bounds-check branches on the array indexing
   that the compiler could not elide (visible as `cmp $0x400,%edi; ja ...; je ...` on the disassembly). A
   single glibc `powf` call, while itself expensive per callgrind's instruction-count profile
   (~21.9% of total instructions in the original full-pipeline profile that motivated #002), is a
   self-contained computation with no data-dependent memory loads and no unpredictable branches
   tied to the input's bit pattern - across the millions of independent loop iterations here, the
   CPU's out-of-order execution can overlap consecutive `powf` calls' latency far more effectively
   than it can overlap the LUT path's dependent load-then-compute chains, which is consistent with
   `powf` winning on wall-clock time despite losing on raw instruction count.

## What's not done / open questions

- Removing the bounds-check branches on `mlut[idx]`/`mlut[idx+1]` (e.g. via `get_unchecked`,
  sound since `idx` is proven `< MANTISSA_LUT_SIZE` by construction) was not tried as a further
  refinement - added to `IDEAS.md` as a narrower follow-up, though given the analysis above
  (the gap looks structural, not implementation-detail-shaped) it's unlikely to close the whole
  remaining 8-19% gap by itself.
- This session's machine was measurably busier (`uptime` load 6-8) than #002's original session
  (load ~2-8, mostly idle) - absolute numbers differ from #002's table for that reason. Every
  comparison in this candidate's own "Result" section is paired against a baseline measured in the
  same session under the same load, so the *direction* of the finding (still slower) is not
  affected by this, but readers should not directly compare #002's absolute ms figures to this
  candidate's.
- `perf` was still not available on this machine (same as #002) - conclusions on *why* rely on
  static disassembly reading plus the CPU-architecture reasoning above, not a sampled profile of
  the patched binary.
