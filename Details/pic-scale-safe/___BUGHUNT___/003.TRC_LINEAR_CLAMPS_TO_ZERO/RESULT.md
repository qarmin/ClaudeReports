# trc_linear clamps to 0 instead of [0,1]

`trc_linear` in `src/trc.rs:219-223` backs `TransferFunction::Linear` for both `linearize()` and
`gamma()` (used by every `trc_handler.rs` entry point when `TransferFunction::Linear` is
selected). It looked suspicious because it's implemented as `v.min(1.).min(0.)` instead of a
`[0,1]` clamp, which chains two `.min()` calls and can never return anything above `0` for any
finite input.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + standalone repro | `repro/` calling `TransferFunction::Linear.linearize(v)` / `.gamma(v)` via the public API, release build, inputs `0.0, 0.25, 0.5, 0.75, 1.0, 0.999` (and later `-0.5, 1.5` for the fix-verification pass) | Confirmed: every non-negative input in `[0,1]` returned `0` instead of the identity value |
| Repeated re-run of the same repro binary | Same release build, 5 consecutive runs, no rebuild in between | Identical output every run (deterministic pure function, no flakiness to account for) |

## Status

**Confirmed** - reproduces 5/5 runs, release build, deterministic (pure function, no randomness
or layout dependence involved).

## Class

logic

## Root cause

`trc_linear` is meant to be the identity function clamped to `[0, 1]` (the Linear transfer
function has no actual curve - both `linearize()` and `gamma()` in `TransferFunction`'s `impl`
dispatch to the same `trc_linear`, since Linear's forward and inverse are the same operation).
The implementation is:

```rust
pub fn trc_linear(v: f32) -> f32 {
    v.min(1.).min(0.)
}
```

The second `.min(0.)` should be `.max(0.)`. As written, `v.min(1.)` first clamps the upper bound
to `1.0`, but the following `.min(0.)` then takes the minimum of *that* and `0.0` - which is
`0.0` for every input `>= 0.0`, and simply `v` (unclamped on the low end) for every input `< 0.0`.
In practice this means every value in the useful `[0, 1]` range - the entire domain this function
is ever called with from `trc_handler.rs` - gets zeroed instead of passed through.

## Introduced by

0852e7f "Preparing for release and some docs" (2024-10-17) - `git log -S"fn trc_linear"` finds this as the commit that introduced `trc_linear` with the `.min(1.).min(0.)` typo already present (should be `.max(0.)`).

**Already fixed upstream**: commit b26f840 "Fixes" (2026-08-25, merged into current HEAD via PR #22) changed the second `.min(0.)` to `.max(0.)` - the current working tree no longer reproduces this bug.

## The fix

Changed the second `.min(0.)` to `.max(0.)`:

```rust
pub fn trc_linear(v: f32) -> f32 {
    v.min(1.).max(0.)
}
```

This makes `trc_linear` a proper `[0, 1]` clamp: identity within the range, `0.0` below it, `1.0`
above it - matching every sibling transfer function in this file (e.g. `pure_gamma_function`,
`smpte428_to_linear`) which all clamp both ends rather than just the upper one.

Re-ran the standalone `repro/` against the patched tree, 3/3 runs, plus two boundary values
outside `[0,1]` (`-0.5 -> 0`, `1.5 -> 1`) not present in the original triggering set, per
`CONFIRMATION.md` section 7 - all inputs now round-trip correctly through both `linearize()` and
`gamma()`.

- Project test suite: 0 passed / 0 failed before -> 0 passed / 0 failed after (`cargo test
  --release`) - N/A in the sense that this package ships no tests at all, not that the fix was
  unverified (see the dedicated `repro/` above for the actual verification).

## What's not done / open questions

- No unit test was added to the crate itself (the package currently has zero tests of any kind);
  a maintainer taking this patch may want to add `#[test] fn trc_linear_is_identity_in_range()`
  directly next to the function.
- Every other transfer function in `src/trc.rs` was read while investigating this one and clamps
  correctly on both ends (`pure_gamma_function`, `smpte428_to_linear`/`_from_linear`, and all the
  piecewise `if`/`else` ladders) - `trc_linear` is the only one with this specific
  double-`.min()` mistake in this file.
