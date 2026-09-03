# sRGB gamma powf() -> IEEE-754-decomposition LUT

`srgb_apply_gamma` (`rawler/src/imgop/srgb.rs`) called `v.powf(1.0 / 2.4)` per pixel per channel
in the final `ProcessingStep::SRgb` stage of `RawDevelop::default()`. Callgrind instruction-count
profiling from an earlier session attributed `__powf_fma` alone ~21.9% of total instructions on a
Canon EOS R5 CRAW full decode+develop run, making it look like the single most universally
applicable candidate in the backlog (every developed image, regardless of camera/CFA, goes
through this stage). This candidate replaces the call with an IEEE-754 mantissa/exponent
decomposition (`v = m * 2^e`), an interpolated 1024-entry lookup table for `m.powf(1/2.4)` over
`m in [1.0, 2.0)`, and an exact per-integer-exponent lookup table for `2^(e/2.4)`, falling back to
real `powf` outside a wide `[-16, 16]` exponent range.

Note: this candidate folder was assembled from a session that had already applied the patch and
run most of the correctness/measurement work before this pass picked it up (uncommitted changes
found already in the working tree, with scratch artifacts in `tmp/` and
`rawler/examples/srgb_gamma_hash.rs` - the accuracy/hash numbers below come from that earlier
work, re-verified; the isolated single-function timing below was re-measured fresh this pass since
the earlier session's own single-core criterion numbers were inconclusive/noisy, see below).

## What was tried

No existing bench isolates the sRGB gamma stage alone (`rawler/benches/demosaic.rs` stops at
`ProcessingStep::Demosaic`, before `SRgb`). Two measurement approaches were used:

1. **Isolated microbenchmark** (`bench/srgb_gamma_bench.rs`, run as a `rawler` example): calls
   `srgb_apply_gamma_inplace` directly on a 134,283,264-element `f32` buffer (matching the real
   Canon EOS R5 full-develop sample's element count), 20 iterations, first 2 discarded as warmup,
   fresh clone of the buffer per iteration (in-place mutation would otherwise change the input
   domain after iteration 1). This is the primary, low-noise number below.
2. **Full-pipeline criterion bench** (`rawler/benches/demosaic.rs`, extended with a
   `decoding-full-develop` group running `RawDevelop::default()` - i.e. demosaic + sRGB - on
   decoded Fuji X-T5 and Canon EOS R5 samples): left in the working tree by the earlier session,
   reused to sanity-check direction. Single-core (`taskset -c 0`) numbers here were very noisy
   (medians ranging 2631-3926ms across repeated runs of the *same* binary) and are **not** used as
   evidence either way - the machine's demosaic step dominates this bench's total time so heavily
   that the gamma stage's contribution is below this bench's noise floor at the sample sizes used
   (`sample_size(20)`). Not included as a numbered variant below for that reason.

| Variant | single-core p50 (isolated, `taskset -c 0`, `RAYON_NUM_THREADS=1`) | multi-core p50 (isolated, unpinned, 24 threads) | notes |
|---|---|---|---|
| baseline (`v.powf(1.0/2.4)`) | 450.90ms, 454.65ms (2 runs) | 47.46ms, 48.36ms (2 runs) | |
| patched (LUT) | 523.13ms, 524.01ms (2 runs) | 52.71ms, 52.94ms (2 runs) | |

(p50 over 18 timed iterations per run, after discarding 2 warmup iterations; `min`/`mean`/`p95`
were also recorded and move in the same direction - see raw output below.)

Raw output, single-core:

```
baseline run1: min=448.36ms p50=450.90ms mean=470.04ms p95=593.36ms max=593.36ms
baseline run2: min=450.29ms p50=454.65ms mean=492.05ms p95=603.73ms max=603.73ms
patched  run1: min=519.88ms p50=523.13ms mean=524.08ms p95=532.09ms max=532.09ms
patched  run2: min=521.10ms p50=524.01ms mean=525.31ms p95=538.63ms max=538.63ms
```

Raw output, multi-core (unpinned, machine has 24 threads / AMD Ryzen 9 7900):

```
baseline run1: min=45.89ms p50=47.46ms mean=47.45ms p95=51.60ms max=51.60ms
baseline run2: min=47.12ms p50=48.36ms mean=49.01ms p95=60.34ms max=60.34ms
patched  run1: min=52.04ms p50=52.71ms mean=53.36ms p95=58.99ms max=58.99ms
patched  run2: min=52.30ms p50=52.94ms mean=53.28ms p95=55.85ms max=55.85ms
```

`uptime` load average was 2.31/6.19/7.88 during the patched runs and similar during baseline runs
(machine otherwise idle-ish); both baseline and patched runs individually show tight, non-
overlapping spreads (run-to-run variation well under the baseline-vs-patched gap), so this isn't
noise-floor overlap.

## Result

The LUT approach is **slower**, not faster, on this machine (AMD Ryzen 9 7900, glibc `powf`):
single-core ~16% slower (451-455ms -> 523-524ms), multi-core (rayon-parallelized
`srgb_apply_gamma_inplace`, all 24 threads) ~10% slower (47-48ms -> 53ms). The gap is well outside
the run-to-run noise for both baseline and patched (each pair of repeat runs is internally tight).

## Does it change behavior/output?

- **Bit-exact?** No.
- Dense scan (20,000,000 samples) over the realistic domain `[0.00304, 3.0]` plus edge values
  (`accuracy_check.rs`, formerly `tmp/check_fastpow.rs`): `max_abs_err=2.384e-7`,
  `max_rel_err=2.833e-7` in the `f32` gamma output itself. Simulating the full downstream
  pipeline (gain/offset, clamp to `[0,1]`, scale to `u16`) on that same scan: **`max_u16_diff = 1`**
  (i.e. the final quantized output never differs by more than 1 LSB out of 65535 from the exact
  `powf`-based result, anywhere in the tested domain).
- On two real samples run through the full decode -> develop -> u16-quantize pipeline (Fujifilm
  X-T5 `X-T5_ISO_125_Bitdepth_14_lossless.RAF`, Canon EOS R5
  `Canon EOS R5_CRAW_ISO_100_nocrop_nodual.CR3`, via `rawler/examples/srgb_gamma_hash.rs` from the
  earlier session, output compared with `diff_u16.py`): X-T5 - 135,985 / 119,443,968 u16 samples
  differ (0.1138%), all by exactly 1 LSB; Canon R5 - 109,337 / 134,283,264 differ (0.0814%), all
  by exactly 1 LSB. No sample anywhere differed by more than 1 LSB on either camera.
- **Is this a breaking change?** No public API/format change; if applied, output pixel values can
  shift by at most 1 LSB (out of 65535) on roughly 0.08-0.11% of pixels, for the two samples
  checked - not visually distinguishable, but not byte-identical either.
- Project test suite not run for this candidate (no existing test asserts exact developed pixel
  values for these samples; the correctness signal above - dense analytic scan on the actual `f32`
  domain plus two real-camera end-to-end runs - was judged sufficient for a documented
  characterized-difference change per this workspace's discipline).

## Why the result looks the way it does

Disassembly (`objdump -d --demangle` on the built example) of the patched `srgb_apply_gamma`
shows two concrete costs the callgrind instruction-count profile didn't capture, both inside the
per-pixel hot path (this function is *not* inlined into the rayon `for_each` loop in either build -
that loop calls it through an indirect `call *%r12` either way, so that part isn't a differentiator
between baseline and patched):

1. **`scaled as usize`** (`let idx = scaled as usize;` in `fast_pow_gamma`) compiles to Rust's
   *saturating* float-to-int cast, not a single truncate instruction - the generated code is two
   `cvttss2si`s plus a chain of `cmovae`/`cmovbe`/sign-check branches to clamp to `[0, usize::MAX]`
   and handle NaN, per Rust's cast semantics since 1.45. This is materially more instructions and a
   longer dependency chain than the single hardware `powf` call it's meant to replace part of the
   cost of.
2. **`mantissa_lut()`/`exp_lut()`** are called fresh on every invocation of `fast_pow_gamma` (once
   per pixel per channel), and despite being backed by `OnceLock`, the generated code still does a
   `mov`+`test`+conditional-jump per call to check the "already initialized" flag before returning
   the table reference - a small but real per-pixel cost that a hoisted-once-per-call-site
   reference would avoid.

Both add real per-element latency/instruction-count that isn't visible in a profile that only
attributes cost to the `powf` call site being removed - the replacement code has its own,
non-trivial cost that ends up larger in wall-clock terms on this CPU, even though the *count* of
"expensive-looking" work (the actual transcendental call) went down. glibc's `__powf_fma` is
presumably well pipelined/ILP-friendly across independent loop iterations, which a scalar
dependent-load-per-element LUT lookup does not obviously beat.

## What's not done / open questions

- A revised LUT implementation avoiding the saturating-cast cost (e.g. `unsafe { scaled.to_int_unchecked::<usize>() }`, valid here since `scaled` is always in `[0, MANTISSA_LUT_SIZE]` by construction once the exponent-range check passes) and hoisting the `mantissa_lut()`/`exp_lut()` references once per outer call (e.g. passed in, or resolved once per `par_iter_mut` chunk rather than per element) was **not** tried as its own variant - added to `IDEAS.md` as a follow-up lead rather than tried in this pass, to keep this candidate to the one fix that was actually asked for.
- The full-pipeline criterion bench extension (`decoding-full-develop` group in
  `rawler/benches/demosaic.rs`) was left in the tree by the earlier session and used only as a
  sanity check (see "What was tried" above) - it is reverted along with everything else per this
  workspace's rule that the main tree always ends clean; a copy of the added group is not kept in
  this candidate folder since it produced no usable numbers (dominated by noise) and isn't needed
  to reproduce the numbers this candidate actually reports (`bench/srgb_gamma_bench.rs` is
  self-sufficient for that).
- `perf` was not available on this machine (`perf_event_paranoid` unrestricted but no `perf`
  binary installed) - the root-cause explanation above is from static disassembly reading, not a
  sampled/annotated profile of the patched binary.
