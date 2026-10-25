# Defer `DependencyEditor`'s and `SceneImportSettingsDialog`'s `EditorFileDialog`s - first candidate measured with the deterministic counter

Continues the per-call-site work of candidates 024/025/026/031 on two more owners named by candidate
032's attribution, and is the **first candidate in this workspace measured primarily with
`Memory::get_mem_max_usage()`** rather than peak RSS, per candidate 035's finding that the tracked
peak is byte-deterministic while RSS is bimodal.

| Owner | Member | Instances at startup | Popup site |
|---|---|---|---|
| `DependencyEditor` | `search` | **2** (`FileSystemDock`'s and `OrphanResourcesDialog`'s) | `_load_pressed()` (click a dependency row) |
| `SceneImportSettingsDialog` | `save_path` | 1 | `_menu_callback()` (Extract Materials / Choose Mesh or Animation Save Paths) |
| `SceneImportSettingsDialog` | `item_save_path` | 1 | the external-path tree's per-row save button |

Four dialogs, three members, two files. `editor/file_system/dependency_editor.cpp`/`.h` and
`editor/import/3d/scene_import_settings.cpp`/`.h` each get `_ensure_*()` accessors and route their
popup sites through them.

**The interesting one is `save_path`.** It is not just a dialog: its constructor also builds an
`HBoxContainer` + `Label` + `OptionButton` (`external_extension_type`) **inside the dialog's own
vbox** (`save_path->get_vbox()->add_child(...)`), and `external_extension_type` is read from three
places in `_save_dir_callback()` and written in `_menu_callback()`. Both are safe: the
`OptionButton` lives inside `save_path`, so it exists exactly when `save_path` does;
`_menu_callback()` now calls the accessor as its first line (before touching
`external_extension_type`), and `_save_dir_callback()` only ever runs via `save_path`'s own
`dir_selected` signal. That invariant is recorded in a two-line comment at the accessor.

Also worth noting as in-tree prior art: `DependencyErrorDialog::replacement_file_dialog`
(`dependency_editor.cpp:981`) **already** uses exactly this `if (dialog == nullptr) { memnew ... }`
shape - in the same file as one of the two dialogs deferred here.

## What was tried

Harness: `bin/godot.linuxbsd.editor.x86_64 --headless --editor --path
___OPTIMIZATIONS___/harness/empty_project --quit-after 1`, same as every startup-memory candidate
here. Both arms built from the same tree with candidate 035's one-line `Main::cleanup()`
instrumentation applied on top (`bench/memstat_instrumentation.diff`, reverted afterwards), so each
run reports Godot's tracked heap counters as well as `VmHWM`. All 10 runs exit 0. Raw output in
`bench/baseline_5_runs.txt` and `bench/patched_5_runs.txt`.

| Variant | tracked peak `get_mem_max_usage()` | Peak RSS (VmHWM) |
|---|---|---|
| baseline (all 4 eager) | **448283253 bytes** - identical in all 5 runs | 702.40MB mean (718716/718928/719324/719384/719948 kB) |
| patched (all 4 deferred) | **439878597 bytes** - identical in all 5 runs | 691.91MB mean (707852/708204/708608/708720/709200 kB) |

RSS distributions don't overlap (baseline min 718716 > patched max 709200).

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimensions measured: Godot's tracked peak heap (primary) and peak RSS (secondary).**

**Tracked peak heap drops by 8404656 bytes = 8.02MB**, exactly and reproducibly (five runs per arm,
zero variance within each arm). **Peak RSS drops by 10.49MB (-1.49%)**, from 702.40MB to 691.91MB.
The RSS/tracked ratio is **1.31x**, in line with candidate 035's 1.23x.

That is **2.0MB of tracked peak per deferred dialog**, noticeably more than the 1.45MB candidate 021
measured per `FileDialog` construction - because these four are configured dialogs, not bare ones
(`save_path` carries an `OptionButton` + `Label` + `HBoxContainer` in its vbox, `item_save_path`
carries two filters, `search` gets its filters via `_setup_search_file_dialog()`), and because
candidate 021's figure counted only what `FileDialog`'s own constructor allocates.

## Does it change behavior/output?

- **Bit-exact?** N/A (UI/interactive-behavior change) - see verification below.
- **Startup-count verification with a baseline control**, under `Xvfb` (1280x800x24), using a
  throwaway `@tool` `EditorPlugin` (`bench/scratch_verification_plugin.gd`, not part of
  `patch.diff`):

  | Check | baseline (control) | patched |
  |---|---|---|
  | `EditorFileDialog`s in tree at startup | **64** | **60** |
  | `DependencyEditor` instances | 2 | 2 |
  | `SceneImportSettingsDialog` instances | 1 | 1 |

  Exactly **4 fewer** file dialogs, with the owner counts unchanged - which is what confirms the 2
  `DependencyEditor` instances each dropped one. The baseline's 64 reproduces candidate 021's count
  for the fourth time.
- **No flow was exercised interactively**, and that is a real gap: `DependencyEditor::search` needs
  a resource with a broken/replaceable dependency, and both `SceneImportSettingsDialog` dialogs need
  an actual 3D scene import (glTF/FBX) in the edited project - neither exists in the empty-project
  harness, and setting one up was out of scope for this candidate's time budget. The three popup
  sites were verified by code reading plus an exhaustive `grep` showing no raw `search->`,
  `save_path->` or `item_save_path->` uses remain outside the accessors. Same limitation as
  candidate 026, and called out rather than glossed.
- **Is this a breaking change?** No. All three members are private with no references outside their
  own `.cpp`, and no `ClassDB`-bound getter exposes them.
- Project test suite: 1427 cases, 1426 passed, 1 failed (known pre-existing `test_ip.cpp` IPv6
  failure, no IPv6 in this container), 3 skipped - identical before and after
  (`bench/test_suite_patched.txt`).

## Why the result looks the way it does

Two of these four dialogs belong to `DependencyEditor`, which exists **twice** at startup - the
same instance-multiplier that made candidate 032 large, and the reason this candidate is worth more
than the two-dialog owners candidate 031 covered. The other two belong to the 3D scene import
settings dialog, i.e. a flow that only runs when a user imports a 3D asset and then chooses to
extract materials or pick save paths.

**The measurement method is the other half of the story.** Candidate 034's -6.83MB needed 24 runs
and mode-matched statistics because RSS is bimodal at ~6MB in this harness; this candidate's headline
number came from **5 runs per arm with zero variance**, and the RSS arm was kept only as
confirmation that the tracked saving reaches the process. Candidate 035's counter is now the cheaper
and stricter measurement for anything in this family.

## What's not done / open questions

- **The three flows above should be exercised** once someone has a harness project with a 3D import
  and a broken dependency. That is the only reason this candidate's verification is weaker than
  candidates 024/031/032/034.
- **52 of the 66 `memnew(EditorFileDialog)` sites remain eager** (60 live instances at startup after
  this patch). The remaining owners are in candidate 032's
  `bench/attribution_64_file_dialogs.txt`; the biggest untouched ones are `EditorNode` (8, of which
  candidate 024 defers 4), `SpriteFramesEditor` (2, ruled out - 106 refs) and
  `EditorFeatureProfileManager` (2, ruled out - unguarded external singleton callers).
- A curiosity, noted for anyone comparing numbers across candidates: the baseline tracked peak here
  is 448283253 bytes against candidate 035's 448283245 - **8 bytes apart** for the same pristine
  source with the same instrumentation, i.e. the counter is byte-deterministic *within* a build but
  not quite *across* builds (plausibly a differing string constant or path length reaching a
  heap allocation). Compare arms built from the same tree, as this candidate did.
