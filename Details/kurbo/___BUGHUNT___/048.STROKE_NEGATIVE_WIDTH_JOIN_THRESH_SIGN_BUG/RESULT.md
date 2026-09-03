# STROKE_NEGATIVE_WIDTH_JOIN_THRESH_SIGN_BUG

`stroke_undashed` (kurbo/src/stroke.rs:309) computes `ctx.join_thresh = 2.0 * tolerance /
style.width` with no validation on `style.width` anywhere between the public `stroke()`/
`stroke_with()` entry points and this line (`Stroke::new`/`Stroke` fields are entirely
unvalidated; `is_finite()`/`is_nan()` exist but are opt-in diagnostics never called
automatically). This candidate started from the hypothesis that `style.width == 0.0` (or
`tolerance == 0.0` too) makes `join_thresh` infinite or `NaN` and could produce visibly wrong
join geometry or a crash. That specific hypothesis (zero width) turned out to be a dead end -
see "Status" below - but investigating it surfaced a real, distinct, confirmed bug in the same
line for **negative** `style.width`, which is an equally unvalidated, equally reachable input.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction via the public `stroke()` API: `Stroke::new(0.0)` and `Stroke::new(0.0)` with `tolerance = 0.0` too (so `join_thresh` is `+inf` or `NaN` respectively), stroking an ordinary rectangular closed path and a path with sharp/slight bends, checking every output point for `NaN`/`inf` | Debug build (`debug-assertions = true`, `overflow-checks = true`), ad hoc binary depending on the crate by path | No `NaN`/`inf` anywhere in the output for `width = 0.0` (any `tolerance`). Root-caused as harmless: `do_join`'s norm/offset vectors scale with `style.width`, so at `width = 0.0` every offset point collapses to the path's own centerline point regardless of whether a join is added or omitted - the join-omission decision genuinely doesn't matter when the stroke itself is already fully degenerate |
| Read `do_join`'s join-omission test (`dot <= 0.0 \|\| cross.abs() >= hypot * self.join_thresh`) algebraically for the sign of `join_thresh` in general (not just the zero/NaN case), since `style.width` is a plain unvalidated `f64` that can also be negative | Manual code reading | For negative `style.width`, `join_thresh` is negative, so `hypot * self.join_thresh <= 0.0` for any non-negative `hypot` - the test becomes true (a join is added) for essentially every joint, however small the bend angle, instead of only where a join is actually visually necessary. This is a real geometry defect distinct from the zero-width dead end: it doesn't require width or tolerance to be exactly zero, and it produces visibly different (extra) path elements, not just harmless degenerate points |
| Manual boundary-value construction: stroke a two-segment near-straight path (bend angles 0.0001-0.01 units over 100 units) with `+w` and `-w` (`w` in {0.3, 2.5}) for all three `Join` variants, comparing the resulting `BezPath`'s element count (join-omission is a property of the path's own bend angle and must not depend on the sign of `style.width`) | Debug build against the crate by path, then release build in the standalone `repro/` against a fresh clone | Every one of the 24 (`4 bends x 3 joins x 2 widths`) combinations mismatched: `+w` correctly omits the join (9 elements) while `-w` always inserts one (11 or 13 elements depending on join style), even at a bend of 0.0001 units over 100 units - a bend far too small to warrant a join under any reasonable tolerance |
| Repeat-run the standalone `repro/` binary 3 times against the same pristine clone (release build) | `cargo run --release`, same clone, no RNG/seed | Byte-for-byte identical output all 3 runs, including the exit code (`1` = bug present) |
| Applied the one-line fix (`style.width` -> `style.width.abs()`) directly to the clone, re-ran the same 24-combination check | Release build | All 24 combinations now match (9 elements for both `+w` and `-w` in every case) |
| Ran the project's full test suite (`cargo test -p kurbo`) both before and after the fix, in the same clone | Debug build (`cargo test` default profile) | 187 unit tests + 55 doctests pass, identically, before and after |

## Status

**Confirmed** - reproduces 3/3 runs, deterministic, release build, on a fresh clone of vanilla
HEAD. The originally-hypothesized zero-width/zero-tolerance case is **Not reproducible** as a
distinct defect (no NaN/inf, no crash, and the join-omission decision is provably irrelevant when
`style.width == 0.0` since every offset point already collapses to the centerline) - this is
noted here rather than as a separate candidate since it was the entry point that led to the
confirmed negative-width finding.

**Fully reproducible**: **Yes**. `repro/` calls the real public `stroke()` entry point with a
`Stroke` value a genuine caller can construct without any validation (`Stroke::new(-2.5)` -
negative widths are never rejected anywhere in the public API), on a plain, unremarkable path.
This is a single-process pure function on caller-controlled parameters - the same shape as "a
malformed file handed to a parser's own public entry point" in `CONFIRMATION.md` section 11 -
there is no second participant to fake here.

## Class

logic

## Root cause

`stroke_undashed` computes a single scalar `join_thresh = 2.0 * tolerance / style.width` that is
later compared against a magnitude (`cross.abs() >= hypot * self.join_thresh`, both `cross.abs()`
and `hypot` are always non-negative) to decide whether a given joint's bend angle is large enough
to need explicit join geometry (bevel/miter/round). This comparison is only meaningful when
`join_thresh` is itself non-negative, which the code silently assumes by dividing by the *signed*
`style.width` instead of its magnitude. `Stroke.width` has no sign constraint anywhere in the
public API (`Stroke::new`/the `width` field/the builder methods all accept any `f64`), so any
caller passing a negative width (e.g. from an upstream sign bug, or a design that uses width's
sign to mean "stroke the other side") makes `join_thresh` negative. A negative `join_thresh` makes
`hypot * self.join_thresh` non-positive, so the join-omission test is satisfied for essentially
every joint regardless of actual bend angle - every corner of a negative-width stroke gets a real
join (bevel/miter/round geometry) inserted, even where the bend is imperceptibly small and a
correctly-signed stroke of the same magnitude would correctly omit it. The result is a stroke
outline with visibly extra path elements (self-crossing notches at nearly-straight joints) purely
as an artifact of the width's sign, not of the path's actual geometry.

## Introduced by

be41b0d "Implement closed subpaths" (2023-05-30) - this is the commit that introduced
`join_thresh` itself (`ctx.join_thresh = 2.0 * tolerance / style.width;` and the
`cross.abs() >= hypot * self.join_thresh` comparison it feeds), unconditionally on the raw signed
`style.width`, from the start. Found via `git log -S"join_thresh"` across the file's full history
(including its `src/stroke.rs` -> `kurbo/src/stroke.rs` rename) and confirmed by reading the
introducing commit's diff directly.

## The fix

Applied directly to `kurbo/src/stroke.rs` (see `patch.diff`, against pristine HEAD
ca273499e3e48bd2de6f02aa8e99a148984e45f3):

```
-    ctx.join_thresh = 2.0 * tolerance / style.width;
+    ctx.join_thresh = 2.0 * tolerance / style.width.abs();
```

This restores the invariant that the join-omission decision depends only on the path's own bend
angle at a joint, never on the sign of `style.width` - `join_thresh` is now always non-negative
(or `+inf`/`NaN` in the already-harmless zero-width/zero-tolerance cases, unchanged from before),
matching the comment directly above the field (`// If hypot < (hypot + dot) * join_thresh, omit
join altogether.`), which already implicitly assumes a non-negative threshold.

Verified against more than just the one triggering input (`CONFIRMATION.md` section 7): checked
across 4 different bend angles (0.0001 to 0.01 units), all 3 `Join` variants, and 2 different
width magnitudes (0.3 and 2.5) - every one of the 24 combinations that mismatched before the fix
matches after it (both `+w` and `-w` correctly omit the join at that bend angle).

- Project test suite: 187 unit tests passed + 55 doctests passed, both before and after the fix
  (no failures either way - the fix does not change behavior for any non-negative width, which is
  what the existing test suite exercises).

## What's not done / open questions

- The zero-width/zero-tolerance case (the original hypothesis for this candidate) is confirmed
  harmless and is not re-checked further here; see "Status" above.
- This candidate only checked the join-omission test at line 461
  (`kurbo/src/stroke.rs`); it did not audit every other place `style.width` is used unsigned
  elsewhere in `stroke.rs` (e.g. `do_join`'s/`do_line`'s own `scale = 0.5 * style.width /
  tan0.hypot()`, which flips the offset side for negative width - this is a distinct, intentional-
  looking use, not flagged as a bug here, but not deeply audited either). A caller relying on
  negative width to mean "offset to the other side" would still get that behavior from `scale`;
  only the join-omission threshold itself was found to be broken by the sign.
- Not checked whether any downstream consumer of `stroke()` (e.g. a renderer) already normalizes
  `style.width` to a non-negative value before calling `stroke()`, which would make this defect
  unreachable in practice for that particular caller - this was not investigated since `stroke()`
  is a public API with no such requirement documented or enforced.
