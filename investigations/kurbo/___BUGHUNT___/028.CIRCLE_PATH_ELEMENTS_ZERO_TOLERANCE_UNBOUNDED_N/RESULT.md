# Circle::path_elements unbounded segment count for zero/near-zero tolerance

`Circle::path_elements(&self, tolerance)` (`kurbo/src/circle.rs:110-122`, `impl Shape for Circle`)
computes `scaled_err = self.radius.abs() / tolerance` and a segment count `n` from it with no
validation of `tolerance`. This looked suspicious for exactly the reason already-confirmed
candidate 005 (`Arc::append_iter`) was confirmed: `n` feeds a saturating `f64 as usize` cast that
becomes the returned iterator's own loop bound, with no upper clamp anywhere in between - but
`Circle`'s own `path_elements` is an independent implementation (it does not delegate to `Arc`,
unlike `Ellipse::path_elements` which already inherits candidate 005's fix/coverage), so it is a
distinct call site with the same missing input validation.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (pure arithmetic, no crate involved) | Standalone `rustc -O` snippet computing `scaled_err`/`n_f`/`n` by hand exactly as `circle.rs` does, for `radius=10.0`, `tolerance=0.0`, `tolerance=1e-300`, and `tolerance=NaN` | `n` (as `f64`) is `inf` for `tolerance=0.0`; `n as usize` saturates to `18446744073709551615` (`usize::MAX`). Same for `tolerance=1e-300` (astronomically large finite `f64`, still saturates on cast). `tolerance=NaN` instead saturates `n` to `0` - a separate, silent-empty-circle failure mode, not investigated further as its own bug here (see "What's not done" below). |
| Real end-to-end run of the public API, debug build (`debug-assertions=true`, `overflow-checks=true`) | Standalone crate depending on `kurbo` via a path dependency to this session's worktree, calling `Circle::new((0,0),10.0).path_elements(0.0)` and driving the returned iterator directly, 3s wall-clock cutoff, no element cap | 78,029,240 elements produced in ~2.9s without the iterator ever returning `None`; no panic, no debug-assertion failure (the cast is a saturating float-to-int cast, not checked arithmetic). |
| Standalone `repro/` (own `Cargo.toml`, bare `[workspace]`) against a real `git clone` of the project at the exact base commit, release build | `cargo run --release`, 20,000,000-element / 5s wall-clock cutoff | Cutoff fired: 20,000,000 elements produced in ~371ms without the iterator finishing on its own - see `output.txt`. Repeated 3 more times in release mode against the worktree harness (no element cap): 129,518,504 / 125,567,664 / 127,020,546 elements in 3s each, never finishing on its own. |

No further tool escalation (`miri`/ASAN/fuzzing) was needed: like candidate 005, this is a pure
floating-point arithmetic/cast boundary-value bug with a fully deterministic mechanism, not
memory-unsafety or input-format-dependent behavior, so the manual/debug-build approaches above
already gave a complete, repeatable picture (see `HEURISTICS.md`'s "arithmetic on untrusted
sizes/casts between integer widths" guidance).

## Status

**Confirmed**. Reproduces deterministically, 5/5 runs (the debug-build probe, the three release
repeat-runs above, plus the standalone `repro/` run against the pristine clone) - every run for
`tolerance = 0.0` (and for a tiny nonzero `tolerance`) produces tens to over a hundred million
elements within a few hundred milliseconds/seconds without the iterator ever completing on its own,
in both debug and release builds.

**Fully reproducible**: **Yes**. The repro calls `kurbo::Circle::path_elements`/`Shape::path_elements`
directly - the real, public API a genuine caller would use (e.g. via `Shape::to_path(0.0)`).
`tolerance` is an ordinary `f64` parameter passed straight through by any caller; there is no
second participant/peer/wire format involved to fake - see `CONFIRMATION.md` section 11's own
carve-out for "a single external artifact with no second participant to fake the reaction of",
which applies here identically to a malformed-file-to-a-parser case.

## Class

logic / resource-exhaustion (unbounded CPU/time; would also be unbounded memory for any caller
that collects the iterator, e.g. `Shape::to_path`/`BezPath::from_path_segments`, since nothing
about the iterator's own bound is used to fail fast - it just never stops producing items).

## Root cause

```rust
fn path_elements(&self, tolerance: f64) -> CirclePathIter {
    let scaled_err = self.radius.abs() / tolerance;
    let (n, arm_len) = if scaled_err < 1.0 / 1.9608e-4 {
        (4, 0.551915024494)
    } else {
        let n = (1.1163 * scaled_err).powf(1.0 / 6.0).ceil() as usize;
        let arm_len = (4.0 / 3.0) * (FRAC_PI_2 / (n as f64)).tan();
        (n, arm_len)
    };
    CirclePathIter { circle: *self, delta_th: 2.0 * PI / (n as f64), arm_len, ix: 0, n }
}
```

`tolerance` is never validated (not checked for `<= 0.0`, not checked for being merely very small
relative to `radius`). For `tolerance == 0.0`, `scaled_err` is `+inf` by IEEE-754 division, which
fails the fast-path check (`scaled_err < 1.0/1.9608e-4`) and falls into the `else` branch, where
`+inf` propagates through `powf`/multiplication/`.ceil()` to make the `f64` value of `n` itself
`+inf`. Rust's `f64 as usize` cast is a *saturating* cast (not a panic, not a wrap, per rustc's
documented float-to-int cast semantics), so `inf as usize` becomes `usize::MAX`
(`18446744073709551615`) silently, with no error and no debug-assertion failure. `CirclePathIter`'s
own `Iterator::next` only stops once `self.ix > self.n + 1`, so the loop bound becomes
`usize::MAX` - indistinguishable from unbounded for any real caller (at the observed rate,
exhausting `usize::MAX` iterations would take on the order of thousands of years). A `tolerance`
that's merely tiny but nonzero (e.g. `1e-300`) hits the exact same saturating-cast failure mode,
since `n` as an `f64` is already astronomically large before the cast. This is the same
unguarded-tolerance -> saturating-cast-to-`usize::MAX` pattern as already-confirmed candidate 005
(`Arc::append_iter`), but in `Circle`'s own independent implementation of the same idea (`Circle`
does not delegate to `Arc`, unlike `Ellipse::path_elements`, which already inherits 005's fix).

## Introduced by

a31e5393 "Add circle" (2019-02-19). Verified via `git log --follow --diff-filter=A --format=%H --
kurbo/src/circle.rs` (which resolves to this commit despite the later workspace-restructuring
rename) and `git show a31e5393 -- src/circle.rs`, which shows this exact `scaled_err`/`powf(1.0 /
6.0)`/`as usize` formula already present verbatim at the file's initial creation, unchanged in this
respect through the current HEAD (ca273499e3e48bd2de6f02aa8e99a148984e45f3).

## The fix

Clamp the computed segment count `n` to a fixed, generous upper bound (`MAX_CIRCLE_SEGMENTS = 1 <<
20` = 1,048,576, matching candidate 005's `Arc::MAX_ARC_SEGMENTS` choice for consistency) right
after the `.ceil()` that currently produces an unbounded value, before the cast to `usize`:

```rust
const MAX_CIRCLE_SEGMENTS: usize = 1 << 20;
...
let n = (1.1163 * scaled_err)
    .powf(1.0 / 6.0)
    .ceil()
    .min(MAX_CIRCLE_SEGMENTS as f64) as usize;
```

`f64::min` returns the non-`NaN` operand when either side is `NaN` (per its documented semantics),
so this also neutralizes the separate `NaN`-tolerance silent-empty-circle case (previously
saturating `n` to `0`) as a side effect, without needing an extra check.

Verified by re-running `repro/` against the patched clone: the iterator now returns `None` after
exactly 1,048,578 elements (`2 * MAX_CIRCLE_SEGMENTS + 2`, for the leading `MoveTo` + `n`
`CurveTo`s + `ClosePath`... actually `n` `CurveTo` + `MoveTo` + `ClosePath` = `n + 2`; the doubled
count in `output.txt` is because `n` itself is `1,048,576` and the iterator yields `n + 2`
elements) in ~20ms instead of running the cutoff out at 20,000,000 elements/371ms (see
`output.txt`). Also checked, per `CONFIRMATION.md` section 7, against inputs varying within the
same bug class rather than just the one triggering value: `tolerance = 0.0`, `tolerance = 1e-300`,
and `tolerance = NaN` all now terminate at the same 1,048,578-element count in ~20ms; normal,
non-degenerate tolerances (`0.1`, `1.0`) and the already-fast-path `tolerance = -0.0` still produce
the same small element counts (6) as before the patch, i.e. the fix does not change behavior for
any real-world tolerance value.

- Project test suite: **before** (main tree, unpatched, `cargo test --workspace`): 187 + 10 + 55
  tests passed, 0 failed, 1 ignored doctest (unrelated, pre-existing). **After** (patched worktree,
  same command): identical - 187 + 10 + 55 passed, 0 failed, 1 ignored. No new failures either
  direction.

## What's not done / open questions

- The `NaN`-tolerance case (silently saturating `n` to `0` pre-fix, i.e. a degenerate/empty circle
  with no visible curve at all) is a distinct silent-wrong-output bug from the `tolerance == 0.0`
  unbounded-iteration bug this candidate targets; it wasn't investigated as its own bug (one bug
  per candidate folder, per `SKILL.md`), though the chosen fix happens to neutralize it too as a
  side effect of `f64::min`'s NaN semantics.
- `CubicBez`'s own subdivision code (`kurbo/src/cubicbez.rs:94`,
  `let n = ((err / max_hypot2).powf(1. / 6.0).ceil() as usize).max(1);`) has a superficially similar
  `powf(1/6)`-then-`as usize` shape, already flagged by candidate 005 as an unconfirmed suspect -
  still not checked as part of this candidate either.
- Whether any other `Shape::path_elements`/`append_iter`-style method elsewhere in the crate shares
  this exact pattern wasn't swept exhaustively this session; only `Arc` (candidate 005) and
  `Circle` (this candidate) have been confirmed so far. `Ellipse::path_elements` was checked by
  reading `kurbo/src/ellipse.rs` and confirmed to delegate to `Arc::append_iter`, so it already
  inherits candidate 005's fix rather than needing its own.
- The chosen bound (`1 << 20`) is a judgment call matching candidate 005's constant for
  consistency across the crate, not derived from a documented project invariant - a maintainer may
  prefer a smaller/larger constant, a shared crate-wide constant instead of one per shape, or a
  `Result`-returning API instead of silent clamping.
