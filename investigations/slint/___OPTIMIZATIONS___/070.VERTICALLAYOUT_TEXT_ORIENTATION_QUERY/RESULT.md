# 70 - `VerticalLayout` of fixed-width `NoWrap` `Text`s only ever queries `Orientation::Vertical`

`IDEAS.md` (entry dated 2026-08-24, noticed while building candidate 069) asked whether a
`VerticalLayout` of fixed-width `Text` children is symmetric to candidate 044's `HorizontalLayout`
finding - and, if `Orientation::Horizontal` is genuinely never queried on such a `Text`, whether the
NoWrap vertical-only fast path candidate 069 flagged (font-metrics-derived height, no shaping) could
actually be built and would be safe.

## How the premise was confirmed

Added a temporary pair of atomic counters to `Text::layout_info` (`internal/core/items/text.rs`)
and a standalone probe binary (`perf_test/src/bin/orientation_probe.rs`, not part of the final
patch - a copy is in this folder's `probe/`) that renders a `VerticalLayout` of three fixed-width,
`NoWrap` `Text`s and a `HorizontalLayout` mirror of the same shape, then prints how many times each
orientation was queried:

```
VerticalLayout, fixed-width NoWrap Text x3:   H=0 V=3
HorizontalLayout, fixed-width NoWrap Text x3: H=3 V=0
```

Confirmed: a `VerticalLayout`'s fixed-width `NoWrap` `Text` children never have `Horizontal`
queried at all, symmetric to candidate 044's `HorizontalLayout` finding for `Vertical`. This makes
the NoWrap vertical-only fast path candidate 069 identified as "the one surviving case" real, not
hypothetical.

## Resolving the fast path's correctness risk (the part 069 left open)

069 explicitly did not resolve "the correctness risk of a parley-independent height computation".
Extended the same probe to compare, for many styling variations, the real shaped `NoWrap` height
(`text_size(..).height`, what `Text::layout_info` currently returns) against a font-metrics-only
estimate (`renderer().font_metrics(font_request).{ascent, descent}`, adjusted by
`FontRequest::line_height_for_natural_height` the same way `char_size`'s ellipsis-width computation
already does):

| text variant | shaped height | estimated height | diff |
|---|---|---|---|
| plain, default font (x3) | 13.96875 | 13.96875 | 0 |
| `font-size: 6px` | 6.984375 | 6.984375 | 0 |
| `font-size: 96px` | 111.75 | 111.75 | 0 |
| `font-weight: 700` | 13.96875 | 13.96875 | 0 |
| `font-italic: true` | 13.96875 | 13.96875 | 0 |
| `letter-spacing: 3px` | 13.96875 | 13.96875 | 0 |
| `font-family: "serif"` | 13.96875 | 13.96875 | 0 |
| `font-family: "monospace"` | 13.96875 | 13.96875 | 0 |
| mixed-case/Unicode/CJK | 13.96875 | 13.96875 | 0 |
| emoji | 13.96875 | 13.96875 | 0 |
| a 5px-wide box, long single line | 13.96875 | 13.96875 | 0 |
| **embedded `\n`, 3 lines** | **41.90625** | **13.96875** | **27.9375** |

Every plain, single-line case matched bit-for-bit. The one real gap: `NoWrap` suppresses
word-wrapping, not hard breaks - `text.split('\n')` in `shaping.rs` still starts a new paragraph per
`\n`, so a font-metrics-only single-line estimate is wrong by exactly `(lines - 1) * line_height` for
any text containing an explicit line break. The empty string and a `PlainOrStyledText::Styled` value
(rich text, which can mix font sizes on one line) are excluded from the fast path for the same
reason (mixed metrics on one line isn't derivable from the item's single default font).

## The fix

`internal/core/items/text.rs`'s `text_layout_info`, `Orientation::Vertical` /
`TextWrap::NoWrap` arm: for `PlainOrStyledText::Plain` text with no `\n`, compute the height from
`font_metrics()` instead of calling `text_size()` (which shapes the whole paragraph through
parley). Falls back to the original shaping call for `Styled` text or any text containing `\n`.

## What was tried

Bench: `perf_test`'s `scroll` mode (non-virtualized `for`, every row a live element, so every row's
`layout_info` is queried during the initial layout pass regardless of visibility - same mechanism
candidate 044 used). Added `RowStyle.vertical-fixed`: a new `VerticalFixedLayoutRecordRow`, the
transpose of the existing `FixedLayoutRecordRow` (`VerticalLayout` of 3 fixed-width `NoWrap` `Text`s
stacked top to bottom instead of side by side). `--mode scroll --row-style vertical-fixed`.

| Variant | first frame (single-core, `taskset -c 2`) | first frame (multi-core, unpinned) | notes |
|---|---|---|---|
| baseline, `--count 20000` | 539.64 ms median (9/9 paired) | 544.99 ms median (9/9 paired) | 60 000 text shapes (3 x every row) |
| patched, `--count 20000` | 96.52 ms median | 97.52 ms median | 80 text shapes (visible rows only) |
| baseline, `--count 100000` | 2.79 s | (not re-measured multi-core; scales the same) | 300 000 text shapes, 1553.5 MB RSS |
| patched, `--count 100000` | 495.49 ms | | 80 text shapes, 478.8 MB RSS |

`later frames` (steady-state scrolling, `--count 20000`, `tools/paired.sh`, `RUNS=9`): baseline
29.31 ms median vs. patched 27.40 ms median, 7/9 paired - not decisive either way, as expected: the
fix only removes work for rows that are laid out but never rendered, and steady-state scrolling
renders (and therefore always fully shapes) whatever just scrolled into view either way.

## Result

**-82.1% first-frame time at `--count 20000`** (539.64 ms -> 96.52 ms), 9/9 paired both single- and
multi-core, reproduced at `--count 100000` (2.79 s -> 495 ms, same ~82% delta, confirming the
removed cost scales with row count as expected). Allocations -65% (9 292 281 -> 3 203 793 at
100 000 rows), resident memory -69% (1553.5 MB -> 478.8 MB). Steady-state per-frame cost after the
first frame is unaffected (within noise, 7/9 paired - not decisive).

## Does it change behavior/output?

- **Bit-exact?** Yes for the tested scenes. `tools/pixelcheck.sh`, `RUNS=6`: `scroll
  --row-style vertical-fixed` (0 px outside a 187 px same-binary noise floor), `scroll
  --row-style layout` (0 px outside 125 px), `scroll --row-style flat` (0 px outside 127 px),
  `text --word-wrap` (0 px outside 608 px), `text` no-wrap (0 px outside 516 px), `list` (0 px
  outside 125 px) - none of the pre-existing scenes changed. A dedicated standalone probe
  (`probe/orientation_probe.rs`, pixel FNV-1a hash of a window rendering every styling variation
  above plus the `\n` case in one scene) produced the identical hash (`dbbb1b3587635382`) built
  against baseline and patched `i-slint-core`.
- **Is this a breaking change?** No public API or file format changes. The only externally
  observable effect is that `font_metrics()` (a `RendererSealed` trait method every custom
  `Renderer` implementor must provide, previously reachable only from the public
  `Text.font-metrics` accessor apps call explicitly) is now also called from inside the engine's
  own layout path for `NoWrap` text. A third-party `Renderer` implementation whose `font_metrics()`
  is less rigorously correct than its `text_size()` (plausible, since it was previously only an
  app-facing convenience, never load-bearing for layout) would now produce a wrong row height for
  plain `NoWrap` text under that renderer - not testable from this repo alone; all three built-in
  renderers (software, femtovg, skia) share the `shared-parley` default implementation of
  `font_metrics()` and were confirmed correct via the probe.
- Project test suite: `cargo test -p i-slint-core --all-features` (319 + 71 + 1 passed, unchanged),
  `cargo test -p slint` (unchanged), `cargo test -p test-driver-interpreter` (858 passed,
  unchanged - covers every `tests/cases/layout/*.slint` fixture, including several combining
  `VerticalLayout`/`HorizontalLayout` with `Text`).

## Why the result looks the way it does

Exactly candidate 044's mechanism, transposed: a `VerticalLayout` unconditionally asks every child
for `Orientation::Vertical` layout-info regardless of whether the answer changes anything (no
stretch, no cross-axis alignment needed for the fixed-width case here) - and a `Text`'s
`Orientation::Vertical` branch under `NoWrap` used to always call the same shaping-backed
`text_size()` the `Horizontal` branch uses, even though (per the probe) `Horizontal` is never
queried at all in this shape of layout. The height that shaping computes for a single line is
provably (see the diff table above) just the font's line height for anything without an explicit
line break, so the shape was pure, avoidable cost for every off-screen row.

## What's not done / open questions

- Not extended to `TextInput`/`TextEdit` (`ComplexText`/`SimpleText`/`StyledTextItem`'s shared
  `text_layout_info` is the only thing patched; `TextInputItem` has its own, separate
  `layout_info`, out of scope here).
- The `\n`-detection guard (`s.contains('\n')`) is an `O(text length)` scan on every
  `Orientation::Vertical` query, cheap next to shaping but not free - not measured separately since
  it's dwarfed by the win, but a pathologically long single-paragraph `NoWrap` `Text` re-queried
  very frequently without ever rendering would pay it repeatedly instead of hitting a cache (the
  old code populated `TextLayoutCache` on the first call; this fast path does not populate any
  cache, so a later `Horizontal` query or an actual render always re-shapes from scratch, same as
  baseline - no regression, just no cache pre-warming from the `Vertical` side either).
- Real-hardware GPU-backed renderer confirmation (femtovg/skia via `--frames`) not done this
  session - only the headless `--bench` software-renderer path was measured, per this workspace's
  usual practice for layout/text-shaping findings (the win is in text shaping, not rendering).
