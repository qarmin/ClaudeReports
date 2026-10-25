# Defer the 4 `ScriptCreateDialog`s and 4 `ShaderCreateDialog`s built at editor startup

Candidate 021 counted 64 `EditorFileDialog` instances at empty-project startup but never attributed
them to their owners. This candidate starts by doing exactly that (see
`bench/attribution_64_file_dialogs.txt`), and the attribution immediately named a target that the
per-call-site sweep of candidates 024/025/026 could not have found: **8 of the 64 file dialogs are
not owned by an editor at all - they sit inside `ScriptCreateDialog` (4 instances) and
`ShaderCreateDialog` (4 instances)**, which are themselves eagerly constructed by three editors.
Each `ScriptCreateDialog` additionally owns a whole `CreateDialog` (its `select_class` class
picker), so 4 of the 9 `CreateDialog`s alive at startup come from the same place.

The 8 owners are:

| Owner | Members | Note |
|---|---|---|
| `ScriptEditor` (x2 instances) | `script_create_dialog`, `shader_create_dialog` | **`ScriptEditor` is instantiated twice**: once by `ScriptEditorPlugin` and once by `ShaderEditorPlugin` (`shader_editor_plugin.cpp:157`, `memnew(ScriptEditor(config_section, "shader_editor_cache.cfg"))`), so everything its constructor builds is paid for twice |
| `SceneTreeDock` | `script_create_dialog`, `shader_create_dialog` | its script dialog backs the GDScript-exposed `EditorPlugin.get_script_create_dialog()` |
| `FileSystemDock` | `make_script_dialog`, `make_shader_dialog` | its script dialog is reachable via `FileSystemDock::get_script_create_dialog()` |

`editor/script/script_editor_plugin.cpp`/`.h`, `editor/docks/scene_tree_dock.cpp`/`.h`,
`editor/docks/filesystem_dock.cpp`/`.h`: each of the six members gets a lazy accessor
(construct-configure-`add_child`-connect on first call, then return) and every popup site is routed
through it. Two of the accessors are the existing public getters made lazy in place
(`SceneTreeDock::get_script_create_dialog()`, moved out of the header, and
`FileSystemDock::get_script_create_dialog()`, which loses its `const` because it now constructs);
the other four are new private `_ensure_*()`/menu-site accessors.

**The pattern is already in this codebase**, which is how the target was recognised as safe:
`EditorAutoloadSettings::_create_script_autoload()` (`editor/settings/editor_autoload_settings.cpp`)
constructs *its* `ScriptCreateDialog` on first use with exactly this
`if (!script_create_dialog) { memnew ... }` shape - which is also why that 4th
`memnew(ScriptCreateDialog)` site never appears in the startup attribution.

## What was tried

Harness: `bin/godot.linuxbsd.editor.x86_64 --headless --editor --path
___OPTIMIZATIONS___/harness/empty_project --quit-after 1`, same as candidates
006/007/013/017/018/019/023/024/025/026/029/030/031. Peak RSS via `/proc/<pid>/status`'s `VmHWM`,
every run's exit code checked.

Both arms were re-measured back-to-back today under identical environment (same `XDG_DATA_HOME`/
`XDG_CONFIG_HOME`, same machine state), rather than reusing the 712.15MB baseline figure recorded
earlier in this workspace - that older figure came from a slightly different XDG state and reads
~10MB higher, so mixing the two would have inflated this candidate's delta. Raw numbers in
`bench/rss_baseline.txt` and `bench/rss_patched.txt`.

| Variant | Peak RSS (VmHWM) | notes |
|---|---|---|
| baseline (all 8 dialogs eager) | 719139.3 kB mean = **702.28MB** (718228/719256/719788/718324/719972/719268, 6 runs, 0.24% spread) | all exit 0 |
| patched (all 8 deferred) | 680757.3 kB mean = **664.80MB** (680944/680836/680072/680772/680364/681556, 6 runs, 0.22% spread) | all exit 0 |

Distributions don't overlap (baseline min 718228 > patched max 681556).

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: peak RSS only** (constructor-phase allocation removal, not a per-call hot
path - matching how candidates 007/013/017/018/019/023/024/025/026/029/030/031 measured this class
of fix).

**Peak RSS drops by 37.48MB (-5.34%)**, from 702.28MB to 664.80MB mean (6 vs 6 runs, no overlap).
That is the second-largest single improvement in this workspace after candidate 023's 46.39MB.

The 8 contained `EditorFileDialog`s alone account for only ~11.6MB at candidate 021's measured
1.45MB each; the rest comes from what else those dialogs own - 4 `CreateDialog` class pickers, and
the `ScriptCreateDialog`/`ShaderCreateDialog` widget trees themselves (template pickers,
`OptionButton`s, `LineEdit`s, `Window`-level allocations per dialog).

## Does it change behavior/output?

- **Bit-exact?** N/A (UI/interactive-behavior change) - see verification below.
- **Interactive verification with a baseline control**, under `Xvfb` (1280x800x24), using a
  throwaway `@tool` `EditorPlugin` (`bench/scratch_verification_plugin.gd`, not part of
  `patch.diff`) that counts the four classes across the whole scene tree (including internal
  children) and then drives three real entry points. Run against **both** builds
  (`bench/verification_baseline_control.txt`, `bench/verification_patched.txt`):

  | Check | baseline (control) | patched |
  |---|---|---|
  | at startup | EditorFileDialog **64**, ScriptCreateDialog **4**, ShaderCreateDialog **4**, CreateDialog **9** | EditorFileDialog **56**, ScriptCreateDialog **0**, ShaderCreateDialog **0**, CreateDialog **5** |
  | `EditorInterface.get_script_editor().open_script_create_dialog("Node", "res://new_script.gd")` | visible `ScriptCreateDialog`, title `Create Script`; counts unchanged | visible `ScriptCreateDialog`, title `Create Script`; 56->57 / 0->1 / 5->6 |
  | `EditorPlugin.get_script_create_dialog()` (GDScript API -> `SceneTreeDock`) | returns `ScriptCreateDialog` | returns `ScriptCreateDialog`; 57->58 / 1->2 / 6->7 |
  | script editor File menu -> "New Shader..." (real `PopupMenu::id_pressed`) | visible `ShaderCreateDialog`, title `Create Shader` | visible `ShaderCreateDialog`, title `Create Shader`; 58->59 / ShaderCreateDialog 0->1 |

  The control makes the claim precise: exactly **8 fewer** file dialogs, **4+4 fewer** create
  dialogs and **4 fewer** `CreateDialog`s exist at startup, each flow creates exactly one on
  demand, and every user-visible outcome (dialog shown, same title) is identical. Screenshot of the
  lazily-created dialog in `bench/screenshot_script_create_dialog.png`. The baseline's 64
  independently reproduces candidate 021's count for the third time.
- **The `EditorPlugin.get_script_create_dialog()` row is the important one.** That method is
  `ClassDB`-bound and callable from any GDScript editor plugin (`editor/plugins/editor_plugin.cpp`
  forwards it to `SceneTreeDock::get_singleton()->get_script_create_dialog()`), so it must never
  start returning null. Making the getter itself the lazy accessor preserves that contract, and the
  verification above exercises it through the real bound method rather than by code reading.
- **Is this a breaking change?** No. `FileSystemDock::get_script_create_dialog()` drops its `const`
  qualifier - it is not `ClassDB`-bound (only `EditorPlugin`'s forwarder to `SceneTreeDock`'s is)
  and its single caller is `modules/visual_shader/editor/visual_shader_editor_plugin.cpp:7451`, so
  no public API signature changes. That call site is also the one place that can force a deferred
  dialog into existence early: `VisualShaderEditor`'s constructor connects to the dock's script
  dialog. It does not affect startup because `VisualShaderEditor` is itself only built by the
  `create_editor` factory registered with `ScriptEditor::register_create_script_editor_function()`,
  i.e. when a visual shader is actually opened - confirmed by the patched startup count of 0
  `ScriptCreateDialog`s.
- `SceneTreeDock::_script_creation_closed()`/`_shader_creation_closed()` still dereference their
  members directly; both only ever run as handlers of those dialogs' own `confirmed`/`canceled`
  signals, so the dialog is guaranteed to exist. Called out in a one-line comment at that site
  (same reasoning as candidate 023's `_make_dir_confirm()`).
- Project test suite: 1427 cases, 1426 passed, 1 failed (known pre-existing `test_ip.cpp` IPv6
  failure, no IPv6 in this container), 3 skipped - identical before and after
  (`bench/test_suite_patched.txt`).

## Why the result looks the way it does

Three editors each built two heavyweight creation dialogs at startup for flows that only run when
the user explicitly asks to create a script or a shader, and one of those three editors
(`ScriptEditor`) exists twice. Because `ScriptCreateDialog` transitively owns both an
`EditorFileDialog` and a `CreateDialog`, each deferral removes a subtree rather than a widget -
which is why 8 deferrals are worth 37.48MB while the 4 dialogs of candidate 024 were worth 9.02MB.

## What's not done / open questions

- **The attribution in `bench/attribution_64_file_dialogs.txt` names the rest of the backlog
  precisely** and is the most reusable artifact here: after this candidate, 56 file dialogs remain,
  the largest remaining owners being `LocalizationEditor` (5, candidate 025),
  `ProjectExportDialog` (3, candidate 026), `EditorNode` (8, of which candidate 024 defers 4),
  `SpriteFramesEditor` (2, ruled out earlier - 106 refs), `SceneImportSettingsDialog` (2),
  `EditorFeatureProfileManager` (2), `EditorBuildProfileManager` (2, candidate 018),
  `DependencyEditor` (2), `EditorAutoloadSettings` (2, candidate 031), plus ~20 single-instance
  owners. Left in `IDEAS.md`.
- The 4 `CreateDialog`s removed here are a hint at a target this session never measured: 9
  `CreateDialog`s exist at startup and `CreateDialog` builds a `Tree` plus an `EditorHelpBit`. Its
  per-instance cost was not isolated (candidate 027's census did not include it).
- Only one of the two `ScriptEditor` instances was exercised interactively (the script editor's
  File menu); the shader editor's copy runs the identical constructor and accessors.
- The `FileSystemDock` flows (`FILE_MENU_NEW_SCRIPT` from the filesystem context menu, and the
  "Create New -> Resource" shader/script branches of `_resource_created()`) were verified by code
  reading plus the accessor-routing `grep` only - they share the shape of the three flows that were
  driven end-to-end.
