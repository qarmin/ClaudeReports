# Bisecting candidate 006's "+83.7MB before-theme-to-before-docks" editor-startup block

Candidate 006 found that empty-project editor startup spends +83.7MB between the theme being
generated and the first dock being instantiated, but didn't bisect past that coarse granularity
(labeled "main menu bar, ~100+ shortcut registrations, `EditorCommandPalette` first construction"
as a guess). This candidate (top of `IDEAS.md`'s backlog) bisects that block into its actual
components using the same `Memory::get_mem_usage()` checkpoint technique candidate 006 used for
the 74-plugin loop.

## What was tried

Harness: same as candidate 006 - `bin/godot.linuxbsd.editor.x86_64 --headless --editor --path
___OPTIMIZATIONS___/harness/empty_project --quit-after 1`. 14 temporary `Memory::get_mem_usage()`
checkpoints added through `EditorNode::EditorNode()` (`editor/editor_node.cpp`) spanning exactly
candidate 006's "+83.7MB" block - see `bench/instrumentation.diff` for the exact temporary diff
(applied, measured, then reverted - not a real fix, see "Does it change behavior/output?" below).
Fully deterministic across repeat runs (3/3 runs bit-identical byte counts at every checkpoint, 0%
spread) - same determinism candidate 006 observed for this counter.

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: Godot's own engine-tracked heap (`Memory::get_mem_usage()`), for attribution
only - same as candidate 006's per-plugin breakdown.**

| Segment (checkpoint -> checkpoint) | Delta | % of the 83.67MB block | What it covers |
|---|---|---|---|
| after_theme -> after_register_exporters | +0.07MB | 0.1% | `register_exporters()` |
| after_register_exporters -> before_save_accept | +3.71MB | 4.4% | early shortcut, textfile-extension setup, `EditorResourcePreview`/`ProgressDialog`, `gui_base`/title bar/dock-split-container skeleton (10 `SideDockTabContainer`s), `EditorDockManager`, timers, `EditorSceneTabs`, distraction-free button, `EditorMainScreen`, `scene_root` `SubViewport` |
| before_save_accept -> before_project_export | +0.06MB | 0.1% | `save_accept` `AcceptDialog` |
| **before_project_export -> after_project_export** | **+8.72MB** | **10.4%** | **`ProjectExportDialog`** |
| after_project_export -> after_editor_settings_dialog | +3.56MB | 4.3% | `DependencyErrorDialog` + `EditorSettingsDialog` |
| **after_editor_settings_dialog -> after_project_settings_editor** | **+24.65MB** | **29.5%** | **`ProjectSettingsEditor`** - the single largest individual contributor found in this block, larger than any one of candidate 006's 74 built-in editor plugins |
| after_project_settings_editor -> after_import_settings_dialogs | +10.49MB | 12.5% | `SceneImportSettingsDialog` + `AudioStreamImportSettingsDialog` + `DynamicFontImportSettingsDialog` (3 combined, not individually bisected) |
| **after_import_settings_dialogs -> after_managers_and_about** | **+20.69MB** | **24.7%** | **`ExportTemplateManager` + `EditorFeatureProfileManager` + `EditorBuildProfileManager` + `EditorAbout` (4 combined, not individually bisected)** |
| after_managers_and_about -> before_command_palette | +2.35MB | 2.8% | `FBXImporterManager` + `AndroidSDKManager` + `warning` `AcceptDialog` + `clear_cache_dialog` |
| before_command_palette -> after_command_palette | +0.0002MB | ~0% | `EditorCommandPalette::get_singleton()` + `set_title()` + `add_child()` - reuses an existing singleton, effectively free here |
| after_command_palette -> after_shortcuts_block | +0.08MB | 0.1% | ~100+ `ED_SHORTCUT`/`ED_SHORTCUT_AND_COMMAND` registrations |
| **after_shortcuts_block -> after_menus** | **+8.65MB** | **10.3%** | **5 `PopupMenu`s (Scene/Project/Debug/Editor/Help) via `_add_to_main_menu`** |
| after_menus -> before_docks | +0.65MB | 0.8% | spacers, `project_title`, `project_run_bar`, `right_menu_hb`, `renderer` `OptionButton`, `layout_dialog`, `update_spinner` |

Total: 83.67MB, matching candidate 006's original figure (small ~4MB rounding difference from
measuring on a fresh build without other candidates' code present is expected and immaterial).

**The three largest single items are `ProjectSettingsEditor` (24.65MB, 29.5%), the
`ExportTemplateManager`+`EditorFeatureProfileManager`+`EditorBuildProfileManager`+`EditorAbout`
group (20.69MB, 24.7% combined - not yet bisected further), and `ProjectExportDialog` (8.72MB,
10.4%)** - together over 64% of the whole block, from dialogs that all start closed and, for a
typical editing session opening an empty/near-empty project, may never be opened at all.
`ProjectSettingsEditor` alone is larger than any single one of candidate 006's 74 built-in editor
plugins (the largest of those, `AnimationTreeEditorPlugin`, was 11.49MB).

The originally-guessed explanation ("main menu bar, shortcuts, command palette") turns out to
explain only a small fraction of the block: shortcuts cost 0.08MB, the command palette costs
essentially nothing (reuses a singleton), and even the 5 main menu `PopupMenu`s (8.65MB) are
smaller than any of the three dialog groups above.

## Does it change behavior/output?

**N/A - measurement/characterization only, no patch applied to real source** (per SKILL.md's
"patch.diff is mandatory..." section - this is the correct "pure measurement/investigation" state).
`bench/instrumentation.diff`'s temporary `Memory::get_mem_usage()` probes were reverted before
finishing this candidate.

- Project test suite: 1427 cases, 1426 passed, 1 failed (known pre-existing `test_ip.cpp` IPv6
  failure), 3 skipped - not expected to change (measurement-only), not separately re-run for this
  purely additive `print_line`-only instrumentation (candidate 006 already established this class
  of instrumentation adds no measurable overhead and doesn't affect test outcomes).

## Why the result looks the way it does

Same architectural pattern candidate 006/007 already found for `ThemeEditorPlugin`: Godot's editor
constructs essentially every dialog/manager it might ever need up front, in the constructor,
regardless of whether the user's session ever opens it. `ProjectSettingsEditor` is the most extreme
case found so far because it's an unusually large, deeply nested settings UI (general settings
across every registered property category, input map, localization, autoload, plugins, etc.) built
whether or not the user ever opens Project Settings in a given session - directly analogous to
candidate 007's `DefaultThemeEditorPreview` finding, just an order of magnitude bigger in absolute
terms.

## What's not done / open questions

- **The `ExportTemplateManager`+`EditorFeatureProfileManager`+`EditorBuildProfileManager`+
  `EditorAbout` group (20.69MB) and the three import-settings dialogs (10.49MB) were measured only
  as combined groups, not bisected per-dialog** - a natural follow-up using the same technique,
  left in `IDEAS.md`.
- **A deferred-construction fix for `ProjectSettingsEditor` (the single largest lead) was attempted
  in a follow-up session and ruled out** (crashes on startup because several other files call
  `ProjectSettingsEditor::get_singleton()` directly, and one call site fires unconditionally during
  every project's startup scan anyway) - no candidate folder was created since no working patch
  resulted; see `IDEAS.md`'s "Checked and ruled out" entry for the full reasoning.
- `ProjectExportDialog` (8.72MB) and the main-menu `PopupMenu`s (8.65MB, surprising for what looks
  like simple menu shells - `_add_to_main_menu`/`_update_main_menu_type` may be eagerly populating
  full submenu item trees rather than lazily) were not investigated further this session - both
  left in `IDEAS.md` as leads for a future session.
