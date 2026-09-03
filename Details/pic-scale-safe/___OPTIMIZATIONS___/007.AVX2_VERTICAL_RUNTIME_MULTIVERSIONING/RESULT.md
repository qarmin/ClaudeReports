# Runtime AVX2 multiversioning of the vertical convolution tap loop

> **Project policy (2026-08-29): not adoptable.** The repo owner has confirmed unsafe code is
> never in scope in this crate, regardless of measured speedup - `#![forbid(unsafe_code)]` is a
> permanent design commitment, not a default to be relaxed for performance. This candidate is kept
> purely as a historical, fully-measured record of what unsafe SIMD intrinsics would buy. Unsafe/
> SIMD-intrinsics optimization work belongs in the sibling crate `~/Projekty/Obce/pic-scale`
> instead, never here.


**Ported from `image` repo's `___OPTIMIZATIONS___/048.PIC_SCALE_SAFE_AVX2_RUNTIME_MULTIVERSIONING/`.**
Originally measured in 2026-08 via a vendored `pic-scale-safe` 0.1.11 copy inside the `image`
project, built with **default** `RUSTFLAGS` (no special build flags). **Not re-measured in this
workspace** - numbers below are copied verbatim. The patch was rebuilt by hand against this repo's
current `HEAD` (`b50e020`); `git apply --check` passes, `cargo check --features rayon` compiles
with **zero warnings** (verified this session), and `cargo test --release --features rayon` passes
5/5.

**This is the single most important candidate in this workspace for a maintainer to read, because
of its policy tradeoff - see "Is this a breaking change?" below before anything else.**

Candidates 005/006 confirmed that rebuilding this crate with `-C target-feature=+avx2` (or
AVX-512) auto-vectorizes the vertical tap loop for a large win, but a blanket compile-time flag
would crash on non-AVX2 hosts, so neither is a safe default. This candidate implements the safe
alternative: runtime CPU-feature detection (`is_x86_feature_detected!("avx2")`) selecting an
explicit `#[target_feature(enable = "avx2")]` variant of the tap loop at runtime (function
multiversioning), with automatic, verified-correct fallback to the original scalar code on hosts
without AVX2.

## What was tried

For each of the three x86_64 tap-loop tiers in `fixed_point_vertical.rs` (8/16/32-column), the
body was extracted into a shared `#[inline(always)]` impl function (no target-feature attribute of
its own), with two thin wrappers: the original scalar path (unchanged) and a new `#[target_feature(
enable = "avx2")] unsafe fn ..._avx2`. The dispatcher computes `is_x86_feature_detected!("avx2")`
once per call and branches to the appropriate tier.

| Variant | single-core p50 | multi-core p50 | scenario |
|---|---|---|---|
| baseline (registry 0.1.11, default RUSTFLAGS) | 19.411 ms | 19.241 ms | Lanczos3, 4032x3024->400x300 |
| this candidate (default RUSTFLAGS) | 14.179 ms (**-27.0%**, 5/5) | 14.073 ms (**-26.9%**, 5/5) | same |
| baseline | 7.495 ms | n/a | Triangle |
| this candidate | 6.406 ms (**-14.5%**, 5/5) | n/a | same |
| baseline | 198.44 us | n/a | Lanczos3, small image |
| this candidate | 145.18 us (**-26.8%**, 5/5) | n/a | same |

## Result

**-27.0%/-26.9%** (single/multi-core) on the large Lanczos3 scenario, **-14.5%** on Triangle,
**-26.8%** on the small-image case - all far outside the same-binary noise floor (-0.7%), on a
completely normal `cargo build --release` with no special `RUSTFLAGS` needed. Smaller than
candidate 005's ~41% whole-crate rebuild because this candidate only widens the vertical tap loop
specifically, not the horizontal pass or floating-point paths - see candidate 008 for closing that
gap.

## Does it change behavior/output?

- **Bit-exact?** Yes, verified two ways in the original measurement: (1) 55 hashes, AVX2 path
  actually executing on an AVX2-capable host, matched the unmodified-crate baseline exactly; (2)
  `has_avx2` temporarily forced to `false` (not part of the shipped patch) to force the scalar
  fallback specifically - same 55 hashes, still matched. `objdump` confirmed real `ymm` codegen was
  genuinely exercised, not silently falling back.
- **Is this a breaking change? Yes, in one specific way that needs a conscious decision.** The
  convolution *output* is unchanged (bit-exact). But `lib.rs`'s `#![forbid(unsafe_code)]` had to be
  relaxed to `#![deny(unsafe_code)]` (a `forbid` lint cannot be locally overridden by `#[allow]`,
  unlike `deny`), with three narrowly-scoped `#[allow(unsafe_code)]` opt-outs (one per tap-loop
  tier's AVX2 wrapper). **This crate's name and its `forbid` lint are a stated "no unsafe, anywhere,
  ever" design commitment** - this patch's whole mechanism (`#[target_feature] unsafe fn`
  multiversioning) is fundamentally impossible to express as `forbid`-compliant safe Rust. A
  downstream consumer relying on this crate being unsafe-free (via `cargo geiger`, a
  workspace-wide `forbid(unsafe_code)` lint on dependencies, or simply this crate's documented
  guarantee) would need to consciously accept this tradeoff. This is a real, user-visible policy
  change, not just an implementation detail - the main reason this candidate is meaningfully
  riskier than every other candidate in this workspace.
- Project test suite, re-verified this session against current `HEAD`: `cargo test --release
  --features rayon` - **5 passed, 0 failed**. Original measurement additionally quoted `image`'s
  own suite: 622 passed, 1 pre-existing unrelated failure, identical with/without this patch.

## Why the result looks the way it does

`is_x86_feature_detected!("avx2")` reads a cached, cheap CPU-feature bitmask, and
`#[target_feature(enable = "avx2")]` lets LLVM emit real `ymm` instructions for that one function
while the rest of the crate stays at the generic baseline - giving the tap loop specifically the
same lane-width doubling candidate 005 found for the whole crate, without needing a non-default
build flag. The gap to candidate 005's ~41% is exactly the contribution of the other,
not-yet-multiversioned code paths (horizontal convolution, floating-point paths) - see candidate
008.

## What's not done / open questions

- **Only the vertical tap loop was multiversioned** - see candidate 008 for extending this to the
  horizontal pass and floating-point paths, which closes most of the remaining gap to candidate
  005's whole-crate number.
- **The `forbid` -> `deny` relaxation is the one thing a maintainer needs to decide about
  consciously**, not review as a mechanical diff. An alternative not explored here: publishing the
  AVX2 variant as a separate, clearly-unsafe companion crate/feature flag rather than loosening the
  base crate's own guarantee.
- No AVX-512 tier was added here - see candidate 009.
- AArch64/NEON was not touched - the existing `#[cfg(target_arch = "aarch64")]` tier is unaffected.
- Not independently re-benchmarked against current `HEAD` in this session - only correctness
  (compiles clean, zero warnings) and the test suite were re-verified; timing numbers are
  historical and directional only.
