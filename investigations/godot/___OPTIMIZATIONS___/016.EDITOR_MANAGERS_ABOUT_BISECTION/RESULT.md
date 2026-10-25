# Bisecting candidate 010's `ExportTemplateManager`+`EditorFeatureProfileManager`+`EditorBuildProfileManager`+`EditorAbout` group (20.69MB)

Candidate 010 measured this group of 4 dialogs, all built unconditionally in a row in
`EditorNode`'s constructor, as one combined 20.69MB checkpoint delta without bisecting per-dialog.
This candidate adds a `Memory::get_mem_usage()` checkpoint after each of the 4 `memnew()` calls to
attribute the cost individually.

## What was tried

Harness: same as candidates 006/007/010/011/012/015 - `bin/godot.linuxbsd.editor.x86_64 --headless
--editor --path ___OPTIMIZATIONS___/harness/empty_project --quit-after 1`. See
`bench/instrumentation.diff` for the exact temporary diff (applied, measured, then reverted - not a
real fix, see "Does it change behavior/output?" below). Fully deterministic across 3 repeat runs
(bit-identical byte counts, 0% spread).

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: Godot's own engine-tracked heap (`Memory::get_mem_usage()`), for attribution
only - same as prior bisection candidates.**

| Dialog | Delta | % of 19.73MB total |
|---|---|---|
| `ExportTemplateManager` | +2.58MB | 13.1% |
| `EditorFeatureProfileManager` | +4.32MB | 21.9% |
| `EditorBuildProfileManager` | +4.10MB | 20.8% |
| **`EditorAbout`** | **+8.73MB** | **44.3%** |

(Total 19.73MB here vs. candidate 010's 20.69MB - measured on a fresh build without other
candidates' instrumentation present, expected small variance already noted in prior candidates.)

`EditorAbout` (the "Help -> About Godot..." dialog, showing contributor/donor/license lists) is
the single largest of the 4, and - checked this session before pursuing further - is referenced
from exactly **one** place in the entire codebase: `EditorNode::_menu_option_confirm()`'s
`HELP_ABOUT` case (`about->popup_centered(...)`). No `EditorAbout::get_singleton()` exists, no
other file touches it. This is the cleanest deferred-construction opportunity found in this
session's whole run of dialog-bisection candidates - a real fix is attempted separately as
candidate 017.

`EditorFeatureProfileManager`, by contrast, is reached via `EditorFeatureProfileManager::
get_singleton()` from many other files (`editor_interface.cpp`, `editor_inspector.cpp`,
`scene_tree_dock.cpp`, `filesystem_dock.cpp`, `editor_help_search.cpp`, `create_dialog.cpp`,
`scene_create_dialog.cpp`, `game_view_plugin.cpp`) - several without a null-check, called from
dock-construction code that runs later in the same `EditorNode` constructor. Deferring it would
need the same broad audit this session's `ProjectSettingsEditor` attempt showed was necessary and
risky - not attempted. `ExportTemplateManager` and `EditorBuildProfileManager` have no external
`get_singleton()` callers found (same clean profile as `EditorAbout`) but were not investigated
further this session - left as leads in `IDEAS.md`.

## Does it change behavior/output?

**N/A - measurement/characterization only, no patch applied to real source** (per SKILL.md's
"patch.diff is mandatory..." section). `bench/instrumentation.diff`'s temporary
`Memory::get_mem_usage()` probes were reverted before finishing this candidate.

- Project test suite: not re-run separately for this purely additive `print_line`-only
  instrumentation - no changes to real source survived this candidate.

## Why the result looks the way it does

Same repeated pattern this whole session: every dialog/manager in this list is built unconditionally
in `EditorNode`'s constructor regardless of whether the user ever opens it in a given session -
`EditorAbout` is simply the most expensive single item found among this group.

## What's not done / open questions

- **`ExportTemplateManager` (2.58MB) and `EditorBuildProfileManager` (4.10MB) have no external
  `get_singleton()` callers found** (same quick grep used for `EditorAbout`) - both are plausible
  further deferred-construction candidates, not attempted this session; left in `IDEAS.md`.
- **`EditorAbout`'s own fix is attempted separately as candidate 017** - see that candidate's
  `RESULT.md`.
