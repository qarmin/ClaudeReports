# Defer all 3 of `ProjectExportDialog`'s `EditorFileDialog`s

Third and last application of candidate 021's per-call-site direction this session, after
candidates 024 (`EditorNode`, 4 dialogs) and 025 (`LocalizationEditor`, 5 dialogs).
`editor/export/project_export.cpp` creates 3 `EditorFileDialog`s at ~1.45MB each; the owning
`ProjectExportDialog` is itself constructed eagerly at editor startup (candidate 011 measured it at
8.54MB), so as with `LocalizationEditor` the dialogs have to be deferred individually.

All three - `patch_dialog`, `export_pck_zip`, `export_project` - are private with no references
outside `project_export.cpp`, and none is touched by any `_notification()` handler (checked
explicitly, per candidate 025's lesson).

`editor/export/project_export.cpp`/`.h`: adds `_get_patch_dialog()`, `_get_export_pck_zip()` and
`_get_export_project()` lazy accessors and routes every use through them. Two subtleties this
candidate had to handle, both of which would have crashed on a null dialog if missed:

- `export_pck_zip`'s and `export_project`'s `add_option()` setup lived in a **different part of the
  constructor** from their `memnew` (a shared block near the end configuring both), so it had to be
  folded into the two accessors rather than moved with the construction block.
- Both `export_pck_zip` and `export_project` have "configure then popup" sequences at their call
  sites (`set_current_dir()` before the popup; and for `export_project` a 7-line block setting
  access, filters, current path/file and file mode) that run **before** the popup line - so routing
  only the `popup_file_dialog()` call would have left an earlier null dereference. All of those
  lines are routed too.

## What was tried

Harness: `bin/godot.linuxbsd.editor.x86_64 --headless --editor --path
___OPTIMIZATIONS___/harness/empty_project --quit-after 1`. Peak RSS via `VmHWM`, exit codes checked.

| Variant | Peak RSS (VmHWM) | notes |
|---|---|---|
| baseline (all 3 eager) | 712.15MB mean over 8 runs (0.51% spread) | same pristine `HEAD` baseline as candidates 023/024/025 |
| patched (all 3 deferred) | 705.45MB mean over 5 runs (723300/721440/723092/722520/721556 kB, 0.26% spread) | all exit 0 |

Distributions don't overlap (baseline min 727268 > patched max 723300).

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: peak RSS only.**

**Peak RSS drops by 6.70MB (-0.94%)**, from 712.15MB to 705.45MB (8 baseline vs 5 patched runs, no
overlap) - 1.54x the ~4.35MB predicted for 3 dialogs at candidate 021's 1.45MB each, in line with
the 1.54-1.73x ratio across this workspace's deferred-construction candidates.

## Does it change behavior/output?

- **Bit-exact?** N/A (UI/interactive-behavior change) - see verification below.
- **Node-count verification with a baseline control**, under `Xvfb`, using a throwaway `@tool`
  plugin (`bench/scratch_verification_plugin.gd`, not part of `patch.diff`) that counts
  `EditorFileDialog` nodes across the whole scene tree:

  | Check | baseline (control) | patched |
  |---|---|---|
  | `EditorFileDialog`s in tree at startup | **64** | **61** |

  Exactly **3 fewer**, matching the 3 deferrals, and the baseline's 64 again reproduces candidate
  021's independent headless count.
- **The popup flow was NOT successfully exercised, in either build.** The test pressed the real
  "Export PCK/ZIP..." button under `ProjectExportDialog`; no file dialog appeared and the count
  stayed unchanged in **both** the patched (61 -> 61) and baseline (64 -> 64) runs, because that
  flow early-returns without a selected export preset and the empty verification project has none.
  So this is a limitation of the test setup rather than a behavioral difference - but it does mean
  this candidate's popup paths are verified by code reading plus the accessor-routing `grep` only,
  unlike candidates 024/025 where a flow was genuinely exercised. Raw output in
  `bench/verification_counts.txt`, including this caveat inline.
- **Is this a breaking change?** No. All three members are private with no external references, and
  every use is routed through its accessor (verified by `grep` after patching: no raw
  `patch_dialog->`, `export_pck_zip->` or `export_project->` uses remain outside the accessor
  bodies).
- Project test suite: 1427 cases, 1426 passed, 1 failed (known pre-existing `test_ip.cpp` IPv6
  failure), 3 skipped - identical before and after.

## Why the result looks the way it does

Three more `EditorFileDialog`s at ~1.45MB each, inside an export dialog that a typical editing
session never opens, in a class that cannot itself be deferred. Same reasoning as candidates
024/025.

## What's not done / open questions

- The popup flows (add base pack, export PCK/ZIP, export project) were not exercised interactively
  - see above. Doing so needs a verification project with a real export preset configured; worth
  adding to the shared harness if this direction is continued.
- **54 of the 66 `memnew(EditorFileDialog)` sites remain eager** after candidates 024 (4), 025 (5)
  and 026 (3), worth up to the ~70MB remainder of candidate 021's 95.24MB. No single file has more
  than 2 left; the remainder is a long tail of ~40 files. Left in `IDEAS.md` with the now
  thrice-validated per-site recipe.
