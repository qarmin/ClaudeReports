# Defer `ExportTemplateManager`'s construction until first actually needed

Candidate 016 found `ExportTemplateManager` (2.58MB) has no external `get_singleton()` callers.
Unlike `EditorAbout`/`EditorBuildProfileManager` (each one call site), `ExportTemplateManager` has
14 internal call sites across 9 functions in `editor_node.cpp` (Android-build-template install/
remove/validate flows, the `EDITOR_MANAGE_EXPORT_TEMPLATES` menu handler, and a deferred
command-line `--export` path in `_fs_changed()`). All 14 were audited this session and confirmed to
only fire from genuine user-initiated actions (menu items, dialog button presses) or an explicit
command-line export request (`export_defer.preset` is only set by `--export-*` CLI flags) - none
fire automatically during ordinary editor use or at startup.

`editor/editor_node.cpp`/`.h`: adds `EditorNode::_get_export_template_manager()`, a lazy-construct
accessor (construct-if-null-then-return, in one place), and routes all 14 call sites through it
instead of the raw `export_template_manager` field - centralizing construction behind a single
accessor reduces the risk of missing a call site, compared to adding an `_ensure_*()` call before
each one individually.

## What was tried

Harness: `bin/godot.linuxbsd.editor.x86_64 --headless --editor --path
___OPTIMIZATIONS___/harness/empty_project --quit-after 1`, same as candidates 006/007/013/017/018.
Peak RSS via `/proc/<pid>/status`'s `VmHWM` (no `/usr/bin/time` in this container).

| Variant | Peak RSS (VmHWM) | notes |
|---|---|---|
| baseline (eager construction) | 730636.8 kB mean (731060/730920/730128/731116/729960, 5 runs, 0.16% spread) | |
| patched (deferred construction) | 726850.4 kB mean (726732/727428/726676/726676/726740, 5 runs, 0.10% spread) | |

Distributions don't overlap (baseline min 729960 > patched max 727428) - a real, clean separation.

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: peak RSS only** (constructor-phase allocation removal, not a per-call hot
path - matching how candidates 007/013/017/018 measured this class of fix).

Peak RSS drops by **3.70MB (-0.52%)**, from 730.6MB to 726.9MB mean (5 vs 5 runs, no overlap). Close
to candidate 016's own 2.58MB `Memory::get_mem_usage()` estimate, same direction/rough-magnitude
relationship as prior candidates in this series.

## Does it change behavior/output?

- **Bit-exact?** N/A (UI/interactive-behavior change) - see interactive verification below.
- **Interactive verification**: built a scratch test project (not part of `patch.diff`) with a
  `@tool` `EditorPlugin` autoload that finds the Editor menu's "Manage Export Templates..." item
  and emits its `id_pressed` signal (the same signal path a real click goes through -
  `PopupMenu::id_pressed` -> `EditorNode::_menu_option` -> `_menu_option_confirm` ->
  `open_export_template_manager()`). Ran under `Xvfb` (1280x800x24): **no crash, no script error**.
  The resulting `ExportTemplateManager` window was found in the scene tree, visible, titled "Export
  Template Manager" - screenshot (`bench/screenshot_export_templates.png`) confirms the full
  template-management UI (available/installed templates tree, download controls) renders
  correctly. The Android-build-template call sites (the other 13 of the 14) were not separately
  interactively exercised - reasoned through via code review (all gated behind explicit user
  actions in dialogs/menu handlers already covered by the same `_get_export_template_manager()`
  accessor, so the same construction guarantee applies uniformly) rather than clicked through one
  by one, given the accessor pattern makes them mechanically identical to the one path tested.
- **Is this a breaking change?** No - only *when* the dialog's widgets are constructed changes.
- Project test suite: 1427 cases, 1426 passed, 1 failed (known pre-existing `test_ip.cpp` IPv6
  failure, no IPv6 in this container), 3 skipped - identical before and after.

## Why the result looks the way it does

Same repeated pattern this session, applied to a dialog with more (but still all safely-gated)
call sites than the single-call-site cases (`EditorAbout`, `EditorBuildProfileManager`) - routing
through one lazy-construct accessor instead of scattering `_ensure_*()` calls kept the fix
mechanically simple despite the larger number of touch points.

## What's not done / open questions

- The 13 Android-build-template call sites were audited by reading, not by individually clicking
  through an actual Android export setup flow (would need the Android SDK/export templates
  configured in this container, not attempted) - the accessor-centralization approach means this
  is lower-risk than it would be with scattered manual `_ensure_*()` calls, but a full interactive
  pass through the Android build-template install/remove/validate flow would be the natural next
  check for a future session with Android export tooling available.
- This closes out candidate 016's whole `ExportTemplateManager`+`EditorFeatureProfileManager`+
  `EditorBuildProfileManager`+`EditorAbout` group - only `EditorFeatureProfileManager` (4.32MB)
  remains unfixed, ruled out as too risky per candidate 016's own findings.
