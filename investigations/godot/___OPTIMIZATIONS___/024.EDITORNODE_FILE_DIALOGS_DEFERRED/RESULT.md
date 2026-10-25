# Defer 4 of `EditorNode`'s own 5 `EditorFileDialog`s

Candidate 021 found 64 `FileDialog` instances at empty-project startup at ~1.45MB each, and named
the complementary fix direction to candidate 023's (which reduced each instance's cost): make the
66 `memnew(EditorFileDialog)` **call sites** lazy. `editor/editor_node.cpp` is the joint-largest
single site (5 of the 66), so it is the natural first bite. This candidate defers 4 of those 5.

The 5th, `file`, is left eager deliberately: it is the general-purpose scene open/save dialog with
many call sites throughout `EditorNode`, i.e. the one dialog a normal editing session actually does
use, and the only one of the five whose call-site audit is not trivial.

`editor/editor_node.cpp`/`.h`: adds `_get_file_android_build_source()`, `_get_file_export_lib()` and
`_get_file_pack_zip()` lazy accessors (construct-configure-add_child on first call, then return),
and routes their popup/configure sites through them - `_menu_option_confirm()`'s
`PROJECT_PACK_AS_ZIP` and `FILE_EXPORT_MESH_LIBRARY` branches, `_dialog_action()`'s
`get_selected_options()` read, and `_android_export_preset_selected()`'s
`popup_centered_ratio()`.

**`file_templates` gets no accessor at all, because nothing in the codebase ever shows it.** It was
constructed, titled "Import Templates From ZIP File", added to `gui_base` and wired to
`_dialog_action`, and then never popped up from anywhere - `grep -rn file_templates` over the whole
repo finds only its own declaration and that construction block (almost certainly a leftover from
before `ExportTemplateManager` took over template importing). So this patch simply stops
constructing it: ~1.45MB per editor startup for a dialog that could never appear. The private
member declaration is deliberately left in place, so the change stays minimal and trivially
reversible rather than turning into a dead-code removal.

## What was tried

Harness: `bin/godot.linuxbsd.editor.x86_64 --headless --editor --path
___OPTIMIZATIONS___/harness/empty_project --quit-after 1`, same as candidates
006/007/013/017/018/019/023. Peak RSS via `/proc/<pid>/status`'s `VmHWM`, with every run's exit code
checked (see candidate 023's "What's not done" for why that matters on this project).

| Variant | Peak RSS (VmHWM) | notes |
|---|---|---|
| baseline (all 5 eager) | 712.15MB mean over **8** runs (730964/730732/728284/730788/730668 + re-verified 727268/727588/727660 kB, 0.51% spread) | all exit 0; same pristine `HEAD` build used for candidate 023's baseline, re-run to confirm it hadn't drifted |
| patched (4 deferred) | 703.14MB mean over 5 runs (720608/720788/718156/721356/719144 kB, 0.44% spread) | all exit 0 |

Distributions don't overlap (baseline min 727268 > patched max 721356).

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: peak RSS only.**

**Peak RSS drops by 9.02MB (-1.27%)**, from 712.15MB to 703.14MB (8 baseline vs 5 patched runs, no
overlap). Against the 5 first-measured baseline runs alone the figure is -10.04MB; the 8-run number
above is the conservative one and is what this candidate claims.

That is 1.73x the ~5.8MB the engine-tracked figure predicts for 4 dialogs at candidate 021's
measured 1.45MB each - the same tracked-vs-RSS ratio as every other deferred-construction candidate
in this workspace (007, 013, 017, 018, 019, 023: 1.6-1.7x).

## Does it change behavior/output?

- **Bit-exact?** N/A (UI/interactive-behavior change) - see verification below.
- **Interactive verification with a baseline control**, under `Xvfb` (1280x800x24), using a
  throwaway `@tool` `EditorPlugin` (`bench/scratch_verification_plugin.gd`, not part of
  `patch.diff`) that counts `EditorFileDialog` nodes in the whole scene tree (including internal
  children), then triggers Project -> "Pack Project as ZIP..." through the real menu signal path
  (`PopupMenu::id_pressed` -> `EditorNode::_menu_option` -> `_menu_option_confirm`):

  | Check | baseline (control) | patched |
  |---|---|---|
  | `EditorFileDialog`s in tree at startup | **64** | **60** |
  | after triggering Pack Project as ZIP | 64 (already existed) | **61** (created on demand) |
  | dialog actually shown, with correct title | yes, `Pack Project as ZIP...` | yes, `Pack Project as ZIP...` |

  The control makes the claim precise: exactly **4 fewer** dialogs exist at startup - matching the 4
  deferrals - and the user-visible outcome is identical (same dialog, same title, shown on demand).
  Raw output in `bench/verification_{patched,baseline_control}.txt`, screenshot of the
  lazily-created dialog in `bench/screenshot_pack_zip_dialog.png`. Note the baseline's 64 here
  independently reproduces candidate 021's headless count of 64.
- **Is this a breaking change?** No. All four members are private, all their uses are routed through
  the accessors (verified by `grep` after patching: no raw `file_android_build_source->`,
  `file_export_lib->`, `file_pack_zip->` or `file_templates` uses remain outside the accessor
  bodies), and `file_templates` was unreachable to begin with.
- Project test suite: 1427 cases, 1426 passed, 1 failed (known pre-existing `test_ip.cpp` IPv6
  failure, no IPv6 in this container), 3 skipped - identical before and after.

## Why the result looks the way it does

Straight "pay only when used", applied to four dialogs that a normal session never opens (Android
source selection, Export Mesh Library, Pack Project as ZIP) or *cannot* open at all
(`file_templates`). It is the same pattern as candidates 017/018/019, but on `EditorFileDialog`s,
which candidate 021 showed to be unusually expensive per instance (~1.45MB) - so four of them are
worth more than most whole dialogs fixed earlier in the session.

## What's not done / open questions

- **Only the Pack-Project-as-ZIP flow was exercised interactively.** `Export Mesh Library` (Scene ->
  Export As -> submenu, and it early-returns unless the edited scene has a MeshInstance) and the
  Android-source flow (needs an Android SDK/export setup this container does not have) were verified
  by code reading plus the accessor-routing `grep` only. `file_templates` cannot be exercised
  because nothing shows it - that is the finding, not a gap.
- **62 of the 66 `memnew(EditorFileDialog)` call sites remain eager**, worth up to the ~90MB
  remainder of candidate 021's 95.24MB. The next-largest single files are
  `editor/translations/localization_editor.cpp` (5 - though it lives inside `ProjectSettingsEditor`,
  whose own deferral was ruled out this session) and `editor/export/project_export.cpp` (3). Left in
  `IDEAS.md`; each site needs its own small audit, but this candidate shows the per-site pattern is
  mechanical once the audit is done.
- Not attempted: sharing one `EditorFileDialog` instance between call sites (a much larger design
  change than deferral, and dialogs carry per-site filters/modes/titles that would have to be reset
  on every use).
