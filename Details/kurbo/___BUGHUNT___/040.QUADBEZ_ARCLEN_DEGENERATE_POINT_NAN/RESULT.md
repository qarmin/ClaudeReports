# QUADBEZ_ARCLEN_DEGENERATE_POINT_NAN

`impl ParamCurveArclen for QuadBez::arclen` (`kurbo/src/quadbez.rs`, line 276 at commit ca27349)
picks between an analytical closed-form computation and a Legendre-Gauss-quadrature fallback for
near-straight curves via the guard `a < 5e-4 * c` (`a = d2.hypot2()`, `c = d1.hypot2()`). For a
fully degenerate `QuadBez` whose three control points all coincide (`p0 == p1 == p2`), both `a` and
`c` are exactly `0.0`, so the guard evaluates `0.0 < 0.0` (false) and the analytical branch is taken
instead. Inside that branch, `a2 = a.powf(-0.5)` becomes `+inf`, while `b = 2.0 * d2.dot(d1)` is
exactly `0.0`, so `b * a2` evaluates `0.0 * inf = NaN`, which poisons `ba_c2`/`v0` and propagates
through to the returned arc length. This looked suspicious because `CubicBez`'s equivalent
recursive-quadrature `arclen` correctly returns `0.0` for the analogous fully-degenerate cubic,
suggesting `QuadBez`'s closed-form branch is the outlier.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction via the real public `QuadBez::arclen` API: `QuadBez::new(Point::new(3.0,4.0), Point::new(3.0,4.0), Point::new(3.0,4.0)).arclen(1e-6)` (the exact case from the candidate report) | Debug build, `cargo run` in a scratch `examples/repro040.rs` inside a scratch worktree of the main project | Returned `NaN` on first try; reproduced 4/4 runs |
| Same, driven through the public `Shape::perimeter`/`Shape::area` entry points on a `BezPath` (a rectangle-like path with one degenerate `QuadTo(p, p)` segment inserted, `p == p == p`), plus additional degenerate variants at the origin and at negative coordinates, plus a real `QuadSpline::new(...).to_quads()` call over a spline with a duplicated interior knot (a realistic trigger: a spline with a repeated control point collapses one produced quad to `p0==p1==p2`) | Debug build, same scratch worktree | `perimeter()` returned `NaN` while `area()` stayed correct (`100.0`); origin/negative-coordinate degenerate cases both returned `NaN`; the spline's middle quad (the one spanning the duplicated knot) returned `NaN` while its two neighboring quads returned correct finite values |
| Standalone `repro/` crate (own `Cargo.toml`, bare `[workspace]`, depends on `kurbo` via its public API only) run against a fresh clone of the real repo at the base commit, via `just test` end-to-end (clone -> reproduce-before -> apply patch.diff -> reproduce-after) | Release build, real fresh `git clone https://github.com/linebender/kurbo.git` + `git checkout ca27349...` | `NaN` observed in all 5 checks (plain degenerate, origin, negative, `BezPath::perimeter`, and the spline's middle quad) before the patch; all become finite (correct or ~1e-15/1e-16 floating-point noise, never `NaN`) after applying `patch.diff` |

## Status

**Confirmed.** Reproduces deterministically (bit-for-bit identical `NaN` output and process exit
code) on every run: 4/4 in the initial worktree investigation, 3/3 further direct repeats of the
standalone `repro/` binary, plus the `just test` end-to-end run captured in `output.txt` - 8/8 total
runs, debug and release builds alike. This is a pure deterministic floating-point computation with
no randomness/seed dependency, so full bit-for-bit repeatability is the expected and observed
behavior.

**Fully reproducible**: **Yes**. `repro/` calls only kurbo's real public API
(`QuadBez::new`/`arclen`, `BezPath`/`Shape::perimeter`/`Shape::area`, `QuadSpline::new`/`to_quads`)
end-to-end - no internal fields are poked and no internal state is hand-constructed. Per
`CONFIRMATION.md` section 11's carve-out, a degenerate `QuadBez`/`BezPath`/`QuadSpline` is itself a
single external artifact handed straight to the library's own public entry points (the same shape
of input a font/SVG parser or a spline-fitting caller could genuinely produce) - there is no second
participant whose behavior needs to be faked here.

## Class

logic (a public API returning `NaN` instead of the correct finite arc length for valid, in-range
input - not a panic/crash, but a silent numeric-correctness violation that then propagates into any
caller relying on `arclen`/`perimeter`, e.g. flattening, stroking, or length-based parametrization
code downstream).

## Root cause

`QuadBez::arclen`'s branch selection guard `a < 5e-4 * c` is meant to route "nearly straight" curves
(where the analytical closed-form formula is numerically unstable) to a quadrature fallback instead.
It fails to also catch the *fully degenerate* case (`p0 == p1 == p2`, i.e. `a == 0.0 && c == 0.0`):
since `c` is also exactly `0.0` there, `5e-4 * c == 0.0`, so `a < 5e-4 * c` becomes `0.0 < 0.0`,
which is false - the guard was written assuming `c` (the chord length squared) stays meaningfully
positive whenever `a` is small, but that assumption breaks when the whole segment collapses to a
point. The analytical branch then computes `a2 = a.powf(-0.5) = 0.0f64.powf(-0.5) = f64::INFINITY`
and separately `b = 2.0 * d2.dot(d1) = 0.0` (both `d2` and `d1` are the zero vector), and the
product `b * a2` evaluates IEEE-754's `0.0 * INFINITY`, which is defined to be `NaN` - this `NaN`
then poisons `ba_c2` and ultimately the returned `v0`/arc length.

Distinct from candidate 011 (`EVAL_NAN_AT_ENDPOINTS_EXTREME_MAGNITUDE`, about `eval()`) and
candidate 015 (`FLATTEN_CUSP_ISSUE426`, about `estimate_subdiv`'s different `cross`-based
division) - this is specific to `arclen`'s own closed-form-vs-fallback branch selection.

## Introduced by

b3184bff12be0ce5c69d0eef4e9fc012a8a0d7ba "Clean up arclength" (2018-12-28) - the commit that moved
the analytical `QuadBez::arclen` implementation (including the `a < 5e-4 * c` guard, unchanged in
substance ever since per `git log -S"5e-4 * c"`) into the main library. This is the original
introduction of the closed-form branch and its guard, not a later regression - the bug has been
present since this commit, over 7 years ago.

## The fix

Widen the fallback guard to also catch the exact-zero-`a` case:
`if a < 5e-4 * c` -> `if a < 5e-4 * c || a == 0.0`. This is precise, not just a broad safety net:
`a == 0.0` is exactly the condition under which `a.powf(-0.5)` becomes `+inf` in the first place (any
`a > 0.0`, however small, produces a large-but-finite `a2`, and `b` in that case is still bounded, so
`b * a2` stays finite); routing that one case to the quadrature fallback (which computes finite
`hypot()` values on zero vectors, correctly yielding `~0`) removes the only source of the `inf * 0`
product without changing behavior for any other input. Verified against more than just the one
triggering input (per `CONFIRMATION.md` section 7): re-ran the standalone `repro/` against the
patched clone with all 5 checks (the reported `(3,4)` case, an origin-degenerate case, a
negative-coordinate degenerate case, the `BezPath::perimeter` case, and the `QuadSpline::to_quads`
duplicated-knot case) - all 5 came back finite (correct or ~1e-15/1e-16 floating-point noise) with no
`NaN`, and `cargo clippy -p kurbo --all-targets` raised no new warning on the changed line (the
`a == 0.0` exact-float comparison is not flagged by this project's configured lint set).

- Project test suite (`cargo test -p kurbo`): 187 unit tests + 55 doctests passed before the patch,
  187 unit tests + 55 doctests passed after the patch - identical pass counts, 0 failures either
  way.

## What's not done / open questions

- Only `QuadBez::arclen`'s own guard was checked/fixed. `CubicBez`'s recursive-quadrature `arclen`
  was checked as a reference and confirmed to already handle the fully-degenerate case correctly
  (returns `0.0`, no closed-form branch to go wrong), so it does not need the same fix - but no
  other quadratic-Bezier-specific arclength-adjacent code path (e.g. any other closed-form formula
  elsewhere in the crate that might share the same `a.powf(-0.5)`-style pattern) was swept for the
  same class of issue this session.
- No longer fuzzing campaign (via `fuzz-bug-repro`, run separately) was pointed at `QuadBez::arclen`
  specifically to look for other near-degenerate inputs producing inaccurate (not necessarily `NaN`)
  results - this candidate targeted the exact reported mechanism (`a == 0.0`) and its immediate
  neighbors, not a broader numerical-stability sweep of the whole formula.
