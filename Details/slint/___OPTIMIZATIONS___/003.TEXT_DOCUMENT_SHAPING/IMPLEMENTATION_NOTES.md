# 03 - how the fix was built: what worked, what did not, in the order it happened

The first half of this file was written *before* any code, after an ablation run, to answer "is
anything blocking an attempt at this". The second half is what actually happened when it was
attempted. The predictions are left as they were, including the one that turned out wrong.

---

# Part 1 - written before the implementation

## Measurement: the width is not the trigger, `layout_info` as a whole is

Ablation: made `TextInput::layout_info` return a constant size for `Orientation::Horizontal`
(no `text_size` call at all, i.e. no widest-line computation):

```
baseline        show + layout  2.08 s   RSS 540.8 MB
no width pass   show + layout  1.99 s   RSS 541.2 MB
```

Unchanged. The vertical orientation calls the same `implicit_size(...)` -> `text_size(...)` for the
preferred height, and `text_size` shapes **every paragraph** whatever it was asked for. So there is
no cheap win by dropping the width: the whole `layout_info` path has to stop shaping the document.

## The three blockers identified up front

1. **One entry point does everything.** `sharedparley::text_size` -> `with_text_layout` ->
   `shape_paragraphs` shapes all paragraphs, and `shaping.rs:6-10` states the invariant that
   measuring and drawing must go through the same function so they register *identical* property
   dependencies. A lazy or partial path must keep that dependency set identical or invalidation
   silently breaks (missed repaints, stale layouts).
2. **Height is cheap, width is not.** For `wrap: no-wrap` the preferred height is
   `paragraph_count * line_height` and needs no shaping at all. The preferred width is the widest
   line, which needs every paragraph's advances - O(document) *time* even if nothing is retained.
   Candidate 005 measured parley at 124 ns/char, so 11 MB of text is ~1.5 s of shaping no matter how
   the caching is arranged. Fixing the *memory* (50 B/char retained) is possible without changing
   semantics; fixing the *time* means `preferred-width` stops being exact.
3. **The cache has no visible-range notion.** `TextLayoutCache` is keyed per item and holds
   `Vec<TextParagraph>` for the whole document; nothing tells it which paragraphs are on screen.

## Planned order

1. Split `TextParagraph` into metrics and glyphs (`Option<parley::Layout>`), no behaviour change.
2. `no-wrap` heights without shaping: `paragraph_count * line_height` from the font metrics.
3. Lazy glyph shaping per paragraph on draw / hit-test / cursor-rect, with an LRU keyed by
   paragraph index and a cap. This is where the 540 MB goes away.
4. Only then decide about `preferred-width`: exact or progressive.

---

# Part 2 - what actually happened

## Step 1 (split) - as planned, and it was indeed the thing that made the rest reviewable

`TextParagraph` became `{ range, y, metrics: ParagraphMetrics, glyphs: RefCell<Option<Box<Layout>>>,
links, code_ranges }`. Every consumer was sorted into "needs metrics" and "needs glyphs":

| needs only metrics | needs glyphs |
| --- | --- |
| `line_limit_cut`, the `max_width` fold, the total height | the draw loop, inline-code capsules |
| `paragraph_by_y`'s binary search, `Layout::draw`'s `partition_point` | `Cursor::from_point`, `Cursor::from_byte_index` |
| the paragraph-level cull in `selection_geometry` | `Selection::geometry_with`, `link_in_layout` |
| `elision_extent`'s "was anything dropped below" | `elision_extent`'s per-line box test |

That table is the whole design: `ParagraphMetrics` is `{ width, height, line_count }` and nothing
else, because that is exactly the left column. Per-*line* metrics deliberately did not go in - only
paragraphs that are drawn need them, and those hold their glyphs anyway.

Two knock-on decisions:

- **Boxing.** `Option<Box<parley::Layout<Brush>>>` rather than `Option<parley::Layout<Brush>>`: an
  unshaped paragraph then costs a pointer instead of a whole `Layout`, of which a 100 000-line
  document has one per line. The price is one allocation per shaped paragraph, which is the entire
  +100 019 allocations in the `show + layout` column.
- **`RefCell` per paragraph, not per document.** `Layout::glyphs` shapes on demand from behind the
  `&Layout` every consumer gets, and while it holds a `Ref` on paragraph *i* it may have to shape
  paragraph *j*. One `RefCell` per document would deadlock on that; per paragraph it cannot.

## Step 2 (cheap `no-wrap` heights) - dropped, it was the wrong idea

The plan said this was "verifiable in isolation: `show + layout` should drop while pixels stay
identical". It is not, and it would not have: `text_size` returns width *and* height from one
`layout()` call, and the width still needs every paragraph shaped. The pre-implementation ablation
in Part 1 already showed the mirror image of this (dropping the *width* alone changed nothing).
Making the height free changes nothing on its own for the same reason. It was skipped, and the
metrics pass shapes every paragraph exactly as before - the win is that it does not *keep* them.

## Step 3 (lazy shaping) - this is the fix, and it needed two things the plan did not anticipate

### The LRU became a window, which is simpler and needs no bookkeeping

Rather than an LRU keyed by paragraph index, `Layout` records the lowest and highest index anything
asked `glyphs()` for, and `dismantle()` keeps that range padded by 64 paragraphs either side. A call
that asks for nothing - a pure `text_size` - records nothing and therefore drops nothing, so a
measuring pass cannot throw away what the last draw shaped. No eviction queue has to be stored or
round-tripped through the cache entry.

### The document was shaped twice, and finding out why was most of the work

First lazy implementation: **`show + layout` went from 1.94 s to 3.40 s** and the allocation count
doubled to 3 200 202. RSS was already down to 38.6 MB, so the mechanism worked - but something was
shaping the document twice.

Instrumenting `layout()` to print its `LineBreakingInputs` whenever it breaks more than 1 000
paragraphs answered it in one run:

```
BREAKPASS n=100001 max_w=None          align=Left max_lines=None overflow=Clip retain_all=false
BREAKPASS n=100001 max_w=Some(1248.0)  align=Left max_lines=None overflow=Clip retain_all=false
```

Two passes differing in **nothing but the width**: the preferred-size pass (`text_size`, no width)
and the real one (the item's width). Before this candidate that second pass only re-*broke* layouts
that were still in memory, which is cheap. With the glyphs dropped it re-*shaped* the document.

The fix is a property of parley, checked in its source rather than assumed
(`parley-0.11.0/src/layout/line_break.rs` and `alignment.rs`): a line's advance, height, break
positions and `inline_min_coord` are computed without the width. The width only sets
`inline_max_coord`, which nothing here reads and which feeds `align()` alone. So under `no-wrap`
**the metrics of a pass are the same for any width, while the glyph positions are not.**

`RetainedLineBreaking` therefore carries `width_independent_metrics`, set when the pass ran under
`no-wrap` and was free to drop glyphs (which rules out `max-lines` and `overflow: elide`, the two
things that make a derived value depend on the width). A later call that differs only in the width
reuses the retained metrics instead of re-shaping. That took `show + layout` to 1.79 s - **below**
the 1.94 s baseline, because the second pass now does no work at all rather than re-breaking 100 000
layouts.

Note this is not only a lazy-path win: it is the reason the *first* number in `RESULT.md` improved
rather than merely held.

### Then it re-shaped the whole visible band every frame

Steady state was 4.3 ms/frame against a 3.6 ms baseline, and it was not the ~13 newly scrolled-in
lines: measuring pass and draw pass alternate every frame, so "the width changed, drop the glyphs"
fired every frame and the draw re-shaped all 47 visible paragraphs.

The width a *pass* runs at and the width the *retained glyphs* were aligned for are different
things, and conflating them was the bug. `glyph_width` now tracks the second, and the drop happens
inside `Layout::glyphs` - on the first call that actually wants a glyph - not when the inputs
change. A measuring pass changes the width without ever asking for a glyph, and so costs nothing.
Steady state fell to 13 paragraph shapes per frame, which is exactly the number of lines that
scroll in.

### The wrapped case was an 18.7x regression, and only a resize benchmark could show it

Everything above was measured on `wrap: no-wrap`, which is what the `text` scene uses. Slint's own
`TextEdit` widget defaults to `word-wrap`, so that had to be checked separately - and the harness
could not do it: `--mode text` had no way to select the wrap mode, and no scene resized a window.

Adding `--word-wrap` was enough to show that scrolling a wrapped document was fine. It was not
enough to show the actual problem, which needed `--resize-step`. Two harness bugs had to be fixed
before that number meant anything, and both would have produced a confident wrong answer:

1. **The render stride.** `renderer.render(buffer, options.width)` was passing the *buffer's* width
   as the stride while the window had been resized to something narrower, which renders a sheared
   image and measures the wrong amount of work.
2. **Where the counters were reset.** They were reset *inside* `draw_if_needed`, i.e. after the
   `set_size` that triggers the re-layout. The re-layout therefore landed in nobody's frame: the
   resize benchmark reported 48 paragraph shapes per frame when the real number was 100 001. The
   counters are now read as running totals and a frame is charged the difference, so work triggered
   before the render belongs to the frame that triggered it.

With that fixed, on the 100 000-line document:

| per resize frame | pristine | lazy for wrapped text too |
| --- | --- | --- |
| time | 96 ms | **1.79 s** |
| paragraphs shaped | 0 | 100 001 |

Under wrapping the line breaking depends on the width, so a pass's metrics are only valid for the
width it ran at; with the glyphs dropped, every width change re-shapes the document, and a drag
resize is one width change per frame. There is no third option: `parley::Layout` *is* the shaped
form and `break_all_lines` is the re-break, with nothing compact in between, so either the 5.25 KB
per line stays resident or every re-break re-shapes.

An adaptive policy was considered - go lazy, and switch to retaining once an entry has been
re-broken at a second width - and rejected: the measure and draw passes of a single frame already
produce a fast-path hit, so "is this entry being resized" cannot be told from "has the width changed
since last time" without more state than the win justifies, and the first resize frame would still
cost 1.79 s. The rule became: **shape lazily only where nothing retained depends on the width.**
That is `no-wrap` without `max-lines` and without `overflow: elide`, and it is the one line
`width_independent_metrics` in `layout.rs`.

The exclusion turned the width-independence work from a fix into a win in its own right: under
`no-wrap` a resize now skips re-breaking the document entirely, **76 ms -> 5.6 ms**, which the
pristine tree pays because it re-breaks all 100 000 paragraphs on every resize step.

### The GPU found an O(document) scan that the software renderer could not

Two document sizes that shape the same 13 paragraphs per frame cost +0.30 ms (10 000 lines) and
+1.32 ms (100 000 lines) on FemtoVG. Identical work, 4.4x apart, so something in the new code
scaled with the document rather than with the window.

It was two scans, both introduced by this candidate and both running on every `with_text_layout`
call - so twice per frame:

- `Layout::dismantle` walked every paragraph to apply the retention window.
- `Layout::glyphs` walked every paragraph to drop glyphs aligned against an old width.

At 100 000 paragraphs that is ~200 000 `RefCell` accesses per frame, about 1 ms - **four times the
cost of the lazy re-shaping this candidate actually added**. On the software renderer, where a frame
is 3.6 ms rather than 2.5 ms and the measurement is noisier, it hid inside the expected regression:
headless the same bug read as +0.46 ms against +0.34 ms, a gap easy to dismiss as noise.

`RetainedLineBreaking` now carries a `shaped: (usize, usize)` range that bounds where glyphs can
be - widened when a paragraph is shaped, narrowed when they are dropped - so both scans are
O(retention window). Headless afterwards the two sizes agree: +0.34 ms and +0.36 ms.

The lesson is about the harness, not the code: a cost that scales with the *document* rather than
with the *window* is invisible unless two document sizes are measured side by side, and the scene
list only had one until the GPU run put a 10 000-line row next to the 100 000-line one.

## What was tried and reverted

**Memoizing the per-paragraph font query.** `ranged_builder` calls `query_fontique` plus a skrifa
metrics read on *every* `build()`, i.e. once per paragraph, to pin the line height. Caching it in
the builder (one builder shapes a whole document) looked like a free win now that shaping sits on
the frame path. Measured: 1.87 s vs 1.86 s, frames unchanged. Reverted - fontique's own cache
already absorbs it, and the `Cell` was not worth carrying.

## Tooling added, because the existing counter stopped meaning anything

`TextLayoutCache::cache_miss_count` counts *entries* that had to be rebuilt. That is 21 for this
scene whether the document is shaped once or a hundred thousand times, so it can no longer answer
the acceptance question. `paragraph_shape_count` counts the parley layouts actually built and
`perf_test --bench` prints it as its own column, per frame and for `show + layout`. Every number in
`RESULT.md` about "how much shaping" comes from it.

`perf_test --bench --snapshot <file.ppm>` was added too: the windowed snapshot path needs
`libxkbcommon-x11`, which is not installed here, so before this there was no way to compare pixels
at all. The bench path renders into a buffer it already owns, so this is a dozen lines and needs
neither a display nor GL.

## What is deliberately unchanged

- **Styled text** keeps every paragraph's glyphs. A styled paragraph carries formatting spans and
  links, so it cannot be reproduced from a byte range the way a plain one can, and (separately) its
  `range` is `0..0` today. Not worth the complexity: the documents that hurt are plain.
- **Below 128 paragraphs** nothing is lazy. A short document's glyphs are cheap to keep, and keeping
  them means never re-shaping - which matters for the `list`/`scroll` scenes, where every row's
  `Text` is a one-paragraph document.
- **`max-lines` and `overflow: elide`** retain everything, so their cut across the whole document
  keeps working off per-line metrics exactly as before.

## Step 4 - `preferred-width`, still a design decision

Unchanged from the pre-implementation analysis, and now the only thing left: the preferred width is
the widest line, so every paragraph is shaped once, and that is the remaining 1.77 s. The evidence
for choosing:

- The time is intrinsic to parley shaping (candidate 005: 124 ns/char), not to how it is cached.
- Candidate 004 measured that skipping ICU line segmentation for `no-wrap` recovers 3% of it, so
  there is no upstream fix of the right size.
- Making it progressive (grow the horizontal scroll range as lines come into view, like real text
  editors) is the only thing that removes it, and it changes what `preferred-width` *means*.

That is an upstream conversation, not an implementation choice, which is why it stops here.

## Acceptance checks used

- `cargo test -p i-slint-core --all-features`, `cargo test -p slint`,
  `cargo test -p test-driver-rust -p test-driver-interpreter --features build-time`. **2 025
  tests, 0 failures**, including five new ones in `sharedparley/tests.rs` that build a
  `PlainParagraphSource` directly: a lazily shaped document must match an eagerly shaped one on
  every observable (metrics, height, preferred width, cursor rects, hit tests, selection geometry),
  the retention window must keep exactly what it claims across successive passes, a short document
  must keep every paragraph shaped, and so must a wrapped, a line-limited and an elided one.
- `perf_test --bench --mode text --count 100000 --frames 8`: `show + layout`, RSS, and the
  **paragraph shapes** column (the `text shapes` column no longer measures this). Run the same
  with `--word-wrap`, and both again with `--resize-step 40 --scroll-step 0`: scrolling and
  resizing stress different things, and only the resize run reaches the width-dependent path.
- Pixels: `perf_test --bench --mode <mode> --count N --frames F --scroll-step S --snapshot`
  compared against a pristine-core binary, first `3148815` bytes only (everything above the status
  line, which prints per-run timings - see `README.md`). Same-binary control run first.
