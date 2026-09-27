# PATHSEG_TANGENTS_QUAD_FALLBACK_ZERO_NAN

`PathSeg::tangents()` (`kurbo/src/bezpath.rs`, `pub(crate)`, backing `kurbo::stroke()`'s
`do_join`/`do_line` calls through `stroke_undashed`) computes an endpoint tangent for a `Quad` with
a single-level fallback (`d0 = if d01.hypot2() > EPS { d01 } else { q.p2 - q.p0 }`) that never
re-checks whether the fallback direction itself is zero. This looked suspicious because the call
site's guard (`p1 != p0 || p2 != p0` in `stroke_undashed`) only rules out the fully-degenerate
"all three points equal" case using *exact* equality, not the same `EPS`-scaled tolerance
`tangents()` itself uses internally - so a quad can pass the guard (`p1 != p0` by a few ULPs) while
still hitting the fallback-is-also-zero case (`p2 == p0` exactly), reported as candidate 057.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (release build, standalone crate against the public `kurbo::stroke()` API) | `BezPath` with `MoveTo(0,0) -> QuadTo((1e-7,0),(0,0)) -> LineTo(10,10)`, `Stroke::new(2.0)`, `kurbo::stroke(&path, &stroke, &Default::default(), 1e-3)` | Confirmed on the first try: stroked `BezPath` contains multiple `NaN` points/curves. No second tool needed - this is a deterministic, pure floating-point control-flow bug in a single-participant public API with no `unsafe`/untrusted memory/adversarial parsing, so escalating to miri/ASAN/fuzzing was judged not to add signal. |
| Same construction with 2 more inputs in the same bug class (different axis/magnitude, negative offset) | release build, same standalone crate | All three cases reproduce `NaN` identically on unpatched code, and all three stay finite after the fix (CONFIRMATION.md section 7) |
| Read the second, independent call site (`do_linear` in `kurbo/src/stroke.rs`, which calls `PathSeg::Cubic(c).tangents()` on the quad-raised-to-cubic curve) after the Quad-only fix didn't fully clear the `NaN` | code reading + re-running the same repro | Found the *same* fallback-can-be-zero flaw in the `Cubic` arm's own last-resort fallback (`c.p3 - c.p0`), reachable here because `q.raise()` turns the degenerate quad into a cubic with `c3 == c0` exactly while `c1`/`c2` are only a few ULPs away - fixed alongside the Quad arm (see "The fix") |

## Status

**Confirmed**. Reproduces deterministically 3/3 runs (release build, `cargo run --release`) for the
original repro input, and again 3/3 for each of two additional same-class inputs. Being pure,
deterministic floating-point control flow with a fixed input and no randomness/allocator/thread
dependency, it is expected to reproduce 100% of the time on any conforming `f64` implementation.

**Fully reproducible**: **Yes**. `repro/` calls the real public `kurbo::stroke()` on an ordinary
`BezPath` built through the public `BezPath` builder methods - the genuine external input a real
caller would supply. This is a single-participant numeric/control-flow bug (a `BezPath` is a single
external artifact, not a stand-in for a second participant's behavior), so CONFIRMATION.md section
11's two-participant repro requirement does not apply here.

## Class

logic (silent `NaN` propagation from a divide-by-zero derived from an incorrectly-zero tangent
vector, not a crash/panic)

## Root cause

`stroke_undashed` (`kurbo/src/stroke.rs`) guards a `QuadTo(p1, p2)` element with:

```rust
if p1 != p0 || p2 != p0 {
    let q = QuadBez::new(p0, p1, p2);
    let (tan0, tan1) = PathSeg::Quad(q).tangents();
    ctx.do_join(style, tan0);
    ...
```

`PathSeg::tangents()`'s `Quad` arm (`kurbo/src/bezpath.rs`) is:

```rust
let d01 = q.p1 - q.p0;
let d0 = if d01.hypot2() > EPS { d01 } else { q.p2 - q.p0 };
```

When `p1` is a few ULPs away from `p0` (so the call-site guard's `p1 != p0` passes, and `d01` is
nonzero but its `hypot2()` is below `EPS = 1e-12`) and `p2 == p0` exactly, the fallback `q.p2 -
q.p0` is also exactly `(0, 0)`, so `d0` (and, symmetrically, `d1`) comes out as an exact-zero
vector even though the segment is not fully degenerate. `do_join` (`kurbo/src/stroke.rs`) then
computes `let scale = 0.5 * style.width / tan0.hypot();` - dividing by `0.0` produces `inf`, which
propagates into `norm` and every subsequently-emitted point/curve as `NaN`.

The `Cubic` arm of `tangents()` has the identical structural flaw at its own last-resort level
(`c.p3 - c.p0`, reached when `d01`/`d02` are both below `EPS`) - this is not merely theoretical
here: `stroke_undashed`'s `QuadTo` handling calls `ctx.do_cubic(style, q.raise(), tolerance)`, which
detects the raised, still-degenerate cubic as "potentially linear" and routes it through
`do_linear`, which makes its *own*, independent call to `PathSeg::Cubic(c).tangents()` on the raised
curve. For the repro input, the raised cubic has `c0 == c3` exactly (both equal to `q.p0 == q.p2`)
while `c1`/`c2` are only a few ULPs away from `c0` - the same "preferred directions below EPS, final
fallback exactly zero" shape, just on the `Cubic` arm instead of the `Quad` arm. Fixing only the
`Quad` arm removed the first `NaN` (from the outer `do_join(style, tan0)` call) but left a second,
independent `NaN` origin at `do_linear`'s own `tangents()` call untouched, confirmed by re-running
the repro after the Quad-only fix and observing `NaN` was still present.

## Introduced by

5196345f6ec7697a5491f8b819338fe2e2af82a1 "Penalize large d values in cubic fit" (2023-04-17),
which added `PathSeg::tangents()` to `src/bezpath.rs` with both the `Quad` arm's single-level
fallback and the `Cubic` arm's own final fallback (`c.p3 - c.p0`) exactly as they read today - found
via `git log --all -S "fn tangents" -- src/bezpath.rs` (the file was later moved to
`kurbo/src/bezpath.rs` by d67b9dd "Make `kurbo` a crate within a workspace (#434)", a pure path
rename with no logic change).

## The fix

For both the `Quad` and `Cubic` arms of `PathSeg::tangents()`, added a further fallback level that
only triggers once every "preferred, significant" candidate direction has already failed the `EPS`
check: pick whichever remaining candidate is *exactly* nonzero, rather than returning whatever the
old code's final, un-rechecked fallback happened to be. This guarantees a nonzero tangent whenever
at least one of the segment's control-point deltas is not exactly `(0, 0)` - i.e. whenever the
segment isn't fully collapsed to a single point - while leaving the existing "prefer the immediate
neighbor's direction, fall back to the whole chord" behavior unchanged for every already-working
(non-degenerate, or fully-degenerate-to-a-point) case.

Verified against `repro/` (see `output.txt` for the actual before/after run via `just test`, a real
clone of the project at `base_commit` with `patch.diff` applied in between): before the fix, all
three cases (A/B/C) produce `BezPath`s containing `NaN`; after the fix, all three stay fully finite.
Also re-checked directly in the working git worktree with the same three inputs, each individually
confirmed to reproduce on pristine code and to stay `NaN`-free after the patch (CONFIRMATION.md
section 7 - not just the one input that first triggered the finding).

- Project test suite (`cargo test --workspace`, from the repo root): 187 unit tests + 10 additional
  `kurbo` tests + 55 doctests passing before the fix (252 total, 0 failed), and the same 252
  passing, 0 failed after the fix. No new failures either way.

## What's not done / open questions

- The `Cubic` arm's own fallback was fixed here only because it was independently reachable from
  this exact repro (via `q.raise()` + `do_linear`), not because a standalone cubic-only trigger was
  separately constructed and confirmed. A `CurveTo` with `p1`/`p2` a few ULPs from `p0` and `p3 ==
  p0` exactly would plausibly hit the same `Cubic`-arm bug directly (without going through a
  raised quad first) - this was not independently verified as its own repro this session, though
  the fix covers it by construction (same code path, same fallback logic).
- `simplify.rs`'s corner-detection code, mentioned in the candidate report as also consuming
  `PathSeg::tangents()`, was not independently re-checked for a still-live symptom after this fix -
  the candidate report describes it as a "non-NaN consequence" (a wrong-but-finite corner
  decision), which is a different observable failure mode than the `stroke()`/`do_join` `NaN`
  path verified here.
- No fuzzing/miri/ASAN was used - this is a pure, deterministic floating-point control-flow bug
  with no `unsafe`, no untrusted memory, and no adversarial parsing/decoding involved, so manual
  boundary-value construction alone was judged sufficient (per the soft-cap-of-3 guidance: one
  clean, deterministic, immediately-confirmed approach, escalated once to read a second call site
  once the first fix didn't fully clear the symptom, is enough here).
