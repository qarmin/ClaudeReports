# shift_tab_trailing_spaces_cursor_jump

`TextBuffer::decrease_indentation` (used by Shift+Tab in a multiline `TextEdit`) at `crates/egui/src/widgets/text_edit/text_buffer.rs:96`. Looked suspicious because it assumes `TAB_SIZE` chars exist when it subtracts the removed length from the cursor (HEURISTICS: length/count assumed rather than measured).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | char/byte index helpers in text_selection and text_buffer | `chars().skip(n).take(4).all(' ')` is vacuously true for 0-3 remaining chars |
| Direct API | `String::decrease_indentation` with text "x\n  ", cursor 4, debug build | cursor ends at 0 |
| End to end | real `TextEdit::multiline(..).lock_focus(true)` run through `Context::run_ui` with a real `Shift+Tab` key event | text correct, caret jumps to 0 |

## Status

**Confirmed** - reproduces 3/3 runs, debug build (deterministic logic bug, no randomness).

**Fully reproducible**: **Yes** (real `TextEdit` driven by a real key event through `Context`).

## Class

logic

## Root cause

When the caret line is indented with 1-3 spaces and sits at the very end of the buffer, `take(TAB_SIZE).all(|c| c == ' ')` holds (fewer than 4 chars left, vacuously all spaces) so `remove_len = Some(TAB_SIZE)`. `delete_char_range` clamps to the real length and removes only 1-3 chars, but `*ccursor -= len` subtracts the full 4 (saturating), so the caret lands 1-3 chars too far left, in the example at 0 instead of 2, i.e. the start of the document. An empty last line is not affected (`ccursor == line_start`).

## Introduced by

f034f6db9 "Refactor: move text selection logic to own module (#3843)" is the oldest commit `git log -S` finds for this expression in the current location; earlier history was not followed (not determined further, not worth the time).

## The fix

Count the chars actually available in the indent window and remove/subtract exactly that many (`patch.diff`, +4/-9). `repro/` after the patch: caret at 2 (both the trait call and the TextEdit path).

- Project test suite (`cargo test -p egui --lib`): 86 passed / 0 failed before -> 86 passed / 0 failed after.

## What's not done / open questions

- Input variations with 1, 2 and 3 trailing spaces share the single code path; only the 2-space case was run.
- Other `TextBuffer` default methods (`delete_next_char` etc.) clamp through `byte_index_from_char_index` and were not found wrong.
