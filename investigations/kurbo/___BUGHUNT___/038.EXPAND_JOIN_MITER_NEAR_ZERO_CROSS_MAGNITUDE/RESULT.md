# EXPAND_JOIN_MITER_NEAR_ZERO_CROSS_MAGNITUDE

`ExpandCtx::do_join`'s `Join::Miter` arm in `kurbo/src/expand.rs` (line 302 at commit ca27349)
computes `h = (n - last_n).cross(tan) / self.last_tan.cross(tan)` and then
`self.result.line_to(self.last_pt + last_n + h * self.last_tan)` with no `is_finite` check. The
reflex/convex gate just above it (`cross * self.expand.xx < 0.0`, already flagged by confirmed
candidate 033 for `expand.xx == 0.0`) only checks `cross`'s sign, not its magnitude. This looked
suspicious because, in the near-parallel-tangent limit (tiny angle between two consecutive
segments), the miter-limit inequality `2.0 * hypot < (hypot + dot) * miter_limit^2` reduces to
roughly `1 < miter_limit^2` (true for any typical `miter_limit > 1`), so the candidate hypothesized
the Miter branch would still be entered and divide by a near-zero `cross`, blowing `h`/`miter_pt`
up towards infinity.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction via the real public `expand_path_signed` API, plus temporary `eprintln!` instrumentation of the real `do_join` code (reverted before writing this candidate) to print `cross`, the miter-limit inequality's `take_branch`, `h`, and `miter_pt` directly | Debug and release builds, `Diagonal2::new(1e6, 1.0)`, a two-segment "near-straight" corner with the raw angle gap between segments swept geometrically from 10 degrees down to 1e-16 rad on both signs | The Miter branch *is* entered for arbitrarily tiny angle gaps (confirming the candidate's premise about the gate), but `h` shrinks roughly linearly (in some cases quadratically) towards `0` as the angle gap shrinks towards `0`, rather than diverging - the numerator `(n - last_n).cross(tan)` shrinks at least as fast as the denominator `cross`, not slower |
| Independent analytic/numeric model of the same formula (`scan.py`, a pure-Python re-implementation of `Diagonal2::scale_normal`/`turn_90`/`cross` matching `kurbo/src/expand.rs`), scanning corner angle broadly (not just near-parallel) and anisotropy ratio up to 1e9 | Grid + refined scan over corner angle theta0 in `[-pi, pi]`, angle gaps from 1e-16 to ~1 rad, anisotropy `xx` up to 1e9 with `yy = 1.0` | The largest `|h|` found across the whole scan occurs at a *generic* (not near-parallel) angle gap of about 1 rad, and scales proportionally with the anisotropy ratio (`xx`) itself - i.e. a large-but-finite miter offset that is an expected consequence of a highly anisotropic `expand`, not a division blow-up specific to near-zero `cross` |
| Real end-to-end sweep of the public `kurbo::expand_path_signed` entry point (baked into `repro/src/main.rs`, run via `just test` - see `output.txt`): (1) a targeted sweep at the empirically most-sensitive corner direction, angle gap geometric from 1e-1 down to 1e-16 rad both signs, anisotropy in `{1e3, 1e6, 1e9, 1e12}` (2,400 cases); (2) a 2,000,000-iteration randomized sweep of corner angle, angle gap (log-uniform 1e-18 to 1e-1 rad, both signs), and anisotropy (log-uniform 1 to 1e12) | Release and debug builds, each call wrapped in `catch_unwind`, checked for non-finite output coordinates | 0 non-finite/panicking cases in both approaches, both builds (2,402,400 total calls) |
| (Not part of the soft-cap-3, noted only for context) A quick supplementary check with `expand.xx` set to an absurd `1e250` (far outside any plausible real usage) did produce an `inf` output coordinate - but this is a generic floating-point-overflow consequence of an extreme `expand` magnitude (`z_hypot2` and similar intermediate products overflow `f64` range well before `do_join` is reached), not the near-zero-`cross`-specific mechanism this candidate describes; not investigated further this session (see "What's not done" below) |

## Status

**Not reproducible.** The specific mechanism hypothesized - a near-zero `cross` in the
near-parallel-tangent limit causing an unbounded division in `h` - does not occur: `cross` and the
numerator `(n - last_n).cross(tan)` both derive from the same underlying angle difference and
shrink together (the numerator at least as fast) as the tangents become more parallel, so `h`
converges towards `0`, not infinity, exactly where the candidate's premise says it should diverge.
Covered: 2 escalating direct/analytic approaches (instrumented real code + an independent numeric
model) plus a real end-to-end sweep of 2,402,400 calls to the public API (targeted boundary values
+ broad randomized search) across both debug and release builds, with 0 non-finite or panicking
results.

**Fully reproducible**: N/A (Not reproducible - no bug to reproduce). The `repro/` in this folder
demonstrates the negative result via the real public `expand_path_signed` API directly (no hand-
built internal state) - see `CONFIRMATION.md` section 11's "single-process, single-participant
repro remains completely fine" carve-out; there is no second participant to fake here.

## Class

N/A (Not reproducible)

## Root cause

The suspect turned out to be safe for the reason above: although the sign-only gate
(`cross * self.expand.xx < 0.0`, shared with confirmed candidate 033) does let the Miter branch be
entered for arbitrarily small `|cross|`, `h`'s numerator is geometrically tied to the same angle
difference that produces `cross` in the first place - both `n - last_n` (via
`Diagonal2::scale_normal`, a continuous, nowhere-discontinuous function of tangent direction away
from a zero tangent) and `cross` (`last_tan.cross(tan)`, a smooth function of the same angle
difference) go to zero together as the two tangents converge, and the numerator's own dependence on
the angle difference is at least as steep near zero, so their ratio `h` does not diverge. The
largest `h` values found anywhere in the scan instead came from *generic* (non-near-parallel) corner
angles under high `Diagonal2` anisotropy, where `h` scales roughly proportionally with the
anisotropy ratio itself - large, but a direct, expected consequence of the requested expansion
factor, not a division-by-near-zero artifact.

## Introduced by

N/A (Not reproducible; for reference, the surrounding code - `do_join`, `Diagonal2`, and the
`cross * self.expand.xx < 0.0` gate - was introduced in c907a3a "Add anisotropic expansion methods
for paths (#575)" (2026-05-07), the same commit `git log -S"h = (n - last_n).cross(tan)"` and
confirmed candidate 033 point to).

## The fix

No fix - the suspected mechanism does not reproduce, so no code change is proposed. See "What's not
done" below for a distinct, more mundane overflow observed only at an extreme (`1e250`) `expand`
magnitude, which is a separate concern from this candidate's specific hypothesis.

- Project test suite: not run (no code change made; see `SKILL.md` candidate loop step 7, which
  only requires this for a verified fix).

## What's not done / open questions

- The supplementary `expand.xx = 1e250` observation (see "What was tried" above) suggests that an
  *extreme* `Diagonal2` magnitude (far outside plausible stroke-width-style usage) can overflow
  `f64` range somewhere in the anisotropic-expansion arithmetic (plausibly in
  `Diagonal2::scale_normal`'s `z.hypot2()`/`sqrt()`, or similar squared intermediate terms
  elsewhere in `expand.rs`) well before reaching `do_join`'s Miter arm specifically. This is a
  distinct bug class (generic overflow from an extreme scalar input, not a near-zero-cross miter
  blow-up) and was not investigated further this session - a plausible new candidate for a future
  session, scoped to `Diagonal2`/`scale_normal` overflow behavior at extreme magnitudes rather than
  `do_join`'s Miter branch specifically.
- Only `Join::Miter`'s `h` computation was checked. `Join::Round`'s arm (line ~317 onward, using a
  different `einv`/`crossn`/`atan2` formulation) was not swept for an analogous near-zero-magnitude
  concern this session.
- The random sweep (approach 2) covers corner angle, angle gap, and one-axis anisotropy
  (`Diagonal2::new(xx, 1.0)`) broadly but not exhaustively - a longer/dedicated fuzzing campaign
  (via `fuzz-bug-repro`, run separately) covering both anisotropic axes independently, negative
  `expand` components, and `Join::Round`/`do_quad`/`do_cubic`'s own join call sites was not
  attempted.
