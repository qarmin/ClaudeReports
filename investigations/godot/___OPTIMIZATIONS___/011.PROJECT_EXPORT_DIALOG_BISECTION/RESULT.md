# Bisecting `ProjectExportDialog`'s 8.72MB editor-startup cost

Candidate 010 found `ProjectExportDialog` (built unconditionally in `EditorNode`'s constructor,
dialog starts closed) as the third-largest single item in its own "+83.7MB" startup block
(8.72MB), but didn't bisect past that single aggregate number - left as a backlog item in
`IDEAS.md`. This candidate bisects `ProjectExportDialog::ProjectExportDialog()`
(`editor/export/project_export.cpp`) itself into its per-section components, using the same
`Memory::get_mem_usage()` checkpoint technique candidates 006/010 used.

A separate attempt this session at a `ProjectSettingsEditor` deferred-construction fix (candidate
010's other top lead) was tried and ruled out as unsafe/low-value - see `IDEAS.md`'s "Checked and
ruled out" entry, no candidate folder for that one since no working patch resulted.

## What was tried

Harness: same as candidates 006/010 - `bin/godot.linuxbsd.editor.x86_64 --headless --editor --path
___OPTIMIZATIONS___/harness/empty_project --quit-after 1`. 15 temporary `Memory::get_mem_usage()`
checkpoints added through `ProjectExportDialog::ProjectExportDialog()` spanning the entire
constructor, one per named section comment already present in the source (`// Presets list.`,
`// Preset settings.`, `// Subsections.`, `// Main preset parameters.`, `// Resources export
parameters.`, `// Patching.`, `// Feature tags.`, `// Encryption export parameters.`, `// Script
export parameters.`, `// Export buttons, dialogs and errors.`, `// Export warnings and errors
bottom section.`, `// Android SDK errors bottom section.`, `// Export project file dialog.`) - see
`bench/instrumentation.diff` for the exact temporary diff (applied, measured, then reverted - not a
real fix, see "Does it change behavior/output?" below). Fully deterministic across repeat runs (3/3
runs bit-identical byte counts at every checkpoint, 0% spread), same determinism candidates 006/010
observed for this counter.

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: Godot's own engine-tracked heap (`Memory::get_mem_usage()`), for attribution
only - same as candidates 006/010's own breakdowns.**

| Segment (checkpoint -> checkpoint) | Delta | % of the 8.54MB total | What it covers |
|---|---|---|---|
| start -> before_presets_list | +0.02MB | 0.2% | dialog flags, title, root `VBoxContainer`/`HSplitContainer` |
| before_presets_list -> before_preset_settings | +0.28MB | 3.3% | `// Presets list.` - preset `ItemList`, add/duplicate/delete buttons |
| before_preset_settings -> before_subsections | +0.26MB | 3.0% | `// Preset settings.` - name field, runnable/advanced-options toggles, export path property, `EditorPropertyPath` |
| before_subsections -> before_main_params | +0.04MB | 0.5% | `// Subsections.` - just the `TabContainer` shell |
| before_main_params -> before_resources_export | +0.23MB | 2.7% | `// Main preset parameters.` - options filter `LineEdit` + `EditorInspector` |
| before_resources_export -> before_patching | +0.51MB | 6.0% | `// Resources export parameters.` |
| **before_patching -> before_feature_tags** | **+2.26MB** | **26.4%** | **`// Patching.` - delta-encoding toggle/spinbox/filters, patches `Tree`, patch file dialog, confirmation dialog - the single largest section** |
| before_feature_tags -> before_encryption | +0.08MB | 1.0% | `// Feature tags.` - custom features `LineEdit` + `RichTextLabel` |
| before_encryption -> before_script_export | +0.34MB | 4.0% | `// Encryption export parameters.` |
| before_script_export -> before_export_buttons | +0.24MB | 2.9% | `// Script export parameters.` + disable-by-default + no-presets label + deletion dialog (4 sub-blocks, not individually bisected) |
| **before_export_buttons -> before_warnings_bottom** | **+1.77MB** | **20.7%** | **`// Export buttons, dialogs and errors.` - second-largest section** |
| before_warnings_bottom -> before_android_sdk_errors | +0.30MB | 3.6% | `// Export warnings and errors bottom section.` |
| before_android_sdk_errors -> before_export_project_file_dialog | +0.24MB | 2.8% | `// Android SDK errors bottom section.` |
| **before_export_project_file_dialog -> end** | **+1.60MB** | **18.7%** | **`// Export project file dialog.` (an `EditorFileDialog`) + trailing default-filename computation - third-largest section** |

Total: 8.54MB (`end` - `start`), consistent with candidate 010's own 8.72MB estimate for this same
dialog (small ~0.2MB difference expected - measured on a fresh build without other candidates'
instrumentation present, and candidate 010's number was itself one checkpoint delta in a larger
sweep, not dedicated to this dialog alone).

**The three largest sections - `// Patching.` (2.26MB, 26.4%), `// Export buttons, dialogs and
errors.` (1.77MB, 20.7%), and the `// Export project file dialog.` block (1.60MB, 18.7%) - together
account for 65.8% of the dialog's total startup cost.** All three are built unconditionally in the
constructor regardless of which (if any) export preset tab the user ever visits, or whether the
Export dialog is ever opened at all in a given session.

## Does it change behavior/output?

**N/A - measurement/characterization only, no patch applied to real source** (per SKILL.md's
"patch.diff is mandatory..." section - this is the correct "pure measurement/investigation" state).
`bench/instrumentation.diff`'s temporary `Memory::get_mem_usage()` probes were reverted before
finishing this candidate.

- Project test suite: not re-run separately for this purely additive `print_line`-only
  instrumentation - candidates 006/010 already established this class of instrumentation adds no
  measurable overhead and doesn't affect test outcomes, and this candidate made no changes to real
  source (instrumentation-only, fully reverted).

## Why the result looks the way it does

Same architectural pattern candidates 006/007/010 already found repeatedly in this editor: Godot's
editor builds essentially every widget of every dialog/tab up front in the constructor, regardless
of whether the user ever opens that dialog or visits that specific tab in a given session.
`ProjectExportDialog` has 6 `TabContainer` tab pages (Options, Resources, Patching, Features,
Encryption, Script) built unconditionally in one constructor call, matching the same shape as
candidate 010's `ProjectSettingsEditor` finding (also a multi-tab settings dialog) and candidate
007's `ThemeEditor` finding (a single eagerly-built preview widget) - just spread across more,
smaller tab sections here rather than one dominant sub-widget.

## What's not done / open questions

- **No fix was attempted for this candidate** - given this session's `ProjectSettingsEditor`
  deferred-construction attempt (see `IDEAS.md`) crashed on ordinary startup due to *other* files
  reaching into that class via its own static `get_singleton()` (bypassing any lazy-construction
  gate `EditorNode` could control), the same risk plausibly applies here: `ProjectExportDialog`
  exposes its own state via `EditorExport`/menu actions and its tabs wire up signal connections
  during construction (e.g. `EditorExport::get_singleton()->connect("export_presets_updated", ...)`
  at line ~1751) that a future session would need to audit for external direct-singleton access
  before attempting a similar per-tab deferred-construction fix. Left in `IDEAS.md` with this
  caveat rather than attempted under this session's time budget.
- The `// Script export parameters.` group (0.24MB) was measured as one combined checkpoint delta
  covering 4 sub-blocks (script export settings, disable-by-default, no-presets label, deletion
  confirmation dialog) - not bisected further, low priority given its small absolute size.
- Not cross-checked whether any of the 3 largest sections (`Patching`, `Export buttons`, `Export
  project file dialog`) could be cheaper by construction (e.g. reusing a shared `EditorFileDialog`
  instance instead of a dedicated one) rather than only deferred - a different angle not
  investigated this session.
