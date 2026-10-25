# Defer both of `EditorAutoloadSettings`' `EditorFileDialog`s

Fourth per-call-site application of candidate 021's direction, after candidates 024
(`EditorNode`, 4 dialogs), 025 (`LocalizationEditor`, 5) and 026 (`ProjectExportDialog`, 3).
`editor/settings/editor_autoload_settings.cpp` builds 2 `EditorFileDialog`s at ~1.45MB each;
`EditorAutoloadSettings` is a tab inside `ProjectSettingsEditor`, which is constructed eagerly at
editor startup and whose own deferral was ruled out earlier this session - so, as with
`LocalizationEditor`, the dialogs must be deferred individually.

`editor/settings/editor_autoload_settings.cpp`/`.h`: adds `_get_autoload_file_dialog()` and
`_get_scene_file_dialog()` lazy accessors, plus a small `_browse_pressed()` handler. Two things
had to move for the deferral to be worth anything, both of which would otherwise have forced
construction at startup (or crashed):

- **`NOTIFICATION_ENTER_TREE` was calling `add_filter()` on both dialogs** (from
  `ResourceLoader::get_recognized_extensions_for_type("PackedScene"/"Script")`) - the same trap
  candidate 025 hit. Each dialog's filter setup moved into its own accessor.
- **The browse button bound its `pressed` signal straight to the dialog object**:
  `browse_button->connect(pressed, callable_mp(autoload_file_dialog, &FileDialog::popup_file_dialog))`
  requires the dialog pointer to exist at connect time. Rewired to
  `callable_mp(this, &EditorAutoloadSettings::_browse_pressed)`, which ensures-then-pops. This is a
  new shape not seen in candidates 024-026, and worth looking for at the remaining sites:
  a `callable_mp()` bound directly to the deferred object is as blocking as a notification handler.

## What was tried

Harness: `bin/godot.linuxbsd.editor.x86_64 --headless --editor --path
___OPTIMIZATIONS___/harness/empty_project --quit-after 1`. Peak RSS via `VmHWM`, exit codes checked.

| Variant | Peak RSS (VmHWM) | notes |
|---|---|---|
| baseline | 712.15MB mean over 8 runs (0.51% spread) | same pristine `HEAD` baseline as candidates 023-026, 029, 030 |
| patched | 708.30MB mean over 5 runs (724740/725764/725084/724820/726112 kB, 0.19% spread) | all exit 0 |

Distributions don't overlap, though by the smallest margin of this session's candidates (baseline
min 727268 vs patched max 726112, a 1156 kB gap).

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: peak RSS only.**

**Peak RSS drops by 3.85MB (-0.54%)**, from 712.15MB to 708.30MB (8 baseline vs 5 patched runs, no
overlap) - 1.33x the ~2.9MB predicted for 2 dialogs at candidate 021's 1.45MB each, within this
workspace's usual 1.3-1.7x tracked-vs-RSS range. This is the smallest win of the session's
per-call-site batch, as expected for the smallest site.

## Does it change behavior/output?

- **Bit-exact?** N/A (UI/interactive-behavior change) - see verification below.
- **Interactive verification with a baseline control**, under `Xvfb`, with a throwaway `@tool`
  plugin (`bench/scratch_verification_plugin.gd`, not part of `patch.diff`) that counts
  `EditorFileDialog` nodes tree-wide and then presses the Autoload tab's real "Select
  Script/Scene" button:

  | Check | baseline (control) | patched |
  |---|---|---|
  | `EditorFileDialog`s in tree at startup | **64** | **62** |
  | press "Select Script/Scene" | dialog shown, title `Open a File` | dialog shown, title `Open a File` |
  | count after the press | 64 | 62 -> **63** |

  Exactly 2 fewer at startup, and the button press produces the same dialog with the same title -
  which also directly exercises the rewired `browse_button` -> `_browse_pressed()` path. Raw output
  in `bench/verification.txt`.
- **Is this a breaking change?** No. Both members are private with no external references, and all
  uses are routed through the accessors.
- Project test suite: 1427 cases, 1426 passed, 1 failed (known pre-existing `test_ip.cpp` IPv6
  failure), 3 skipped - identical before and after.

## Why the result looks the way it does

Two more `EditorFileDialog`s at ~1.45MB each, in a Project Settings tab that most sessions never
open, inside a dialog that cannot itself be deferred - the same situation as candidate 025.

## What's not done / open questions

- The "create scene autoload" flow (`_get_scene_file_dialog()`, reached from the Autoload tab's
  create-script/scene action) was not exercised interactively; only the browse flow was. Same
  one-popup-site shape, verified by code reading plus the accessor-routing `grep`.
- **52 of the 66 `memnew(EditorFileDialog)` sites remain eager** after candidates 024 (4), 025 (5),
  026 (3) and 031 (2), worth up to the ~66MB remainder of candidate 021's 95.24MB. No file has more
  than 2 left. Left in `IDEAS.md`, now with three distinct blocking shapes to check per site:
  notification-handler configuration (025), configure-then-popup sequences (026), and
  `callable_mp()` bound directly to the deferred object (this candidate).
