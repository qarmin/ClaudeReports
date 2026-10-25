# Defer all 5 of `LocalizationEditor`'s `EditorFileDialog`s

Continues candidate 024's per-call-site direction on the largest remaining single file:
`editor/translations/localization_editor.cpp` creates **5** `EditorFileDialog`s (joint-largest of
the 66 sites candidate 021 counted, at ~1.45MB each). `LocalizationEditor` lives inside
`ProjectSettingsEditor`, whose own deferral was ruled out earlier this session (see `IDEAS.md`), so
the dialogs have to be deferred individually - which is exactly what this candidate does.

All five (`translation_file_open`, `translation_res_file_open_dialog`,
`translation_res_option_file_open_dialog`, `template_generate_dialog`,
`template_source_open_dialog`) have **one popup site each** and **zero references outside
`localization_editor.cpp`**.

`editor/translations/localization_editor.cpp`/`.h`: adds a `_get_*()` lazy accessor per dialog
(construct-configure-add_child-set-filters on first call, then return) and routes the popup sites
through them.

**The interesting part of this patch is the filters.** `LocalizationEditor::_notification()`'s
`NOTIFICATION_ENTER_TREE` handler was calling `add_filter()` on four of the five dialogs (built from
`ResourceLoader::get_recognized_extensions_for_type("Translation"/"Resource")` and a literal
`*.pot`/`*.csv` pair) plus `_update_template_source_file_extensions()` for the fifth. Since
`LocalizationEditor` enters the tree at editor startup, leaving that code in place would have
forced all five dialogs to be constructed anyway - **a deferral worth exactly nothing**, and the
first version of this patch did precisely that until the unguarded uses were found. The fix moves
each dialog's filter setup into its own accessor, and makes
`_update_template_source_file_extensions()` a no-op while its dialog doesn't exist yet (it is also
called from `update_translations()`, for the "a new translation parser plugin may have added
extensions" case, which still works for an already-created dialog and is recomputed at creation
otherwise).

## What was tried

Harness: `bin/godot.linuxbsd.editor.x86_64 --headless --editor --path
___OPTIMIZATIONS___/harness/empty_project --quit-after 1`, same as candidates
006/007/013/017/018/019/023/024. Peak RSS via `VmHWM`, every run's exit code checked.

| Variant | Peak RSS (VmHWM) | notes |
|---|---|---|
| baseline (all 5 eager) | 712.15MB mean over 8 runs (0.51% spread) | same pristine `HEAD` baseline used by candidates 023/024 |
| patched (all 5 deferred) | 700.94MB mean over 5 runs (718120/717388/717648/717620/718060 kB, 0.10% spread) | all exit 0 |

Distributions don't overlap (baseline min 727268 > patched max 718120).

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: peak RSS only.**

**Peak RSS drops by 11.21MB (-1.57%)**, from 712.15MB to 700.94MB (8 baseline vs 5 patched runs, no
overlap). That is 1.55x the ~7.25MB predicted from 5 dialogs at candidate 021's measured 1.45MB
each - in line with the 1.55-1.73x tracked-vs-RSS ratio seen across every deferred-construction
candidate in this workspace.

## Does it change behavior/output?

- **Bit-exact?** N/A (UI/interactive-behavior change) - see verification below.
- **Interactive verification with a baseline control**, under `Xvfb` (1280x800x24), using a
  throwaway `@tool` `EditorPlugin` (`bench/scratch_verification_plugin.gd`, not part of
  `patch.diff`) that counts `EditorFileDialog` nodes across the whole scene tree, then finds the
  Localization tab's real "Add..." button under `LocalizationEditor` and emits its `pressed`
  signal:

  | Check | baseline (control) | patched |
  |---|---|---|
  | `EditorFileDialog`s in tree at startup | **64** | **59** |
  | press Localization "Add..." | dialog shown, title `Open File(s)`, **5 filters** | dialog shown, title `Open File(s)`, **5 filters** |
  | count after the press | 64 (already existed) | 59 -> **60** (created on demand) |

  Exactly **5 fewer** dialogs at startup, matching the 5 deferrals. **The matching filter count (5
  vs 5) is the key check for this candidate specifically**: it demonstrates that moving filter setup
  out of `NOTIFICATION_ENTER_TREE` and into the accessors produces the identical filter set the old
  code did - the one genuinely risky part of the change. Raw output in
  `bench/verification_{patched,baseline_control}.txt`.
- **Is this a breaking change?** No. All five members are private with no external references, and
  all uses are routed through the accessors. One deliberate behavioral nuance: a dialog's filters
  are now computed at first-open rather than at editor startup, so a translation-parser plugin
  registered *after* startup is now picked up for a not-yet-opened dialog as well (previously only
  `template_source_open_dialog` got that treatment, via `update_translations()`). That is a strict
  improvement in freshness, and the verification above shows the resulting filter set is unchanged
  in the normal case.
- Project test suite: 1427 cases, 1426 passed, 1 failed (known pre-existing `test_ip.cpp` IPv6
  failure, no IPv6 in this container), 3 skipped - identical before and after.

## Why the result looks the way it does

Five `EditorFileDialog`s at ~1.45MB each, in a settings tab most sessions never open, inside a
dialog (`ProjectSettingsEditor`) that itself cannot be deferred. This is the case that shows why
candidate 021's per-call-site direction matters even where the owning dialog is stuck being eager.

## What's not done / open questions

- Only the `translation_file_open` flow ("Add..." on the Localization tab) was exercised
  interactively; the other four popup sites (resource remap add, remap-option add, POT template
  generate, template source add) were verified by code reading plus the accessor-routing `grep`,
  and share the identical one-popup-site shape.
- **57 of the 66 `memnew(EditorFileDialog)` sites remain eager** after candidates 024 (4) and 025
  (5), worth up to the ~75MB remainder of candidate 021's 95.24MB. Next-largest single file is
  `editor/export/project_export.cpp` (3). Left in `IDEAS.md` - the per-site recipe is now
  established twice over and is mechanical, with the node-count control as its verification.
- The filter-setup trap found here is worth remembering for every remaining site: a
  `_notification(NOTIFICATION_ENTER_TREE)`/`_ready` handler that configures the dialog will silently
  defeat the deferral (or crash on null), so each site's audit must cover notification handlers, not
  just the popup call.
