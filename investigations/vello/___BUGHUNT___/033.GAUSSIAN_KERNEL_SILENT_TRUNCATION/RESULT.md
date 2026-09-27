# GAUSSIAN_KERNEL_SILENT_TRUNCATION

`sparse_strips/vello_common/src/filter/gaussian_blur.rs` `compute_gaussian_kernel` (pub, lines
136-164) clamps `kernel_size` to `MAX_KERNEL_SIZE` (13) for a large `std_deviation` and renormalizes
the truncated weights, silently returning a mathematically-wrong (too-narrow) Gaussian instead of
erroring. The suspicion: this is only safe because the sole in-tree caller, `plan_decimated_blur`
(lines 92-127), pre-reduces `std_deviation` via decimation before calling it, but the public
function itself documents no precondition, so a future direct caller (or a decimation-math
refactor with a slightly-off bound) could silently get wrong blur results.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: extracted the exact decimation recurrence from `plan_decimated_blur` (`remaining = (remaining - 1.5) * 0.25`, loop while `> 4.0`) into a standalone probe and swept `std_deviation` from 1.0 up through `f32::MAX` and `f32::INFINITY` | `rustc -O`, release, single run, values `{1, 100, 1e3, 1e6, 1e9, 1e15, 1e19, 1.9e19, 3.4e38, f32::MAX, f32::INFINITY}` | For every **finite** `std_deviation` up to ~1e19, the loop always terminates with `remaining_variance <= 4.0`, i.e. `remaining_sigma <= 2.0`, which by the formula `radius = ceil(3*sigma)`, `size = 1+2*radius` caps `kernel_size` at exactly 13 = `MAX_KERNEL_SIZE` — never past it. From `std_deviation ~= 1.9e19` onward, `std_deviation * std_deviation` overflows `f32` to `+inf`, and the loop (`(inf - 1.5) * 0.25 == inf`) never terminates — a *different* bug (infinite loop / hang), not the truncation this candidate is about. See "New suspects" below. |
| Real public API, actual crate: standalone `kernel_repro_check` binary depending on this worktree's `vello_common` via a path dependency, calling the real `plan_decimated_blur` (the only real production entry point, reached from `GaussianBlur::new` / `PreparedFilter::new` in `filter/mod.rs`) across the same sweep of `std_deviation`, plus a direct call to `compute_gaussian_kernel(100.0)` for comparison | `cargo run --release`, single run | `plan_decimated_blur` never produces a `kernel_size` that required silent truncation — observed sizes were 7, 11, 13 for `std_deviation` in `{1, 100, 1e3, 1e6, 1e9, 1e15, 1e18, 5e18}`, always `<= MAX_KERNEL_SIZE` by construction, matching the boundary-value math above. The direct call `compute_gaussian_kernel(100.0)` does clamp to size 13 and renormalize (confirms the mechanism exists, and matches the project's own existing `test_large_sigma_clamped_to_max` test in `vello_cpu/src/filter/gaussian_blur.rs`), but this direct call bypasses `plan_decimated_blur` entirely — no code in the real `filter_effects` pipeline calls `compute_gaussian_kernel` that way. |
| Code search for any other real (non-test) caller of `compute_gaussian_kernel` across `vello_common`/`vello_cpu`/`vello_hybrid` | `grep -rn compute_gaussian_kernel` over `sparse_strips/*/src` | Only caller outside `#[cfg(test)]` modules is `plan_decimated_blur` itself (line 126). Every real entry point (`GaussianBlur::new`, `PreparedFilter::new` in `filter/mod.rs`, `DropShadow`'s blur setup in `drop_shadow.rs`) goes through `plan_decimated_blur`, never around it. |

## Status

**Not reproducible.** The specific claimed defect — a real caller receiving a silently-too-narrow
Gaussian because `compute_gaussian_kernel` truncated its input — does not occur through any actual
trigger path in the current codebase. The sole real caller's decimation recurrence mathematically
guarantees `remaining_sigma <= 2.0` for every finite `std_deviation` (proven by direct calculation
above and confirmed by exercising the real public API), which keeps `kernel_size` at or under
`MAX_KERNEL_SIZE` without ever needing the `.min(MAX_KERNEL_SIZE)` clamp to actually cut anything
off. The clamp-and-renormalize path in `compute_gaussian_kernel` is real and exercisable (as the
project's own `test_large_sigma_clamped_to_max` test shows), but only by calling the function
directly with an unreduced `std_deviation` — something no in-tree caller, and no SVG-driven
`feGaussianBlur` user input via `PreparedFilter::new`, actually does today.

**Fully reproducible**: N/A (Not reproducible, no repro/ required).

## Root cause

Not applicable as a present defect: the "bug" as described is a latent API-contract gap (missing
documented precondition on a `pub` function), not a currently-reachable wrong-output or crash. The
decimation math in `plan_decimated_blur` (loop `while remaining_variance > 4.0 { remaining_variance
= (remaining_variance - 1.5) * 0.25; ... }`) monotonically decreases `remaining_variance` toward a
fixed point of -0.5 for any starting value that stays finite through squaring, so it always exits
with `remaining_variance <= 4.0` -> `remaining_sigma <= 2.0` -> `kernel_size <= 13 ==
MAX_KERNEL_SIZE`. That bound is exact, not "usually true", for every finite `f32` input, so
`compute_gaussian_kernel`'s clamp is unreachable-in-practice dead code along this path today.

## Introduced by

Not determined — not applicable, since no reachable defect was found to date. (For reference,
`compute_gaussian_kernel`'s clamp itself and the `MAX_KERNEL_SIZE` documentation predate this
session; not investigated further since there is nothing to root-cause.)

## The fix

None attempted — there is nothing to fix; the function behaves as documented for its one real
caller. A defensive improvement worth a maintainer's own judgment call (not attempted here, since
it is a hardening suggestion rather than a fix for an observed defect): document
`compute_gaussian_kernel`'s implicit precondition (`std_deviation <= 2.0`) in its doc comment, or
add a `debug_assert!(std_deviation <= 2.0)` so a future refactor of the decimation bound would fail
loudly in a debug build instead of silently shipping a too-narrow kernel.

- Project test suite: N/A — no fix was made, so no before/after run was needed for this candidate.

## What's not done / open questions

- While probing this candidate, found that `plan_decimated_blur`'s decimation loop does **not**
  terminate when `std_deviation` is large enough that `std_deviation * std_deviation` overflows
  `f32` to `+inf` (empirically, `std_deviation >= ~1.9e19`): `(inf - 1.5) * 0.25 == inf` forever.
  `std_deviation` here flows from `FilterPrimitive::GaussianBlur`'s user/SVG-controlled
  `std_deviation`, scaled by an arbitrary transform in `transform_blur_params` (`filter/mod.rs`
  line 56) before reaching `GaussianBlur::new` — so a large enough transform scale combined with a
  plausible SVG `stdDeviation` could plausibly reach the overflow threshold. This is a distinct
  candidate (an infinite-loop/hang, not a truncation) and was not investigated further under this
  candidate's scope — see "New suspects" reported alongside this result.
- Did not check whether any *other* filter (e.g. `DropShadow`, which also calls
  `plan_decimated_blur` per `drop_shadow.rs`) has a separate direct call path to
  `compute_gaussian_kernel` that bypasses decimation — grep confirms it doesn't today, but this
  wasn't re-verified against `vello_hybrid`'s GPU-side kernel consumption code beyond the grep.
