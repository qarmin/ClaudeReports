# SVG arc: debug_assert / div-by-zero on a tiny-but-nonzero rotated half-chord

`Arc::from_svg_arc` (kurbo/src/svg.rs), reachable from the public `BezPath::from_svg`, computes
`sum_of_sq = (rx*p.y)^2 + (ry*p.x)^2` from the rotated half-chord `p` between an SVG arc's `from`
and `to` endpoints, then asserts it is nonzero before dividing by it. `from != to` is checked
earlier (`SvgArc::is_straight_line()`), but for an endpoint difference small enough (e.g. `1e-200`),
squaring `p`'s components still underflows to exactly `0.0` in `f64` even though `p` itself is
nonzero, so the assumption `sum_of_sq != 0.0` can be false anyway. This candidate came from reading
`svg.rs` directly (a `debug_assert` gating a division, right after a comment admitting the case it
guards against, is a standard "boundary value that slips past the informal guard" pattern).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction | `cargo run` (debug), input `"M0,0 A25,25 0 0,1 1e-200,0"` via `BezPath::from_svg` | Panics: `assertion failed: sum_of_sq != 0.0` at svg.rs:463, every run (3/3) |
| Same input, `cargo run --release` (debug_assert compiled out) | release build | No panic; `Arc::from_svg_arc` returns `Some(Arc)` with the division at svg.rs:471-473 producing `NaN`; downstream `Arc::to_cubic_beziers` silently emits zero `CurveTo`s (every comparison against `NaN` is false), so the arc segment is silently dropped from the parsed path |
| 3 additional boundary-value variants (different sign/rotation/radii/magnitude) | debug build, patched tree | No panics; one variant (non-degenerate tiny-radii arc) still produces a real `CurveTo`-based arc, confirming the fix doesn't misfire on legitimate arcs |

Only one tool was needed (manual boundary-value construction, per `HEURISTICS.md`'s guidance for
"arithmetic on untrusted sizes" - this is arithmetic on untrusted `f64` magnitudes) - it reproduced
on the first attempt with no ambiguity, so `miri`/fuzzing were not additionally invoked.

## Status

**Confirmed**. Reproduces 3/3 runs, debug build (`cargo build`/`cargo run` default profile,
`debug-assertions = true`), deterministic (no seed, no layout dependence - pure floating-point
underflow from a fixed input). Also confirmed the release-mode counterpart (NaN silently dropping
the arc segment from the parsed path) on the same input, single observation each for the "OK, path
= ..." result at both the pristine and patched revisions (deterministic floating-point computation,
not a flaky signal - re-run trivially via `just test` in this folder).

**Fully reproducible**: Yes. The repro calls `kurbo::BezPath::from_svg`, the exact public entry
point a real caller parsing an externally-supplied SVG path string would use, with a single
self-contained string literal as the "adversarial input" - there is no second participant to fake
here (a malformed/adversarial string handed directly to a parser's own public parsing entry point
is the genuine external input itself, per `CONFIRMATION.md` section 11's own carve-out for this
shape of finding).

## Class

crash (debug build: `debug_assert!` panic) / logic (release build: `NaN` silently propagates into
the parsed `Arc`, and the affected arc segment is silently dropped from the output path instead of
raising an error or falling back to a line, unlike the neighboring `is_straight_line()` case).

## Root cause

`Arc::from_svg_arc` computes the half-chord `p` between `from` and `to` (already rotated by
`x_rotation`), then `sum_of_sq = (rx * p.y)^2 + (ry * p.x)^2`, and immediately divides by
`sum_of_sq` (via `coe`, then `transformed_cx`/`transformed_cy`) assuming it is nonzero. The function
already special-cases the true degenerate case (`from == to`, or a radius of ~0) via
`SvgArc::is_straight_line()`, with the comment "Have to check this first, otherwise `sum_of_sq` will
be 0" - but that guard only rules out `p == (0.0, 0.0)` exactly. It does not account for `p` being
merely small enough that squaring its components underflows `f64` to `0.0` while `p` itself is still
representably nonzero (e.g. `p.x` on the order of `5e-201`: `p.x * p.x` underflows below
`f64::MIN_POSITIVE`'s subnormal range to exactly `0.0`). The `debug_assert!` at svg.rs:463 exists
precisely because the author was aware `sum_of_sq` could theoretically be `0.0` if this assumption
is violated, but only enforced it as a debug-only invariant rather than handling it, so:
- **Debug builds**: the `debug_assert!` panics, denial-of-service on adversarial/degenerate but
  well-formed SVG path strings passed to a safe public API.
- **Release builds**: the assert is compiled out, `coe = sign_coe * ((rxry*rxry - 0.0) / 0.0).abs().sqrt()`
  evaluates to `NaN` (or `inf` depending on `rxry`), and that `NaN` propagates into `transformed_cx`/
  `transformed_cy`, then into the returned `Arc`'s `center`/`start_angle`/`sweep_angle`. The caller
  in `BezPath::from_svg` (svg.rs:219-226) then calls `arc.to_cubic_beziers(...)`, whose internal
  angle-stepping loop compares against `sweep_angle`; since every comparison against `NaN` is
  `false`, the loop's closure is never invoked and the arc segment silently vanishes from the parsed
  path (no `CurveTo`, no error, no fallback `LineTo`) - a silent, non-crashing correctness bug for
  any consumer of the release build.

## Introduced by

dfa4c377732b49d6cdee6c9b6ee043b8ad5ba899 "Minor tweaks to bezier paths" (2019-05-03) - this commit
introduced `fn from_svg_arc`, the `sum_of_sq` computation, and the original
`debug_assert_ne!(sum_of_sq, 0.0)` (later reformatted to `debug_assert!(sum_of_sq != 0.0)` by
d67b9dd, a pure workspace-restructure/rustfmt commit that did not change the logic) in what was then
`src/svg.rs`. Confirmed via `git log -S "fn from_svg_arc" --all -- src/svg.rs` /
`git log -S "sum_of_sq" --all -- src/svg.rs`, both bottoming out at dfa4c377. The logic at the
affected lines has not materially changed since.

## The fix

Replace the `debug_assert!(sum_of_sq != 0.0)` with a real runtime check that treats
`sum_of_sq == 0.0` the same way `SvgArc::is_straight_line()`'s existing degenerate case is treated:
return `None` (the existing "this is effectively a straight line" signal `BezPath::from_svg`'s
caller already handles by emitting a `LineTo` instead of a curved arc segment - see svg.rs:219-226).
This is the minimal, already-supported fallback the codebase uses for every other arc degeneracy, so
no new error variant or public API change is needed.

```
-        debug_assert!(sum_of_sq != 0.0);
+        // `from != to`, but for a rotated half-chord `p` tiny enough, squaring its
+        // components can still underflow to exactly 0.0 even though `p` itself is
+        // nonzero: treat that as effectively a straight line rather than panicking
+        // (debug) or dividing by zero to produce a `NaN` `Arc` (release).
+        if sum_of_sq == 0.0 {
+            return None;
+        }
```

Verified against the original triggering input (no longer panics in debug, no longer produces NaN
in release - both builds now return `BezPath([MoveTo((0.0, 0.0)), LineTo((1e-200, 0.0))])`) and
against 3 additional boundary-value variants in the same class (different sign/rotation/radii/
magnitude - see "What was tried"), one of which is a legitimate non-degenerate tiny-radii arc that
still correctly produces a `CurveTo`-based arc after the fix, confirming the new check doesn't
over-trigger on real arcs (`CONFIRMATION.md` section 7).

- Project test suite (`cargo test -p kurbo`, unit + doctests): 187 unit + 55 doctests passed, 0
  failed, both before and after the patch (identical pass counts, no new failures).

## What's not done / open questions

- The same underflow-to-zero pattern could in principle recur anywhere else in kurbo that divides
  by a squared-magnitude quantity derived directly from untrusted input without an explicit
  nonzero check; this session only checked `Arc::from_svg_arc`, not a codebase-wide sweep for the
  same class (worth a dedicated `grep` for `debug_assert.*!= 0` / `debug_assert_ne!(.*, 0` pairs
  guarding a division elsewhere, as a follow-up candidate).
- Whether `f64::MIN_POSITIVE`-class underflow could also affect the `rf > 1.0` scale-up branch
  earlier in the same function (a separate division by `rx*rx`/`ry*ry`) was not specifically
  investigated - `rx`/`ry` come from `arc.radii`, already checked nonzero by
  `is_straight_line()`'s `1e-5` threshold, so this looked out of scope for the same bug class, but
  wasn't exhaustively boundary-tested.
- No longer-running fuzz campaign (`fuzz-bug-repro`) was pointed at `BezPath::from_svg` this
  session; the boundary-value approach found this on the first attempt, but a real corpus-based
  fuzz run could surface sibling issues in the same parser.
