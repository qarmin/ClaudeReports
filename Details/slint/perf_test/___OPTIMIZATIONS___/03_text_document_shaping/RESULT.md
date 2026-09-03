# 03 - TextInput shapes and retains the whole document

**Status: fixed and applied, confirmed on real hardware.** For `wrap: no-wrap`, 11 MB of text now
costs **38.6 MB instead of 540 MB** resident (**130 MB instead of 646 MB** on FemtoVG), **1.73 s
instead of 2.01 s** in the first layout, and a window resize costs **5.6 ms instead of 76 ms**,
pixel-identically, at the price of **+0.2 ms per scrolling frame**.
Wrapped text is deliberately left alone - see "Why wrapping is excluded" below, it is not an
oversight. Patch: `lazy_paragraph_shaping.diff`. How it was built and what did not work:
`IMPLEMENTATION_NOTES.md`. The plan as written before any of it: `PLAN.md`.

## The problem, as measured before the fix

`--mode text`: one `TextInput` (`single-line: false`, `wrap: no-wrap`) inside a `ScrollView`,
N lines of ~110 unique characters, window 1280x860 (about 40 lines visible).

| lines | text size | fill | show + layout | frame 0 | later frames | RSS |
| --- | --- | --- | --- | --- | --- | --- |
| 1 000 | 0.11 MB | - | - | - | - | 21.6 MB |
| 10 000 | 1.1 MB | 2.1 ms | - | 4.8 ms | 2.6 ms | 69.6 MB |
| 100 000 | 11 MB | 21.5 ms | **2.02 s** | 5.4 ms | 3.67 ms | **543.3 MB** |

`create_text_paragraphs` built one `parley::Layout<Brush>` **per paragraph of the whole document**
and `TextParagraph` retained it in the per-item cache, so anything that asked the `TextInput` for
its size shaped and retained the entire document - **~21 us and ~5.25 KB per line**, independent of
what is visible. Root cause stack, `--alloc-profile 5000 --profile-phase show`:

```
<TextInput as Item>::layout_info::{closure#0}   [internal/core/items/text.rs:830]
  -> <SoftwareRenderer as RendererSealed>::text_size   [renderers/software/lib.rs:818]
    -> sharedparley::with_text_layout -> shape_paragraphs   [sharedparley/shaping.rs:376]
      -> create_text_paragraphs   [sharedparley/shaping.rs:265]  // loop over ALL paragraphs
        -> parley RangedBuilder::build -> break_all_lines
```

It is not `preferred-width`/`preferred-height` in the .slint that does it: with
`content-width: visible-width` and `content-height: visible-height` (experiment T1, no `preferred-*`
read anywhere) the cost was identical, because the enclosing `VerticalLayout` still asks the
`ScrollView` -> `Flickable` -> `TextInput` for `layout_info`.

## What changed

`TextParagraph` was split into what everything needs (`ParagraphMetrics`: width, height, line count)
and what only painting needs (`glyphs: Option<Box<parley::Layout<Brush>>>`). A plain-text `no-wrap`
document of more than 128 paragraphs is now shaped **one paragraph at a time**: the layout pass
shapes a paragraph, breaks it, records its metrics and drops the glyphs again, and
`Layout::glyphs(index)` re-shapes a paragraph from its byte range when something actually has to
paint, hit-test or select in it. What was painted last keeps its glyphs, plus 64 paragraphs either
side.

Three things had to hold for that to be safe, and each is enforced in one place:

- **The dependency set may not change.** `shape_paragraphs` still reads the text, the font request,
  the stroke and the link color inside the cache entry's dependency tracker; the lazy path only
  skips the *work*, not the reads. (`shaping.rs`, the comment on `shape_paragraphs`.)
- **Nothing retained may depend on the width.** See below.
- **A re-shaped paragraph must be indistinguishable from one that was never dropped**, so
  `Layout::glyphs` breaks and aligns it with the same `LineBreakingInputs` the metrics pass used.

## Why wrapping is excluded, with the number that decided it

Under `word-wrap`/`char-wrap` the line breaking depends on the width, so a pass's metrics are only
valid for the width it ran at. Dropping the glyphs would therefore mean re-shaping the whole
document on **every width change** - and a window resize is one width change per frame of the drag.
Measured with `--resize-step 40 --word-wrap` on the 100 000-line document, before the exclusion was
added:

| | pristine | lazy for wrapped text too |
| --- | --- | --- |
| per resize frame | 96 ms | **1.79 s** (18.7x worse) |
| paragraphs shaped per resize frame | 0 | 100 001 |

Slint's own `TextEdit` widget defaults to `word-wrap`, so that is not a corner case. Wrapped text,
`max-lines` and `overflow: elide` (whose cut is derived from per-line metrics of paragraphs anywhere
in the document) therefore keep every paragraph's glyphs, exactly as before - as does styled text,
whose paragraphs cannot be reproduced from a byte range alone. `layout.rs` computes this as
`width_independent_metrics`, and three unit tests pin it.

Making wrapped documents lazy too needs a compact intermediate between "shaped" and "nothing" -
advances plus break opportunities, a few hundred bytes per line instead of 5.25 KB - so that
re-breaking at a new width doesn't need re-shaping. `parley::Layout` *is* the shaped form and
`break_all_lines` is the re-break, with nothing in between, so that is a parley-side change.

## Results

`--bench --mode text --count 100000`, median of three runs, against a binary built from the
pristine `internal/core/textlayout`:

### `wrap: no-wrap` (what this candidate is about)

| | before | after | |
| --- | --- | --- | --- |
| `show + layout` | 2.01 s | **1.73 s** | -14% |
| resident after 10 frames | 540.5 MB | **38.6 MB** | **-93%** |
| **resize, per frame** (`--resize-step 40`) | 76 ms | **5.6 ms** | **-93%** |
| scrolling, per frame (median) | 3.83 ms | 4.04 ms | +0.2 ms |
| allocations in `show + layout` | 1 500 175 | 1 600 194 | +6.7% (one `Box` per paragraph) |
| paragraph shapes in `show + layout` | 100 022 | 100 022 | unchanged - the *work* is the same |
| paragraph shapes per scrolling frame | 0 | 13 | the 13 lines scrolled in at 200 px/frame |

The resize win was not the goal and is worth more than it looks: under `no-wrap` a width change
cannot change any retained value, so `metrics_reusable_at_any_width` lets the whole document skip
being re-broken. The pristine tree re-breaks all 100 000 paragraphs on every resize step.

Resident memory against document size - the retained cost per line falls from 5.25 KB to 233 B,
which is the `TextParagraph` bookkeeping plus the text itself:

| lines | before | after |
| --- | --- | --- |
| 1 000 | 21.6 MB | 15.1 MB |
| 10 000 | 69.6 MB | 17.5 MB |
| 100 000 | 544.1 MB | 38.5 MB |

The `show + layout` win is not from doing less shaping - the paragraph counter says both do 100 022.
It comes from two things: the glyphs are freed as they are produced instead of accumulating half a
gigabyte, and the second layout pass no longer re-breaks the document (the width-independence note
in `IMPLEMENTATION_NOTES.md`).

The per-frame scrolling cost is the honest price: 13 newly visible lines shaped per frame at
200 px/frame, ~25 us each. It is O(visible) where the old code was O(document) once - which is the
trade this candidate is about.

### `wrap: word-wrap`, and the other scenes: unchanged

| scene | pristine | session |
| --- | --- | --- |
| `text` 100k `--word-wrap`, `show + layout` | 1.96-2.04 s | 1.96-1.99 s |
| `text` 100k `--word-wrap`, resident | 543.3 MB | 545.3 MB |
| `text` 100k `--word-wrap`, per frame | 3.63 ms | 3.64 ms |
| `text` 100k `--word-wrap --resize-step 40` | 102 ms | 103 ms |
| `list` 100 000 | 2.51 ms | 2.47 ms |
| `scroll` 100 000 | 152.9 ms | 140.6 ms |
| `rects` 3 000 | 2.18 ms | 2.30 ms |

## On real hardware (FemtoVG, 1280x860)

`just gpu-03` builds a pristine-`textlayout` binary and runs both on the display, each several
times so the noise floor shows. Full table in `../GPU_03.md`.

| `text` 100 000, two runs of each binary | pristine | session |
| --- | --- | --- |
| resident, `no-wrap` | 638.5 / 638.7 MB | **122.5 / 122.6 MB** (-81%) |
| resident, `--word-wrap` | 638.4 / 638.3 MB | 634.9 / 635.1 MB (unchanged) |
| `show + layout` | 7.88 / 7.85 s | 7.53 / 7.48 s (-0.36 s) |
| per frame, `no-wrap` | 2.23 / 2.10 ms | 2.68 / 2.80 ms (+0.57 ms) |
| per frame, `--word-wrap` | 2.18 / 2.26 ms | 2.06 / 2.18 ms (unchanged) |
| paragraphs shaped per frame | 0 | **11** `no-wrap`, **0** `--word-wrap` |

11 is exactly the number of lines entering a 1280x860 viewport at 200 px/frame, so the per-frame
cost is the design working, not something extra. Wrapped text shaping 0 is the exclusion working.
(Numbers from the fifth run; five runs agree on every sign and on the memory, with absolute times
moving with the machine. Only within-run pairs are compared - see `../GPU_03.md`.)

The FemtoVG baseline - window, GL context, glyph atlases - is ~109 MB (the `rects` rows show it), so
the document's own share falls from ~537 MB to ~21 MB, matching the software renderer's
543 -> 38 MB above its own ~15 MB baseline. `text` 10 000: 155.4 -> 104.7 MB. The startup win is
-3.3% here against -15% headless, because the windowed number also carries window creation, GL init
and font loading.

**The GPU run found a cost that the software renderer hid.** `text` 10 000 and `text` 100 000 shape
the same 13 paragraphs per frame, yet cost +0.30 ms and +1.32 ms. The difference was two scans that
walked the whole paragraph list per `with_text_layout` call - the retention window and the
width-change check - which is ~1 ms per frame over 100 000 `RefCell`s, four times the cost of the
shaping itself, and invisible in a 3.6 ms software frame. `RetainedLineBreaking` now carries a
`shaped` range that bounds both scans. Confirmed fixed on hardware across two runs: the two sizes
cost +1.06/+0.92 ms and +0.61/+0.37 ms, gaps of 0.14 and 0.24 ms against spreads of 0.32 - where
before the fix they were 4.4x apart.

`scroll` 100 000 pays a real +1.4% resident (1503.45 -> 1524.9 MB, 0.1 MB spread on both sides):
~32 bytes per retained paragraph for boxing the glyphs, which is what makes the lazy case cheap in
the first place.

## Under cursor movement, the interaction it was never tested with

`--key-input` dispatches one `PageDown` per frame into the focused `TextInput` (a click is
dispatched first, or the keys go nowhere and the benchmark measures an idle window). A PageDown
moves the cursor a full screen, so an entirely new band of paragraphs becomes visible - the worst
case for lazy shaping.

| 100 000 lines, one PageDown/frame | before 03 | after 03 |
| --- | --- | --- |
| frame time | 3.65 ms | 4.60 ms (+0.95 ms) |
| paragraphs shaped/frame | 0 | **47** |
| resident | 543.5 MB | **38.4 MB** |

47 is the number of lines on a screen, and +0.95 ms / 47 is ~20 us per shape - the same per-shape
cost the scrolling case shows (13 shapes, +0.2-0.35 ms). Nothing about cursor movement is special;
it costs exactly what becoming visible costs. Scrolling and paging at the same time gives the same
figure (4.52 ms), because the band moves once either way.

## Validation

- **Pixels identical** above the status line on ten scenes, against a pristine-core binary: `text`
  5 000 and 100 000 (scrolled 12 frames deep), both with and without `--word-wrap`, both with
  `--resize-step 40`, a `text` document below the lazy threshold, `list`, `scroll` and `rects`. The
  same-binary control was run first (see `PIPELINE.md`).
- **2 025 tests pass, 0 failures**: `i-slint-core` (all features, including five new tests that
  exercise the lazy path), `slint`, and `test-driver-rust` + `test-driver-interpreter` with
  `--features build-time`. `just -f 03_text_document_shaping/justfile test`.
- `cargo clippy -p i-slint-core --features testing --lib --tests` clean for `sharedparley`.

## Reproduce

```sh
just -f 03_text_document_shaping/justfile run       # the headline table and the scaling
just -f 03_text_document_shaping/justfile matrix    # no-wrap vs word-wrap, scrolling vs resizing
just -f 03_text_document_shaping/justfile pixels    # needs a pristine binary, see the justfile
```

## What is still open

1. `preferred-width` remains O(document) in *time*: it is the widest line, so every paragraph still
   has to be shaped once. Making it cheaper means it stops being exact (real editors grow the
   horizontal scroll range as you scroll into new lines) - a semantic change that needs a design
   discussion upstream. Candidate 04 measured that skipping ICU line segmentation for `no-wrap`
   would only take 3% off that time, so there is no upstream fix of the right size.
2. Wrapped documents keep the old cost entirely (543 MB for 11 MB of text). See above: a compact
   shaped-but-unbroken representation in parley is what that needs.
3. Nothing about the per-frame cost is unexplained any more: the windowed run shapes **11**
   paragraphs per frame (measured, not assumed), which is exactly the lines entering the viewport,
   and `word-wrap` shapes **0** - the exclusion working as designed. The 2x difference in apparent
   per-shape wall time against the headless run is CPU state (live compositor, GL driver threads),
   and at +0.6 ms per frame it is not worth chasing.
