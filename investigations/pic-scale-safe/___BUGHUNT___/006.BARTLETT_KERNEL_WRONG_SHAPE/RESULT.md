# bartlett kernel has the wrong shape (`src/math/bartlett.rs:34-45`)

`bartlett()` (the `ResamplingFunction::Bartlett` window used by the general resampling path in
`compute_weights.rs`) does not implement a triangular window at all: it returns `0` at its own
centre instead of the peak weight, rises to `2` at the edge of its declared support, then jumps
discontinuously down to near-`0` and falls to `-2` beyond that. Flagged by a prior
`code-audit-structured` pass and picked as the first/cheapest entry in `IDEAS.md`'s backlog.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (debug build, `cargo test --lib` with a temporary probe test) | Called `bartlett::<f32>(x)` for `x` in `0.0..=2.0` step `0.1` directly in-tree | Confirmed: `bartlett(0.0)=0`, rises linearly to `bartlett(1.0)=2`, jumps to `bartlett(1.1)=-0.2`, falls to `bartlett(2.0)=-2` |
| Differential test against the textbook triangular window, `1 - \|x\|/2` clamped to `[0,2]` (the domain `compute_weights.rs` actually calls this kernel over, given `ResamplingFilter::new(bartlett, 2f32)`'s `min_kernel_size == 2.0`) | Standalone `repro/`, `f64`, 2001 sample points over `x` in `[0,2]` | Confirmed: max abs error `2.0` at `x=2`; also checked centre value, monotonicity, sign, and continuity at the `x=1` branch boundary - all 5 checks failed |
| Public-API observation (ad hoc, not part of `repro/`) | `resize_plane_f32` on a 1-D "single bright pixel among zeros" image, `ResamplingFunction::Bartlett`, 8x upsample | Confirmed real-world effect: output ranges from `-7.5` to `+8.5` around the spike (source values are only `0.0`/`1.0`) |

## Status

**Confirmed** - deterministic, bit-for-bit repeatable: `bartlett` is a pure function with no
randomness or threading, so every run against the same input produces byte-identical output. Ran
the standalone `repro/`'s full check suite twice against the pristine clone, byte-for-byte
identical both times (see `output.txt`).

## Class

logic

## Root cause

`bartlett(x)` in `src/math/bartlett.rs` is:

```rust
if x >= 0f32.as_() && x <= 1f32.as_() {
    return 2f32.as_() * x;
}
2f32.as_() - 2f32.as_() * x
```

Two independent problems, both visible from the boundary values:

1. **No domain normalization.** Every call site (`compute_weights.rs`'s non-window branch) invokes
   the kernel as `resampling_function(dx * filter_scale)`, where `dx = |k - center| >= 0` and, for
   `ResamplingFilter::new(bartlett, 2f32)` at scale 1, the resulting `x` ranges over `[0, 2]` (the
   declared `min_kernel_size`). The formula never divides `x` by this radius, so it treats the
   `[0, 2]` input domain as if it were `[0, 1]`: `bartlett(0) = 0` (should be the peak, `1`) and
   `bartlett(1) = 2` (should be roughly the midpoint of the falloff, `0.5`).
2. **No clamp outside the true support**, unlike every sibling kernel in the module (`welch`,
   `hamming`, `hanning`, `hann`, `bartlett_hann` all explicitly return `0` once `\|x\|` exceeds
   their domain). Past `x = 1` the second branch `2 - 2x` keeps decreasing linearly with no floor,
   reaching `-2` at `x = 2`, and there is a jump discontinuity of size `2` exactly at the `x = 1`
   branch boundary (approaching from below: `2*1 = 2`; from above: `2 - 2*1 = 0`).

Net effect in the real resize pipeline: because `compute_weights.rs` normalizes weights by their
sum (`weights_sum`) before writing them out, a constant-value image is still reconstructed
correctly (any weights summing to 1 preserve a constant), which is why this doesn't show up as an
obviously "broken" resize on a flat test image. But for any non-constant input, the destination
pixel closest to a source sample gets the *smallest* relative weight from that sample (should get
the largest), immediate neighbours get inflated/negative weights, and the effective kernel has a
sharp discontinuity - visible directly through the public API as large overshoot/undershoot (see
"Public-API observation" above: output values outside the source's `[0, 1]` range by more than a
factor of 7).

This is not specific to `pic-scale-safe`: the sibling (non-`-safe`) `pic-scale` repo, checked out
locally at `/home/rafal/test/pic-scale`, has the exact same `bartlett()` body (`2*x` /
`mla(-2, x, 2)`, mathematically identical to `2 - 2*x`) - unlike its already-fixed
`bartlett_hann`, which got a normalized-domain rewrite with tests. So `bartlett` itself was not
independently fixed there either; this is a pre-existing bug in both repos' shared heritage, not
something introduced by `pic-scale-safe`.

## Introduced by

5e72885 "Initial commit" (2024-10-16) - `git log -S"fn bartlett"` finds this as the commit that introduced `bartlett()` with the missing domain normalization and missing outside-support clamp already present.

**Already fixed upstream**: commit b26f840 "Fixes" (2026-08-25, merged into current HEAD via PR #22) rewrote `bartlett()` to fold to `\|x\|`, clamp to `0` beyond radius `2.0`, and use `1 - 0.5*x` on `[0,2]`, plus added a `bartlett_is_triangle_over_support` regression test - the current working tree no longer reproduces this bug. See this candidate's own `CORRECTION.md`.

## The fix

Changed `bartlett()` to the textbook triangular window normalized to its actual call-site domain
(`\|x\| <= 2`, matching `ResamplingFilter::new(bartlett, 2f32)`'s `min_kernel_size`), mirroring the
style of the already-correct siblings in this same module (`.abs()` first, explicit zero outside
the support):

```rust
pub(crate) fn bartlett<
    V: Copy + Sub<Output = V> + Mul<Output = V> + Div<Output = V> + Float + 'static,
>(
    x: V,
) -> V
where
    f32: AsPrimitive<V>,
{
    let x = x.abs();
    let radius = 2f32.as_();
    if x >= radius {
        return 0f32.as_();
    }
    1f32.as_() - x / radius
}
```

Verified against more than just the triggering input: the differential test in `repro/` sweeps
2001 points across the entire `[0, 2]` domain (not just `x=0`/`x=1`), and separately checks
monotonicity, sign, and continuity at the old branch boundary - all pass after the fix (see
`output.txt`'s "AFTER" section: max abs error vs. the textbook window drops from `2.0` to `0.0`
across the whole sweep). The public-API spike test also confirms the fixed kernel keeps output
within the source's `[0, 1]` range and produces a smooth, symmetric, monotonically-decreasing
ramp around the spike instead of the `-7.5`/`+8.5` overshoot.

- Project test suite: `cargo test` at the workspace root has 0 lib tests, 0 doc tests, and an
  empty `tests/` directory both before and after the fix - **N/A, no tests in this package**.

## What's not done / open questions

- `bartlett_hann` (`src/math/bartlett.rs:47-63`) is a separate suspect already queued in
  `IDEAS.md`'s backlog (different formula, different symptom - a possibly-flipped sign and a
  `0.4832` vs `0.48` constant) and was not touched by this fix.
- Every other `src/math/*.rs` kernel flagged in `IDEAS.md` (jinc, kaiser, welch's missing `abs()`,
  etc.) shares the "wrong shape" bug class but lives in different functions with different
  formulas; none of them were re-checked as part of confirming this one.
- Not checked whether the sibling `pic-scale` repo's `bartlett` (same bug, confirmed present by
  inspection above) should also get this fix - that repo was only used as a read-only reference,
  per this skill's workspace being scoped to `pic-scale-safe`.
