# Arc::append_iter unbounded segment count for near-zero tolerance

`Arc::append_iter(&self, tolerance)` in `kurbo/src/arc.rs` computes `scaled_err = radii.max /
tolerance` and a segment count `n` from it with no validation of `tolerance`. This looked
suspicious because `n` feeds directly into a saturating `f64 as usize` cast and then becomes the
iterator's own loop bound (`self.idx >= self.n`), with no upper clamp anywhere in between.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (pure arithmetic, no crate involved) | Computed `scaled_err`/`n_err`/`n` by hand in a standalone `rustc -O` snippet for `radii.max=10.0`, `tolerance=0.0` and `tolerance=1e-300`, `sweep_angle=PI` | `n` (as `f64`) is `inf` for `tolerance=0.0`; `n as usize` saturates to `18446744073709551615` (`usize::MAX`). For `tolerance=1e-300` (not exactly zero, still "near-zero" per the candidate description), `n` is already an astronomically large finite `f64` whose cast to `usize` also saturates to `usize::MAX`. |
| Real end-to-end run of the public API, debug build (`debug-assertions = true`, `overflow-checks = true`) | Standalone crate depending on `kurbo` via a path dependency to this session's worktree, calling `Arc::new((0,0),(10,10),0,PI,0).append_iter(0.0)` and `.to_cubic_beziers(0.0, ...)`, time-boxed loop (3s cutoff) | 24,282,733 elements produced via `append_iter` in 3s without the iterator ever returning `None`; `to_cubic_beziers` (release build) produced 130,597,437 and 103,367,920 elements in two further 3s-boxed runs, again never returning control to the caller on its own. |
| Standalone `repro/` (own `Cargo.toml`, bare `[workspace]`) against a real `git clone` of the project at the exact base commit, release build | `cargo run --release`, 20,000,000-element / 5s wall-clock cutoff | Cutoff fired: 20,000,000 elements produced in ~430ms without the iterator finishing on its own - see `output.txt`. |

No further tool escalation (miri/ASAN/fuzzing) was needed: this is a pure floating-point
arithmetic/cast boundary-value bug with a fully deterministic mechanism, not memory-unsafety or
input-format-dependent behavior, so the manual/debug-build approaches above already gave a
complete, repeatable picture on the first attempt (see "Tool selection" guidance in `HEURISTICS.md`
for "arithmetic on untrusted sizes/casts between integer widths").

## Status

**Confirmed**. Reproduces deterministically, 4/4 runs (the two ad hoc probes above, plus the
standalone `repro/` run against the pristine clone reproduced twice more directly for this
report - see `output.txt`'s "Repeat runs" section) - every run for `tolerance = 0.0` (and for a
tiny nonzero `tolerance`) produces tens of millions of elements within a few hundred
milliseconds/seconds without the iterator ever completing on its own, in both debug and release
builds.

**Fully reproducible**: **Yes**. The repro calls `kurbo::Arc::append_iter`/`to_cubic_beziers`
directly - the real, public, single-argument API a genuine caller would use. `tolerance` is an
ordinary `f64` parameter passed straight through by any caller (there is no second
participant/peer/wire format involved to fake - see `CONFIRMATION.md` section 11's own carve-out
for "a single external artifact with no second participant to fake the reaction of", which applies
here identically to a malformed-file-to-a-parser case).

## Class

logic / resource-exhaustion (unbounded CPU/time; would also be unbounded memory for any caller
that collects the iterator, e.g. `Shape::to_path`/`BezPath::from_path_segments`, since nothing
about the iterator's own `size_hint` is used to fail fast - it just never stops producing items).

## Root cause

```rust
let scaled_err = self.radii.x.max(self.radii.y) / tolerance;
let n_err = (1.1163 * scaled_err).powf(1.0 / 6.0).max(3.999_999);
let n = (n_err * self.sweep_angle.abs() * (1.0 / (2.0 * PI))).ceil();
let angle_step = self.sweep_angle / n;
let n = n as usize;
```

`tolerance` is never validated (not checked for `<= 0.0`, not checked for being merely very small
relative to `radii`). For `tolerance == 0.0`, `scaled_err` is `+inf` by IEEE-754 division, which
propagates through `powf`/`.max()`/multiplication/`.ceil()` to make the `f64` value of `n` itself
`+inf`. Rust's `f64 as usize` cast is a *saturating* cast (not a panic, not a wrap, per
`rustc`'s documented float-to-int cast semantics), so `inf as usize` becomes `usize::MAX`
(`18446744073709551615`) silently, with no error and no debug-assertion failure. `ArcAppendIter`'s
own `Iterator::next` only stops once `self.idx >= self.n`, so the loop bound becomes
`usize::MAX` - not technically "infinite", but indistinguishable from it for any real caller
(at the observed ~50-300M elements/sec `to_cubic_beziers` rate, exhausting `usize::MAX` iterations
would take on the order of hundreds of thousands of years). A `tolerance` that's merely tiny but
nonzero (e.g. `1e-300`) hits the exact same saturating-cast failure mode for the same reason: `n`
as an `f64` is already astronomically large before the cast, well past what `usize` can represent.

## Introduced by

7477774 "Implement path generation for rounded rectangle" (2019-06-20). This commit added
`src/arc.rs` (later moved to `kurbo/src/arc.rs` by d67b9dd, the workspace-restructuring commit,
with no logic change to this function) with this exact `scaled_err`/`n_err`/`n` formula and the
unclamped `n as usize` cast, unchanged in this respect through the current `HEAD`
(ca273499e3e48bd2de6f02aa8e99a148984e45f3). Verified via `git log --diff-filter=A -- kurbo/src/arc.rs`
(only shows the workspace-move commit, since the file's history was truncated by that rename) and
`git log --follow --oneline -- kurbo/src/arc.rs` combined with `git show <hash> -- src/arc.rs` at
the earliest commit in that follow chain, which shows this exact code already present verbatim at
file creation.

## The fix

Clamp the computed segment count `n` to a fixed, generous upper bound (`MAX_ARC_SEGMENTS = 1 <<
20` = 1,048,576) right after the `.ceil()` that currently produces an unbounded value, before the
cast to `usize`:

```rust
const MAX_ARC_SEGMENTS: usize = 1 << 20;
...
let n = (n_err * self.sweep_angle.abs() * (1.0 / (2.0 * PI)))
    .ceil()
    .min(MAX_ARC_SEGMENTS as f64);
```

`f64::min` returns the non-`NaN` operand when either side is `NaN` (per its documented semantics),
so this also neutralizes a `NaN` `tolerance` (e.g. from a negative `tolerance`, whose `powf` of a
negative base can itself be `NaN`) as a side effect, without needing a separate check. 1,048,576
segments for a single arc is far beyond what any real rendering/tessellation use case needs (well
past screen-resolution pixel density even for the largest practical displays), so this bound only
ever engages for a degenerate/adversarial `tolerance`, never for realistic inputs.

Verified by re-running `repro/` against the patched worktree: the iterator now returns `None` after
exactly 1,048,576 elements in ~20ms instead of running the cutoff out at 20,000,000 elements/430ms
(see `output.txt`). Also checked, per `CONFIRMATION.md` section 7, against inputs varying within
the same bug class rather than just the one triggering value: `tolerance = 0.0` with a full-circle
sweep, `tolerance = 1e-300`, and `tolerance = -0.0` all now terminate at exactly 1,048,576 elements
in the same ~0ms; normal, non-degenerate tolerances (`0.1`, `1.0`) still produce the same small
segment counts (2 elements) as before the patch, i.e. the fix does not change behavior for any
real-world tolerance value.

- Project test suite: **before** (main tree, unpatched, `cargo test --workspace`): 187 + 10 + 55
  tests passed, 0 failed, 1 ignored doctest (unrelated, pre-existing). **After** (patched worktree,
  same command): identical - 187 + 10 + 55 passed, 0 failed, 1 ignored. No new failures either
  direction.

## What's not done / open questions

- `Circle::path_elements` (`kurbo/src/circle.rs`, around line 110-122) has the *exact same*
  `scaled_err`/`(1.1163 * scaled_err).powf(1.0 / 6.0)`/`as usize` shape with no clamp - a
  `tolerance == 0.0` there almost certainly saturates `n` to `usize::MAX` the same way. Not
  investigated or fixed as part of this candidate (one bug per candidate folder, per `SKILL.md`) -
  flagged as a new suspect below.
- `CubicBez`'s own subdivision code (`kurbo/src/cubicbez.rs:94`,
  `let n = ((err / max_hypot2).powf(1. / 6.0).ceil() as usize).max(1);`) has a superficially similar
  shape (`powf(1/6)` then `as usize`), but wasn't checked for whether `err`/`max_hypot2` can reach
  the same near-zero-denominator situation from a public entry point - noted as a suspect, not
  confirmed either way.
- Whether any other `Shape::path_elements`/`append_iter`-style method elsewhere in the crate shares
  this pattern wasn't swept exhaustively; only `Arc` (this candidate) and the two suspects above
  were noticed.
- The chosen bound (`1 << 20`) is a judgment call, not derived from a documented project
  invariant - a maintainer may prefer a smaller/larger constant, or may prefer returning a
  `Result`/documenting a minimum valid `tolerance` instead of silently clamping. Silently clamping
  was chosen here because it keeps `append_iter`'s existing infallible signature and matches the
  "best effort, always makes *some* progress" spirit of the rest of the function (which already
  clamps `n_err` at the low end via `.max(3.999_999)`).
