# RoundedRect::path_elements / CircleSegment::path_elements unbounded segment count via Arc::append_iter

`RoundedRect::path_elements` (`kurbo/src/rounded_rect.rs:159`) and `CircleSegment::path_elements`
(`kurbo/src/circle.rs:326`) both build one or more `Arc` values from the shape's own geometry and
delegate straight to `Arc::append_iter(tolerance)` (`kurbo/src/arc.rs:77`) for the actual curved
segments. That function has the unguarded-tolerance/saturating-cast bug already confirmed by
candidate 005 (`Arc::append_iter` itself) still present verbatim in the current codebase - 005's
own `patch.diff` was never applied to the real source (per the workspace's own rule that the main
tree is always reverted after a candidate; `MAX_ARC_SEGMENTS` is absent from `arc.rs` at
`HEAD=ca273499e3e48bd2de6f02aa8e99a148984e45f3`, confirmed by `grep`). This candidate is not a new
root cause - it is two additional public-API call sites (beyond `Arc` itself and `Circle`'s own
independent implementation, already flagged by 005/028) that inherit the exact same bug because
they funnel through the same unfixed function.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (pure arithmetic, no crate involved) | Confirmed by inspection that `RoundedRect::path_elements` and `CircleSegment::path_elements` build `Arc { .. }.append_iter(tolerance)` with the caller's `tolerance` passed straight through, unmodified/unvalidated, at each of the (4 for RoundedRect, 2 for CircleSegment) call sites | Same arithmetic as 005: `tolerance=0.0` makes `scaled_err` `+inf`, propagating through `powf`/`.max()`/multiplication/`.ceil()` to make `n` (as `f64`) `+inf`; `n as usize` is a saturating cast, so it becomes `usize::MAX` at every one of these call sites identically. |
| Real end-to-end run of the public API, debug and release builds | Standalone probe crate depending on `kurbo` via a path dependency to an isolated git worktree at `HEAD`, calling `RoundedRect::new(0.0,0.0,100.0,100.0,10.0).path_elements(0.0)` and `CircleSegment::new((0,0),100.0,10.0,0.0,PI).path_elements(0.0)`, driving each returned iterator directly, 3s wall-clock cutoff + 20,000,000-element cap, no cap on element production itself | Debug build: 20,000,000 elements produced for each of the two shapes in ~2.5s without either iterator ever returning `None`. Release build, 4 repeat runs: `RoundedRect` produced 20,000,000 elements in ~1.05s each run; `CircleSegment` produced 20,000,000 elements in ~0.84s each run - neither ever finished on its own. See `output.txt`'s "Repeat runs" section. |
| Standalone `repro/` (own `Cargo.toml`, bare `[workspace]`) against a real `git clone` of the project at the exact base commit, release build | `cargo run --release`, 20,000,000-element / 5s wall-clock cutoff, via the candidate's own `justfile`'s `test` recipe (`clone` -> `reproduce-before` -> `apply` -> `reproduce-after`), actually run end-to-end | `reproduce-before` (unpatched clone): cutoff fired for both shapes - `RoundedRect` 20,000,000 elements/971ms, `CircleSegment` 20,000,000 elements/359ms, neither iterator finishing on its own (exit code 1). `reproduce-after` (same clone, `patch.diff` applied to `arc.rs`): `RoundedRect` finished on its own at 4,194,309 elements/209ms, `CircleSegment` finished on its own at 2,097,155 elements/38ms (exit code 0). See `output.txt`. |

No further tool escalation (`miri`/ASAN/fuzzing) was needed: like 005 and 028, this is a pure
floating-point arithmetic/cast boundary-value bug with a fully deterministic mechanism inherited
unchanged from an already-confirmed root cause, not memory-unsafety or input-format-dependent
behavior - the manual/debug-build/real-clone approaches above already gave a complete, repeatable
picture (see `HEURISTICS.md`'s "arithmetic on untrusted sizes/casts between integer widths"
guidance).

## Status

**Confirmed**. Reproduces deterministically, 5/5 runs for each shape (1 debug-build run + 4
release-build repeat runs against the isolated worktree, plus the standalone `repro/` run against
the pristine clone via `just test`) - every run for `tolerance = 0.0` produces 20,000,000+ elements
within roughly 0.2-2.5 seconds without either iterator ever completing on its own, in both debug
and release builds.

**Fully reproducible**: **Yes**. The repro calls `kurbo::RoundedRect::path_elements` and
`kurbo::CircleSegment::path_elements` directly via the `Shape` trait - the real, public,
single-argument API a genuine caller would use (e.g. via `Shape::to_path(0.0)`). `tolerance` is an
ordinary `f64` parameter passed straight through by any caller; there is no second
participant/peer/wire format involved to fake - see `CONFIRMATION.md` section 11's own carve-out
for "a single external artifact with no second participant to fake the reaction of", identical to
005's and 028's own justification.

## Class

logic / resource-exhaustion (unbounded CPU/time; would also be unbounded memory for any caller
that collects the iterator, e.g. `Shape::to_path`/`BezPath::from_path_segments`, since nothing
about either iterator's own bound is used to fail fast - it just never stops producing items).

## Root cause

`RoundedRect::path_elements` (`kurbo/src/rounded_rect.rs`):

```rust
let build_arc_iter = |i, center, ellipse_radii| {
    let arc = Arc {
        center,
        radii: ellipse_radii,
        start_angle: FRAC_PI_2 * i as f64,
        sweep_angle: FRAC_PI_2,
        x_rotation: 0.0,
    };
    arc.append_iter(tolerance)
};
```

`CircleSegment::path_elements` (`kurbo/src/circle.rs`):

```rust
.chain(self.outer_arc().append_iter(tolerance))
...
.chain(self.inner_arc().append_iter(tolerance))
```

Both pass the caller's `tolerance` straight through, unmodified, into `Arc::append_iter`
(`kurbo/src/arc.rs:77-85`), which is exactly candidate 005's already-confirmed root cause:

```rust
let scaled_err = self.radii.x.max(self.radii.y) / tolerance;
let n_err = (1.1163 * scaled_err).powf(1.0 / 6.0).max(3.999_999);
let n = (n_err * self.sweep_angle.abs() * (1.0 / (2.0 * PI))).ceil();
let angle_step = self.sweep_angle / n;
let n = n as usize;
```

For `tolerance == 0.0`, `scaled_err` is `+inf` by IEEE-754 division, which propagates through
`powf`/`.max()`/multiplication/`.ceil()` to make the `f64` value of `n` itself `+inf`. Rust's
`f64 as usize` cast is a *saturating* cast (not a panic, not a wrap), so `inf as usize` becomes
`usize::MAX` (`18446744073709551615`) silently, with no error and no debug-assertion failure.
`ArcAppendIter::next` only stops once `self.idx >= self.n`, so the loop bound becomes `usize::MAX`
at each of `RoundedRect`'s 4 corner-arc call sites and `CircleSegment`'s 2 arc call sites (outer +
inner) identically - not technically "infinite", but indistinguishable from it for any real
caller. This is not a new arithmetic bug - it is the exact same unfixed function as 005, reached
from two additional public entry points that neither shape validates or clamps `tolerance` for
before delegating.

## Introduced by

Two separate commits, one per call site (both predate the current bug's root cause fix attempt
in 005, which itself was never merged into real source per the workspace's own tree-reverting
rule):

- `RoundedRect::path_elements`: 7477774 "Implement path generation for rounded rectangle"
  (2019-06-20). This is the *same* commit candidate 005 identified as introducing `arc.rs`'s
  `scaled_err`/`n_err`/`n` formula - it added both `src/arc.rs` and the `RoundedRect::path_elements`
  delegation to `arc.append_iter(tolerance)` in the same commit (`git show
  747777464ab50a089573df867bb6338b2d8b7e7d -- '*rounded_rect.rs'` shows the `+ arc.append_iter(tolerance)`
  line added alongside `arc.rs`'s own introduction).
- `CircleSegment::path_elements`: ff0c599 "Add circle segments and ellipses." (2020-05-13).
  Verified via `git show ff0c5998369e24c375a9c40dfcd2fb42bd4b03c0 -- '*circle.rs'`, which shows both
  of `CircleSegment::path_elements`'s `.append_iter(tolerance)` call sites (outer and inner arc)
  added verbatim in this commit, unchanged in this respect through the current `HEAD`
  (ca273499e3e48bd2de6f02aa8e99a148984e45f3).

Both call sites have delegated to `Arc::append_iter`'s unguarded arithmetic since their own
introduction - neither commit added any `tolerance` validation of its own.

## The fix

Because the root cause lives entirely in `Arc::append_iter` (`kurbo/src/arc.rs`), not in either
call site, the fix is the same one described in candidate 005's `RESULT.md` and was re-applied and
re-verified here rather than re-derived: clamp the computed segment count `n` to a fixed, generous
upper bound (`MAX_ARC_SEGMENTS = 1 << 20` = 1,048,576) right after the `.ceil()` that currently
produces an unbounded value, before the cast to `usize`:

```rust
pub fn append_iter(&self, tolerance: f64) -> ArcAppendIter {
    const MAX_ARC_SEGMENTS: usize = 1 << 20;
    let sign = self.sweep_angle.signum();
    let scaled_err = self.radii.x.max(self.radii.y) / tolerance;
    let n_err = (1.1163 * scaled_err).powf(1.0 / 6.0).max(3.999_999);
    let n = (n_err * self.sweep_angle.abs() * (1.0 / (2.0 * PI)))
        .ceil()
        .min(MAX_ARC_SEGMENTS as f64);
    let angle_step = self.sweep_angle / n;
    let n = n as usize;
    ...
```

`f64::min` returns the non-`NaN` operand when either side is `NaN` (per its documented semantics),
so this also neutralizes a `NaN` `tolerance` as a side effect, without needing a separate check.
Fixing it in `arc.rs` (rather than adding a redundant clamp at each of `RoundedRect`'s and
`CircleSegment`'s own call sites) fixes both call sites in this candidate at once, plus `Arc`
itself (005) and any future caller of `Arc::append_iter`/`Arc::to_cubic_beziers` - `Circle`'s own
independent implementation (028) is a separate code path and is not affected by this fix, since it
does not call `Arc::append_iter` at all.

Verified by re-running `repro/` against the patched clone (via `just test`'s `apply` +
`reproduce-after` recipes, actually executed, not just read): both iterators now return `None`
after 4,194,309 (`RoundedRect`, 4 arcs x `1,048,576 + small overhead` each) and 2,097,155
(`CircleSegment`, 2 arcs) elements respectively, in ~209ms and ~38ms, instead of running the cutoff
out at 20,000,000 elements/~1s and ~0.36s (see `output.txt`). Also checked, per `CONFIRMATION.md`
section 7, against inputs varying within the same bug class rather than just the one triggering
value: `tolerance = 0.0`, `tolerance = 1e-300`, and `tolerance = -0.0` all now terminate at the
same bounded element counts for both shapes; normal, non-degenerate tolerances (`0.1`, `1.0`)
produce the same small element counts (9 for `RoundedRect`, 7 for `CircleSegment`) both before and
after the patch, i.e. the fix does not change behavior for any real-world tolerance value (see
`output.txt`'s "Post-fix boundary-value sweep" section).

- Project test suite: **before** (isolated worktree, unpatched, `cargo test --workspace`):
  187 + 10 + 55 tests passed, 0 failed, 1 ignored doctest (unrelated, pre-existing). **After**
  (same worktree, patch applied, same command): identical - 187 + 10 + 55 passed, 0 failed,
  1 ignored. No new failures either direction.

## What's not done / open questions

- This patch is identical in substance to candidate 005's own proposed fix for `Arc::append_iter`
  - it was re-derived and re-verified independently in this candidate's own isolated worktree
  (rather than assumed from 005's `RESULT.md`) precisely because 005's patch was never actually
  applied to the real tree (per the workspace's own revert-after-every-candidate rule), so the bug
  is still live in `arc.rs` and needed re-confirming end-to-end through these two additional call
  sites rather than taken on faith.
- `Circle::path_elements`'s own independent implementation (candidate 028) is not fixed by this
  patch and needs its own separate clamp, as already noted in 028's `RESULT.md`.
- `Ellipse::path_elements` was not re-checked in this candidate; 005's own `RESULT.md` already
  established it delegates to `Arc::append_iter` and would already inherit this same fix once
  applied, without needing separate confirmation.
- `CubicBez`'s own subdivision code (`kurbo/src/cubicbez.rs:94`) remains an unconfirmed suspect
  flagged by both 005 and 028, not investigated further here either.
- Whether any other `Shape::path_elements` implementation delegates to `Arc::append_iter` (or
  shares this exact pattern independently) was not swept exhaustively beyond `RoundedRect` and
  `CircleSegment`; only these two, `Arc` itself (005), and `Circle` (028) are confirmed so far.
- The chosen bound (`1 << 20`) is the same judgment call as 005/028 made for consistency across
  the crate, not derived from a documented project invariant - a maintainer may prefer a different
  constant, a shared crate-wide constant, or a `Result`-returning API instead of silent clamping.
- Since one fix in `arc.rs` covers `Arc`, `RoundedRect`, `CircleSegment`, and (by delegation)
  `Ellipse` all at once, a maintainer applying 005's/this candidate's patch for real would resolve
  all of these in a single change - only `Circle`'s own independent implementation (028) would
  still need its own separate patch.
