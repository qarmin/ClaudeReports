# AVX2/SSE u8 unpremultiply: `rcp_ps`-approximation rounding gap vs. the scalar reference

Backlog row from `IDEAS.md` (sourced from candidate `009`): "SIMD-vs-scalar +/-1 rounding-tie-break
gap in u8 unpremultiply (AVX2 and SSE, both `WorkloadStrategy` variants)", described there as
`_mm_cvtps_epi32`/`_mm256_cvtps_epi32`'s round-to-nearest-even diverging from the scalar
reference's round-half-up formula specifically on exact `.5` ties. This candidate confirmed the
`+/-1` divergence is real, common, and reproducible via the crate's real public resize API - but
found the backlog's proposed *mechanism* was only a minor contributor. See "Root cause".

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Exhaustive direct-function differential test (temporary `#[cfg(test)]` module, reverted): every `(alpha, pixel)` pair in `1..=255 x 0..=255` (65280 combinations) fed to `Avx2DisassociateAlpha::disassociate_chunk` (`PreferQuality`), `Avx2DisassociateAlphaFast::disassociate_chunk` (`PreferSpeed`, default), and `sse_unpremultiply_row`, compared against `unpremultiplication_table()` (the crate's own scalar ground truth) | debug build, `cargo test --lib`, this host (avx2+sse4.1) | On pristine `HEAD` (`7eec7cf`, candidate `009`'s separate `packus_epi32`/`packus_epi16` signedness bug still present): AVX2-Quality 939 mismatches (maxdiff=255, dominated by 009's bug), AVX2-Fast 17563 mismatches (maxdiff=255). After manually re-applying candidate `009`'s already-recorded fix on top (isolating this candidate's own gap): AVX2-Quality 812 mismatches/maxdiff=**1**, AVX2-Fast 17436 mismatches/maxdiff=**1**, SSE 812 mismatches/maxdiff=**1** (SSE already had 009's fix's `_mm_packs_epi32` shape from the start, so no re-application needed there) - confirms the backlog's core claim (a real, bounded +/-1 gap, present in both instruction sets and both `WorkloadStrategy` variants) |
| Root-cause probe: standalone Rust program calling `_mm_rcp_ps`/`_mm256_rcp_ps` directly and comparing against the exact `1/x` value | release build, `rustc -O -C target-feature=+avx2` | `_mm_rcp_ps(2.0) = 0.4998779297` vs exact `0.5` (relative error ~1.2e-4, consistent with the ISA's documented ~12-bit-precision approximate reciprocal); for the `(alpha=2, pixel=1)` mismatch found above (scalar table says `128`, SIMD says `127`), the exact intermediate value is `1*255/2 = 127.5` (an exact tie) but `rcp_ps`'s approximation computes `127.4688...` - a value that already rounds down to `127` under ordinary nearest-rounding, **not** a round-to-even-vs-round-half-up tie-break disagreement (a correctly-computed `127.5` rounds to `128` under round-to-even too, since `128` is the even neighbor) | Falsified the backlog's proposed mechanism for this specific example: the divergence here is `rcp_ps`'s precision loss, not `cvtps_epi32`'s tie-breaking rule |
| Categorized every mismatch from the exhaustive probe (after 009's fix) as "exact `.5` tie" vs "not" (`(pixel*255) % alpha) * 2 == alpha`) | same debug build/test | AVX2-Quality/SSE: 585/812 (72%) exact ties, 227/812 (28%) not. AVX2-Fast: 615/17436 (3.5%) exact ties, 16821/17436 (96.5%) not. Confirms the tie-break framing explains only a minority (Quality/SSE) or a small fraction (Fast) of the actual divergence - the dominant mechanism is a genuine precision gap, not a rounding-rule mismatch | |
| Real end-to-end public-API repro (`repro/`, three builds of identical source differing only in the `pic-scale` dependency's Cargo features - `avx`+`sse` default, `sse`-only, neither/scalar - `Scaler::plan_rgba_resampling`, `premultiply_alpha: true`, Lanczos3, 4000->3800px, uncorrelated random RGBA content, both `WorkloadStrategy` variants, 5 seeds each) | release build, this host, pristine clone at `7eec7cfde2aed58a1606a48b5c7bd084536f9624` | 10/10 (strategy, seed) combinations reproduce: AVX2-PreferSpeed (crate default) diverges on ~36.5% of output color bytes, AVX2-PreferQuality/SSE on ~2% - both always by exactly 1, always confined to color channels (0 alpha-channel mismatches, cleanly isolating this from any alpha-handling bug), bit-for-bit deterministic across repeat runs of the same seed. Unlike candidate `009`'s own bug, this one needs no adversarial construction - plain random translucent content triggers it on a large fraction of pixels, and does **not** simultaneously trigger 009's separate bug (maxdiff never exceeds 1 in this repro, before or after 009's own fix) |

## Status

**Confirmed.** Deterministic and 100% repeatable: the exhaustive direct-function probe is
inherently deterministic (no randomness), and the public-API repro reproduces on 10/10
(strategy, seed) combinations, bit-for-bit identical across repeat runs of the same seed (verified
by re-running seed 1 twice and diffing the output - identical).

**Fully reproducible: Yes.** `repro/` drives the crate's real, fully public API
(`Scaler::plan_rgba_resampling` with `premultiply_alpha: true`) end-to-end on realistic (random,
uncorrelated-channel, translucent) RGBA content - no internal-function shim was needed (unlike
candidate `009`, whose narrower trigger condition required one). This is a single-participant
finding (one RGBA buffer handed to the crate's own public resize entry point, the same shape of
input a real caller would supply) - not a two-participant scenario, so `CONFIRMATION.md` section
11's "hand-constructed peer input" concern does not apply.

## Class

logic

## Root cause

Both `Avx2DisassociateAlpha::disassociate_chunk` (`WorkloadStrategy::PreferQuality`, AVX2) and
`sse_unpremultiply_row` (SSE, used by both `WorkloadStrategy` variants) compute each channel's
unpremultiplied value as `pixel * (255 * rcp_ps(alpha))`, where `rcp_ps`/`_mm256_rcp_ps` is x86's
hardware-approximate-reciprocal instruction (per Intel's SDM, ~12 bits of precision, relative
error up to ~1.5*2^-12 ~ 3.7e-4) rather than an exact division. This approximation error, not
`_mm_cvtps_epi32`'s round-to-nearest-even rounding mode (the mechanism the originating backlog
note proposed), is the dominant cause of the observed +/-1 divergence from the scalar reference's
exact `(pixel*255+alpha/2)/alpha` round-half-up formula - confirmed by directly probing
`rcp_ps`'s output against the exact reciprocal (see "What was tried") and by the tie/non-tie
breakdown, where non-tie mismatches (unexplainable by any rounding-mode difference, since a
correctly-computed non-tie value rounds the same way under either rule) dominate, especially for
the `PreferSpeed`/AVX2-Fast path (96.5% non-tie).

`Avx2DisassociateAlphaFast::disassociate_chunk` (`WorkloadStrategy::PreferSpeed`, the crate's own
**default** strategy) has a related but structurally different root cause: it never calls
`rcp_ps` on the per-pixel value at all, instead computing a shared Q16.16 fixed-point reciprocal
approximation (`recip_i32 = round(65536 * rcp_ps(1/alpha))`, itself `rcp_ps`-imprecise) once, then
applying it to all three channels via `_mm256_mulhi_epu16` (a multiply-then-shift-right-16
fixed-point division approximation). This fixed-point scheme has its own, larger, inherent
quantization error independent of `rcp_ps`'s contribution (confirmed by a standalone simulation
swapping in an exactly-computed `65536/alpha` reciprocal: mismatches only dropped from 16931 to
16919 out of 65280, a ~0.07% change) - see "What's not done" for why this path's own error source
was not fixed this session.

The observable divergence stays bounded to `+/-1` in practice (not the much larger raw arithmetic
error `rcp_ps`'s ~3.7e-4 relative error can produce, up to ~16 absolute for `alpha=1`) only because
the crate's own saturating pack instructions (`_mm256_packus_epi16`/`_mm_packus_epi16`, and
`_mm256_packs_epi32`/`_mm_packs_epi32` upstream of it) absorb any error that pushes an
already-large value further past the `255` clamp threshold - both the exact and the approximate
result saturate to the same `255` in that region, hiding the larger raw error.

## The fix

Replaced the `rcp_ps`-approximated-reciprocal-then-multiply with an exact division
(`_mm256_div_ps`/`_mm_div_ps`, IEEE correctly-rounded) plus an explicit `+0.5` bias truncated
towards zero (`_mm256_cvttps_epi32`/`_mm_cvttps_epi32`) instead of the default-rounding-mode
`_mm256_cvtps_epi32`/`_mm_cvtps_epi32` - `floor(x + 0.5)` for a non-negative `x` is exactly the
scalar reference's own round-half-up rule, so this closes both the precision gap and the
tie-breaking-rule gap at once. Applied to:

- `Avx2DisassociateAlpha::disassociate_chunk` (AVX2, `WorkloadStrategy::PreferQuality`) in
  `src/avx2/alpha_u8.rs`.
- `sse_unpremultiply_row` (SSE, both `WorkloadStrategy` variants) in `src/sse/alpha_u8.rs`.

**Not applied** to `Avx2DisassociateAlphaFast::disassociate_chunk` (AVX2,
`WorkloadStrategy::PreferSpeed`, the crate's default) - see "What's not done".

Verified two ways:

1. Exhaustive direct-function probe (same 65280-combination sweep as "What was tried"), re-run
   against the patched tree: AVX2-Quality and SSE both reach **0 mismatches** (bit-exact match
   with the scalar reference across the *entire* domain, not just the inputs originally found
   mismatching - satisfying `CONFIRMATION.md` section 7). This holds independent of candidate
   `009`'s own fix (checked both with and without 009's fix manually re-applied on top - this
   candidate's fix and 009's are orthogonal; without 009's fix, 127 mismatches remain, all with
   `maxdiff=255`, i.e. 100% attributable to 009's separate, still-present, out-of-scope-here bug,
   not this one).
2. `repro/`'s real public-API comparison, re-run against the patched clone (see `output.txt`,
   "AFTER PATCH" section): `WorkloadStrategy::PreferQuality` drops from 5/5 to **0/5**
   reproducing (AVX2 and SSE both bit-exact against the scalar build); `WorkloadStrategy::PreferSpeed`
   (default) stays at 5/5 on AVX2 (expected - unfixed, see below) but SSE (which ignores
   `WorkloadStrategy` entirely and always uses the now-fixed `sse_unpremultiply_row`) drops to 0
   mismatches there too.

- Project test suite: **66 passed, 0 failed** both before and after the fix (`cargo test --lib`,
  default features; a temporary diagnostic/probe test module used during investigation is excluded
  from this count and was reverted before finalizing, matching candidate `009`'s own convention).
  No regression; no pre-existing test's behavior changed.

## What's not done / open questions

- **`Avx2DisassociateAlphaFast` (`WorkloadStrategy::PreferSpeed`, this crate's own default) is
  still not bit-exact against the scalar reference.** Its fixed-point Q16.16 mulhi-based algorithm
  is a genuinely different, faster-but-lossier scheme, not just an imprecise-reciprocal instance
  of the same shape as the two paths fixed here - a standalone simulation (see "Root cause")
  showed replacing only its `rcp_ps`-based reciprocal step with an exact one barely moves its
  mismatch count (16931 -> 16919 out of 65280). Closing this path's own gap fully would mean
  replacing its whole algorithm (e.g. with the same exact-division approach used here, at a real
  throughput cost, defeating the purpose of a separate "fast" path) rather than a bounded,
  one-line-shape patch - a decision this session left to the project maintainer rather than making
  unilaterally. This path therefore still reproduces the bug after this candidate's fix (see
  `output.txt`'s "AFTER PATCH" section and the `justfile`'s `reproduce-after` comment for the
  precise, expected shape of that remaining reproduction).
- Only u8 RGBA unpremultiply was checked. u8 premultiply (the opposite direction) uses a different,
  exact-integer `avx2_div_by255`/`_mm_div_by_255_epi16` helper in both backends (grep-confirmed, no
  `rcp_ps`/`cvtps_epi32` involved) and was not re-examined in depth here since it doesn't share
  this bug's `rcp_ps`-based shape. u16/f32/f16 alpha handling and gray-alpha (LA) variants were not
  re-examined either - u16's own, different, already-documented f32-reciprocal-vs-exact-division
  gap is candidate `005`'s separate backlog item.
- `src/neon/alpha_u8.rs` was not checked for an analogous `vrecpeq_f32` (NEON's own
  approximate-reciprocal instruction, roughly comparable precision to x86's `rcp_ps`) rounding gap
  - this host has no ARM hardware to exercise NEON natively, the same limitation noted by several
    prior candidates' own NEON follow-ups.
