# 92 - Carry `RetainedLineBreaking` across an incrementally-reshaped edit

Candidate 091 made per-keystroke *shaping* close to O(1) (reuses every paragraph an edit didn't
touch) but deliberately left `line_breaking: None` on every edit, so `layout()`'s retained-breaking
fast path (candidate 003) never applied to a reshaped document - every keystroke still ran the full
O(paragraphs) `break_all_lines`/`align` pass over every paragraph, touched or not (091's own
`RESULT.md`, "What's not done"). This candidate carries the previous `RetainedLineBreaking` through
a reshape too, alongside a new per-call "dirty range" that says exactly which paragraphs the
reshape actually touched, so `layout()` can re-break only those and reuse every other paragraph's
already-broken lines outside it, provided the line-breaking inputs (width/alignment/max-lines/
overflow) haven't also changed. Applied **on top of** candidate 091's patch (the shaping side has
to already be incremental for a dirty range to exist at all) - this folder's `patch.diff` bundles
both, following the same "self-contained against pristine `master`" convention candidate 073 used
for the same reason.

## What was tried

- `ItemCache::take_dirty_entry`'s caller (`cache.rs`) now also keeps the about-to-be-discarded
  entry's `line_breaking`, instead of dropping it. It's only ever handed back to the new entry
  when `shape` also reports a dirty range (a real incremental reshape happened, not a full one) -
  a full reshape's paragraphs are freshly built and entirely unbroken, so reusing the old aggregate
  in that case would be wrong, not just unhelpful.
- `shaping::reshape_diff` (091's diff) now also returns the range of *new* paragraph indices it
  actually reshaped (`lead..len-tail`); everything outside that range is the exact same
  `TextParagraph`/`parley::Layout` instance as before the edit.
- `layout()` gained a `dirty_range: Option<Range<usize>>` parameter. When it's `Some(range)` and
  the previous `RetainedLineBreaking.inputs` still match the current call's, only `range` gets
  `break_all_lines`/`align`; every other paragraph keeps its already-broken lines untouched. The
  existing all-or-nothing fast path (`dirty_range: None`, ordinary repeated calls with no edit) is
  unchanged. A width/alignment/etc. mismatch (or no known dirty range) falls back to breaking
  everything, same as before this candidate.

| Variant | scenario | single-core (`taskset -c 3`) | multi-core | notes |
|---|---|---|---|---|
| A = 091 alone, B = 091 + this candidate | `--type-text --scroll-step 0 --word-wrap`, count 50000 | 125.57ms -> 31.70ms (**-74.8%**, 9/9 pairs) | 65.12ms -> 16.54ms (**-74.6%**, 9/9 pairs) | consistent |
| same | count 10000 | 29.60ms -> 11.13ms (**-62.4%**, 9/9) | 15.42ms -> 5.33ms (**-65.4%**, 9/9) | consistent |
| same | count 1000 | 12.99ms -> 9.14ms (**-29.6%**, 9/9) | not measured | consistent |
| A = 091 alone, B = 091 + this candidate | `--type-text --scroll-step 0` (**no** `--word-wrap`, the harness's default), count 50000 | 170.55ms -> 168.52ms (+... -1.2%, 3/9 pairs) | not measured | **NOT consistent - no measurable effect, see below** |
| same | count 10000 | 37.89ms -> 38.69ms (+2.1%, 4/9) | not measured | **NOT consistent** |

All runs: `tools/paired.sh`, `RUNS=9 FRAMES=6`, alternating A/B per MEASUREMENT.md's paired-run
discipline. `uptime` load average 2.7-3.1 throughout (this session's earlier candidates left the
host quieter than 091's own session).

Allocations/frame (word-wrap, count 50000, `--frames 8`): 647 (A) vs 646 (B) - essentially
unchanged, as expected: this candidate removes CPU work (`break_all_lines`/`align` calls), not
allocations, which 091 already brought down to a small constant.

## Result

A large, consistent, reproducible win (**-63% to -75%**, single- and multi-core alike) specifically
in the scenario the idea targeted - typing into a **word-wrapped** `TextEdit`/`TextInput` - and
literally **zero** measurable effect (differences within noise, "NOT consistent" per
`paired.sh`'s own criterion) in `perf_test`'s **default** (non-word-wrap) text scene, which is the
scenario 091's own `RESULT.md` quoted its "85-90ms at 50000 lines" leftover cost from. See "Why the
result looks the way it does" for the mechanism that splits these two outcomes.

## Does it change behavior/output?

- **Bit-exact?** Yes. `tools/pixelcheck.sh` (`RUNS=5`, `--mode text --count 10000 --type-text
  --scroll-step 0[--word-wrap]`, comparing the 091-alone binary against the 091+this-candidate
  binary): noise floor 3687-3705px (same binary against itself), raw difference 223-267px, **0px
  difference outside the noise floor** in both the word-wrap and no-wrap scenes. "0 paragraph
  shapes" / stable allocation counts per frame also match between the two binaries in the raw
  per-frame table.
- **Is this a breaking change?** No - internal cache/layout implementation detail only. `layout()`
  gained one new parameter (`dirty_range`); every call site was updated, no public API changed.
- Project test suite, all run against **the 091-alone tree as the "before" and 091+this-candidate
  as the "after"** (091 is a prerequisite this candidate's dirty range depends on, not something
  this session re-validated independently - see candidate 091's own `RESULT.md` for its own
  before/after against pristine `master`):
  - `cargo test -p i-slint-core --all-features --lib`: **319 passed, 0 failed**, identical before
    and after.
  - `cd tests && cargo test -p test-driver-rust --features build-time`: **790 passed, 0 failed**,
    identical before and after (35 test binaries, all `ok`).
  - `cd tests && cargo test -p test-driver-interpreter`: **858 passed, 0 failed**, identical
    before and after.

## Why the result looks the way it does

`layout()`'s reuse guarantee needs two things to hold at once: the paragraph is unchanged (091's
job) *and* the line-breaking inputs (width, alignment, max-lines, overflow) match what it was last
broken for (this candidate's job). Debug-traced call-by-call (via a temporary `eprintln!`, removed
before finalizing this patch) revealed that a single edited frame issues **more than one** `layout()`
call for the *same* `TextEdit` item with **different** `LineBreakingInputs`:

- The default (non-word-wrap) text scene is horizontally scrollable, so besides the actual
  constrained draw call (`max_width: Some(1248.0)` in this harness), the same frame also runs an
  **unwrapped natural-width query** (`max_width: None`) that the `ScrollView` needs for its
  horizontal scroll extent. These two calls' `RetainedLineBreaking` overwrite each other every
  single frame (there is only one slot per cache entry), so whichever call runs is always compared
  against the *other* call's width from a moment ago - never its own. The reshape call (which is
  the one carrying a `dirty_range`) lands on whichever width happens to be "stale" from the
  previous call, so `reusable_range` collapses to `None` and falls back to a full break, exactly
  as before this candidate. Both the natural-width and the constrained-width call therefore run a
  **full** `break_all_lines`/`align` pass on every keystroke regardless of this patch - two full
  passes per keystroke, not one, which is worse than 091's own single-pass characterization and
  explains why this candidate moves nothing here.
- The word-wrapped scene has no separate natural-width query (wrapped text's preferred width for
  scrolling is derived differently), so every `layout()` call for that item shares the same
  `max_width`, `line_breaking.inputs` keeps matching, and `dirty_range` reliably narrows the re-break
  to the one or two paragraphs an edit actually touched - hence the large, consistent win there.

This is exactly the generalization the original idea (`IDEAS.md`) anticipated needing -
`layout()` proving paragraphs are "both unchanged *and* already broken for the current width" - but
turned out to need more than a single retained-breaking slot to actually deliver on a horizontally
scrollable (non-wrapped) `TextEdit`: a *single* `Option<RetainedLineBreaking>` per cache entry
cannot remember two different widths' breaking at once, so whichever width was asked for
last always evicts the other. That's a materially bigger change (a small cache keyed by
`LineBreakingInputs`, not a single slot) than this candidate implements - see below.

## What's not done / open questions

- **The natural-width/constrained-width ping-pong is not fixed by this candidate** - it's the
  reason the default (non-word-wrap) `TextEdit` scene sees zero improvement despite 091 + this
  candidate both being logically correct and active. Fixing *that* would need `layout()`/the cache
  entry to retain breaking for more than one `LineBreakingInputs` at a time (e.g. two slots, since
  the observed pattern always alternates between exactly two widths per frame) rather than the
  current single `Option<RetainedLineBreaking>`. Added to `IDEAS.md` as its own candidate -
  distinct from, and larger in scope than, this one.
- Not measured: whether the natural-width query happens on *every* edited frame regardless of
  document size/content, or only when the natural width could plausibly have changed (e.g. only
  when the longest line changed) - if the latter, a cheaper fix might be to skip the natural-width
  recompute itself on edits that provably can't have widened the longest line, rather than making
  the breaking cache smarter. Not investigated this session; added to `IDEAS.md`.
- Styled text and the vacant/no-cache paths are unaffected by this candidate the same way they are
  unaffected by 091 (no dirty range is ever produced for them, so behavior is identical to
  pre-091/pre-this-candidate `master` there).
- The temporary `eprintln!`/`Debug`-derive instrumentation used to find the ping-pong (described
  above) was removed before capturing `patch.diff`; it is not part of this patch.

## Reproduce

```sh
cd perf_test
just build  # or: cargo build --bin perf_test --profile rdebug
for c in 1000 10000 50000; do
  taskset -c 3 ../target/rdebug/perf_test --bench --mode text --count $c --type-text --scroll-step 0 --word-wrap --frames 8
  taskset -c 3 ../target/rdebug/perf_test --bench --mode text --count $c --type-text --scroll-step 0 --frames 8
done
```
