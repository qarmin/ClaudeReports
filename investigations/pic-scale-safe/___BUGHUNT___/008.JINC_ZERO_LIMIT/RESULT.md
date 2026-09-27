# jinc_f64/jinc_f32/lanczos_jinc zero-limit bug (`bessel_order_one.rs` / `lanczos.rs`)

`jinc_f64`/`jinc_f32` (`src/math/bessel_order_one.rs:320-334`) and `lanczos_jinc`
(`src/math/lanczos.rs:48-49`) special-case `x == 0` to return `0.0` instead of the correct
mathematical limit (`J1(x)/x -> 0.5` as `x -> 0`). Flagged by a prior `code-audit-structured` pass
and picked as the first entry in `IDEAS.md`'s backlog.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction, public API (`resize_plane_f32`, release build) - first attempt used a same-size (identity) resize | 9x1 plane, spike at index 4, `HaasnSoft`/`Ginseng`/`Lanczos3Jinc` | No signal at first: `resize_floating_point` has an identity fast path (`source_size == destination_size`) that bypasses resampling entirely - had to pick a real up/downscale instead |
| Manual boundary-value construction, public API, corrected input (odd integer scale factors, which put a destination sample's window centre exactly on a source pixel, `dx == 0.0` exactly in `compute_weights.rs`) | 27->9 (scale 3) and 45->9 (scale 5), 1-D "single spike among zeros" planes, release build, all 6 Jinc-family `ResamplingFunction` variants | Confirmed: the destination pixel whose window is centred exactly on the spike came out `0.0` (or, for `Lanczos3/4/6Jinc`, that one tap read `0.0` while its symmetric neighbours were nonzero) for all 6 variants |
| Internal probe (temporary `#[cfg(test)]` block directly calling `pub(crate) jinc_f64`, reverted after use) sweeping `x` from `1e-2` down to `1e-8` and comparing to `x == 0` | `cargo test --release`, in-tree only (not part of the delivered `repro/`, since `jinc_f64` isn't public) | Confirmed the discontinuity directly: `jinc_f64(1e-8) = 0.5`, but `jinc_f64(0.0) = 0` - a hard jump at the one point that should be the smoothest (a removable singularity) |

## Status

**Confirmed** - deterministic, bit-for-bit repeatable: every function involved is pure with no
randomness or threading. `repro/`'s full 6-check suite ran twice against the pristine clone with
byte-for-byte identical output both times (see `output.txt`), exit code 1 both times.

## Class

logic

## Root cause

`jinc_f64`/`jinc_f32` compute `J1(x)/x` (the definition used throughout this module, without the
factor of `2` some other "jinc" conventions use), which has a classic removable singularity at
`x = 0`: the limit is well-defined (`0.5`, since `J1(t) ~ t/2` for small `t`) but the naive
expression `0.0/0.0` is not, so the code special-cases it - to the wrong value, `0.0`, instead of
the correct `0.5`. The internal probe above shows this isn't a rounding-precision question; the
function is smooth and flat at `0.5` all the way down to `1e-8`, then jumps to exactly `0` only at
the literal value `0.0`.

Both call sites for `jinc_f64`/`jinc_f32` reach `x == 0` in real resizes, not just in theory:

- **`ResamplingFunction::HaasnSoft`** uses `T::jinc()` (i.e. `jinc_f32`/`jinc_f64` directly) as
  its **kernel** (`sampler.rs`'s `get_resampling_filter`), called as
  `resampling_function(x_kernel_scaled)` in `compute_weights.rs`'s general (non-area) branch.
- **`ResamplingFunction::Ginseng`** uses `T::jinc()` as its **window** function, called as
  `window_producer(x_kernel_scaled * window_size)`; the overall weight is
  `window * resampling_function(...)`, so a zeroed window still zeroes the whole weight.
- In `compute_weights.rs`, the argument passed is `x_kernel_scaled = dx.abs() * filter_scale`,
  where `dx = k - (center_x - 0.5)` for a source index `k` and a destination sample's
  `center_x = (i + 0.5) * scale`. For any **odd integer** `scale`, `center_x - 0.5` is itself an
  integer for every `i`, so there is always some `k` with `dx == 0.0` exactly - i.e. a source
  pixel that sits exactly at a destination sample's window centre. This is not a rare edge case:
  it is the single most common alignment in any resize with an odd integer downscale/upscale
  factor (3x, 5x, 7x, ...).

`lanczos_jinc` (`src/math/lanczos.rs`) has its **own**, separate `x == 0` special case (also
returning `0.0`) guarding the `jinc(d) * jinc(d * scale_a)` formula used by
`Lanczos2Jinc`/`Lanczos3Jinc`/`Lanczos4Jinc`/`Lanczos6Jinc`. Because this check runs *before*
`jinc_f64`/`jinc_f32` are ever called, it would have kept zeroing the centre tap for those four
filters even after fixing `jinc_f64`/`jinc_f32` alone - it needed its own, separate fix (see
below). Note the correct limit at `lanczos_jinc(0, a)` is `jinc(0) * jinc(0) = 0.5 * 0.5 = 0.25`
(the *product* of the two fixed halves), not `0.5` - `IDEAS.md`'s original description of this
half of the bug assumed the same `0.5` limit as the plain `jinc` functions, which is not quite
right for the squared/windowed form; the fix below does not hard-code either constant and instead
lets the corrected `jinc_f64`/`jinc_f32` produce the right value naturally.

Net effect in the real resize pipeline: for any of these 6 `ResamplingFunction` variants, at
every destination sample whose window centre lands exactly on a source pixel, that source pixel's
own contribution is silently dropped to `0.0` and the destination value is reconstructed *only*
from its (much smaller, off-centre) neighbours instead - the opposite of what a peaked
interpolation kernel is supposed to do at its own centre.

## Introduced by

5e72885 "Initial commit" (2024-10-16) - `jinc_f64`/`jinc_f32` have special-cased `x == 0` to `0.0` (instead of the correct `0.5`) since this crate's very first commit; `git log -S"fn jinc_f64"` finds no earlier revision.

**Already fixed upstream**: commit b26f840 "Fixes" (2026-08-25, merged into current HEAD via PR #22) changed both special cases to return `0.5`, and added a `jinc_matches_limit_at_zero`-style regression test - the current working tree no longer reproduces this bug.

## The fix

1. `jinc_f64`/`jinc_f32` (`src/math/bessel_order_one.rs`): changed the `x == 0` special case to
   return `0.5` (the function's actual limit at that point) instead of `0.0`.
2. `lanczos_jinc` (`src/math/lanczos.rs`): removed the separate `x == 0` special case entirely.
   Since `a` is always `> 0` at every call site (`2f32`/`3f32`/`4f32`/`6f32`), `x == 0` already
   satisfies `x.abs() < a`, so it now falls through to the normal `jinc(d) * jinc(d * scale_a)`
   formula with `d = 0`, which - once (1) is applied - naturally evaluates to `0.5 * 0.5 = 0.25`,
   the correct limit, without hard-coding it.

Verified against more than the one triggering input: `repro/` exercises two different odd
downscale factors (3 and 5) across both call paths (`HaasnSoft`/`Ginseng` via `jinc_f64`/`f32`
directly, and `Lanczos2Jinc`/`Lanczos3Jinc`/`Lanczos4Jinc`/`Lanczos6Jinc` via `lanczos_jinc`'s own
special case) - all 6 checks pass after the fix, and the "before" (pristine) run showed all 6
failing. The internal probe additionally sweeps `x` continuously down to `1e-8` and confirms the
fixed `jinc_f64(0.0) = 0.5` sits exactly on the same flat curve as the neighbouring values (no
remaining discontinuity).

- Project test suite: `cargo test --release` at the workspace root has 0 lib tests and 0 doc
  tests both before and after the fix (`running 0 tests ... 0 passed; 0 failed` both times) -
  **N/A, no tests in this package**.

## What's not done / open questions

- Not checked whether the sibling `pic-scale` repo (checked out locally at
  `/home/rafal/test/pic-scale`, referenced by earlier candidates in this workspace) has the same
  bug in its own `jinc`/`lanczos_jinc` - that repo was not touched or re-checked as part of this
  candidate.
- The other `src/math/*.rs` kernels already queued in `IDEAS.md` (`kaiser`, `welch`, the
  `generate_weights` all-zero-sum fallback, etc.) share the general "wrong value at/near a
  removable singularity or boundary" bug class but live in different functions with different
  formulas; none of them were re-checked as part of confirming this one.
- Did not check whether any *other* resampling function in this module reaches `x == 0` through a
  different, not-yet-noticed path (e.g. a future caller passing an unscaled `x` directly to
  `jinc_f64`/`jinc_f32` rather than through `compute_weights.rs`) - the fix is at the source of the
  singularity itself, so this should be safe by construction, but wasn't exhaustively enumerated
  against every current and hypothetical caller.
