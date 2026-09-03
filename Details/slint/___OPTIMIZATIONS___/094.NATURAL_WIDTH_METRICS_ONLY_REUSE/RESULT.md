# 94 - Let a metrics-only natural-width query reuse breaking regardless of width

Top-of-backlog `IDEAS.md` entry from candidate 092/093's session: check whether the unwrapped
natural-width query (`text_size` with `max_width: None`, what a horizontally-scrollable `TextEdit`
asks for `ScrollView`'s content-width every edited frame) can itself be made cheap, as an
alternative to candidate 093's "smarter breaking cache" (which found a large win but a confirmed
rendering regression). Applied **on top of** candidates 091 and 092 (091's incremental reshape is
what produces the dirty range this candidate also relies on); `patch.diff` bundles all three
against pristine `master`, the same convention 073/092/093 used. It is an **alternative to 093**,
not stacked on it - do not apply both to the same tree.

## What was tried

Rather than caching two widths' breaking simultaneously (093's approach), this candidate observes
that for a `NoWrap` paragraph, `parley::Layout::full_width()`/line count/height are set by
`break_all_lines` from the content's own advance and are **not affected by the `max_advance`
parameter passed to it** (confirmed by reading `parley` 0.11.1's `line_break.rs`: `full_width` is
computed from `line.metrics.advance`, which `TextWrapMode::NoWrap` never truncates regardless of
the width argument - only `align()`'s glyph *positions* depend on width/alignment, and a
metrics-only caller never reads those). So a caller that only reads `Layout::max_width`/`.height`
(currently only `text_size`, tagged via a new `LayoutOptions::metrics_only: bool`) can reuse a
previous `RetainedLineBreaking`'s aggregate even when `max_physical_width`/alignment differ from
its own call, as long as the document is `NoWrap` and `max_lines`/`text_overflow` (which do change
what the aggregate means) still match. Combined with 091/092's dirty range, this lets the
natural-width query re-break only the edited paragraph(s) and read every other paragraph's
already-known `full_width()` for the rest, instead of a full `break_all_lines`/`align` pass over
the whole document. A rendering/hit-testing/selection caller never sets `metrics_only`, so its own
behavior and matching logic are completely unchanged by this patch.

`perf_test` gained a `--long-line <chars>` flag (see `perf_test_harness.diff`, not part of
`patch.diff`): prepends one line of `chars` characters to the generated document before the first
frame, at a position `--type-text`/`--key-input` never click into (they click a fixed row further
down). This reproduces the idea's own motivating scenario - editing a line while a much longer,
untouched line elsewhere is the document's actual longest line - which the harness's existing
`--type-text` (typing into a fixed row that itself becomes the longest line within a handful of
frames) does not exercise at all.

| Variant | scenario | single-core (`taskset -c 3`) | multi-core | notes |
|---|---|---|---|---|
| A = 091+092, B = 091+092+this candidate | `--type-text --scroll-step 0 --long-line 3000`, count 10000 | 20.42ms -> 13.38ms (**-34.5%**, 9/9) | 20.27ms -> 13.47ms (**-33.5%**, 9/9) | consistent, **bit-exact** (see Correctness) |
| same | count 50000 | 84.84ms -> 48.97ms (**-42.3%**, 9/9) | not measured | consistent, bit-exact |
| A = 091+092, B = 091+092+this candidate | `--type-text --scroll-step 0` (no `--long-line`, the harness's ordinary scene: the typed line becomes and stays the document's longest), count 10000 | 19.73ms -> 13.05ms (**-33.9%**, 9/9) | not measured | consistent, but **confirmed rendering regression** (see Correctness) |
| same | `--word-wrap`, count 10000 | 5.45ms -> 5.36ms (-1.7%, 5/9) | not measured | **NOT consistent** (expected - 092 already covers word-wrap; this candidate's cache entry is keyed by wrap and only ever applies to the `NoWrap` case) |

All runs: `tools/paired.sh`, `RUNS=9 FRAMES=8`, alternating A/B per MEASUREMENT.md's paired-run
discipline. `uptime` load average 3.4-4.4 throughout.

## Result

A large, consistent win (**-33% to -42%**, single- and multi-core alike) in exactly the scenario
the idea targeted, **and** in the harness's pre-existing default `--type-text` scene too - both
reach essentially the same speedup as candidate 093 found. The two scenarios diverge sharply on
correctness: the `--long-line` scenario (edited line never becomes the document's longest) is
**bit-exact**; the ordinary `--type-text` scene (edited line becomes and stays the longest) has the
**same class of scrollbar-thumb-length regression candidate 093 found**, appearing after a similar
number of edited frames. See "Why the result looks the way it does" below - this candidate did
*not* avoid 093's problem, despite a structurally different design intended to.

## Does it change behavior/output?

- **Bit-exact in the `--long-line` scenario:** `cmp -l` between the two binaries' `--snapshot`
  output at `--frames` 6/50/100/150/200 (`--long-line 3000`, `--type-text --scroll-step 0`,
  `count 10000`) found **0 pixels differing** outside the status-label region (`y >= 839`, which
  always differs between any two runs due to the printed timing text - not a rendering
  difference) at every frame count checked.
- **Not bit-exact in the ordinary (no `--long-line`) scene:** same comparison without
  `--long-line` found 0 px differing (outside the label) at `--frames` 6/20/50, but a real,
  reproducible difference starting around **80 edited frames**: at frame 80, binary A's horizontal
  scrollbar thumb spans track pixels 1000+69..1000+118 (its right edge at x=1118/1119) and binary
  B's spans 1000+69..1000+76 (right edge at x=1076/1077) - a **~42 px** difference in thumb length,
  same order of magnitude as 093's own finding (**41 px**) in the same region of the same scene.
  See `A_80_scrollbar.png`/`B_80_scrollbar.png` in this folder (4x-scaled crops of the same region
  of the same frame, one per binary) - the thumb visibly ends earlier in B. Confirmed against the
  noise floor: 6 same-binary repeats at `--frames 80` in the same pixel region (`y < 839`) differ
  by at most 4 px between runs of the identical binary; A-vs-B differs by 86 px at the same frame -
  far outside the noise floor, not a rendering timing artifact.
- **What changes, for whom, under what conditions:** the same class of issue as candidate 093 -
  the horizontal scroll range/thumb size of a non-word-wrapped, horizontally-scrollable
  `TextEdit`/`TextInput` can become measurably wrong after enough edits, specifically when the
  **edited line is also the document's current longest line** (so the natural-width query's result
  feeds back into the item's own actual width every frame - see below). When the edited line is
  never the longest (this candidate's own `--long-line` scenario, and plausibly many real
  documents - editing a short line while a long line elsewhere is untouched), no difference was
  found at any frame count checked.
- **Is this a breaking change?** No public API changed (`LayoutOptions::metrics_only` is a private
  field; `text_size`'s public signature is unchanged). The behavior change is a rendering
  correctness regression under the condition above, not an API break.
- Project test suite:
  - `cargo test -p i-slint-core --all-features --lib`, tree = 091+092 (before this candidate):
    **319 passed, 0 failed**.
  - Same command, tree = 091+092+this candidate (after): **319 passed, 0 failed**. Identical
    pass/fail set before and after. These tests don't exercise `ScrollView` extent computation
    (consistent with the regression needing ~80 edited frames and a specific scene to manifest),
    so they did not catch the difference above.
  - The broader `tests/` driver suites (`test-driver-rust`, `test-driver-interpreter`) were **not**
    re-run for this candidate, for the same reason 093 skipped them: the difference found is a
    `ScrollView`-extent-only issue that a non-pixel-comparing test suite would not catch either.

## Why the result looks the way it does

**The speedup mechanism works as designed** for the case it targets: a `NoWrap` document's
per-paragraph `full_width()`/height are genuinely independent of the width `break_all_lines` was
last called with (verified against `parley` 0.11.1's source, not just inferred from behavior), so
letting a metrics-only caller reuse them regardless of width - combined with 091/092's dirty range
to re-break only the edited paragraph(s) - removes the full O(document) `break_all_lines`/`align`
pass the natural-width query paid every edited frame, in both the targeted scenario and the
harness's ordinary one.

**The correctness gap, characterized but not fully root-caused (same bar 093 was reported at):**
instrumented tracing (`eprintln`, later removed, matching 092/093's own diagnostic technique) shows
that `ti.preferred-width` (the natural-width query's own result) feeds back into the `TextInput`'s
**own actual rendered width** every frame, because the `ScrollView`'s `content-width` binding
(`max(self.visible-width, ti.preferred-width)`) is what the horizontally-unbounded `TextInput`
stretches to fill. In the `--long-line` scenario this feedback loop is inert: the value driving
`content-width` is the long line's width, computed once (frame 0) from a paragraph that is never
in the dirty range again, so it never depends on this candidate's reuse path after that. In the
ordinary scene, the edited paragraph *is* the value driving `content-width`, every single frame -
tracing showed both the metrics call's own computed value and the render call's resulting item
width grow together, frame over frame, in a value that *looks* internally consistent frame-to-frame
(smooth, monotonic, matching the expected per-character growth) yet a `cmp` against the unpatched
binary's independently-computed value shows a real, accumulating divergence by frame 80. This
shape - individually plausible per-frame values that still diverge from ground truth once
compounded through many frames of a feedback loop - was not narrowed down to an exact single wrong
line of code in the time budgeted for this candidate; the leading hypothesis (not confirmed) is a
sub-pixel floating-point difference between a paragraph broken via `break_all_lines(None)` versus
`break_all_lines(Some(w))`, invisible per-frame, compounding because each frame's item width partly
depends on the previous frame's computed preferred-width.

## What's not done / open questions

- **The exact trigger was not root-caused**, same as 093's own unresolved item - see above for the
  leading (unconfirmed) hypothesis. A per-call dump of the *raw* `full_width()` float bits (not
  just the rounded decimal this session printed) for the edited paragraph, compared bit-for-bit
  between a `break_all_lines(None)` and a `break_all_lines(Some(w))` call on an otherwise-identical
  paragraph, would likely settle whether the compounding floating-point-drift hypothesis is
  correct - not done this session.
- **This candidate does not avoid 093's regression in general** - it narrows the *safe* scenario
  (edited line never the document's current longest) but does not fix the unsafe one. Whether a
  further-hardened version (e.g., only take the metrics-only relaxed path when the edited
  paragraph is *not* currently within some margin of the cached max, falling back to a full
  recompute otherwise) could keep the speedup while closing this gap was not attempted - added to
  `IDEAS.md`.
- This candidate should **not** be taken as-is for the general case; it is reported here, with
  both the safe and unsafe scenarios characterized, per this workspace's
  "characterized-difference is in scope, not just tolerated" policy - the reader decides what to
  do with a result that is fast-and-correct in one real scenario and fast-but-wrong in another.
- Not measured: `--resize-step` scenes, or `--select-all`/cursor-placement queries (which read
  `.lines()` directly, per 093's own doc-comment analysis of the higher-risk callers) - not
  exercised by any scenario checked this session.
- The temporary `eprintln`/env-var-gated instrumentation used to trace the feedback loop (described
  above) was removed before capturing `patch.diff`; it is not part of this patch.

## Reproduce

```sh
cd perf_test
# apply perf_test_harness.diff from this folder first (adds --long-line; see justfile)
just build  # or: cargo build --bin perf_test --profile rdebug --features software
RUNS=9 FRAMES=8 taskset -c 3 bash ../___OPTIMIZATIONS___/tools/paired.sh <A binary> <B binary> -- \
  --mode text --count 10000 --type-text --scroll-step 0 --long-line 3000

# rendering difference (no --long-line; need >= ~80 edited frames, matching 093's own recipe):
<A binary> --bench --mode text --count 10000 --type-text --scroll-step 0 --frames 80 \
  --snapshot /tmp/a80.ppm
<B binary> --bench --mode text --count 10000 --type-text --scroll-step 0 --frames 80 \
  --snapshot /tmp/b80.ppm
cmp -l /tmp/a80.ppm /tmp/b80.ppm   # differs; see this folder's *_scrollbar.png for a visual crop
```
