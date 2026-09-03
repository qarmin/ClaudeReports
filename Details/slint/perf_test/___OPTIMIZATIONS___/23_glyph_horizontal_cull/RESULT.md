# 23 - The software renderer panics on a long text line (glyphs are never culled horizontally)

**Status: crash found, cause identified, fixed, pixel-identical. Pre-existing.**
Patch: `horizontal_glyph_cull.diff`.

## The crash

```
panicked at euclid-0.22.14/src/point.rs:471:25: called `Option::unwrap()` on a `None` value
  -> <Point2D<f32, PhysicalPx>>::cast::<i16>
  -> <Rect<f32, PhysicalPx>>::cast::<i16>
  -> i_slint_renderer_software ... GlyphRenderer::draw_glyph_run
  -> sharedparley::draw_text_input
```

`internal/renderers/software/lib.rs:3516` builds a glyph's device rect as `Rect<f32>` and
`.cast()`s it to the renderer's `PhysicalRect` (`Rect<i16>`). `euclid`'s `cast` unwraps, so any
coordinate past `i16::MAX` (32 767) aborts the process.

**Threshold: ~4 500-5 000 characters on one line** at a 13 px font, in a `no-wrap` `TextInput`.

| chars/line | 4 000 | 4 500 | 5 000 | 8 000 | 50 000 |
| --- | --- | --- | --- | --- | --- |
| before | ok | ok | **panic** | **panic** | **panic** |
| after | ok | ok | ok | ok | ok |

Reachable with ordinary content: a minified JSON file, a CSV row, a stack trace, a base64 blob.

## Cause

Lines are culled vertically against the visible band; **glyphs were never culled horizontally**. A
`no-wrap` line wider than the viewport handed the renderer every glyph on it, including ones tens of
thousands of pixels off-screen, and one of those is what overflows the cast.

The cast is the symptom. Submitting off-screen glyphs at all is the cause, and it is also pure
waste: see the timing below.

## Not a regression

A binary built from a pristine `internal/core/textlayout` (no candidate 03, no 19) panics
identically at 5 000 characters. The bug predates this session's work.

## The fix, and why the obvious version is not enough

The first attempt culled whole **glyph runs** against the clip's horizontal band. It did not work:
a `no-wrap` line of uniform style is a **single** glyph run spanning the entire line, so the run
always intersects the clip and nothing is skipped.

The fix therefore filters **individual glyphs**. Two details that are load-bearing:

- A `filter`, not a `take_while`: bidi reorders glyphs within a run, so glyph x is not monotonic.
- Disabled while eliding (`Layout::is_eliding()`): the ellipsis is positioned from the run that
  overflows the width, so dropping glyphs there could lose it.

The run-level cull is kept as well - it is cheap and skips whole runs on wrapped text.

## Results

| | un-culled | culled |
| --- | --- | --- |
| 5 000 / 8 000 / 20 000 / 50 000 chars per line | **panic** | ok |
| 200 lines x 2 000 chars, per frame | 18.24 ms | **11.93 ms (-35%)** |
| first frame | 28.62 ms | 21.75 ms |
| resident | 23.9 MB | 21.9 MB |

## Validation

- **Pixels identical** on 8 scenes: `text` 5 000 and 100 000, `--word-wrap`, `--select-all`, two
  long-line configurations, `list`, `scroll`.
- The reference binary was verified to be the un-culled one by checking that it **still panics** on
  a long line - a first attempt at this comparison used a reference whose build had failed, so the
  culled binary was silently compared against itself and reported eight false "identical".
- 16 `sharedparley` unit tests pass.

## Reproduce

```sh
../../target/rdebug/perf_test --bench --mode text --count 20 --line-length 8000 --frames 3
```
