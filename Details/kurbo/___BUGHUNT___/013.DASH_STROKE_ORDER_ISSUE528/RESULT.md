# DASH_STROKE_ORDER_ISSUE528

`kurbo::stroke()`/`kurbo::dash()` with a dashed `Stroke` (or `StrokeOpts::default()`, i.e. the
default `stable_dash_order: false`) emit the dash pieces of each subpath out of path order: the
first dash piece of every subpath is deferred and emitted *last* instead of first, for every
subpath in the input. Source: issue #528 ("stroke() function with dashed stroke returns first
piece last"). Also checked whether open PR #597 ("Fix zero-length `stroke` and `dash` handling")
incidentally touches this - it doesn't; its scope is unrelated (zero-length/degenerate geometry
through `dash()`), though it edits the same `kurbo/src/stroke.rs` file.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value reasoning: read `DashIterator`'s `ToStash`/`Working`/`FromStash` state machine in `kurbo/src/stroke.rs` and hand-traced its execution against the existing internal `dash_sequence` unit test's own expected output | reading only, no build | Confirmed by hand-tracing that the existing (pre-this-session) `dash_sequence`/`dash_sequence_closed_path` tests already encode the exact reordering the issue reports as their literal `expected` value - i.e. this behavior is not hypothetical, it is already pinned by the test suite. |
| Public-API repro (`repro/src/main.rs`): a two-subpath open `BezPath` (two separate 21-unit horizontal lines), dashed with `[1.0, 5.0, 2.0, 5.0]`, run through the public `dash()` iterator and through the public `stroke()` entry point with `StrokeOpts::default()` | release build, `cargo run --release`, deterministic pure computation, run 3/3 times | Reproduced every time: both subpaths' emitted dash-piece order is `[6, 13, 19, 0]` (relative x-starts) instead of the path-ordered `[0, 6, 13, 19]` - the true first piece (starting at the subpath's own start point) is emitted last, matching the issue exactly. Also confirmed `StrokeOpts::default().stable_dash_order(true)` (the opt-in already present at `base_commit`, added by upstream PR #550) produces the correctly-ordered `[0, 6, 13, 19, 106, 113, 119]`. |
| GitHub issue/PR history check (`gh issue view 528 --json ...`, `gh pr view 550/597`) | read-only, network | Found that upstream already merged PR #550 ("fixes: stroke() function with dashed stroke returns first piece last") on 2026-05-05, which is an ancestor of this candidate's `base_commit` (`git merge-base --is-ancestor` confirms). PR #550 adds the `StrokeOpts::stable_dash_order` opt-in seen in the current source, but explicitly does **not** change the default - see "Root cause"/"The fix" below for why. The GitHub issue itself is still open as of this session (last activity 2026-03-20, asking the reporter to verify PR #550's fix, no confirmation yet in the thread). |

Only two approaches were needed (code reading + a public-API repro); the third "approach" here was
reading the project's own issue/PR history rather than an escalating debugging tool, since this is
a pure floating-point-free logic/ordering bug with a fully deterministic, already-well-understood
mechanism - no sanitizer/miri/fuzzing was relevant (matches `___BUGHUNT___/README.md`'s existing
toolchain notes: no `unsafe` in `kurbo/src`).

## Status

**Confirmed**

Reproduces deterministically, 3/3 runs, release build, using the crate's public `dash()` and
`stroke()` entry points with `StrokeOpts::default()` (no randomness/seed involved - this is a pure,
input-independent structural property of the default dash-iteration state machine, not
input-specific). Also reproduces for every subpath tested (2 subpaths in the repro's own input),
matching the issue's own claim that it "affects every individual subpath" of a multi-subpath input.

**Fully reproducible**: **Yes**. The repro calls `kurbo::dash()` and `kurbo::stroke()` (the crate's
real, public dashing/stroking entry points, the exact functions named in the issue title) directly
on a `BezPath` built via `BezPath`'s own public builder methods - the same public API and the same
kind of caller-authored path data a real user of the crate would pass in. There is no second
participant to fake here (see `CONFIRMATION.md` section 11): the "input" is a single path + a dash
pattern authored directly by the repro itself, not something produced by a peer/upstream caller
whose behavior would need to be simulated.

## Class

logic (wrong output order, not a crash/panic/UB - the crate has zero `unsafe` in `kurbo/src`)

## Root cause

`DashIterator` (`kurbo/src/stroke.rs`) processes a subpath's dash pieces through a small state
machine (`NeedInput` / `ToStash` / `Working` / `FromStash`). When `stable_dash_order` is `false`
(the default), the very first dash-piece it produces for a subpath is pushed onto an internal
`stash: Vec<PathEl>` instead of being returned immediately; `Iterator::next()`'s `ToStash` match arm
only re-checks `self.state` on its *next* loop iteration, but the underlying `step()` call that
produced that first piece already flips `self.state` to `Working` internally before returning
(the "next transition is a dash transition, `is_active`" branch in `step()`, which unconditionally
sets `self.state = DashState::Working`). As a result, exactly one dash piece - the first one - gets
stashed, and every subsequent piece for that subpath is returned immediately via the `Working` arm,
in normal path order. Only once the subpath is fully exhausted (`get_input()` hits end-of-input or
a pending `ClosePath`, transitioning to `FromStash`) does the iterator finally play back the stash,
appending that one deferred first piece to the very end of the subpath's output.

This mechanism exists so a dash spanning a `ClosePath` seam (the path's own end wrapping back into
its own start) can be merged into a single piece instead of being artificially split at the seam -
see `handle_closepath()`'s `stash_ix = 1` skip-the-duplicate-MoveTo logic, which only makes sense
for closed subpaths. But the defer-and-replay happens unconditionally for *every* subpath,
including open ones with no `ClosePath` at all (confirmed directly: the repro's two subpaths are
both plain open lines, no `ClosePath` anywhere in the input), where there is nothing to merge and
the reordering serves no purpose beyond producing the exact reported symptom.

`stroke_with()` (`kurbo/src/stroke.rs`) feeds `dash_iter()`'s output straight into
`stroke_undashed()`, so `stroke()`'s fill-outline output inherits the exact same per-subpath
reordering as the raw `dash()`/`dash_iter()` iterator - confirmed directly in the repro (item 2 in
`repro/src/main.rs`'s output).

## Introduced by

af22c87 "Add dashing" (2023-05-26) - `git log --all -S "DashState" -- kurbo/src/stroke.rs` (and
independently `-S "ToStash"`) both return exactly this one commit, which introduced the
`DashIterator`/`ToStash`/`FromStash` stash-and-replay mechanism from scratch along with the rest of
the dashing feature. A later commit, e234d26 "Don't skip first dash in dash pattern (#353)"
(2024-08-17), fixed an unrelated initial-offset-computation bug in the same function and, in the
process, added the `dash_sequence`/`dash_sequence_offset` unit tests whose literal `expected`
arrays are the first place this reordering got pinned down as "expected" test output - per the
issue thread, its author (dominikh) has since said this was incidental ("I didn't write the test
with the intention of enforcing this order... I was entirely focused on fixing a different bug"),
not a deliberate encoding of the ordering as correct behavior.

## The fix

No `patch.diff` is shipped with this candidate. The correct-order behavior already exists in this
exact commit (`base_commit`) as an *opt-in*: `StrokeOpts::default().stable_dash_order(true)`,
added by upstream PR #550 (merged 2026-05-05, already an ancestor of `base_commit`) - verified
directly in `repro/src/main.rs`'s own output (item 3), which shows `stable_dash_order(true)`
producing the correctly path-ordered sequence on the same input that reproduces the bug under the
default.

Flipping the *default* itself (`stable_dash_order: false` -> `true` in `StrokeOpts::default()`) is
a trivial one-line code change, and was deliberately **not** attempted as this candidate's
`patch.diff`: per the issue thread, the project's own lead maintainer (raphlinus) explained the
non-stable default exists specifically "to minimize the amount of state we need to track" and
noted enforcing order everywhere "might be slightly tricky... I wouldn't be surprised if enforcing
order required either (a) allocating and copying more stuff or (b) doing a second pass" - i.e. this
is a considered performance/complexity tradeoff the maintainers have discussed and explicitly chose
to ship as an opt-in rather than a default-behavior change (PR #550's own description: "Adds
`stable_dash_order` option to StrokeOpts... Adds 12 bytes to the `DashIterator`"). Overriding that
already-discussed product decision unilaterally in this workspace's `patch.diff` would not be a
"bounded bug fix" in the sense this skill's step 6 means - it's a decision reserved for the project
maintainers, who have already had this exact discussion and chosen the opt-in. The GitHub issue
itself remains open (not yet closed as resolved) pending confirmation from the original reporter
that the opt-in is an acceptable resolution for their use case.

- Project test suite: **N/A, no patch applied.** Baseline (`base_commit`, unmodified,
  `cargo test --workspace --all-features`): 187 + 10 + 55 = 252 passed, 0 failed, 1 ignored
  (doctest) - i.e. the existing suite already passes cleanly with the bug present, since the
  `dash_sequence`/`dash_sequence_closed_path` tests assert the buggy order as their expected value
  (see "Introduced by" above).

## What's not done / open questions

- Whether flipping the default is something the maintainers now want, given PR #550 already
  shipped the opt-in and issue #528 is still open awaiting the reporter's (Graphite's) confirmation
  - this is a product decision for upstream, not something this session resolved.
- Not independently checked: whether the same defer-the-first-piece stash mechanism can also
  produce an incorrect (not just reordered) result for degenerate inputs (zero-length dashes,
  single-element dash pattern, a dash pattern longer than the whole subpath) - open PR #597 is
  actively reworking nearby code for a related but distinct class of degenerate-geometry issues,
  worth revisiting once that PR lands in case it changes this state machine's shape.
- The repro here uses only two simple open subpaths (`LineTo` only); not separately re-verified
  against closed subpaths or curved (`QuadTo`/`CurveTo`) segments, though the mechanism traced above
  is generic to `DashIterator`'s state machine and not specific to straight lines - the existing
  `dash_sequence_closed_path` unit test (already in the tree, `kurbo/src/stroke.rs`) independently
  demonstrates the same reordering for a closed rectangular path.
