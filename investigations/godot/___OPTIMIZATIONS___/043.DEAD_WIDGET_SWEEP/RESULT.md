# 043. Sweep for other "built but never shown" widgets: `file_templates` was the only one

- **Project**: godotengine/godot
- **Base commit**: `9552dfb6859a1aaba1e570b8e0ef5c599b830f19`
- **Metric(s) measured**: none - this is a static audit of reachability, not a measurement
- **No `patch.diff`**: audit only, no patch applied to real source. Nothing was found to fix.

## What was tried

Candidate 042 removed `EditorNode::file_templates`, a dialog built at every startup that no code path
could ever show, for -2.50MB peak RSS from 11 deleted lines and **zero** behavioral risk - the best
risk/reward in this workspace, because dead code cannot break a flow the way a deferral can. It was
found incidentally, so `IDEAS.md` recorded the obvious follow-up: is it the only one?

This candidate answers that. All `member = memnew(<DialogClass>)` assignments in `editor/**.cpp` and
`scene/gui/**.cpp` were enumerated (240 distinct member/class pairs across
`EditorFileDialog`/`ConfirmationDialog`/`AcceptDialog`/`FileDialog`/`EditorHelpSearch`/`PopupMenu`/
`PopupPanel`), and each member was then searched repo-wide for anything that could make it visible.

## Result

**`file_templates` is the only genuinely unreachable one.** 40 members had no direct
`member->popup*()`/`show()`/`set_visible()` call, but 39 of those are reachable through indirection
that a naive grep cannot see. Full listing in `bench/sweep_output.txt`; the three patterns are:

1. **`PopupMenu` attached as a submenu or to a `MenuBar`** (32 of the 39) - shown by its parent menu,
   never by a call on itself: `file_menu`, `debug_menu`, `gizmos_menu`, `display_submenu`,
   `menu_ctl`/`menu_dir` (LineEdit's own context submenus), and so on.
2. **Popped via a callable bound to the dialog** (4) - the dialog is the receiver inside a bind
   rather than the subject of a call: `callable_mp(autoload_file_dialog,
   &FileDialog::popup_file_dialog)` on a browse button, and
   `callable_mp(this, &...::_popup_file_dialog).bind(dialog)` for the two SSH key dialogs and
   `tpz_selection_dialog`. Candidate 031 hit this same shape as a deferral blocker.
3. **Popped by passing the dialog as an argument to a helper** (3) -
   `EditorInterface::popup_dialog_centered*(dialog, ...)` for `save_accept`, `load_error_dialog` and
   `execute_output_dialog`.

So the dead-widget class is **bounded at one instance**, and it is already fixed. A future session
should not re-run this sweep without handling patterns 1-3, or it will re-derive the same 39 false
positives.

## Does it change behavior/output?

N/A - nothing was patched, nothing measured. No test-suite comparison applies.

## Why the result looks the way it does

Godot's editor wires most dialogs to buttons and menu items through `callable_mp(...).bind(dialog)`
or via `EditorInterface`'s popup helpers, so the dialog often never appears as the receiver of a
popup call at all. That is precisely why `file_templates` was invisible to normal code review for so
long (it was orphaned by commit b5a06cecff): a reader scanning for `file_templates->popup` finds
nothing, which looks identical to these 39 legitimate cases.

## What's not done / open questions

- **A new lead came out of pattern 3**, recorded in `IDEAS.md`: `save_accept`, `warning`,
  `load_error_dialog`, `execute_output_dialog` and `progress_dialog` in `EditorNode` are eagerly
  constructed *and* carry `set_unparent_when_invisible(true)`, which removes them from the tree when
  hidden but does **not** free them (`Window::_notification`'s `NOTIFICATION_VISIBILITY_CHANGED` only
  calls `p->remove_child(this)`; the actual `queue_free()` happens at `NOTIFICATION_EXIT_TREE`). They
  are therefore paid for at startup and kept for the whole session. Their use sites already read
  `if (dialog) { ... }`, which makes deferral look trivial but is a trap: a null pointer would make
  the guard silently skip the popup rather than build it, so each guard must become an accessor call.
- Only `editor/` and `scene/gui/` were swept; `modules/` (e.g. the GridMap and GDScript editors) was
  not.
- Only dialog-ish classes were swept. The same "built but unreachable" question for plain `Control`
  subtrees (a `VBoxContainer` of widgets that nothing ever shows) is much harder to answer statically
  and was not attempted.
