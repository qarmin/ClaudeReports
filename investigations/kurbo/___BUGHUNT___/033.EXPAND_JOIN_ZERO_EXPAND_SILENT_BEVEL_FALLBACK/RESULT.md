# EXPAND_JOIN_ZERO_EXPAND_SILENT_BEVEL_FALLBACK

`ExpandCtx::do_join` in `kurbo/src/expand.rs` (line 294 at commit ca27349) gates entry into the
`Join::Miter`/`Join::Round` handling with `if cross * self.expand.xx < 0.0`, using `expand.xx`
purely as a sign proxy. This looked suspicious because `expand.xx == 0.0` (a fully-public,
directly-constructible `Diagonal2` value, reachable via the public `expand_path`/
`expand_path_signed` API) makes the product always `0.0`, which is never `< 0.0` - so the branch
would always be skipped regardless of the actual corner geometry or the caller's requested join
type. Line 336's `let tolerance = self.tolerance / einv.xx.abs().max(einv.yy.abs());` (inside the
`Join::Round` arm) was also flagged as a possible division-by-zero when both expand components are
zero.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + direct calls to the public API (`expand_path_signed`) | Debug build, a 3-point path with a 90-degree corner, `Diagonal2::new(0.0, y)` for `y` in `{2.0, -2.0}`, compared `Join::Miter`/`Join::Round`/`Join::Bevel` output element-by-element | Confirmed: with `expand.xx == 0.0`, all three join types produce byte-identical output (silent Bevel fallback), while a tiny nonzero `expand.xx` of the same sign as `expand.yy` produces genuinely different Miter/Round geometry for the same corner |
| Differential comparison against a "same sign, tiny nonzero `expand.xx`" control input (`-1e-6` instead of `0.0`) | Debug build, same path/corner | Confirms the corner genuinely needs a real Miter/Round join under this sign convention (rules out "this corner just happens to reduce to Bevel anyway") - so the `expand.xx == 0.0` case is a real behavior change, not a coincidence of degenerate geometry |
| Code reading of the `Join::Round` arm's line 336 (`self.tolerance / einv.xx.abs().max(einv.yy.abs())`) to check the div-by-zero sub-claim | Traced `einv = self.expand.inv()`; `expand.xx == 0.0` implies `einv.xx == f64::INFINITY`, and the outer guard's product is then always `0.0`, so the match arm containing line 336 is unreachable whenever `expand.xx == 0.0`, for either sign of `expand.yy`. Also, even hypothetically ignoring the guard, `einv.xx.abs().max(einv.yy.abs())` would be `+inf` (not `0.0`) when `expand.xx == 0.0`, so line 336 would compute `tolerance / inf = 0.0`, not a NaN/inf - the div-by-zero would actually require both `expand.xx` and `expand.yy` to be *infinite* (making `einv` components `0.0`), not zero | Not reproducible as described: the line-336 divide-by-zero sub-claim is unreachable through `expand.xx == 0.0` (pre-empted by the same guard bug above), and the "both zero" framing in the candidate description appears to have the zero/infinite roles reversed for this specific line |

## Status

**Confirmed** (the silent Bevel fallback for `expand.xx == 0.0`) - reproduces 3/3 runs, debug and
release builds, deterministic (pure arithmetic, no randomness/threading involved). The line-336
divide-by-zero sub-claim from the candidate description is **Not reproducible** via `expand.xx ==
0.0` specifically: traced by code reading (not by running an actual repro) to be unreachable given
the guard-branch bug just above, for both signs of `expand.yy`. It was not chased further (e.g. via
truly-infinite `expand` components, a different scenario than described) since that's outside what
this candidate specifically asked about.

**Fully reproducible**: **Yes**. The repro calls the crate's real public API
(`kurbo::expand_path_signed`) directly with a `Diagonal2` value any caller can construct - there is
no second participant to fake here (this is a single-process, single-caller API-misbehavior finding,
not an interaction between two independent participants); see `CONFIRMATION.md` section 11's "A
single-process, single-participant repro remains completely fine" carve-out.

## Class

logic

## Root cause

`do_join`'s reflex/convex test `cross * self.expand.xx < 0.0` treats `expand.xx`'s sign as a proxy
for the overall expansion's sign (consistent with `expand_path`'s doc note that `expand.xx` and
`expand.yy` are expected to share the same sign). This works for any nonzero `expand.xx`, but
degenerates completely when `expand.xx == 0.0`: the product is then always exactly `0.0`, which
fails the `< 0.0` test unconditionally, so the `match self.join { Join::Miter => ..., Join::Round
=> ... }` arm is never entered - the code falls straight through to the Bevel-join `self.result.
line_to(p)` regardless of what `Join` the caller actually requested and regardless of `miter_limit`.
This is silent: no panic, no NaN, no debug assertion - just wrong (unrequested) output geometry,
which is why it wasn't caught by the project's existing test suite.

## Introduced by

c907a3a "Add anisotropic expansion methods for paths (#575)" (2026-05-07) - this is the commit that
introduced `expand.rs`, `Diagonal2`, and the `cross * self.expand.xx < 0.0` guard itself (confirmed
via `git log -S"cross * self.expand.xx" --oneline -- kurbo/src/expand.rs`, which returns only this
one commit; only one other commit, 6836244, has touched this file since, and it doesn't touch this
guard).

## The fix

Changed the sign gate to fall back to `expand.yy`'s sign when `expand.xx` is exactly `0.0`, instead
of using `expand.xx` unconditionally:

```rust
let expand_sign = if self.expand.xx != 0.0 {
    self.expand.xx
} else {
    self.expand.yy
};
if cross * expand_sign < 0.0 {
```

This is bounded and minimally invasive: for any `expand.xx != 0.0` (the previously-working case)
`expand_sign == self.expand.xx`, so behavior is byte-for-byte unchanged; only the `expand.xx ==
0.0` case is affected, and it now uses `expand.yy`'s sign instead of unconditionally treating the
corner as non-reflex. Checked against more than the one triggering sign combination (per
`CONFIRMATION.md` section 7): both `Diagonal2::new(0.0, 2.0)` and `Diagonal2::new(0.0, -2.0)` were
tried - the fix correctly restores the sign-appropriate Miter/Round behavior for both, matching
what a same-sign tiny-nonzero `expand.xx` produces in each case.

Before/after re-run of `repro/` against the patched tree: before, `Join::Miter`/`Join::Round`
output was byte-identical to `Join::Bevel` (exit code 1, "REPRODUCED"); after, `Join::Miter`
collapses the corner to a real miter point (5 elements instead of 7) and `Join::Round` emits actual
arc `CurveTo`s (9 elements with curves instead of 7 straight `LineTo`s), both diverging from Bevel
as expected (exit code 0, "did not reproduce"). See `output.txt` for the full captured output.

- Project test suite (`cargo test --lib` / `cargo test --doc`, `kurbo` crate): 187 passed / 0
  failed (`--lib`) and 55 passed / 0 failed (`--doc`) before the patch; identical counts, 187
  passed / 0 failed and 55 passed / 0 failed, after the patch. No regressions, no new failures.

## What's not done / open questions

- The line-336 divide-by-zero sub-claim from the candidate description was traced (not repro'd) to
  be unreachable via `expand.xx == 0.0` specifically - see "What was tried" above. Whether a
  *genuinely infinite* `expand.xx`/`expand.yy` (not zero) can reach that line and produce a NaN via
  `self.tolerance / 0.0` when `self.tolerance == 0.0` too was not investigated this session; it's a
  distinct scenario from what was asked here and would need its own candidate.
- Only the `Join::Round`/`Join::Miter` dispatch inside `do_join` was checked. The isotropic
  `expand.xx == expand.yy == 0.0` case was considered but is unreachable through this same code
  path for an unrelated reason: `scale_normal` returns `Vec2::ZERO` for every segment when the
  whole `expand` is zero, so `n - last_n` is always within tolerance and `do_join`'s outer `if
  !self.in_tolerance(...)` never even fires - there is no join geometry to silently get wrong in
  that case, so it isn't a variant of this bug.
- Whether any other place in the crate uses a single-axis product as a sign proxy for an
  anisotropic `Diagonal2`/similar pair (the same bug class) was not swept for; this session only
  looked at `do_join`.
