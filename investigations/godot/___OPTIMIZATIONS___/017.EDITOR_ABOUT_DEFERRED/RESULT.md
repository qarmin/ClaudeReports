# Defer `EditorAbout`'s construction until Help -> About Godot... is first opened

Candidate 016 root-caused `EditorAbout` as the single largest item (8.73MB, 44.3%) in candidate
010's own "`ExportTemplateManager`+`EditorFeatureProfileManager`+`EditorBuildProfileManager`+
`EditorAbout`" group, and found it has exactly **one** reference anywhere in the codebase
(`EditorNode::_menu_option_confirm()`'s `HELP_ABOUT` case) - no `get_singleton()`, no external
file touches it. This candidate implements the deferred-construction fix, matching the same
`_ensure_*()` pattern already used by candidates 007/013.

`editor/editor_node.cpp`/`.h`: adds `EditorNode::_ensure_about_dialog()`, which `memnew()`s
`EditorAbout` and adds it to `gui_base` only the first time it's needed, instead of unconditionally
in the constructor. Called from the `HELP_ABOUT` menu handler, the dialog's only call site.

## What was tried

Harness: `bin/godot.linuxbsd.editor.x86_64 --headless --editor --path
___OPTIMIZATIONS___/harness/empty_project --quit-after 1`, same as candidates 006/007/013. Peak
RSS via `/proc/<pid>/status`'s `VmHWM` (no `/usr/bin/time` in this container).

| Variant | Peak RSS (VmHWM) | notes |
|---|---|---|
| baseline (eager `EditorAbout` construction) | 731093.6 kB mean (730952/731388/730832/731092/731204, 5 runs, 0.08% spread) | |
| patched (deferred `EditorAbout` construction) | 716947.2 kB mean (717628/715588/717728/718128/715664, 5 runs, 0.35% spread) | |

Distributions don't overlap (baseline min 730832 > patched max 718128) - a real, clean separation.

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: peak RSS only** (this candidate removes a one-time constructor-phase
allocation of a dialog's contributor/donor/license list widgets, not a per-call hot path - no
separate timing benchmark was built, matching how candidates 006/007/013 measured this class of
fix).

Peak RSS drops by **13.81MB (-1.93%)**, from 731.1MB to 716.9MB mean (5 vs 5 runs, no overlap).
Larger than candidate 016's own 8.73MB `Memory::get_mem_usage()` estimate - same non-1:1
relationship between the engine's own allocator-wrapper counter and total process peak RSS already
noted in candidates 007/013 (allocator fragmentation/reuse, page-level effects).

## Does it change behavior/output?

- **Bit-exact?** N/A (this is a UI/interactive-behavior change, not a computation whose output is
  hashed) - see interactive verification below instead.
- **Interactive verification**: built a scratch test project (not part of `patch.diff` - throwaway
  test scaffolding, not committed anywhere) with a `@tool` `EditorPlugin` autoload that finds the
  Help menu's "About Godot..." item by text, emits its `id_pressed` signal (the same signal path a
  real click goes through - `PopupMenu::id_pressed` -> `EditorNode::_menu_option` ->
  `_menu_option_confirm`), then locates the resulting `EditorAbout` window in the scene tree and
  screenshots it. Ran under `Xvfb` (1280x800x24): **no crash, no script error**. The screenshot
  (`bench/screenshot_about_dialog.png`) confirms the dialog renders correctly - title "Thanks from
  the Godot community!", version string, all 4 tabs (Authors/Donors/License/Third-party Licenses),
  and the Authors tab's contributor list populated correctly.
- **Is this a breaking change?** No - `EditorAbout`'s public-facing behavior (what the user sees
  when opening Help -> About Godot...) is unchanged; only *when* the dialog's widgets are
  constructed changes (lazily on first open, instead of eagerly at editor startup).
- Project test suite: 1427 cases, 1426 passed, 1 failed (known pre-existing `test_ip.cpp` IPv6
  failure, no IPv6 in this container), 3 skipped - identical before and after.

## Why the result looks the way it does

Same repeated pattern this whole session (candidates 007/010/011/012/013/015/016): eager
construction of UI regardless of use. This is one of the cleanest instances found - a single,
unambiguous call site with no external singleton or cross-file dependency to audit, unlike the
`ProjectSettingsEditor` attempt (ruled out) or `EditorFeatureProfileManager` (flagged as
similarly risky in candidate 016, not attempted).

## What's not done / open questions

- **`ExportTemplateManager` (2.58MB) and `EditorBuildProfileManager` (4.10MB)**, the other two
  members of candidate 016's group with no external `get_singleton()` callers found, were not
  attempted this session - left in `IDEAS.md` as further leads with the same clean risk profile as
  this candidate.
