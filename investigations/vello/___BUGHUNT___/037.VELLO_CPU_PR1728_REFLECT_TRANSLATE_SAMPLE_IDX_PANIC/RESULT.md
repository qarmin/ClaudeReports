# vello_cpu: `Extend::Reflect` + translating transform panics in `Pixmap::sample_idx` (specific to open PR #1728)

Found incidentally while investigating perf candidate
`___OPTIMIZATIONS___/011.VELLO_CPU_AVOID_SPLAT_POS_PER_CHUNK`, which measured open upstream PR
[linebender/vello#1728](https://github.com/linebender/vello/pull/1728) ("vello_cpu: Avoid
continuously splatting positions"). That PR changes how `vello_cpu`'s image/gradient painters
compute the current sample position per pixel-chunk (from a per-chunk `splat_pos` off a scalar
`f64`/`Point`, to a SIMD position advanced once per chunk via `i * step + base` arithmetic). This
is a **bug specific to that not-yet-merged PR's own patch** - it does not reproduce against
current upstream `HEAD` (`75168314`, this repo's `main`) without the PR's patch applied.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Run the existing failing `vello_sparse_tests` snapshot test in a worktree with PR #1728's patch applied | `cargo test --release -p vello_sparse_tests --test tests -- image::image_bilinear_with_translation_cpu_u8_avx2 --exact` | Panics: `index out of bounds: the len is 4 but the index is 4294967293` in `Pixmap::sample_idx` |
| Standalone `repro/` using `vello_cpu`'s public API directly (no test harness), same image/transform/extend combination | `cargo run --release`, patch applied | Panics identically (`repro/output_after_patch.txt`) |
| Same standalone `repro/`, patch reverted (pristine `HEAD`) | `cargo run --release`, no patch | No crash, exits 0, renders a 100x100 pixmap (`repro/output_before_patch_pristine.txt`) |

## Status

**Confirmed** - deterministic, reproduces every run (checked twice independently: once via the
project's own test suite, once via a from-scratch standalone repro using only `vello_cpu`'s public
`RenderContext`/`Image`/`ImageSampler` API) against the PR-#1728-patched tree, and confirmed absent
against the identical pristine tree with the patch reverted.

**Fully reproducible**: **Yes** - `repro/src/main.rs` drives the real public `vello_cpu` API
end-to-end (`RenderContext::set_paint`/`fill_rect`/`flush`/`render`), the same entry point any real
caller would use, with no internal test-harness shortcuts.

## Class

crash

## Root cause

`sparse_strips/vello_cpu/src/fine/common/image.rs`'s `extend()` function has an `Extend::Reflect`
branch that nudges the reflected sample position by a bit-level integer trick
(`m_bits - bias_in_ulps.to_int()`), whose own code comment states it relies on an invariant: since
the reflection point (`max`) is always an integer, the fractional/mantissa part (`m_bits`) must be
exactly `0` whenever a `bias_in_ulps` correction is applied. Before PR #1728, the per-chunk sample
position was rebuilt from a scalar `f64` via `splat_pos` at the start of every chunk; PR #1728
replaces this with an all-SIMD `f32` accumulation (`current = i * step + base`, computed once and
advanced every chunk via `mul_add`/`+=`). For the specific combination this repro uses
(`Affine::scale(5.0) * Affine::translate((10.0, 10.0))`, `Extend::Reflect`, `ImageQuality::Medium`
bilinear sampling of a 2x2 source image, AVX2), the new accumulation path produces a floating-point
result that is off by a rounding ULP relative to the old scalar-then-splat path, breaking the
"exactly `0`" invariant `extend()`'s reflect branch assumes. The resulting nudge then produces a
`u32` value one below `0` (i.e. `u32::MAX - 1`/`u32::MAX`, observed as `4294967293`/`4294967295`
across the two crashing sub-tests), which `Pixmap::sample_idx`
(`sparse_strips/vello_common/src/pixmap.rs:378`) indexes into a 4-element channel buffer with no
bounds check, panicking.

## Introduced by

Not applicable in the "which commit introduced this" sense - this bug does not exist in any
committed state of the `linebender/vello` repository. It is introduced by applying open PR
[#1728](https://github.com/linebender/vello/pull/1728)'s own patch (fetched via `gh pr diff 1728`,
same patch content as `___OPTIMIZATIONS___/011.VELLO_CPU_AVOID_SPLAT_POS_PER_CHUNK/patch.diff`, and
included here verbatim as `pr1728.diff`) on top of pristine `HEAD` (`75168314`, 2026-09-01). If
that PR is merged as-is, the commit that merges it would be the one to blame for this crash. The
pre-existing `extend()` reflect-branch invariant it breaks was itself introduced in the image
Extend-mode support commits (not independently bisected further here, since the invariant is only
*violated*, not created, by PR #1728 - the invariant's own origin is not the interesting fact for
this candidate).

## The fix

**Not attempted.** This candidate's job (a perf-optimize session measuring PR #1728) was to
measure and characterize, not to design a correctness fix for someone else's open PR. A real fix
would need one of:
- Making `extend()`'s `Extend::Reflect` bit-nudge tolerant of the new position-accumulation path's
  rounding (e.g. rounding to nearest integer before checking/assuming `m_bits == 0`, or reworking
  the nudge to not depend on that exact-integer invariant at all), or
- Reverting to a position-accumulation method that preserves whatever exact-integer property the
  old scalar-`f64`-then-`splat_pos` path happened to guarantee for this case, or
- At minimum, making `Pixmap::sample_idx` (or its caller) bounds-check/clamp instead of trusting
  the extend-mode math to always stay in range - which would convert this from a panic into
  (at worst) a wrong-but-safe pixel, and would also harden against any other not-yet-found input
  that breaks the same invariant.

No `patch.diff` is included, since no fix was found/verified this session - only `pr1728.diff`,
which is the *cause*, not a fix (see justfile).

- Project test suite: N/A for a fix (none attempted). For reference, applying `pr1728.diff` alone
  (no further changes) turns the following `vello_sparse_tests` tests from passing to panicking:
  `image::image_bilinear_with_translation_cpu_{f32,u8}_avx2`,
  `image::image_bicubic_with_translation_cpu_{f32,u8}_avx2` (4 tests; see
  `___OPTIMIZATIONS___/011.VELLO_CPU_AVOID_SPLAT_POS_PER_CHUNK/RESULT.md` for the full before/after
  test-suite counts across the whole PR).

## What's not done / open questions

- Only the AVX2 code path was checked (this machine is `x86_64`/AVX2); PR #1728 also touches the
  `sse4_2` and portable-fallback SIMD paths and NEON - not checked whether the same rounding
  mismatch reproduces there too (the two crashing sub-tests in the wider test suite are both
  `_avx2`-suffixed, so `sse4_2`/scalar may happen to round the same way here, or may simply not
  have been exercised the same way - not determined).
- Only one specific transform/extend/quality/image-size combination was tried (the one the
  existing snapshot test already happened to hit). Other `Extend::Reflect` + translating-transform
  combinations (different scale factors, `ImageQuality::High`/bicubic with a different-sized
  source image, `Extend::Reflect` on only one axis) were not swept - it's plausible more of the
  `Extend::Reflect` parameter space is affected than just this one input.
- Not fuzzed - a short `fuzz-bug-repro` campaign specifically targeting `extend()`'s
  `Extend::Reflect` branch (varying transform/scale/position) against PR #1728's patched tree could
  find the boundary condition precisely and/or other affected inputs, if this PR moves toward
  merging.
