# simplify_bezpath unwrap panics on a PathEl sequence not starting with MoveTo

`kurbo::simplify::simplify_bezpath` (`kurbo/src/simplify.rs:298`) accepts
`impl IntoIterator<Item = PathEl>` - any iterator of path elements, not specifically a `BezPath`.
Its `LineTo`/`QuadTo`/`CurveTo` arms (lines 315, 322, 329) each call `last_pt.unwrap()`, but
`last_pt` is only ever set to `Some` after a `MoveTo` element has been seen. A caller-supplied
sequence whose first element is `LineTo`/`QuadTo`/`CurveTo` (no preceding `MoveTo`) panics
unconditionally, in both debug and release builds (this is a plain `Option::unwrap()`, not
overflow/debug-assertion-gated). Looked suspicious because `BezPath`'s own "must start with
MoveTo" invariant is enforced only by a `debug_assert!` on `BezPath`'s own builder methods
(`line_to`/`quad_to`/`curve_to`), not on an arbitrary `PathEl` iterator handed to this free
function - so nothing at the type level or in the release build prevents a caller from passing
a malformed sequence straight through.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (empty state, first element not MoveTo) | `cargo run` debug build, `vec![PathEl::LineTo(Point::new(1.0,1.0))]` fed to `simplify_bezpath` | Panics immediately at simplify.rs:315:36, `Option::unwrap()` on `None` |
| Same input, release build (`cargo run --release`) | release profile | Panics identically - `.unwrap()` is unconditional, not a debug-assertion artifact |
| Repeat runs (repeatability check) | 3x debug build, fresh independent clone each session | 3/3 identical panic, same location/message |
| Boundary variants for the other two arms (`QuadTo`-first, `CurveTo`-first) and a mixed case (garbage draw commands before `MoveTo`, then a real subpath) | debug build, against the *patched* tree, to validate the fix generalizes | All handled without panicking (see `output.txt`) |

No sanitizer/miri/fuzzing was needed - this is a plain, deterministic logic error reachable by
direct boundary-value construction of the input, not memory-unsafety or something needing
adversarial search.

## Status

**Confirmed**. Reproduces 3/3 runs, both debug and release builds, deterministic (no seed/layout
dependency - a plain `Option::unwrap()` on `None`, not a race or a floating-point edge case).

**Fully reproducible**: **Yes**. The repro calls the real public API (`kurbo::simplify::simplify_bezpath`)
directly with a `Vec<PathEl>` a real caller could construct by hand (or produce by filtering/
transforming an existing path's elements) - this is a single-artifact input (a `PathEl` sequence),
not an interaction between two participants, so no second participant needs to be simulated
(see `CONFIRMATION.md` section 11's "single-process repro remains completely fine" carve-out: a
malformed input handed to a function's real public entry point *is* the genuine external input).

## Class

crash

## Root cause

`simplify_bezpath` (kurbo/src/simplify.rs:298-358) tracks the current point in a local
`last_pt: Option<Point>`, initialized to `None` and only set to `Some` inside the `MoveTo` arm
(line 312). The `LineTo`/`QuadTo`/`CurveTo` arms all assume a preceding `MoveTo` already ran and
call `last_pt.unwrap()` unconditionally (lines 315, 322, 329) to get the segment's start point.
Nothing enforces that assumption for this function's actual parameter type
(`impl IntoIterator<Item = PathEl>`): `BezPath`'s own "must start with MoveTo" contract is
enforced by a `debug_assert!` inside `BezPath`'s *own* builder methods (`line_to`, `quad_to`,
`curve_to`, `close_path` - see `kurbo/src/bezpath.rs:242,254,266,275`), which (a) only fires in
debug builds and (b) only applies when a `BezPath` is built through those methods - not when an
arbitrary `Vec<PathEl>`/iterator (never built through `BezPath`'s own API at all) is handed
straight to `simplify_bezpath`. So a caller that constructs `PathEl`s directly (a very natural
thing to do - `PathEl` is a plain public enum) and passes a sequence starting with a drawing
command instead of `MoveTo` hits an unconditional panic with no debug/release distinction.

## Introduced by

5196345 "Penalize large d values in cubic fit" (2023-04-17, Raph Levien) - this is the commit
that introduced `simplify_bezpath` itself (`git log -S last_pt.unwrap() --follow`, cross-checked
against `git log --follow` for `kurbo/src/simplify.rs`'s full history including its pre-workspace
path `src/simplify.rs`). Despite the commit's subject being about cubic-fit tuning, its diff shows
it's the commit that added the whole `simplify_bezpath` function (150 insertions across
`src/bezpath.rs`/`src/fit.rs`/`src/simplify.rs`), already containing all three `last_pt.unwrap()`
call sites verbatim. The function was not present at all in the prior commit on this file's
history (900cffb "Improve documentation"). The `unwrap()`s survived unchanged through later
edits, including b2a16a5 "Allow lines in simplify input" (2024-03-13, adds the `LineTo` arm's
existing-point-equality check but keeps the same unguarded `last_pt.unwrap()` pattern) and the
2024 workspace restructuring (d67b9dd) that moved the file to `kurbo/src/simplify.rs`.

## The fix

Replace each `let last = last_pt.unwrap();` with `let Some(last) = last_pt else { continue; };` -
i.e. treat a drawing command with no established current point as a no-op (skip that element)
rather than panicking, matching how `BezPath`'s own "uninitialized subpath" debug-assertion
already documents the intended semantics of that state (an operation on a path with no current
point is meaningless, not fatal). This is the minimal, targeted fix: it does not change behavior
for any well-formed input (a `BezPath` built normally always emits `MoveTo` before any drawing
command, so `last_pt` is always `Some` by the time these arms run for real callers) and does not
change the crate's public API.

Checked against more than just the exact triggering input (`CONFIRMATION.md` section 7): verified
`QuadTo`-first and `CurveTo`-first (the other two `unwrap()` sites) no longer panic either, and a
mixed case (a `LineTo` and a `QuadTo` with no preceding `MoveTo`, followed by a real, well-formed
`MoveTo`-started subpath) correctly discards the leading garbage and simplifies only the real
subpath - see `output.txt`. Also re-ran the pre-existing `simplify_lines_corner` unit test
(exercises the normal, well-formed path) to confirm well-formed inputs are byte-for-byte
unaffected.

- Project test suite (kurbo package): 187 unit tests + 55 doctests passed before the patch; 187
  unit tests + 55 doctests passed after the patch (0 failures either way - the patch doesn't
  change behavior for any currently-tested input, only for the previously-panicking malformed one).

## What's not done / open questions

- Only `simplify_bezpath` itself was checked. `SimplifyBezPath::new` (called internally by
  `simplify_bezpath` on its own accumulated queue, which is always well-formed by construction)
  was not separately audited for the same pattern - not applicable here since it's fed only
  internally-generated, always-valid input, not raw caller `PathEl`s.
- Did not check other kurbo public functions accepting `impl IntoIterator<Item = PathEl>` (e.g.
  `flatten`, `segments`) for the same "assumes MoveTo-first, iterator input isn't a `BezPath`"
  pattern - worth a follow-up sweep since the underlying gap (this function's parameter type
  doesn't carry the "well-formed path" invariant `BezPath` documents but doesn't enforce in
  release) is generic to any function with this same signature shape, not unique to `simplify.rs`.
- No fuzzing was run; this bug class (deterministic panic on a specific, small, hand-constructible
  boundary input) doesn't need it - boundary-value construction found and confirmed it directly.
