# 95 - Root-causing candidate 093's scrollbar-thumb rendering regression

This is an **investigation candidate, not a performance candidate**: no timing was measured. It
follows up on candidate 093's own unresolved item ("the exact trigger for the scrollbar-thumb
divergence was not root-caused") by instrumenting `internal/core/textlayout/sharedparley/layout.rs`'s
`layout()` function with `eprintln!` tracing of `LineBreakingInputs`/`dirty_range` per call (the
same technique candidate 092 used to find the original single-slot ping-pong), gated behind a new,
temporary `candidate-095-trace` Cargo feature so it compiles to nothing by default. Applied **on
top of** candidates 091+092+093 (`patch.diff` bundles all four against pristine `master`, the same
convention 092/093/094 used).

## What was tried

1. **Trace `layout()`'s own computed numbers frame-by-frame**, comparing the *exact same* traced
   call sequence between the pre-093 tree (091+092, single slot) and the 093 tree (2-slot cache),
   both instrumented identically, for the harness's own `--type-text --scroll-step 0` scene at
   200 edited frames.
2. **Bisect the actual pixel divergence** (`cmp -l` between an A/B snapshot pair at frame 80, per
   093's own reproduction recipe) down to exact row/column ranges, to confirm which rendered
   element it belongs to.
3. **Test a one-line experimental fix** derived from the trace findings (see below), measuring its
   effect on the same pixel comparison, repeated several times to separate signal from the
   mechanism's own run-to-run jitter (discovered along the way - see below).

No timing table: this candidate does not change any hot-path behavior in a way meant to be
measured for speed (the experimental fix in step 3 only removes one line's `Some(other)` payload
in an already-cold `Option`, not a loop or allocation).

## Result

**The leading suspect from 093/094 ("a stale/wrong number in a `RetainedLineBreaking` slot") is
empirically ruled out**: tracing every single `layout()` call for the big (10 001-paragraph) text
item across 200 edited frames, in both the pre-093 and 093 trees, found the traced input (`width`,
`dirty_range`) and the resulting `max_width` **bit-identical, line-for-line, at every call** (`diff`
of the two extracted `max_width` sequences: zero differing lines, both 100- and 200-frame runs).
Whatever produces the wrong scrollbar thumb, it is **not** a wrong number computed or cached by
`RetainedLineBreaking`/`layout()` itself.

**The actual exact call sequence was found** by looking at what candidate 093's own "full
recompute, no dirty_range" branch does that the "partial re-break" branch deliberately avoids:

- `layout()`'s **partial-rebreak branch** (an incremental reshape happened - `dirty_range: Some`)
  already does `drop(other)` before returning, specifically because it knows it is about to
  `break_all_lines` a *subset* of paragraphs, invalidating whatever the *other* width's cached
  numbers assumed about "every paragraph outside its own dirty_range stays broken for its width".
- `layout()`'s **"full fast path, no match" branch** (`dirty_range: None`, but this call's own
  width doesn't match either retained slot) calls `break_and_layout(..., reusable_range: None)` -
  which, per its own doc comment, means "nothing is known to already be broken for `inputs`", i.e.
  it calls `break_all_lines`/`align` on **every single paragraph in the document**, not just the
  edited one. This branch then does `layout.other_retained_line_breaking = other;` - **carrying
  the other slot through unchanged**, even though this call's own full recompute just physically
  overwrote the shared, single-owner `parley::Layout` state for every paragraph the *other* slot's
  own future partial-rebreak calls will assume remained untouched.

This is the exact call sequence: in the harness's steady-state edited frame, call 1 (say the
natural-width query) does a partial re-break and correctly drops `other`; call 2 (the constrained
draw call, different width, no dirty_range left by the time it runs) does **not** find a matching
slot, takes the full-recompute branch, and carries `other` through - reintroducing exactly the
kind of stale slot the partial-rebreak branch's own `drop(other)` was written to prevent, just via
the sibling branch that does the same "touch every paragraph" thing but forgot the same guard.

**A one-line experimental fix** (`drop(other)` instead of `layout.other_retained_line_breaking =
other;` in that branch - see `experimental_partial_fix.diff`) was tested against the same pixel
comparison, repeated 3x per binary to establish each binary's own run-to-run noise floor (see
"Why the result looks the way it does" for why repeats were necessary - this mechanism has real
jitter of its own):

| Binaries compared (frame 80, `--type-text --scroll-step 0`, count 10000) | scrollbar-row (796-797) pixel divergence |
|---|---|
| A (091+092) vs A (repeat) - noise floor | 2-5 px wide, cols ~1111-1115 |
| B (091+092+093, unfixed) vs B (repeat) - noise floor | 6-12 px wide, cols ~1054-1067 |
| A vs B (unfixed) - 9 pairs across 3x3 repeats | **45-60 px wide**, cols ~1056-1115 (consistent, well outside either noise floor) |
| C (091+092+093 + the one-line fix) vs C (repeat) - noise floor | 0-2 px wide |
| A vs C (fixed) - 9 pairs across 3x3 repeats | **0-4 px wide**, within A's own noise floor in every pair |
| A vs B (unfixed) at frame 150 | 803 px wide (grows further with more edited frames) |
| A vs C (fixed) at frame 150 | **0 px** (bit-exact) |

The one-line fix takes the measured divergence from far outside any noise floor (45-60 px at
frame 80, growing to 803 px by frame 150) down to *within* binary A's own run-to-run noise floor at
frame 80, and to exactly 0 at frame 150.

## Does it change behavior/output?

This candidate's own delivered `patch.diff` is diagnostic-only (`eprintln!` tracing behind a
feature flag that is off by default) and changes no behavior when the feature is disabled - the
default build is unaffected (`cargo check -p i-slint-core` with no extra features still compiles
and `cargo test -p i-slint-core --all-features --lib` still passes 319/319, both before and after
adding the instrumentation).

The **experimental one-line fix** (`experimental_partial_fix.diff`, not part of `patch.diff`,
applies on top of candidate 093 alone) does change behavior: it removes candidate 093's "second
slot" from surviving one specific case (a full, all-paragraph recompute triggered by a
non-matching width with no known dirty range). Measured effect: closes the great majority of the
scrollbar-thumb divergence 093 found (see table above), but **does not prove it closes all of it**
in every scenario - only the exact scene checked (`--type-text --scroll-step 0`, up to 150 edited
frames) was tested; 093's own flagged higher-risk scenarios (`--resize-step`, `--select-all`/
cursor placement reading `.lines()` directly) were not re-checked against this fix this session.
- **Is this a breaking change?** No public API changed either way.
- Project test suite (`i-slint-core`'s own unit tests):
  - `cargo test -p i-slint-core --all-features --lib`, tree = 091+092+093 (candidate 093 alone,
    matching its own reported baseline): **319 passed, 0 failed**.
  - Same command, tree = 091+092+093 + this candidate's trace instrumentation (`patch.diff`):
    **319 passed, 0 failed**. Identical pass/fail set.
  - Same command, tree = 091+092+093 + the one-line experimental fix
    (`experimental_partial_fix.diff`, no trace): **319 passed, 0 failed**. Identical pass/fail set
    in all three trees.
  - The broader `tests/` driver suites were not re-run this session, for the same reason 093/094
    skipped them (a `ScrollView`-extent-only pixel difference that a non-pixel-comparing suite
    would not catch).

## Why the result looks the way it does

The mechanism candidates 093/094 suspected ("a slot's cached *numbers* go stale/wrong") turned out
to be the wrong layer to look at - the numbers are always internally consistent for the call that
produces them. The actual bug is a **cross-call contamination via the physically shared
`parley::Layout`**: `RetainedLineBreaking`'s doc comment already warns that a slot's numbers are
metrics-only-safe regardless of which width is "live" on the shared `TextParagraph::layout`, but
that safety only holds as long as nothing *else* has invalidated the assumption a slot's own future
partial-rebreak depends on ("everything outside my dirty_range is still broken for my width"). The
partial-rebreak branch protects this assumption for itself by dropping the other slot whenever it
does its own subset re-break; the full-recompute-no-match branch does an equally invasive
operation (breaking literally every paragraph) but was not given the same protection, because it
doesn't "feel" like a reshape - it's reached via a cache-arithmetic miss (`dirty_range: None`, no
slot match), not via an actual edit. Both branches call `break_all_lines` on paragraphs the *other*
slot depends on remaining untouched; only one of the two remembered to say so.

An unplanned, secondary finding along the way: **candidate 093's mechanism introduces its own
run-to-run rendering non-determinism** that candidate 092 (single slot) does not have - two
identical runs of the unfixed 093 binary differ by up to ~12 px in the exact scrollbar row, where
two runs of the pre-093 binary never differ there at all. This is consistent with the same root
cause: which specific call order happens to run first in a given frame (natural-width query vs.
draw call) determines which slot ends up "the one dropped, the one contaminated", and that order
is not itself fully deterministic run-to-run in this harness. This made repeated (not single) A/B
comparisons necessary to trust the fix's measured effect - a single before/after pair could have
been noise in either direction.

## What's not done / open questions

- **The one-line fix is not verified as a complete fix.** It was checked against exactly one scene
  (`--type-text --scroll-step 0`, up to 150 edited frames) at one count (10000). 093's own flagged
  higher-risk scenarios (`--resize-step`, `--select-all`/cursor-placement reading `.lines()`
  directly - see 093's `RETAINED_LINE_BREAKING_SLOTS` doc comment) were not exercised against
  either the bug or the fix this session.
- **The source of candidate 093's own run-to-run non-determinism** (which call runs first in a
  given frame) was not traced further - it's plausible this is ordinary Slint property-evaluation
  scheduling order rather than anything specific to text layout, but this was not confirmed.
- **Whether the same `drop(other)`-everywhere-a-full-recompute-happens fix generalizes** to a
  hypothetical 3+-slot design (an `IDEAS.md`-listed follow-up) was not considered - this fix is
  specific to the 2-slot shape.
- This candidate does not touch `perf_test` itself (no `perf_test_harness.diff`); the existing
  `--type-text`/`--frames`/`--snapshot`/`--bench` flags used by 093/094 were reused as-is.

## Reproduce

```sh
cd perf_test
# apply this candidate's patch.diff to get the trace build (091+092+093+eprintln tracing):
git apply ../___OPTIMIZATIONS___/095.RETAINED_LINE_BREAKING_STALE_SLOT_TRACE/patch.diff
cargo build --bin perf_test --profile rdebug --features software
# trace one run (adjust --frames as needed):
target/rdebug/perf_test --bench --mode text --count 10000 --type-text --scroll-step 0 \
  --frames 200 2>trace.log 1>/dev/null
grep -A1 'n_para=10001' trace.log | less   # per-call LineBreakingInputs/dirty_range/branch/result

# to test the experimental fix instead: revert, apply just 093's own patch.diff, then
# experimental_partial_fix.diff on top, and repeat the --snapshot/cmp -l recipe from 093's own
# RESULT.md at --frames 80 and --frames 150.
```
