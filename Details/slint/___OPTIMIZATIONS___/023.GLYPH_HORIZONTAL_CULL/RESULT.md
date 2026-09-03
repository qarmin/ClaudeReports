# 23 - Glyphs are never culled horizontally, only vertically (software renderer)

Rebased in place on 2026-08-23 onto slint HEAD `cdb6c43f7ba9826cf0ec453d6e09c5a9a6975f25` (`base_commit`
in `justfile`). The original patch (captured on top of an unrelated, also-unapplied lazy-shaping
candidate, `003.PARAGRAPH_SHAPING_COST`) no longer applied cleanly; this is a straight rebase of the
same fix onto the current, simpler `sharedparley/layout.rs`/`draw.rs` (`TextParagraph::layout` is a
plain `parley::Layout<Brush>` field again on this base, not the lazy/retained kind the old diff
assumed) - not a design change. `patch.diff` now touches only `draw.rs` and `layout.rs`, nothing else.

Patch: `patch.diff`.

**Note (2026-08-25):** this candidate was originally written up as fixing a crash (a
`euclid`/`i16` cast overflow on very long `no-wrap` lines) as well as a performance win. The crash
side has since been fixed upstream, independently of this patch - this file now documents the
performance finding only; see `___OTHER___/001.SOFTWARE_RENDERER_LONG_LINE_PANIC/` if the original
crash report is still needed for reference.

## Cause

Lines are culled vertically against the visible band; **glyphs were never culled horizontally**. A
`no-wrap` line wider than the viewport handed the renderer every glyph on it, including ones tens of
thousands of pixels off-screen - pure wasted rasterization work, independent of anything else.

## The fix, and why the obvious version is not enough

The first attempt (in the original, now-superseded version of this candidate) culled whole **glyph
runs** against the clip's horizontal band. It did not work: a `no-wrap` line of uniform style is a
**single** glyph run spanning the entire line, so the run always intersects the clip and nothing is
skipped.

The fix therefore filters **individual glyphs**, in addition to keeping the (cheap, and useful for
wrapped text) run-level cull. Two details that are load-bearing:

- A `filter`, not a `take_while`: bidi reorders glyphs within a run, so glyph x is not monotonic.
- Disabled while eliding (`Layout::is_eliding()`, added by this patch): the ellipsis is positioned
  from the run that overflows the width, so dropping glyphs there could lose it.

## What was tried

Measured with `___OPTIMIZATIONS___/tools/paired.sh` (alternating runs, 9 pairs, 20 frames/run,
`--mode text --count 200 --line-length 2000`).

| Variant | single-core (`taskset -c 3`) | multi-core (unpinned) | notes |
| --- | --- | --- | --- |
| baseline (un-culled) | median 13.67 ms | median 13.68 ms | submits every off-screen glyph on the line |
| individual-glyph culling (this patch) | median 8.23 ms (**-39.8%**, 9/9 pairs) | median 8.25 ms (**-39.7%**, 9/9 pairs) | off-screen glyphs skipped before rasterization |

Single-frame numbers from one direct run of the same scene at 20 frames (not paired, just for the
first-frame/steady-state shape): first frame 18.03 ms -> 11.34 ms, later-frames median 16.80 ms ->
10.31 ms. Resident memory was unchanged in this scene (28.4 MB both sides) - this rebase, unlike the
original candidate, has no lazy-shaping companion patch applied, so there is no separate memory
story to report here.

This bench is single-threaded software rendering, so single-core and multi-core numbers are
expected to (and do) match closely.

## Result

Paired comparison on a 200-line x 2000-char/line scene shows -39.8% single-core / -39.7%
multi-core (median 13.67-13.68 ms -> 8.23-8.25 ms), consistent in 9/9 alternating pairs both ways -
well above the noise floor described in `paired.sh`.

## Does it change behavior/output?

- **Bit-exact?** yes. Verified with `tools/pixelcheck.sh` (5 runs/side, noise floor subtracted) on
  6 scenes: `text --count 200` (default), `text --count 200 --word-wrap`,
  `text --count 1 --line-length 2000` (long no-wrap line), the same with `--select-all`,
  `list --count 2000` (exercises `overflow: elide`, i.e. the path where the horizontal cull is
  disabled), and `scroll --count 2000`. All six report "pixel-identical outside the noise floor"
  (0 px difference outside the per-binary noise floor in every case).
- **Is this a breaking change?** no - does not change any documented behavior, public API, or file
  format.
- `cargo test -p i-slint-core sharedparley`: 12/12 tests pass, same as before the patch.

## Why the result looks the way it does

Vertical culling against the visible band already existed, but nothing culled horizontally within
a line. A `no-wrap` line wider than the viewport therefore submitted every glyph on it - including
glyphs tens of thousands of pixels off-screen - to the renderer, purely wasted rasterization work.
Whole-run culling could not fix this because a uniformly-styled `no-wrap` line is exactly one glyph
run spanning its entire length, so the run always intersects the clip. Filtering individual glyphs
(not `take_while`, because bidi reordering makes glyph x non-monotonic within a run; and disabled
while eliding, because the ellipsis glyph is positioned from the overflowing run) removes the
wasted work.

## What's not done / open questions

- Only the software renderer's `GlyphRenderer::draw_glyph_run` path was patched; other renderers
  were not audited for the same culling gap.
- This rebase is independent of, and does not require, candidate `003.PARAGRAPH_SHAPING_COST`
  (lazy paragraph shaping), which is itself still unapplied/stale on this base per `IDEAS.md`. The
  original candidate's patch bundled both; this one does not, per the "one fix per candidate
  folder" rule - if 003 is rebased later, re-verify this fix still composes cleanly with it.

## History

- 2026-08-23: original candidate found and fixed a wasted-rasterization-work issue (glyphs never
  culled horizontally) while investigating text-rendering performance; patch captured on top of an
  also-applied lazy-shaping candidate, bundling both.
- 2026-08-23 (this rebase): patch rewritten against `cdb6c43f7` directly (no lazy-shaping
  dependency), rebuilt, re-measured, re-verified bit-exact.
- 2026-08-24 (composition check, after `003.TEXT_DOCUMENT_SHAPING` was itself rebased): confirmed
  `git apply` this folder's `patch.diff` **on top of** `003`'s rebased `patch.diff` and it conflicts
  in both `draw.rs` (the `visible_columns` fn and the `draw()` parameter) and `layout.rs` - **not**
  rebase drift, but because `003`'s own `patch.diff` already carries an equivalent horizontal-cull
  fix bundled into its lazy-shaping diff (visible in its rebased `draw.rs`: the same
  `visible_columns` helper this candidate's fix uses, adapted to call
  `layout.glyphs(paragraph_index)` instead of the plain `self.layout` field this candidate's
  version reads). This matches this file's own first bullet above ("captured on top of an
  also-applied lazy-shaping candidate, bundling both") - the two patches are **alternative**
  implementations of the same fix at two different bases (pristine vs. lazy-shaped), not
  independent changes meant to stack. Applying `003` to a real tree already includes this
  candidate's fix; applying this candidate's own `patch.diff` is the way to get the fix *without*
  `003`'s lazy-shaping behavior change. Not something to "fix" by further rebasing - just don't
  apply both.
- 2026-08-25: reframed as a performance-only finding - the crash this candidate originally also
  fixed has since been fixed upstream, independently of this patch.
