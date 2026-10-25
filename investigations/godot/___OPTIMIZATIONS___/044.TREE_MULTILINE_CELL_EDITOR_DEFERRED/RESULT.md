# 044. `Tree`'s multiline cell editor (a `TextEdit`) built in every Tree, deferred to first use

- **Project**: godotengine/godot
- **Base commit**: `9552dfb6859a1aaba1e570b8e0ef5c599b830f19`
- **Files**: `scene/gui/tree.cpp`, `scene/gui/tree.h` (**+26/-13 lines**)
- **Metric(s) measured**: Godot's tracked peak heap (primary) and peak RSS (secondary)
- **Workload**: empty-project editor startup,
  `--headless --editor --path harness/empty_project --quit-after 1`

## What was tried

Candidate 028 measured `Tree`'s inline cell-editor cluster (`popup_editor` `Popup` +
`popup_editor_vb` + `line_editor` `LineEdit` + `text_editor` `TextEdit`) at **8.20MB across the 113
Trees built at empty-project startup, 74% of Tree's entire construction cost**, and ruled it out for
that session on three blockers: `NOTIFICATION_DRAW` computes `content_scale_factor` from
`popup_editor->is_embedded()`/`get_parent_visible_window()` on **every draw** (so deferring the
Popup is a guaranteed null result unless that is rewritten), a `popup_editor->hide()` needing a null
guard, and ~72 references across 14 functions on a public API class with exactly 1 `TEST_CASE`.

Rather than attempt that rewrite, this candidate **bisected the cluster first** - and the blockers
turn out to guard the cheap part:

| member | per instance | total across 113 |
|---|---|---|
| `popup_editor` (`Popup`) | 11843 B | 1.28 MB |
| `popup_editor_vb` (`VBoxContainer`) | 6026 B | 0.65 MB |
| `line_editor` (`LineEdit`) | 12505 B | 1.35 MB |
| **`text_editor` (`TextEdit`)** | **45620 B** | **4.92 MB (60% of the cluster)** |

`text_editor` is only used by **multiline** string cells (`TreeItem::set_edit_multiline`), and every
one of its 5 non-constructor reference sites is inside that editing flow. So deferring just it gets
60% of candidate 028's prize while **never touching `popup_editor`**, which is what all three of
that candidate's blockers were about.

The patch adds `_ensure_text_editor()` (construct, configure, parent into `popup_editor_vb`, connect
`gui_input`), calls it from `edit_selected()`'s multiline branch, null-guards the two sites that only
*hide* or *select in* an editor that may not exist yet, and leaves `_text_editor_gui_input()` and
`_apply_multiline_edit()` untouched - those are reachable only once the editor exists (the former is
the editor's own signal; the latter is guarded by `is_edit_multiline(...)` at both call sites).

## Result

**Tracked peak heap: -5585240 bytes (-5.33 MB, -1.25%)**, 448282957 -> 442697717 B, byte-identical
across 3 runs per arm.
Tracked live heap at `Main::cleanup()`: -529928 B (-0.51 MB).

**Peak RSS: -6.64 MB (-0.95%)**, 702.47 -> 695.83 MB mean, 6 interleaved pairs, patched lower in
**6/6** with non-overlapping distributions (baseline 718424-720152 kB, patched 711772-713176 kB).
RSS/tracked ratio 1.25x, in line with the workspace's usual 1.23-1.32x, so the saving reaches the
process.

The measured tracked-peak saving (5.33MB) exceeds the bisection's prediction for `text_editor`
(4.92MB), consistent with candidate 035's finding that `get_mem_usage()` checkpoint deltas
undercount transient peaks.

## Does it change behavior/output?

Verified with a scratch `TEST_CASE` run against **both** arms
(`bench/scratch_verification_test.cpp`), which drives the real bound public API rather than
inspecting internals: it builds a `Tree` with a multiline-editable string cell and calls
`Tree::edit_selected(true)`.

| | baseline | patched |
|---|---|---|
| `TextEdit` instances before editing | **1** | **0** |
| `LineEdit` instances before editing | 1 | 1 |
| `edit_selected(true)` returned | true | true |
| `is_editing()` | true | true |
| `TextEdit` instances after editing | 1 | 1 |
| editor visible, text | true, `hello\nworld` | true, `hello\nworld` |
| assertions | 4/4 pass | 4/4 pass |

The baseline arm is what makes this a controlled result rather than a self-confirming one: it
reports 1 instance before the edit where the patched arm reports 0, and both converge on a visible
editor holding the cell's text once the real edit path runs.

**Full test suite, both arms** (scratch case excluded via `--test-case-exclude`): 1427 cases, 1426
passed, 1 failed, 3-4 skipped; 424066 assertions, 423066 passed, 1000 failed - the known
`test_ip.cpp` IPv6 failure.

Two real differences, neither user-visible in the tested flows:

1. A `Tree` reports one fewer internal child until a multiline cell is first edited, observable to
   anything walking the node tree with `get_children(true)` (the same caveat candidates 023 and 037
   documented).
2. `set_editor_selection()` (public C++, not script-bound) is now a no-op for the multiline case
   when no multiline edit has ever been opened, where before it would have selected inside a hidden
   editor. Both are no-ops from the user's point of view, but the code path differs.

## Why the result looks the way it does

A `TextEdit` is a full text-editing control - its own caret/selection state, scrollbars, minimap
plumbing and text-server buffers - so at ~45.6KB it costs about as much as the other three members of
the cluster combined. Multiline cells are rare in the editor's own Trees (the Inspector, the scene
tree, the filesystem dock and the debugger panels all use single-line or non-editable cells), so
almost all 113 instances were paying for an editor that would never be shown.

This is also a useful correction to candidate 028's framing: it treated the cluster as a single
8.20MB unit gated behind the `content_scale_factor` problem, when in fact 60% of it sits behind no
blocker at all. Bisecting a cluster before accepting its blockers is the transferable lesson.

## What's not done / open questions

- **The remaining 3.27MB of the cluster** (`popup_editor` 1.28MB + `popup_editor_vb` 0.65MB +
  `line_editor` 1.35MB) still requires candidate 028's `content_scale_factor` rewrite, since
  `NOTIFICATION_DRAW` touches `popup_editor` on every draw. Recorded in `IDEAS.md` with the concrete
  equivalent expression to derive it from the `Tree` instead (`get_viewport()`'s embedding state and
  the walk `Window::get_parent_visible_window()` performs), plus the warning that it feeds
  sub-pixel line alignment, so getting it subtly wrong is a visual regression the test suite would
  not catch.
- **Not verified interactively under Xvfb.** The scratch `TEST_CASE` drives the real
  `edit_selected()` path headlessly, which is stronger evidence than a screenshot for this
  particular flow, but a human clicking into a multiline cell was not part of this.
- **Only the editor's own Trees were counted.** A game using `Tree` with many multiline-editable
  cells pays the deferred cost on first edit instead, which is the intended trade but is unmeasured.
