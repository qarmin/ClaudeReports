# Add a third, AVX-512-gated tier to the vertical convolution's runtime dispatch cascade

> **Project policy (2026-08-29): not adoptable.** The repo owner has confirmed unsafe code is
> never in scope in this crate, regardless of measured speedup - `#![forbid(unsafe_code)]` is a
> permanent design commitment, not a default to be relaxed for performance. This candidate is kept
> purely as a historical, fully-measured record of what unsafe SIMD intrinsics would buy. Unsafe/
> SIMD-intrinsics optimization work belongs in the sibling crate `~/Projekty/Obce/pic-scale`
> instead, never here.


**Ported from `image` repo's `___OPTIMIZATIONS___/053.PIC_SCALE_SAFE_AVX512_RUNTIME_DISPATCH_TIER/`.**
Originally measured in 2026-08, built on top of candidates 007/008 (048/049 in `image`'s own
numbering). **Not re-measured in this workspace** - numbers below are copied verbatim. Patch
rebuilt by hand against current `HEAD` (`b50e020`) on top of candidate 008; `git apply --check`
passes, `cargo check --features rayon` compiles clean, `cargo test --release --features rayon`
passes 5/5. Same `forbid` -> `deny(unsafe_code)` tradeoff as candidates 007/008 applies (inherited,
not repeated).

**A build-configuration gotcha the original session found and worth carrying forward**: at the
time this was written in `image`'s workspace, `pic-scale-safe` had just republished as 0.1.12 on
crates.io (this repo's `Cargo.toml` here already says `version = "0.1.12"`, so this specific gotcha
does not apply to using this candidate's own patch directly against this repo - it only mattered
for `image`'s own `[patch.crates-io]` vendoring mechanism, which silently no-ops if the vendored
copy's declared version doesn't match what `Cargo.lock` resolves to). Recorded here so a future
session re-vendoring this crate elsewhere doesn't rediscover it from scratch.

Candidates 007/008 added AVX2 multiversioning; candidates 005/006 separately confirmed AVX-512
gives a further real ~6% win over AVX2 on the same vertical tap loop via a whole-crate rebuild, but
only via a compile-time flag. This candidate adds a third dispatch tier
(`has_avx512 -> has_avx2 -> scalar`) to the vertical dispatcher only (`fixed_point_vertical.rs`).

## What was tried

Added `#[target_feature(enable = "avx512f,avx512bw,avx512vl")] unsafe fn ..._avx512` wrappers
alongside the existing AVX2 ones from candidate 007, for the same three tap-loop tiers, calling the
same shared impl function. The dispatcher checks `has_avx512` first, then `has_avx2`, then scalar.

| Variant | single-core p50 | multi-core p50 | scenario |
|---|---|---|---|
| baseline (007+008's AVX2/scalar dispatch) | 11.743 ms | 11.554 ms | Lanczos3, 4032x3024->400x300 |
| this candidate (+AVX-512 tier) | 11.358 ms (**-3.3%**, 5/5) | 11.148 ms (**-3.5%**, 5/5) | same |
| baseline | 5.694 ms | n/a | Triangle |
| this candidate | 5.519 ms (**-3.1%**, 4/5) | n/a | same |
| baseline | 125.56 us | n/a | Lanczos3, small image |
| this candidate | 122.79 us (**-2.2%**, 4/5) | n/a | same |

A pinned single-shot criterion comparison independently confirmed the direction (**-4.2%**/-5.1%
across two runs, p<0.05); an *unpinned* attempt at the same comparison failed to detect any change
(p=0.66) purely from background system noise swamping this candidate's ~3% effect size - included
as a confidence caveat, not a contradicting result.

## Result

**Real but modest**: -2.2% to -3.5% on top of candidates 007/008's already-measured AVX2 win, 4-5
of 5 paired runs faster in every scenario. Smaller than candidate 006's own -6.0% whole-crate
AVX2-vs-AVX-512 rebuild delta, because this candidate only widens the vertical tap loop
specifically (candidate 010 extends AVX-512 to the horizontal/floating-point paths the same way
008 did for AVX2).

## Does it change behavior/output?

- **Bit-exact?** Yes, verified three ways in the original measurement: AVX-512 path executing for
  real (55 hashes matched), `has_avx512` forced off to exercise the AVX-512-to-AVX2 fallback edge
  (matched), and both `has_avx512`/`has_avx2` forced off to exercise the scalar fallback with the
  new tier's code present (matched). `objdump` confirmed real `zmm` codegen was genuinely exercised.
- **Is this a breaking change?** Same answer as candidates 007/008 - no additional crate-level lint
  change beyond what 007 already made; this candidate adds three more narrowly-scoped
  `#[allow(unsafe_code)]` sites.
- Project test suite, re-verified this session against current `HEAD`: `cargo test --release
  --features rayon` - **5 passed, 0 failed**. Original measurement additionally quoted `image`'s
  own suite: 622 passed, 1 pre-existing unrelated failure, identical with/without this patch.

## Why the result looks the way it does

Same mechanism as candidates 006/007 (a `#[target_feature]`-gated function selected via cached
runtime detection), wired into runtime dispatch instead of a whole-crate rebuild. The magnitude is
noticeably smaller than candidate 006's whole-crate AVX2-vs-AVX-512 delta because 006 widened the
*entire* crate under one blanket flag, while this candidate touches only the vertical tap loop -
the same narrower-scope-vs-whole-crate-rebuild gap candidate 007 itself showed relative to
candidate 005.

## What's not done / open questions

- **Only the vertical tap loop got an AVX-512 tier** - see candidate 010 for extending it to the
  horizontal/floating-point paths.
- This candidate's effect size (-2% to -3.5%) is small enough to be noise-sensitive on a loaded
  host (an unpinned measurement attempt failed to detect it at all) - expect more run-to-run
  variance re-measuring this than candidates 007/008/005/006's much larger effect sizes.
- AVX-512 downclocking was not directly instrumented (wall-clock only, one host's CPU).
- Not independently re-benchmarked against current `HEAD` in this session - only correctness and
  the test suite were re-verified; timing numbers are historical and directional only.
