# offset-find-subdivision-point-index-oob

`kurbo/src/offset.rs`'s `CubicOffset::find_subdivision_point` (lines 459-517, reached from the
public `offset_cubic` entry point whenever a cubic segment's offset-arc approximation doesn't
converge within `MAX_DEPTH` and the segment straddles an inflection point) builds an
`ArrayVec<f64, 3>` of `arc_angles`, sets `target = sum * 0.5` (computed via a running `sum +=
th.abs()` in a separate loop), then consumes `arc_angles` with `while arc_angles[i].abs() <
target { target -= arc_angles[i].abs(); i += 1; }` with no bound check on `i` against
`arc_angles.len()`. The candidate hypothesis was that floating-point rounding could let `target`
stay positive by an ULP or two after every element is consumed, indexing `arc_angles[i]` one past
the end and panicking.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value / invariant analysis | Derived a closed form for `target` after consuming `k` terms and showed continuing past the last valid index requires a nonnegative sum to be negative (impossible in exact arithmetic); checked whether floating-point rounding alone could bridge that gap given all `arc_angles` entries are bounded in `[0, pi]` (`atan2` outputs) | No floating-point-bridgeable gap found; see Root cause below |
| Exhaustive/randomized check of the isolated `arc_angles`-consumption arithmetic (IEEE-754 double semantics identical to Rust `f64`) - a standalone Python prototype (`isolated_invariant_check.py`, kept alongside this folder for reference) plus the Rust port baked into `repro/src/main.rs`'s "Approach 1+2" (run as part of `repro/`, see `output.txt`) | 22,736 combinations of `n=2` and `n=3` tuples drawn from an adversarial grid (0, smallest denormal, values near 0/pi/4/pi/2/0.75*pi/pi and their nearest representable neighbors via `next_up`/`next_down`), plus an earlier 2,000,000-sample purely-random Python pre-pass over the same value families | 0 out-of-bounds cases in every pass |
| Real end-to-end fuzz of the public `kurbo::offset::offset_cubic` API - `repro/src/main.rs`'s "Approach 3" (release build, `ITERS=2000000`, run via `just test` / `cargo run --release` in `repro/`), plus a supplementary standalone debug-assertions/overflow-checks build used only during investigation (`fuzz_offset_cubic.rs`, ~100-300k iterations in a 100s budget) | Random `CubicBez` biased toward S-shaped (inflection-bearing) curves, coordinate scale in `{1, 1e-3, 1e3, 1e6, 1e-6, 1e9, 1e-9}`, `d` scaled the same way, `tolerance` in `{1e-3, 1e-6, 1e-9, 1e-12}` (tiny tolerances chosen to push recursion toward `MAX_DEPTH`), each call wrapped in `catch_unwind` | 0 panics of any kind in either build |

## Status

**Not reproducible.**

The isolated-arithmetic analysis (approach 1+2) shows the out-of-bounds indexing described by the
candidate is not just empirically rare but appears to be *impossible* given the loop's structure:
see Root cause. The real end-to-end fuzz (approach 3, 2,000,000 release-build iterations plus a
partial debug-build run) found zero panics of any kind (not just this specific one), consistent
with that conclusion. This does not rule out a panic reachable through some other floating-point
pathology not covered by the random/grid search (e.g. an input that also hits IEEE special values
inside `atan2`/`cross`/`dot` in a way this session didn't construct), but no such path was found
and the invariant argument below explains why the specific mechanism in the candidate description
cannot occur.

## Class

N/A (not reproducible).

## Root cause (why the suspect turns out to be safe)

Let `a_i = arc_angles[i].abs()` for `i` in `0..n` (`n = arc_angles.len() <= 3`); all `a_i >= 0`
since each is `atan2(...).abs()`, bounded in `[0, pi]`. `sum = Σ a_i` is computed by forward
accumulation; `target` starts at `sum * 0.5` - an exact halving in IEEE-754 (multiplying by 0.5 is
just an exponent decrement, no rounding, except at the subnormal boundary which doesn't apply to
values bounded by `pi`).

After the loop has consumed indices `0..k` (i.e. `i == k`), `target` equals, in exact arithmetic,
`(Σ_{j>=k} a_j - Σ_{j<k} a_j) / 2`. The loop continues past index `k` only if `a_k < target`,
i.e. `a_k + Σ_{j<k} a_j < Σ_{j>k} a_j`. At `k = n-1` (the last valid index), the right-hand side
`Σ_{j>k} a_j` is an empty sum (`0`), so continuing requires `a_{n-1} + Σ_{j<n-1} a_j < 0` -
impossible, since every term is nonnegative. So in exact arithmetic the loop can never advance `i`
past `n-1`, i.e. it can never index `arc_angles[n]`.

The only way floating-point rounding could break this is if the *independently computed* `sum`
(forward-accumulated) diverges from the *self-referential* `target` (which is decremented by the
exact same `a_i` values it later compares against) by enough to flip the strict inequality at the
last index. Because `target`'s decrements reuse the very values being compared (not a separately
rounded copy), the only source of divergence is the single `sum * 0.5` halving (exact) and the
per-step subtraction rounding (at most ~0.5 ULP each, at most 2 steps since `n <= 3`) - many orders
of magnitude too small to bridge the `a_{n-1} + Σ_{j<n-1} a_j >= 0` margin, which is only exactly
zero when every `a_i` is exactly `0` (and in that case the strict `<` comparison never fires in
the first place, since `0 < 0` is false). The exhaustive grid search over denormals and
near-boundary doubles (approach 2) empirically confirms no combination bridges this gap.

In short: this isn't a coincidentally-unobserved bug, it's a structural property of the
"cumulative-sum pivot search" pattern this loop implements over nonnegative terms with `target`
initialized to exactly half the total and decremented by the same values it's compared against -
that pattern is self-correcting against rounding error in a way that prevents overshoot past the
last element, given `ArrayVec<f64, 3>`'s small fixed size (2 or 3 elements) and each term being
`atan2`-bounded.

## Introduced by

N/A (not reproducible; no bug to attribute).

## The fix

Not applicable - no reproducible defect found this session, so no patch was written. (If a
defensive `debug_assert!(i < arc_angles.len())` or `.get(i)` were still wanted purely as a
belt-and-suspenders guard against a code path this session didn't find, that would be a
stylistic/defensive-programming change, not a bug fix backed by a reproduction - left to the
project maintainers' judgment, not proposed here.)

- Project test suite: not run against a patch (none was written). N/A.

## What's not done / open questions

- The isolated-arithmetic proof and grid search cover the `arc_angles`/`target` consumption loop
  itself in full generality (all `n in {1,2,3}`, adversarial `a_i` values including denormals and
  near-`pi`/near-`0` boundaries) - this is believed to be a complete argument, not just an
  absence-of-evidence result, but it was not formally verified with a proof assistant.
- The real end-to-end fuzz (2,000,000 release-build iterations) covers `offset_cubic` broadly but
  did not specifically target getting `arc_angles.len() == 3` (two interior inflection points
  within one recursion's `[t0, t1]` window) as the common case - most inflection-bearing random
  cubics likely hit `n == 1` or `n == 2` far more often than `n == 3`, since `CubicBez::inflections()`
  typically returns 0-2 roots and both need to land strictly inside `(rec.t0, rec.t1)` for a given
  recursion level. A fuzzer or generator deliberately constructing cubics with two well-separated
  inflection points (rather than relying on random sampling to find them) was not built this
  session; given the analytical proof above, this was judged not worth the added effort, but a
  future session revisiting this candidate should know the `n == 3` case specifically was covered
  by the isolated-arithmetic test but only lightly by the real end-to-end fuzz.
- No other code in `offset.rs` was re-examined for the same "unchecked loop index after a
  fixed-size `ArrayVec` accumulation" pattern; `subdivide_for_tangent` and `refine_least_squares`
  were read in the course of this investigation but not specifically re-audited for this class of
  issue.
