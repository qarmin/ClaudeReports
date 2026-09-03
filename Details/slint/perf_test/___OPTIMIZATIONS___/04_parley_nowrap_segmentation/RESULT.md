# 04 - Skip ICU line segmentation for text that cannot wrap

**Status: null result. Correct in principle, but only ~3% of the cost - not worth vendoring parley.**
Hypothesis and plan as written before measuring: `PLAN.md`.

## The hypothesis was right about the waste

`parley::analysis::analyze_text` runs the ICU4X line segmenter unconditionally, and for
`TextWrapMode::NoWrap` the resulting `Boundary::Line` markers are never consulted
(`parley-0.11.0/src/layout/line_break.rs:523,574,597,644` all gate on
`text_wrap_mode == TextWrapMode::Wrap`). Mandatory breaks come from character properties
(`analysis/mod.rs:493-497`), not from the segmenter, and word boundaries come from the
separate word segmenter - so skipping line segmentation for no-wrap text looks safe.
Slint asks for `TextWrapMode::NoWrap` on `TextWrap::NoWrap`
(`internal/core/textlayout/sharedparley/shaping.rs:148-156`), which is the **default for
`Text`**, so this would apply to most text in most Slint apps.

## But the ceiling is 3%

`segmenter_bench/` runs the same two ICU segmenters parley uses over the exact corpus the
perf_test text mode generates (100 000 paragraphs, 10.4 MB):

```
$ cd segmenter_bench && cargo run --release
100000 paragraphs, 10.38 MB
line segmentation  63.24ms  (1558334 breaks, 632 ns per paragraph)
word segmentation  50.97ms  (3016668 breaks, 510 ns per paragraph)
```

63 ms against the 2.08 s that candidate 03 measured for shaping the same document = **3.0%**.
Word segmentation (another 51 ms) is needed for cursor movement anyway and was never a
candidate.

**Decision: stop here.** Vendoring parley, carrying a patch and re-verifying its test suite
is not worth 3%, and the same 3% would be recovered for free if the remaining ~97% (per
paragraph shaping + line breaking + retained layouts) is attacked instead. Filed as a note
in case parley upstream wants it: it is a one-line guard in `analyze_text`.

## What the ceiling measurement redirects to

2.08 s / 100 000 paragraphs = **20.8 us per ~110 character paragraph**, of which segmentation
is 0.6 us. The rest is the per-paragraph work in `create_text_paragraphs` ->
`LayoutWithoutLineBreaksBuilder::build` -> parley `RangedBuilder`: font queries, style
resolution, shaping, `break_all_lines`, and the ~5 KB `parley::Layout` that is kept.
Candidate 05 looks at the per-paragraph constant costs.
