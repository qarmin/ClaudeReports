# galley_cursor_end_of_row_panics_with_zero_rows

`Galley::cursor_end_of_row` and `Galley::cursor_end_of_paragraph` in `crates/epaint/src/text/text_layout_types.rs` (~1419, ~1452). Looked suspicious because the sibling `Galley::end()` and `layout_from_cursor` explicitly handle `rows.is_empty()`, but these two index `self.rows[..]` directly (HEURISTICS: unchecked indexing next to code that knows the container can be empty).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Probe of all cursor methods on `Galley` | text "", "abc", "a\nb" with `LayoutJob::wrap.max_rows` = usize::MAX and 0, debug build | `max_rows = 0` yields a galley with 0 rows; `cursor_end_of_row` and `cursor_end_of_paragraph` panic (index out of bounds), all other cursor methods are fine |
| End to end | real `TextEdit::multiline` with a custom `layouter` setting `max_rows = 0`, real `Key::End` event through `Context::run_ui` | panics at `text_layout_types.rs:1423` (`rows[0]`), same input with `max_rows = 1` is fine |

## Status

**Confirmed** - reproduces 3/3 runs, debug build (deterministic).

**Fully reproducible**: **Yes** (real `TextEdit` driven by a real key event; the trigger is a layouter that returns a zero-row galley).

## Class

crash

## Root cause

A `LayoutJob` with `wrap.max_rows == 0` produces a `Galley` with no rows. `layout_from_cursor` then returns `LayoutCursor::default()` (row 0), and `cursor_end_of_row`/`cursor_end_of_paragraph` index `self.rows[0]` unconditionally, panicking. `TextEdit` calls them for the End key (and Ctrl+Shift+K style paragraph actions), so a layouter that happens to compute `max_rows = 0` crashes the app on End.

## Introduced by

267485976 "Simplify the text cursor API (#5785)" is the oldest commit `git log -S` finds for the unchecked `rows[layout_cursor.row]` indexing; the zero-row galley itself is older behaviour (not determined further).

## The fix

Return `self.end()` (the same default cursor `end()` already gives for an empty galley) when the row lookup fails (`patch.diff`, +7/-2). After the patch `repro/` finishes without a panic and the probe reports 0 panics.

- Project test suite (`cargo test -p epaint -p egui --lib --features epaint/default_fonts`): 86 + 95 passed / 0 failed before -> same after. (epaint's lib tests do not compile without the `default_fonts` feature, pre-existing.)

## What's not done / open questions

- Other callers that index `galley.rows[0]`/`rows.last().unwrap()` for zero-row galleys were not audited.
- `max_rows = 0` may be considered invalid input by maintainers; the alternative is to make the layout always emit one row.
