# FIT_TO_BEZPATH_OPT_INNER_UNWRAP_PANIC

`fit_to_bezpath_opt_inner` (`kurbo/src/fit.rs`) ends its "build" loop with
`fit_to_cubic(source, t0..t1, accuracy).unwrap()`, assuming a sub-range that the preceding search
pass accepted at a *tightened* accuracy `x` must also fit at the *final*, caller-supplied
`accuracy`. Flagged by GitHub issue #601 ("panics on a sliver") with a concrete repro and root
cause already worked out; PR #603 (open, unmerged) proposes a fix touching both this call site and
shared `fit_to_cubic` behavior.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Reused issue #601's exact repro (public API: `SimplifyBezPath` wrapping a hand-built near-collinear closed `BezPath`, fed to `fit_to_bezpath_opt` at `accuracy = 2.0`) | debug build, `cargo run` | Panicked immediately: `called \`Option::unwrap()\` on a \`None\` value` at `kurbo/src/fit.rs:646:61` |
| Repeat-run the same repro | debug build, 3 consecutive runs | Panicked 3/3 runs (deterministic - no RNG/seed/threading involved) |
| Varied `accuracy` across the reported "window" (0.1, 0.5, 1.0, 1.5, 1.8, 1.9, 2.0, 2.1, 2.5, 3.0, 4.0, 8.0) with the same geometry | debug build | Panics specifically at `accuracy = 2.0` in this project's current build too (matches issue #601's report of an accuracy-dependent window); after the fix below, all values complete without panicking |
| Manual code reading of `fit_to_cubic`'s short-chord branch vs. the general cubic solver, and of every other `fit_to_cubic(...)` call site in `fit.rs` | - | Confirmed root cause (below); confirmed `fit_to_bezpath` (the non-`_opt` sibling, line 194) and `measure_one_seg` (line 661) already treat `fit_to_cubic`'s `None` return as fallible - only the `_opt_inner` call site at line 646 unwraps it unguarded |

## Status

**Confirmed** - reproduces 3/3 runs, deterministic, stable debug build (`cargo run`, no release-only
or sanitizer-only manifestation; not build-dependent).

**Fully reproducible**: **Yes**. The repro calls the real public entry point
(`kurbo::fit_to_bezpath_opt`) on a real `SimplifyBezPath` built from an ordinary, valid, finite
`BezPath` - the same kind of geometric input a real caller of `simplify_bezpath(...,
SimplifyOptLevel::Optimize)` produces internally. There is no second participant to fake here (this
is a single-process pure-function panic on a specific curve/accuracy combination, not a
peer/protocol interaction), so `CONFIRMATION.md` section 11's two-participant recipe does not apply
- a single-process repro through the public API is the genuine external input.

## Class

crash

## Root cause

`fit_to_bezpath_opt_inner` (kurbo/src/fit.rs) runs in two passes over a range with no cusp:

1. A **search pass** (`fit_opt_segment` / `solve_itp_fallible`) finds how many segments `n` are
   needed and a *tightened* per-segment accuracy `x <= accuracy` at which they all fit.
2. A **build pass** re-walks the same sub-ranges and calls
   `fit_to_cubic(source, t0..t1, accuracy).unwrap()` (line 646) - now at the *larger*, original
   `accuracy` - to actually construct each cubic segment.

The `.unwrap()` assumes a range accepted during the search pass at the smaller `x` must also fit
during the build pass at the larger `accuracy`. That assumption fails because `fit_to_cubic`
(kurbo/src/fit.rs:387) does not simply relax as `accuracy` grows - it switches strategy:

```rust
if chord2 <= acc2 {
    // Special case very short chords; try to fit a line.
    return try_fit_line(source, accuracy, range, start.p, end.p);
}
```

A *larger* `accuracy` makes `chord2 <= acc2` (chord-length-squared vs. accuracy-squared) more
likely to hold, routing the *same* sub-range onto the short-chord line-fit shortcut instead of the
general cubic solver used during the search pass. `try_fit_line` returns `None` when the source
curve strays further than `accuracy` from the straight chord - which is exactly what happens for a
sub-range whose chord is short but whose actual arc length is not (the curve "doubles back" on
itself, as in a near-degenerate/near-collinear polyline fed through `SimplifyBezPath`). So
loosening `accuracy` for the build pass can make fitting *harder* by flipping the code onto a
strictly weaker code path, and the `.unwrap()` panics on the resulting `None`. This matches issue
#601's own root-cause analysis exactly (verified independently by reading the code, not just taken
on faith from the issue).

## Introduced by

f87631d "Apply tighter accuracy bounds" (2023-04-14). Before this commit, the same call site read
`fit_to_cubic(source, t0..t1, HUGE).unwrap()` (`HUGE = 1e12`) - an accuracy so large that
`try_fit_line`'s own error check could never fail, making the `.unwrap()` safe by construction.
This commit changed that final-pass accuracy from the always-safe `HUGE` sentinel to the real,
caller-supplied `accuracy` (to "give up more quickly when accuracy is exceeded, which improves
performance"), which is what introduced the possibility of `fit_to_cubic` legitimately returning
`None` at this call site. Found via `git log -S "fit_to_cubic(source, t0..t1, accuracy).unwrap()"`
and confirmed by reading the commit's full diff (`git show f87631d -- src/fit.rs`).

## The fix

Applied directly at the panicking call site in `fit_to_bezpath_opt_inner` (kurbo/src/fit.rs:646):
instead of unwrapping, match on `fit_to_cubic`'s result and fall back to a straight line between
the sub-range's sampled endpoints (`start_p`/`end_p` via `source.sample_pt_tangent`) when it
returns `None` - the same escape-hatch shape already used by `fit_to_bezpath_rec`'s analogous
"infinite recursion, just draw a line" case a few dozen lines above in the same file. This is
deliberately narrower than PR #603's approach (which also changes `fit_to_cubic`'s own short-chord
branch to fall through to the general solver instead of giving up - a change PR #603 itself flags
as touching shared code that stroke expansion/offsetting also depend on, "worth a careful look").
This session's fix only changes the one call site that was actually unguarded, without altering
`fit_to_cubic`'s behavior or return contract for any other caller.

Verified:

- Issue #601's exact repro: panics before the fix (3/3 runs), completes normally after (see
  `output.txt`).
- Varied within the same bug class: all 12 accuracy values from the "window" noted in issue #601
  (0.1 through 8.0, including the exact failing 2.0) complete without panicking after the fix -
  not just the one value that happened to trigger the original report.
- `cargo build -p kurbo` and `cargo clippy -p kurbo --all-targets` are clean with the patch applied
  (one pre-existing, unrelated clippy suggestion in `common.rs` is present both before and after
  the patch).

Project test suite (`cargo test --workspace`, run in this worktree before and after applying
`patch.diff`, then reverted):

- Before: kurbo lib tests 187 passed / 0 failed; polycool lib tests 10 passed / 0 failed; combined
  doctests 55 passed / 0 failed (1 ignored, pre-existing/unrelated).
- After: kurbo lib tests 187 passed / 0 failed; polycool lib tests 10 passed / 0 failed; combined
  doctests 55 passed / 0 failed (1 ignored) - identical counts, no regressions, no new failures.

## What's not done / open questions

- This fix only removes the panic at this one call site; it does not change `fit_to_cubic`'s
  short-chord shortcut itself, so a range that hits this exact situation now silently degrades to
  a straight-line segment (which may need more subsequent subdivision elsewhere, or a visibly
  straighter result for that one segment) rather than PR #603's approach of trying harder to fit a
  real cubic via the general solver. A maintainer preferring output quality over this session's
  narrower/lower-risk change may want PR #603's `fit_to_cubic` change instead (or in addition).
  - Whichever repo maintainers ultimately take was not decided here - see PR #603 (open) for the
    fuller alternative already under review upstream.
- The non-`_opt` sibling `fit_to_bezpath` was checked and does **not** share this bug: its own
  `fit_to_cubic` call (fit.rs:194) is already behind `if let Some(...)`, falling back to midpoint
  subdivision on `None` - confirmed by reading the code, not independently fuzzed.
- No fuzzing was run this session (the concrete issue #601 repro plus manual code reading were
  sufficient to reach Confirmed quickly); a short fuzz harness over `fit_to_bezpath_opt` with
  varied near-degenerate `SimplifyBezPath`/`BezPath` inputs might turn up further instances of the
  same "search pass and build pass disagree" class, but wasn't attempted here.
