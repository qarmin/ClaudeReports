# SIMD re-implementations of src/math/*.rs kernel formulas

Backlog entry (`IDEAS.md`, added 2026-08-25, source: candidate `002`'s own "what's not done"
note): candidate `002` differential-tested every closed-form resampling-kernel formula in
`src/math/*.rs` (Gaussian, Lanczos/jinc, Bartlett/Bartlett-Hann, Blackman, Kaiser, Welch, Hann/
Hamming/Hanning, cubic/bicubic, quadric, spline_n, lagrange) and found no bug, but explicitly
noted it never checked whether `src/avx2`, `src/avx512`, `src/sse`, `src/neon` contain their own
*hand-vectorized re-implementation* of any of these same formulas (as opposed to SIMD code that
just consumes a scalar-computed weight table) - a second, independent implementation of the same
math could diverge from the scalar reference without any test ever comparing them.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Structural/data-flow check: find every definition and every caller of `generate_weights` (the one function in the whole crate that actually evaluates a kernel formula - `src/math/weights.rs:112`) | `grep -rn` across `src/`, current HEAD (`7eec7cf`) | All 6 call sites of `generate_weights` are inside `src/math/weights.rs` itself (the `pub fn` wrappers `generate_weights_*`); zero call sites anywhere under `src/avx2`, `src/avx512`, `src/sse`, `src/neon` |
| Check what type SIMD backends actually receive: grep every SIMD file for the `FilterWeights` struct name (what `generate_weights` produces - a plain array of already-computed `i16`/`f32`/`f64` coefficients, defined in `src/filter_weights.rs`) | `grep -rln` across `src/avx2`, `src/avx512`, `src/sse`, `src/neon` | 65 of 124 SIMD `.rs` files reference `FilterWeights` directly as a plain data parameter to their convolution/pass functions (e.g. `src/avx2/rgb_u8.rs`'s `fn pass(&self, src: &[u8], dst: &mut [u8], filter_weights: &FilterWeights<i16>)`); a broader pattern also matching `crate::filter_weights` module references (not just the struct name, e.g. helper conversion functions taking a raw `&[f32]`/`&[i16]` weight slice instead of the struct itself) hits 92 of 124 - either way, every SIMD convolution kernel found is a multiply-accumulate over precomputed weight data, not a formula evaluator |
| Search every SIMD file for a locally-defined function named after any of the 15 kernel formulas (`fn gaussian`, `fn lanczos`, `fn kaiser`, `fn welch`, `fn bartlett`, `fn blackman`, `fn hann`/`hamming`/`hanning`, `fn cubic`/`quadric`/`spline`/`lagrange`/`bohman`/`sphinx`/`bc_spline`) | `grep -rn` across `src/avx2`, `src/avx512`, `src/sse`, `src/neon` | 0 matches - no SIMD file defines any of these functions |
| Search every SIMD file for any transcendental-math call a hand-vectorized kernel formula would need (`consts::PI`, `.exp(`, `.cos(`, `.sin(`, `.sqrt(`, `f_jincpi`/`f_sincpi` (the external `pxfm` functions `jinc.rs`/`sinc.rs` delegate to), `bessel`) | `grep -rn` across `src/avx2`, `src/avx512`, `src/sse`, `src/neon` | 0 matches anywhere - no SIMD file performs any transcendental-math computation at all |

## Status

**Not reproducible.** There is no hand-vectorized re-implementation of any `src/math/*.rs` kernel
formula anywhere in `src/avx2`, `src/avx512`, `src/sse`, or `src/neon` to have diverged in the
first place - the candidate's premise (a second, independently-derived closed-form implementation
existing in the SIMD backends) does not hold in the current source tree. Every SIMD backend is
exclusively the "SIMD paths that just call the scalar weight table" case the candidate description
itself carved out as already out of scope; it is not an unchecked third case. Four grep-based
structural checks (call-site enumeration of `generate_weights`, `FilterWeights` consumer count,
kernel-function-name search, transcendental-math-call search), covering all 124 `.rs` files across
the four SIMD backends, converge on the same conclusion, so no further tool (miri/ASAN/fuzzing)
was warranted - there is no numeric formula in this code to differentially test against a
reference in the first place.

**Fully reproducible**: N/A (Not reproducible - no repro applicable).

## Class

N/A (Not reproducible).

## Root cause

Not applicable to the "no bug found" result. For *why the candidate's suspicion doesn't apply*:
this crate's architecture computes every resampling-kernel weight exactly once, in scalar
(`f32`/`f64`) precision, in `src/math/weights.rs`'s `generate_weights`, which calls the per-filter
closed-form functions in `src/math/*.rs` (the files candidate `002` already checked) and stores
the results in a `FilterWeights` struct. Every SIMD backend (AVX2, AVX512, SSE, NEON) receives
that same `FilterWeights` data as a plain parameter and performs only vectorized
multiply-accumulate/convolution over it - there is no second code path anywhere that re-derives a
kernel value from `x`/`support`/`sigma`/etc. inside SIMD code, so there is nothing for a
scalar-vs-SIMD kernel-formula divergence to occur in. (The SIMD backends do have their own,
separately-audited correctness issues in the alpha premultiply/unpremultiply arithmetic - see
candidates `004`, `006`-`009`, `011` - but those are downstream integer/float rounding bugs in
alpha math, not kernel-formula re-implementations, and are out of this candidate's scope.)

## The fix

No fix - no bug confirmed, no source change made or needed.

- Project test suite: not run (no source or test change was made this candidate; the check was
  purely structural/read-only via `grep`).

## What's not done / open questions

- This candidate only checked for a *hand-vectorized formula re-implementation*. It did not
  re-audit the SIMD convolution/multiply-accumulate code itself (the part that *does* exist and
  *does* consume the weight data) for a different kind of correctness bug (e.g. an accumulator
  overflow, an off-by-one in kernel-window indexing, a rounding mismatch in how a `FilterWeights<i16>`
  fixed-point weight is applied vs. the scalar `f32`/`f64` reference) - that is a distinct suspect
  class from "wrong closed-form formula" and worth its own candidate if not already covered
  elsewhere in `SUMMARY.md`.
- `src/neon/weights.rs` exists (unlike the other three SIMD backends, NEON has its own
  `weights.rs` file); read directly (86 lines) as a follow-up check: it defines only
  `convert_weights_to_f16_impl`/`convert_weights_to_f16_fhm`, a plain `f32 -> f16` cast/repack of
  already-computed weight values for NEON's half-precision lane width - no kernel-formula
  computation, consistent with every other SIMD backend.
