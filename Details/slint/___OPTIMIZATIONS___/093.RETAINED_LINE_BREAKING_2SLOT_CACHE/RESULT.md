# 93 - 2-slot `RetainedLineBreaking` cache

Candidate 092 carried `RetainedLineBreaking` across an incrementally-reshaped edit, but found it
had **zero** measurable effect on `perf_test`'s default (non-word-wrap) text scene: a
horizontally-scrollable `TextEdit` issues an unwrapped natural-width query (`max_width: None`,
for the `ScrollView`'s horizontal extent) and the actual constrained draw call
(`max_width: Some(w)`) every edited frame, and both calls' breaking overwrote each other in the
single `Option<RetainedLineBreaking>` slot every time, so neither call ever found its own width
still cached. This candidate replaces that single slot with a 2-slot cache
(`[Option<RetainedLineBreaking>; 2]`, `internal/core/textlayout/sharedparley/layout.rs`'s new
`RETAINED_LINE_BREAKING_SLOTS`), matched by `LineBreakingInputs` on each `layout()` call, so both
alternating widths can retain their own breaking independently. Applied **on top of** candidates
091 and 092 (091's incremental reshape is what produces the dirty range 092/093 rely on);
`patch.diff` bundles all three against pristine `master`, the same convention 073/092 used.

## What was tried

Only one variant: a fixed 2-slot cache (matching IDEAS.md's proposal literally - "even just 2
slots, since the pattern always alternates between exactly two widths"). `layout()`'s matching
logic (`take_matching`) picks whichever of the two slots matches the current call's
`LineBreakingInputs`, treats it as before (full fast path / partial reusable-range re-break), and
hands the other slot back untouched so a second, different-width caller sharing the same cache
entry still finds its own slot intact. On an actual reshape, the non-matching slot is discarded
rather than carried forward (see "Why the result looks the way it does" for why this matters and
turned out not to be sufficient).

| Variant | scenario | single-core (`taskset -c 3`) | multi-core | notes |
|---|---|---|---|---|
| A = 091+092, B = 091+092+this candidate | `--type-text --scroll-step 0` (no `--word-wrap`, the harness's default), count 10000 | 18.23ms -> 11.70ms (**-35.8%**, 9/9 pairs) | 18.28ms -> 11.56ms (**-36.8%**, 9/9) | consistent |
| same | count 10000, repeat | 18.49ms -> 11.48ms (**-37.9%**, 9/9) | 20.69ms -> 13.88ms (**-32.9%**, 9/9) | consistent |
| same | count 50000 | 83.26ms -> 49.01ms (**-41.1%**, 9/9) | not measured | consistent |
| same | `--word-wrap`, count 10000 | 6.08ms -> 5.95ms (-2.1%, 6/9) | not measured | **NOT consistent** (expected - 092 already covers word-wrap fully; this candidate targets the non-word-wrap ping-pong case) |

All runs: `tools/paired.sh`, `RUNS=9 FRAMES=8`, alternating A/B per MEASUREMENT.md's paired-run
discipline. `uptime` load average 1.5-3.7 throughout.

## Result

A large, consistent, reproducible win (**-33% to -41%**, single- and multi-core alike, two
repeats each) in exactly the scenario 092 identified as unfixed: typing into a **non-word-wrapped**
`TextEdit` inside a `ScrollView`. **However, this candidate also introduces a confirmed,
reproducible rendering difference in that same scenario** - see below. This is not a marginal
or noise-floor result on either axis: the speedup is large and 9/9-consistent, and the rendering
difference is large and reproducible, not a 1-pixel antialiasing wobble.

## Does it change behavior/output?

- **Bit-exact? No.** `tools/pixelcheck.sh`-style noise-floor-aware comparison (6-10 runs per side,
  `--mode text --count 10000 --type-text --scroll-step 0`, no `--word-wrap`) found:
  - At low frame counts (6-60 edited frames), 0 px differ outside the noise floor - the harness's
    own default `pixelcheck.sh` invocation (`--frames 6`) reports **pixel-identical**, which is why
    this needed manually sweeping frame counts to catch.
  - Starting around **80 edited frames**, a real, reproducible (confirmed with both 6 and 10 runs
    per side, always outside the sampled noise floor) difference appears: the `ScrollView`'s
    **horizontal scrollbar thumb is a different length** between the two binaries. At frame 80,
    binary A's thumb spans track pixels 750-1121 (372 px) and binary B's spans 750-1080 (331 px) -
    a **41 px / ~11%** difference in thumb length, not a subpixel antialiasing difference. See
    `A_80_scrollbar.png` / `B_80_scrollbar.png` in this folder (2x-scaled crops of the same region
    of the same frame, one per binary) - the thumb visibly ends earlier in B.
  - `--word-wrap` scenes showed no difference at any frame count checked (6-200): expected, since
    092's own `RESULT.md` established word-wrapped `TextEdit` has no separate natural-width query,
    so there is no second width for a "other slot" bug to affect.
- **What changes, for whom, under what conditions:** the horizontal scroll range/thumb size of a
  non-word-wrapped, horizontally-scrollable `TextEdit`/`TextInput` can become measurably wrong
  after enough edits, i.e. the natural (unwrapped) width the `ScrollView` computes for its extent
  drifts from the value the unpatched code computes. Text *rendering itself* was not observed to
  differ (no wrong glyph positions/wrapping found in any comparison) - only the derived scroll
  metric. Plausible cause (not fully confirmed - see below): `RetainedLineBreaking`'s cached
  numbers for a given width are valid regardless of which width is *currently* physically broken
  into the shared, single-owner `TextParagraph::layout` (`parley::Layout`) - but this cache
  doesn't track that at all, and a case was not found (or ruled out) where a slot's cached numbers
  get reused past a point where they should have been invalidated by an intervening edit. The
  `RETAINED_LINE_BREAKING_SLOTS` doc comment in `layout.rs` (part of this patch) documents this
  risk in the abstract; the pixel comparison confirms it is not just theoretical.
- **Is this a breaking change?** No public API changed (`layout()`'s new parameter and
  `RetainedLineBreaking`'s new slot count are both `pub(super)`/private). The behavior change is a
  rendering correctness regression, not an API break.
- Project test suite (`i-slint-core`'s own unit tests; these don't exercise `ScrollView` extent
  computation, so they did not catch the difference above - consistent with it needing ~80 edited
  frames to manifest):
  - `cargo test -p i-slint-core --all-features --lib`, tree = 091+092 (before this candidate):
    **319 passed, 0 failed**.
  - Same command, tree = 091+092+this candidate (after): **319 passed, 0 failed**. Identical
    pass/fail set before and after.
  - The broader `tests/` driver suites (`test-driver-rust`, `test-driver-interpreter`, ~1600 tests
    total per candidate 092's own run) were **not** re-run for this candidate given the rendering
    difference already found by the pixel comparison above; not expected to catch a
    `ScrollView`-extent-only difference either, since they don't render/compare pixels.

## Why the result looks the way it does

**The speedup mechanism works exactly as intended**: with 2 slots, both the natural-width query
and the constrained draw call retain their own `RetainedLineBreaking` across edits, so 091's dirty
range actually gets used for both, cutting the O(paragraphs) `break_all_lines`/`align` pass down to
just the touched paragraph(s) - previously *both* calls paid a full pass on every edited frame
(092's own finding), now typically neither does once warmed. This is a real, large, structural
speedup for a whole class of documents (non-word-wrapped, horizontally-scrollable, frequently
edited) that 091+092 alone could not reach.

**The correctness gap**: `RetainedLineBreaking`'s cached numbers (`max_width`, `height`,
`elision_info`, `line_limit_cut`) are snapshots decoupled from `TextParagraph::layout`'s actual,
*shared, single-owner* broken-line state - only one width can be physically broken into a given
paragraph's `parley::Layout` at a time, regardless of how many `RetainedLineBreaking` snapshots the
cache remembers. This patch's cache-invalidation rule (drop the *other* slot whenever a reshape
happens, since paragraph content changed and any snapshot depending on removed/changed content is
stale) turned out to be **necessary but not sufficient** to keep both slots' snapshots
trustworthy indefinitely under this cache's actual call pattern - the pixel comparison shows a
concrete case where it isn't. The exact triggering sequence (which of the two calls needs to see
`dirty_range: None` while its own cached slot has already gone stale) was not fully traced line by
line this session; see "What's not done" below.

## What's not done / open questions

- **The exact trigger for the scrollbar-thumb divergence was not root-caused.** It was
  characterized (magnitude, onset around 80 edited frames, localized to the `ScrollView` thumb,
  reproducible) but not traced back to the precise call sequence inside `with_text_layout`/
  `cached_paragraphs` that produces it. A per-call `eprintln!` trace of `LineBreakingInputs` and
  `dirty_range` across ~100 edited frames (the same technique 092 used to find the original
  ping-pong) would likely pin it down quickly - not done this session.
- **A correct fix likely needs more than matching-by-width.** As reasoned in `layout.rs`'s
  `RETAINED_LINE_BREAKING_SLOTS` doc comment, a cached slot's numbers are only unconditionally safe
  for a caller that never reads `TextParagraph::layout.lines()` directly (a metrics-only caller,
  e.g. the natural-width query) - a caller that draws/hit-tests needs the shared physical layout to
  actually currently be broken for its own width, which this cache does not track or guarantee.
  Whether the observed bug is exactly this failure mode, or a separate one (e.g. an off-by-one-edit
  staleness in how a slot is invalidated), is unresolved.
- This candidate should **not** be taken as-is; it is reported here, with the regression fully
  characterized, per this workspace's "characterized-difference is in scope, not just tolerated"
  policy - the reader decides what to do with a fast-but-wrong result.
- Not measured: whether the same divergence appears in `--resize-step` scenes (a third width
  entering the mix would exceed the 2-slot capacity) or with `--select-all`/cursor-placement
  queries, which do read `.lines()` directly (`selection.rs`) and were flagged as the
  higher-risk caller in the doc-comment analysis above - not exercised by any scenario checked
  this session.

## Reproduce

```sh
cd perf_test
just build  # or: cargo build --bin perf_test --profile rdebug --features software
RUNS=9 FRAMES=8 taskset -c 3 bash ___OPTIMIZATIONS___/tools/paired.sh <A binary> <B binary> -- \
  --mode text --count 10000 --type-text --scroll-step 0

# rendering difference (need >= ~80 edited frames to see it; the default pixelcheck.sh's
# --frames 6 will report identical):
<A binary> --bench --mode text --count 10000 --type-text --scroll-step 0 --frames 80 \
  --snapshot /tmp/a80.ppm
<B binary> --bench --mode text --count 10000 --type-text --scroll-step 0 --frames 80 \
  --snapshot /tmp/b80.ppm
cmp -l /tmp/a80.ppm /tmp/b80.ppm   # differs; see this folder's *_scrollbar.png for a visual crop
```
