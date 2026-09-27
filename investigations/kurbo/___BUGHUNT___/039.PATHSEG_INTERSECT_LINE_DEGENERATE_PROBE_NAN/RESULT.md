# PATHSEG_INTERSECT_LINE_DEGENERATE_PROBE_NAN

`kurbo/src/bezpath.rs`, `PathSeg::intersect_line` (fn starts at line 1186). In the `PathSeg::Quad`
and `PathSeg::Cubic` match arms, `invlen2 = (dx*dx + dy*dy).recip()` is computed unguarded from the
probe `Line`'s own direction (`dx`/`dy`). A near-zero-length probe `Line` (e.g. built from two
equal points, or two points a tiny epsilon apart) makes `invlen2` blow toward `f64::INFINITY`, and
the subsequent `u = ((x-p0.x)*dx + (y-p0.y)*dy) * invlen2` computes `0.0 * inf == NaN` whenever the
numerator is exactly zero (which it always is when `dx == dy == 0.0`, since the numerator is a
linear combination of `dx`/`dy`). `(0.0..=1.0).contains(&NaN)` is always `false`, so a genuine
on-curve intersection point is silently dropped instead of being reported.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (public API): `QuadBez`/`CubicBez`, evaluate a genuine curve point at `t=0.5`/`t=0.3` via `ParamCurve::eval`, build a degenerate probe `Line::new(p, p)` (and a near-zero but nonzero variant, `dx = 1e-100`), call `PathSeg::intersect_line` | debug build (`cargo run`), standalone crate depending on kurbo's public API via a path dependency | Reproduced: `intersect_line` returns `[]` for a point that is genuinely on the curve, both for the exactly-degenerate probe and for probe lengths as small as 1e-100 |
| Sweep of probe-line lengths (`eps` from `0.0` down through `1e-300`, `1e-100`, `1e-20`, `1e-10`, `1e-9`, `1e-8`) against the same on-curve `QuadBez` point | debug build, one-off standalone probe program (kept outside the repo at `/tmp/kurbo_probe039`) | False negative persists for every `eps` from `0.0` through `1e-10` (inclusive); at `eps=1e-9` and above the existing non-degenerate math already recovers the correct answer (matches the candidate's own framing: this is specifically the near-zero-length-probe regime) |
| Repeat-run determinism check: ran the standalone `repro/` 5 times in a row | debug build | 5/5 identical output and exit code (`1`, i.e. reproduced) every time - this is pure floating-point arithmetic with no seed/randomness, fully deterministic given the same input |

## Status

**Confirmed**, reproduces 5/5 runs, debug build (also reproduces identically in a release build via
`just test`, see `output.txt`/the candidate's own `justfile`).

**Fully reproducible**: **Yes**. The repro (`repro/src/main.rs`) calls only kurbo's real public API
(`QuadBez`, `CubicBez`, `PathSeg::intersect_line`, `Line`, `ParamCurve::eval`) end-to-end - no
internal fields are poked, no internal-only test scaffolding is used. This finding is about a pure
function's own reaction to a legitimate value of its own single input parameter (a `Line` that
happens to be degenerate), not about how one participant reacts to another participant's produced
input - there is no second participant to fake here (see `CONFIRMATION.md` section 11's own
carve-out for exactly this shape of finding), so a single-process repro is the correct and
sufficient shape.

## Class

logic (silent false negative - no crash, no panic, no UB; a wrong/incomplete result returned
without any error).

## Root cause

`PathSeg::intersect_line`'s `Quad`/`Cubic` arms determine the curve's own point `(x, y)` at each
candidate root `t` (from `solve_quadratic`/`solve_cubic`), then compute `u`, the fractional
position of that point *along the probe line* (`(x,y)` projected onto the vector `p1 - p0`), by
dividing the projection by the probe line's squared length: `u = ((x-p0.x)*dx + (y-p0.y)*dy) *
invlen2`. This is only well-defined when the probe line actually has a direction. When `p0 == p1`
(or the two points are extremely close), `dx` and `dy` are both `(near-)zero`, so:

- The numerator `(x-p0.x)*dx + (y-p0.y)*dy` is *always* `(near-)zero` too, regardless of `x`/`y` -
  it carries no information about where the candidate point actually is.
- `invlen2 = (dx*dx+dy*dy).recip()` blows up toward `f64::INFINITY` as the probe length shrinks
  toward zero, and is exactly `f64::INFINITY` when `dx == dy == 0.0` exactly.
- `(near-)zero * (near-)inf` produces `NaN` (or a numerically meaningless finite value just above
  the near-zero-length threshold), which then fails the `(0.0..=1.0).contains(&u)` range check
  every time (`NaN` compares `false` against everything), silently discarding what may be a
  perfectly genuine intersection.

There is a deeper reason a naive reciprocal-guard alone cannot fix this: when `dx == dy == 0.0`
*exactly*, the candidate-root coefficients themselves (`c0 = dy*(px0-p0.x) - dx*(py0-p0.y)`, `c1 =
dy*px1 - dx*py1`, `c2 = dy*px2 - dx*py2`, and `c3` for the cubic) are *also* identically zero for
every `t` - the whole "signed distance to the probe line" equation degenerates to `0 == 0`,
carrying no information about which `t` is the real intersection at all (`solve_quadratic(0, 0, 0)`
returns an arbitrary `t = 0.0` via its own degenerate-case fallback, unrelated to any real curve
point). So a correct fix for the exactly-degenerate case has to abandon the "signed distance along
the probe line's direction" formulation entirely and solve a different, but closely related,
problem instead: whether the point `p0` itself lies on the curve (see "The fix" below).

## Introduced by

e5a1c11 "Path segment intersection against line" (2020-05-28) - the original implementation of
`PathSeg::intersect_line`, including the unguarded `invlen2 = (dx*dx+dy*dy).recip()` in both the
`Quad` and `Cubic` arms, has been present essentially unchanged since this commit (`git blame`
confirms both `invlen2` lines are still attributed to it; a handful of later commits only touched
adjacent lines - iterator-loop style changes and the `EPSILON` range-check tolerance - not the
division itself).

## The fix

For each of the `Quad`/`Cubic` arms, branch on whether the probe line's squared length (`len2 =
dx*dx + dy*dy`) is below `EPSILON*EPSILON` (reusing the function's own existing `EPSILON = 1e-9`
tolerance, already used elsewhere in the same function for range checks):

- **Degenerate branch** (`len2 < EPSILON*EPSILON`): instead of projecting onto the (meaningless)
  probe direction, solve directly for `t` where the curve's own `x(t)` coordinate equals `p0.x`
  (`solve_quadratic(px0 - p0.x, px1, px2)` / `solve_cubic(px0 - p0.x, px1, px2, px3)` - the same
  coefficients already computed for the curve, just solved against a horizontal-line-style
  equation instead of the probe's own signed-distance equation), then, for each candidate `t`
  within range, accept it only if the curve's `y(t)` is also within `EPSILON` of `p0.y` - i.e.
  directly testing "does the curve pass through this point", which is the only well-posed question
  a truly zero-length probe line can ask. `line_t` (`u`) is reported as `0.0` in this branch (an
  arbitrary but always in-range choice, since a degenerate line has no meaningful "position along
  it" other than the single point it represents).
- **Non-degenerate branch** (`len2 >= EPSILON*EPSILON`): unchanged existing logic, just using the
  already-computed `len2` instead of recomputing `dx*dx+dy*dy`.

Verified against more than the one triggering input (per `CONFIRMATION.md` section 7): re-ran with
an exactly-degenerate probe (`Line::new(p, p)`), a near-zero-but-nonzero probe (`dx = 1e-100`), a
degenerate probe at the curve's own `t=0` endpoint, and - to check the fix doesn't turn *every*
degenerate probe into a false positive - a degenerate probe at a point 1.0 unit *off* the curve
(both Quad and Cubic). All four behaved correctly: the three on-curve cases each produced exactly
one `LineIntersection` at the expected `segment_t` (`0.5`, `0.3`, and `-0.0`/`0.0` respectively,
`line_t == 0.0` in every case), and the off-curve cases produced `[]`. See `output.txt` for the
full captured output of both `repro/`'s own run and these additional manual checks.

- Project test suite (`kurbo` crate): `cargo test -p kurbo --lib`: 187 passed, 0 failed before and
  after the patch (identical). `cargo test -p kurbo --doc`: 55 passed, 0 failed before and after
  the patch (identical). `cargo clippy -p kurbo --all-targets`: one pre-existing warning in
  `kurbo/src/common.rs` (`needless_late_init` around the `K_C` rescale in the quartic solver,
  unrelated to `bezpath.rs`) present identically before and after; no new warnings introduced.

## What's not done / open questions

- The `PathSeg::Line` arm was not touched. It already returns no intersection for a degenerate
  probe line via its own `det.abs() < EPSILON` "coincident lines" early return (mentioned in the
  candidate description as "unaffected" - it never produces `NaN`, since `det` itself, not a
  reciprocal of it, is what's compared). Whether `Line` vs `Line` *should* also detect "probe point
  lies on this line segment" for a degenerate probe (the same conceptual gap this fix closes for
  `Quad`/`Cubic`) was not investigated this session - it's a separate, narrower piece of the same
  general "what does the API mean by intersecting against a degenerate probe line" question, left
  open for a future candidate if it's judged worth closing.
- The fix's `EPSILON*EPSILON` threshold and the `(y - p0.y).abs() < EPSILON` point-on-curve
  tolerance both reuse the function's pre-existing `EPSILON = 1e-9` constant for consistency with
  the rest of the function, rather than introducing a new tunable - this is a reasonable default
  but not independently re-derived/justified against kurbo's general accuracy conventions
  elsewhere in the crate.
- Not checked: whether any other public API in kurbo builds a `Line` internally and could pass a
  degenerate one into `intersect_line` without the caller ever constructing one explicitly (e.g. a
  path-simplification or offsetting routine that could, for some inputs, compute two coincident
  points and treat them as a probe line) - this session only exercised `intersect_line` directly
  via its own public signature, not via other kurbo call sites that might feed it a degenerate line
  indirectly.
