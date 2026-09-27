# 04 - Skip ICU line segmentation when the text cannot wrap (plan, before experiments)

## Hypothesis

`parley::analysis::analyze_text` (parley-0.11.0/src/analysis/mod.rs:330-380) always runs the
ICU4X line segmenter over the text to find soft break opportunities (`Boundary::Line`), even
when the resolved style has `TextWrapMode::NoWrap`. In that mode the line breaker never
consults them: `layout/line_break.rs:523,574,597,644` all gate on
`text_wrap_mode == TextWrapMode::Wrap`.

Two facts make this interesting for Slint:

1. Slint sets `TextWrapMode::NoWrap` exactly when `TextWrap::NoWrap`
   (`internal/core/textlayout/sharedparley/shaping.rs:148-156`), and **`no-wrap` is the
   default for `Text`** and for single-line inputs. So most text in most Slint apps pays for
   line-break analysis it cannot use.
2. Mandatory breaks do *not* come from the segmenter (they come from character properties,
   `analysis/mod.rs:493-497`), and word boundaries come from the separate word segmenter, so
   skipping the line segmenter should not change any observable behaviour for NoWrap text.

## Plan

1. Measure the ceiling first, without touching anything: micro-bench the ICU line segmenter
   alone over the same 11 MB / 100 000-paragraph corpus (`icu_segmenter` is already in the
   dependency graph). If it is a few percent of the 2.08 s from candidate 03, stop here and
   write that down.
2. If it is worth it: vendor parley 0.11.0 next to `femtovg` (`../../../parley`), add
   `[patch.crates-io] parley` to `perf_test/Cargo.toml`, and skip the line-boundary loop when
   every style run resolves to `TextWrapMode::NoWrap`.
3. Measure: `--mode text --count 100000` (show+layout), `--mode scroll --count 100000`
   (frame 0 shapes 300 000 texts), `--mode list` (steady state, 18 shapes/frame).
4. Verify correctness: `cargo test` in `api/rs/slint` (text_* tests), the `tests/screenshots`
   suite if it runs headless, and a targeted check that wrapping still wraps
   (`--mode text` with the `word wrap` checkbox / `wrap: word-wrap`).

## Risks

- `TextWrapMode` is a per-span style; the check must cover every style run, not just the
  default.
- parley may expose break opportunities in its public cluster API; skipping changes what a
  NoWrap layout reports there. Needs a grep before claiming "no behaviour change".
- Vendoring parley means the repo's other workspaces keep using crates.io parley - only
  `perf_test` sees the patch. Fine for measuring, must be stated in the result.
