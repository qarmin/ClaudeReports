# `Tree`'s inline cell-editor UI costs 8.20MB across the 113 Trees built at editor startup

Candidate 027's census flagged `Tree` as the third-largest per-instance widget (113 instances,
11.02MB, mean 99.8KB) and left "is a meaningful part of that optional?" open. It is:
`Tree::Tree()` unconditionally builds a complete **inline cell-editing UI** - a `Popup`, a
`VBoxContainer`, a `LineEdit` and a `TextEdit` - which is only ever shown when a user actually
edits a tree cell in place. Most Trees in the editor (and in games) never see one.

## What was tried

Harness: `bin/godot.linuxbsd.editor.x86_64 --headless --editor --path
___OPTIMIZATIONS___/harness/empty_project --quit-after 1`. Four `Memory::get_mem_usage()`
checkpoints through `Tree::Tree()` (`scene/gui/tree.cpp`) bracketing the cell-editor cluster and
the scrollbars - see `bench/instrumentation.diff` (applied, measured, reverted). Because `Tree` is
instantiated 113 times, the per-instance deltas were aggregated across every instance rather than
read off one.

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: Godot's own engine-tracked heap (`Memory::get_mem_usage()`), aggregated over
all 113 instances.**

| Segment of `Tree::Tree()` | Total across 113 instances | Mean per instance | Share of `Tree`'s 11.02MB |
|---|---|---|---|
| **cell-editor cluster** (`popup_editor` `Popup` + `popup_editor_vb` `VBoxContainer` + `line_editor` `LineEdit` + `text_editor` `TextEdit`) | **8.20MB** | **74.3KB** | **74%** |
| scrollbars (`HScrollBar` + `VScrollBar`) | 1.37MB | 12.4KB | 12% |
| everything else (canvas items, timers, theme setup) | ~1.45MB | ~13KB | ~13% |

So nearly three quarters of every `Tree`'s construction cost is UI for a feature - inline cell
editing - that a given `Tree` may well never use. `text_editor` alone is a full `TextEdit`
(~28.9KB mean per candidate 027's census).

## Does it change behavior/output?

**N/A - measurement/characterization only, no patch applied to real source** (per SKILL.md's
"patch.diff is mandatory..." section). Instrumentation reverted before finishing this candidate.

- Project test suite: not re-run for this purely additive `print_line` instrumentation - no changes
  to real source survived this candidate.

## Why a fix was NOT attempted (and what a future session needs to solve first)

Deliberately ruled out for this session on audit-surface grounds, the same judgment applied to
`ProjectSettingsEditor` and `SpriteFramesEditor` earlier - recorded here so the next session starts
from the blockers rather than rediscovering them:

- **~72 references across 14 functions** in `tree.cpp` touch the four members (`popup_editor` 32,
  `line_editor` 18, `text_editor` 15, `popup_editor_vb` 7): `edit_selected`, `is_editing`,
  `_apply_multiline_edit`, `_line_editor_submit`, `_text_editor_gui_input`,
  `_text_editor_popup_modal_close`, `_update_value_editor`, `value_editor_changed`,
  `set_editor_selection`, `_accessibility_action_edit_custom`, `select_single_item`, `_scroll_moved`,
  `_bind_methods` and `_notification`. (The apparent "external" hits in other files are those
  classes' own identically-named members, not `Tree`'s - checked.)
- **Two `_notification()` sites are the real blockers**, and one of them defeats the deferral
  outright rather than merely needing a guard:
  - `popup_editor->hide()` - needs a null guard, trivial.
  - `content_scale_factor = popup_editor->is_embedded() ? 1.0 : popup_editor->get_parent_visible_window()->get_content_scale_factor();` - this runs during ordinary
    notification handling at startup, so leaving it as-is would **force every `Tree` to build its
    cell editor anyway**, making the deferral worth nothing (candidate 015's null-result shape). It
    would have to be rewritten to derive the scale factor from the `Tree` itself rather than from
    the popup.
  - A third site (`popup_edited_item != nullptr && popup_editor->is_visible()`) is already safe by
    short-circuit, since `popup_edited_item` is only non-null while editing.
- **`Tree` is public API used by games**, and `tests/scene/test_tree.cpp` contains exactly **one**
  `TEST_CASE`, so the existing suite provides almost no safety net - verification would have to be
  built from scratch (interactive inline-edit of a text cell, a numeric/range cell, and a
  multiline cell, plus the accessibility edit action).

The prize (8.20MB) is real and the pattern is idiomatic in this codebase (`LineEdit`/`TextEdit`
already defer their own context menus - see candidate 027), so this is a genuinely good lead; it
just needs more verification budget than remained in this session.

## What's not done / open questions

- The fix itself - see above. Left in `IDEAS.md` with these blockers spelled out.
- Not measured: how many of the 113 Trees ever actually show their cell editor in a real session
  (i.e. how much of the 8.20MB a *typical* user would still pay after a deferral). Given the editor
  uses `Tree` for the scene tree, inspector sub-properties, file lists, signal/group lists and
  more, and inline editing is only reachable in some of them, the expectation is "most of it" - but
  that is an assumption, not a measurement.
