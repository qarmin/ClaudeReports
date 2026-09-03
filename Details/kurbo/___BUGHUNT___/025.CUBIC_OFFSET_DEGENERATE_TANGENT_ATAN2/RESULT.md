# CUBIC_OFFSET_DEGENERATE_TANGENT_ATAN2

Follow-up to candidate 010 (`SIMPLIFY_SAMPLE_PT_TANGENT_ATAN2_HAIRPIN`), which checked the same
"degenerate tangent at endpoint -> silently substituted wrong direction" suspicion against
`CubicOffset` (kurbo/src/offset.rs) and ruled it out for the exact input shapes it tried (a
line-as-cubic and a fully-degenerate all-four-points-equal cubic). This candidate re-examines
`CubicOffset`'s own tangent computation (`offset_cubic`'s call into `PathSeg::tangents()`,
kurbo/src/bezpath.rs) for a related but distinct trigger the earlier session didn't try: a
*partially* degenerate endpoint where the fallback-triggering check itself is unreliable, rather
than a fully zero difference vector.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: re-derive `PathSeg::tangents()`'s exact fallback logic (kurbo/src/bezpath.rs) and how `offset_cubic` (kurbo/src/offset.rs:123-125) consumes it | Read the full function body, its `EPS: f64 = 1e-12` constant, and every call site | The fallback check (`d01.hypot2() > EPS`) uses an **absolute** threshold on squared distance. At coordinate magnitudes where a single f64 ULP already has a squared length above `1e-12` (magnitude >~ 3.8e5, i.e. `ulp^2 > 1e-12` once `ulp > 1e-6`, which happens once coordinates exceed roughly `1e-6 / 2^-52 ~ 4.5e8`; empirically confirmed at `1e10` below), a control point that differs from its neighbor by only floating-point rounding noise is treated as a *real* tangent direction instead of triggering the "fall back to a farther point" logic. |
| Manual boundary-value construction: single f64 ULP perturbation at large coordinate magnitude (1e10), through the real public API | Standalone `repro/` (own `Cargo.toml`), calling `kurbo::offset::offset_cubic` directly and, more importantly, the real end-to-end public entry point `kurbo::stroke()`. Deterministic (no RNG) - re-ran 3x, byte-identical output every run. | **Reproduces.** A cubic with `p1` offset from `p0` by exactly 1 (or 2) f64 ULPs in an arbitrary direction (unrelated to the curve's actual shape, defined by `p2`/`p3`) causes `offset_cubic`'s very first control point to be placed in a direction ~14 degrees off from the geometrically-correct one (compared against a same-shape reference curve where `p1 == p0` exactly, which *does* hit the existing, already-correct, fully-degenerate fallback path). Confirmed both via `offset_cubic` called directly and via the real public `kurbo::stroke()` driver end to end (not just the internal helper) - see `output.txt`. |
| Differential comparison against a known-correct oracle (same curve shape, `p1 == p0` exactly vs. `p1` perturbed by 1-2 ULPs) | Same `repro/` binary; angle between the two strokes' initial offset directions | 3-2 ULP perturbations both produced the exact same wrong direction (`(5.0, 0.0)` instead of the correct `(4.8507..., -1.2127...)`), i.e. not an isolated fluke of one specific perturbation value - the bug is in the threshold logic itself, not a one-off floating-point coincidence. |

Stopped at 2 of the soft cap of 3 - the first two approaches already produced a clear, deterministic,
directly-verified signal via the real public API, and the differential check ruled out "one lucky
input" as an explanation before writing this up.

## Status

**Confirmed.** Deterministic, no randomness involved - reproduces byte-for-byte identically on
every run (verified 3/3, and again after the fix, 3/3 fixed). This is a pure, single-process
numerical/logic bug in the library's own tangent computation; there is no second participant
(peer/caller producing untrusted input) involved, so CONFIRMATION.md section 11's "fakes a second
participant" concern does not apply here - the repro drives the real public `kurbo::stroke()`
entry point directly with a self-contained input.

**Fully reproducible**: Yes. `repro/` calls the real public `kurbo::stroke()` function (not
`offset_cubic` in isolation, and not any internal/private helper) with an ordinary `BezPath`, the
same way any real caller of `kurbo::stroke()` would.

## Class

logic (silently wrong geometric output - a plausible-looking but incorrect offset-curve direction
at the start of an affected segment; no panic, no NaN, no crash).

## Root cause

`PathSeg::tangents()` (kurbo/src/bezpath.rs) computes a path segment's endpoint tangent robustly by
falling back to a farther control point ("p1-p0" too small -> try "p2-p0" -> try "p3-p0") when the
nearest control-point difference is "too close to zero to be a meaningful direction". That
"too close to zero" check is `d01.hypot2() > EPS` with `const EPS: f64 = 1e-12` - an **absolute**
threshold, independent of the actual coordinate magnitude of the curve. At large enough coordinate
values, a single f64 ULP of separation between two control points already produces a squared
distance comfortably above `1e-12` (confirmed empirically: at `base = 1e10`, one ULP is
`~1.907e-6`, whose square is `~3.638e-12 > 1e-12`). So a control point that is only
floating-point-rounding-noise away from an endpoint - not a genuine, intentional near-coincidence -
is accepted as if it were a real tangent direction, and the resulting (essentially arbitrary, noise
determined) unit vector is used directly by `offset_cubic` to place the offset curve's very first
control point (`c.p0 + d * utan0.turn_90()` in `offset_cubic`, kurbo/src/offset.rs:128, and
`rec.utan0` threaded through `CubicOffset::apply`/`draw_arc`/`subdivide` for the rest of the
recursive offset computation). The same flawed `EPS` is used for the `Quad` and `Cubic` branches of
`tangents()` alike, and for both the start and end tangent computation, so the same failure mode
applies symmetrically (start vs. end, cubic vs. quadratic segment) - only one representative case
(start tangent of a cubic) was exercised in the repro.

This is the same *family* of bug flagged in `projects/kurbo.md`'s risk map ("degenerate tangent at
endpoint -> silently substitutes a wrong direction") and checked (and ruled out for different exact
inputs) in candidate 010, but a different specific mechanism: not `fit.rs`'s `atan2(0,0) == 0`
special case (which candidate 010 correctly established `CubicOffset` never goes through), but a
scale-non-invariant epsilon in the shared, `CubicOffset`-consumed `PathSeg::tangents()` helper.

## Introduced by

5196345 "Penalize large d values in cubic fit" (2023-04-17, Raph Levien) - this commit added
`PathSeg::tangents()` (including the `const EPS: f64 = 1e-12` absolute-threshold fallback check)
to kurbo/src/bezpath.rs. `offset.rs`'s own use of `tangents()` for `offset_cubic`'s endpoint
tangent computation was confirmed present using the same absolute-threshold function at the
`base_commit` this candidate was investigated against; whether `offset.rs` started calling
`tangents()` in that same commit or adopted the already-existing (already-flawed) helper in a
later commit was not traced further - the flaw itself originates in `tangents()`, not in
`offset.rs`'s later use of it.

## The fix

Change `PathSeg::tangents()`'s threshold from an absolute `1e-12` on `hypot2()` to one scaled by
the segment's own extent, using the largest of the farther candidate-difference vectors already
computed by the fallback chain itself (`p2-p0`, `p3-p0`, `p3-p1` for a cubic; `p2-p0` for a quad)
as the reference scale, multiplied by the same `1e-12` relative factor. This mirrors the existing
"take the longer/more reliable reference vector" idea already used elsewhere in this crate (e.g.
`stroke.rs::do_cubic`'s `chord_ref` selection), rather than inventing a new pattern. See
`patch.diff` (kurbo/src/bezpath.rs, `+19/-7` lines).

Verified: `repro/` no longer reproduces after the patch is applied (angle between the perturbed and
reference curves' stroked start directions drops from -14.036 degrees to exactly 0.000 degrees; see
`output.txt` for both the before and after runs from a single `just test` invocation). Checked
against more than the one triggering perturbation value: both a 1-ULP and a 2-ULP offset (same
`repro/` binary, both variants tried during investigation) produce the identical wrong result before
the fix and the identical correct result after.

- Project test suite (`cargo test --workspace`, in the worktree used for this candidate):
  **before fix: 187 (kurbo lib) + 10 (polycool lib) + 55 (kurbo doctests) = 252 passed, 0 failed**
  (run 5x in a row for stability; a single earlier, unrelated flake was observed once in
  `polycool`'s own test binary before this session's controlled re-runs, disappeared on every
  subsequent run with or without this patch - not connected to `bezpath.rs`/`offset.rs`, not
  investigated further as part of this candidate).
  **after fix: same counts, 252 passed, 0 failed** (also run 5x in a row).

## What's not done / open questions

- Only the cubic segment's *start* tangent was exercised end-to-end through the public `stroke()`
  driver in `repro/`. The same absolute-threshold flaw symmetrically affects: the cubic's *end*
  tangent (`d23`/`d13` branch), and the `Quad` branch's start/end tangents (`PathSeg::tangents()`'s
  `PathSeg::Quad` arm) - not separately exercised this session, though the fix in `patch.diff`
  covers all of them via the same mechanism (each given its own scale-relative `eps`).
  `PathSeg::Line` has no fallback logic (a line's tangent is always `p1-p0`, degenerate or not) and
  was not touched.
- Other callers of `PathSeg::tangents()` besides `offset_cubic` (e.g. `stroke.rs::do_linear`'s own
  direct call for join tangents) were not individually checked for a matching observable artifact -
  the root cause is common to all of them since they share the one function, but no separate repro
  was built for `do_linear`'s specific usage.
- The exact coordinate-magnitude threshold at which this first becomes observable (~ok below
  roughly 1e8-1e9, confirmed reproducible at 1e10) was not precisely bisected - "large enough that
  1 ULP squared exceeds 1e-12" is stated analytically above but not swept across magnitudes.
- Whether real-world callers of `kurbo`/`stroke()` commonly operate at coordinate magnitudes large
  enough to trigger this (vs. typical UI/vector-graphics coordinate ranges, usually far below 1e8)
  was not investigated - this candidate only establishes that the bug is real and reachable through
  the public API at a magnitude that does occur (e.g. after a large `Affine` scale/translate, or in
  a rendering coordinate space that isn't pre-normalized), not how frequently it manifests in
  practice.
