# plan_*_resampling entry-point coverage of the source_size overflow-panic fix (candidate 010 follow-up)

Backlog follow-up to candidate `010.public-entrypoint-weights-len-overflow-panic`: that candidate
confirmed and fixed an unvalidated-`source_size`-into-`generate_weights` overflow panic
(`kernel_size * out_size` in `src/math/weights.rs`), but only actually exercised it through
`Scaler::plan_rgba_resampling` (Lanczos3, u8). `src/scaler.rs` exposes 17 public `plan_*` entry
points across 5 pixel formats x {u8, u16/i16, f32} x {DefaultPlanner, AlphaPlanner} x
{PreferQuality, PreferSpeed} plus AR30/RA30, all reached by code reading through the same
unvalidated path - never independently re-verified per entry point. This candidate drives the
identical overflow-adjacent input through every one of them.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: a temporary `examples/entrypoint_overflow_probe.rs` in the main checkout (reverted afterward) called all 17 public `plan_*` entry points (24 concrete invocations counting premultiply-alpha true/false, PreferQuality/PreferSpeed, and an AR30 Nearest-bypass control) with the same `source_size=(1<<62, 1)`, `destination_size=(2, 1)` pair that panicked candidate 010's narrower repro, on the pristine (unfixed) tree | `cargo run --example`, debug build (Cargo default `dev` profile, `overflow-checks = true`), this host | 23/24 invocations panicked with the identical message (`attempt to multiply with overflow` at `src/math/weights.rs`), 2/2 repeat runs byte-identical. The one exception, `plan_ar30_resampling`/`Ar30ByteOrder::Host` under `ResamplingFunction::Nearest`, correctly returned `Ok`/`Err` without panicking - by code reading (`src/scaler.rs::plan_resize_ar30`), `Nearest` short-circuits to `ResampleNearestPlan` before ever calling `generate_weights`, so this is expected, not a gap |
| Same 24 invocations re-run against the tree with candidate 010's `patch.diff` applied (`checked_mul` fix in `src/math/weights.rs`) | same debug build, same host | 0/24 panicked - the existing fix covers every entry point tested, not just `plan_rgba_resampling` |
| Standalone `repro/` (own `Cargo.toml`, bare `[workspace]`, `pic-scale` from a fresh clone) built and run via `just test` against a fresh clone at the base commit, release build with `overflow-checks = true` set explicitly in `repro/Cargo.toml` | `cargo run --release`, fresh clone, this host | Reproduced the same 23/24 before `patch.diff` is applied, 0/24 after - confirms the finding isn't specific to the in-tree debug-profile setup |

No further escalation needed (soft cap of 3 not exceeded) - the signal was consistent and
unambiguous across every approach tried, and code reading fully explains the one non-reproducing
case (`Nearest` bypasses `generate_weights` entirely for every entry point, not just AR30/RA30).

## Status

**Confirmed.** Deterministic panic across 23 of 24 tested `plan_*` entry-point invocations, 2/2
repeat runs in-tree plus a fresh-clone `just test` run (3/3 total per invocation, same debug/release
builds, same inputs, no randomness). This does not establish a *new* distinct bug beyond candidate
010's - it establishes that the *same* bug and the *same already-existing fix* apply uniformly
across every public resize entry point, closing the specific "was this actually re-verified"
question the backlog row raised.

**Fully reproducible: Yes.** `repro/` calls the real, unmodified public API (all 17
`Scaler::plan_*_resampling*` methods, 24 concrete invocations) with plain `usize`/`ImageSize`
values a real caller could pass directly - no internal-only shim, no hand-authored bytes standing
in for another participant's output. Single-participant finding (a caller passing raw integers to
a public constructor/entry point); `CONFIRMATION.md` section 11's "two participants" bar does not
apply.

## Class

crash

## Root cause

Identical to candidate 010's: `DefaultPlanner::plan_generic_resize` and
`AlphaPlanner::plan_generic_resize_with_alpha` (`src/plan/default_planner.rs` /
`src/plan/alpha_planner.rs`) both validate `destination_size` via `check_image_size_overflow`
before planning but never validate `source_size` at all; `source_size.{width,height}` reaches
`T::make_weights` -> `generate_weights` (`src/math/weights.rs`) unchecked, where
`kernel_size * out_size` is a plain unchecked `usize` multiply that overflows for a sufficiently
large `source_size`/`destination_size` ratio. Every `plan_*` entry point in `src/scaler.rs`
ultimately calls one of these two functions (confirmed by reading all 17, not just tracing
`plan_rgba_resampling`): the u8/u16/i16/f32 variants all pass through generic over `T`/`W`, so the
vulnerable `usize` arithmetic in `generate_weights` is identical regardless of pixel type or
accumulator width; `plan_gray_alpha_resampling`/`plan_rgba_resampling`/`_16` route through
`AlphaPlanner` when `premultiply_alpha` is `true` and `DefaultPlanner` otherwise, both of which
have the identical validation asymmetry; `plan_ar30_resampling`/`plan_ra30_resampling` both
delegate to `plan_rgb_resampling16` (`src/scaler.rs::plan_resize_ar30`) and inherit the same path,
*except* when `ResamplingFunction::Nearest` is selected, which both `DefaultPlanner` and
`AlphaPlanner` short-circuit to a weight-free `ResampleNearestPlan` before `generate_weights` is
ever reached - this is the one legitimately safe path found, not a coverage gap.

No entry point was found to take a "subtly different path" that the existing `checked_mul` fix
doesn't cover - the backlog row's suspicion that one might exist was not borne out.

## The fix

No new fix needed - candidate 010's existing `patch.diff` (`kernel_size.checked_mul(out_size)` at
both call sites in `src/math/weights.rs`, converting the unchecked multiply into
`PicScaleError::OutOfMemory` on overflow) already covers every entry point tested here, verified
by re-running the same 24 invocations against the patched tree (0/24 panicked, up from 23/24
before). This candidate's own `patch.diff` is a copy of candidate 010's for this folder's
self-contained `just test` to apply; it is not a separately-derived fix.

- Project test suite (`cargo test --lib`, default features): **66 passed, 0 failed** both before
  and after applying candidate 010's patch (isolated via toggling only `src/math/weights.rs`,
  identical to candidate 010's own verification). No regression.

## What's not done / open questions

- Only one boundary-value input (`source_size=(1<<62, 1)`, `destination_size=(2, 1)`) was swept
  across entry points; candidate 010's own "what's not done" section (a release-build input that
  wraps to a small-but-plausible value instead of a still-huge one) remains equally open for every
  entry point checked here, not just `plan_rgba_resampling` - not re-investigated this session.
  Every entry point's non-`Nearest` calls to `T::make_weights` share the identical `usize`
  arithmetic in `generate_weights`, so nothing entry-point-specific would change that
  investigation's outcome, but it wasn't re-run per entry point.
- `plan_resize_ar30`'s AVX2/SSE-specialized `Ar30DestructuringImpl` construction path (after the
  weights-planning step this candidate targeted) was not otherwise exercised - out of scope for
  this specific `source_size` question, noted here only because it's adjacent code in the same
  function.
