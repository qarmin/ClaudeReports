# plan_resize_ar30 Ar30DestructuringImpl size-arithmetic audit

Backlog follow-up noted at the end of candidate `021.plan-entrypoint-coverage-source-size-overflow`:
`Scaler::plan_resize_ar30` (src/scaler.rs), immediately after the `plan_rgb_resampling16`
weights-planning step candidate 021 covered, constructs an AR30/RA30-specific decomposer
(`Ar30DestructuringImpl`) and, on hosts/builds without AVX2, falls through to `Ar30Plan`
(src/plan/ar30_plan.rs) - a code path never independently checked for its own size arithmetic.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of `src/plan/ar30_plan.rs` and `src/scaler.rs::plan_resize_ar30` to map every unchecked size computation reachable from the public `plan_ar30_resampling`/`plan_ra30_resampling` entry points | - | Found `ScratchLayout::new` (`src/plan/ar30_plan.rs`) computes `source_size.height * source_size.width * 3 * size_of::<u16>() + 2` (and the equivalent for `target_size`) as plain unchecked `usize` arithmetic, used to size the intermediate RGB16 scratch buffer for AR30 decompose/compose. Unlike `destination_size`, `source_size` is never passed through `check_image_size_overflow` anywhere on this path - `plan_rgb_resampling16` (called first inside `plan_resize_ar30`) only validates `destination_size` (confirmed by reading `DefaultPlanner::plan_generic_resize`, src/plan/default_planner.rs) |
| Manual boundary-value construction: a temporary `examples/ar30_scratch_overflow_probe.rs` in the main checkout (reverted afterward), built with `--no-default-features --features threading` (forces the scalar `Ar30Plan` fallback on this AVX2-capable host, since the AVX2-specialized branch in `plan_resize_ar30` never constructs `Ar30Plan` at all), calling the real `plan_ar30_resampling` then the real `resample()` entry point with `source_size = (10_000_000_000_000_000_000, 100)` (chosen so `width*height*3*2` overflows `usize`), `destination_size = (100, 100)`, `ResamplingFunction::Spline16` (a fixed-kernel filter so `generate_weights`'s own already-audited `kernel_size * out_size` multiply - candidates 010/021 - stays tiny regardless of how huge `source_size` is, isolating this candidate's distinct arithmetic) | `cargo run --example`, debug build (Cargo default `dev` profile, `overflow-checks = true`), this host | 3/3 runs panicked identically: `thread 'main' panicked at src/plan/ar30_plan.rs:129:29: attempt to multiply with overflow`, reached via the real `resample()` call (which internally calls `scratch_size()` before any store validation) |
| Same input, release build (`cargo run --release`, `overflow-checks` off by default) | same host | No panic; `width*height*3*2` wraps to `4808176044395784806`, which `try_reserve_exact` inside the `try_vec!` macro then fails to allocate, so `resample()` returns `Err(PicScaleError::OutOfMemory(..))` instead of crashing - a different, input-dependent outcome from the debug-build panic (see "What's not done" below) |

No further escalation needed (soft cap of 3 not exceeded) - the debug-build panic was deterministic
and unambiguous, and code reading fully explains the mechanism.

## Status

**Confirmed.** Deterministic panic, 3/3 repeat runs, same debug build (Cargo default `dev`
profile, `overflow-checks = true`), same host, same input, no randomness.

**Fully reproducible: Yes.** `repro/` calls the real, unmodified public API
(`Scaler::plan_ar30_resampling` then `ResamplingPlan::resample`) with plain `usize`/`ImageSize`
values a real caller could pass directly - no internal-only shim, no hand-authored bytes standing
in for another participant's output. Single-participant finding (a caller passing raw integers to
a public constructor/entry point, then calling the documented `resample()` method); `CONFIRMATION.md`
section 11's "two participants" bar does not apply.

## Class

crash

## Root cause

`Scaler::plan_resize_ar30` (src/scaler.rs) calls `self.plan_rgb_resampling16(source_size,
destination_size, 10)` to build the inner u16/3-channel resampling plan. That call validates only
`destination_size` for overflow (`DefaultPlanner::plan_generic_resize` ->
`check_image_size_overflow(destination_size.width, destination_size.height, N, size_of::<T>())`,
src/plan/default_planner.rs) - `source_size` is passed through unchecked. On hosts/builds where the
AVX2-specialized branch in the same function doesn't apply (no `avx` feature, not x86_64, or AVX2
not detected at runtime), `plan_resize_ar30` falls through to constructing an `Ar30Plan`
(src/plan/ar30_plan.rs), whose `ScratchLayout::new` computes:

```rust
src_rgb16_size: plan.source_size.height * plan.source_size.width * 3 * size_of::<u16>() + 2,
tgt_rgb16_size: plan.target_size.height * plan.target_size.width * 3 * size_of::<u16>() + 2,
```

both as plain unchecked `usize` multiplication/addition (no `checked_mul`, no `try_from`). This
arithmetic runs every time `scratch_size()` is called - which the real `resample()` entry point
does internally (`Ar30Plan::resample`, line ~154: `try_vec![u8::default(); self.scratch_size()]`)
before any source/destination store is even validated. A caller who passes a sufficiently large
`source_size` (e.g. from untrusted metadata, a malformed header, or simply a very large real
image) reaches this multiplication with no prior bounds check anywhere on the path, causing:

- a debug-build (`overflow-checks = true`, Cargo's own default `dev` profile) panic, or
- a release-build silent `usize` wraparound to an arbitrary smaller value, which then either fails
  a subsequent `try_reserve_exact` (observed here: a graceful `OutOfMemory` error) or, for a
  different wrapped value, could produce an undersized scratch buffer used for subsequent slicing
  (not exercised further this session - see "What's not done" below).

This is a distinct bug from candidates 010/021: those cover `kernel_size * out_size` inside
`generate_weights` (src/math/weights.rs), which depends on the source/destination *ratio* via
`scale`, not on `source_size` alone, and is already `checked_mul`-protected by candidate 010's fix.
`Ar30Plan::ScratchLayout`'s arithmetic depends only on the raw `source_size`/`target_size`
magnitudes and has no equivalent protection - a huge `source_size` paired with a modest,
non-extreme-ratio `destination_size` (so `generate_weights`'s own kernel-size-driven overflow never
triggers) sails straight through to this unguarded multiplication.

## The fix

Added an explicit `check_image_size_overflow(source_size.width, source_size.height, 3,
size_of::<u16>() as isize)` guard in `Scaler::plan_resize_ar30` (src/scaler.rs), immediately before
the existing `plan_rgb_resampling16` call, mirroring the same check already used for
`destination_size` inside `DefaultPlanner`/`AlphaPlanner`. On overflow it now returns
`Err(PicScaleError::Generic("Source width or height overflows isize"))` instead of ever
constructing an `Ar30Plan` whose `ScratchLayout` could overflow.

Verified against more than the one triggering input: re-ran the same debug-build probe (3/3 runs)
against the patched tree - `plan_ar30_resampling` now returns a clean `Err` instead of panicking,
confirmed via both the standalone `scratch_size()` getter and the full `resample()` call path.
Also re-ran the release-build variant against the patched tree: also returns the same clean `Err`
(the wraparound/`OutOfMemory` fallback path is no longer reached at all, since the new check now
runs unconditionally regardless of build profile).

- Project test suite (`cargo test --lib`, default features): **66 passed, 0 failed** both before
  and after applying the patch (isolated via toggling only `src/scaler.rs`). No regression.

## What's not done / open questions

- The release-build wraparound case where `width*height*3*2` wraps to a *small* (not huge) value
  was not constructed and exercised this session - only the one input tried happened to wrap to a
  huge value that `try_reserve_exact` rejected gracefully. A wrap-to-small value could plausibly
  produce an undersized-but-successfully-allocated scratch buffer, which downstream `split_at_mut`/
  `as_chunks` calls in `decompose_rows`/`compose_rows` (src/plan/ar30_plan.rs) would then need to
  handle - Rust slice operations are bounds-checked so this would most likely surface as a
  different, safe panic (index/slice-length mismatch) rather than memory-unsafety, but this was not
  independently confirmed.
- `plan.inner_filter.scratch_size()` (the first field evaluated in `ScratchLayout::new`, from
  whichever concrete plan `plan_rgb_resampling16` returned - e.g. `TrampolineFiltering`,
  `HorizontalConvolvePlan`) has its own independent unchecked size arithmetic (observed directly:
  `TrampolineFiltering::scratch_size()` in src/plan/trampoline_filtering.rs panics identically at
  `self.source_size.width * 4.min(self.target_size.height) * N` for a `source_size` large enough
  even when both axes need scaling). This candidate's fix happens to also close that path for the
  AR30 entry points specifically (since it rejects the same oversized `source_size` before
  `plan_rgb_resampling16` is even called), but `TrampolineFiltering`/other generic plan types are
  reachable from every other `plan_*_resampling` entry point too (`plan_rgba_resampling`,
  `plan_rgb_resampling16`, etc. directly, not just through AR30) - not independently re-checked or
  fixed at its own source this session, since it's out of this candidate's AR30-specific scope. This
  is a strong new lead for a follow-up candidate.
- The AVX2-specialized branch of `plan_resize_ar30` was not itself examined for its own unchecked
  size arithmetic (it doesn't use `Ar30Plan`/`ScratchLayout` at all - it returns
  `BothAxesConvolvePlan`/`HorizontalConvolvePlan`/`VerticalConvolvePlan`/`NoopPlan`, which are
  generic non-AR30-specific plan types already in scope for the `TrampolineFiltering` lead above).
