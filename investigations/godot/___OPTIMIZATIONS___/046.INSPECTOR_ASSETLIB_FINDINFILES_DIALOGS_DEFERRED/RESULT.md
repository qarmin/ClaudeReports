# 046. Three more single-site `EditorFileDialog`s deferred (Inspector dock, Asset Library, Find in Files)

- **Project**: godotengine/godot
- **Base commit**: `9552dfb6859a1aaba1e570b8e0ef5c599b830f19`
- **Files**: `editor/docks/inspector_dock.{cpp,h}`, `editor/asset_library/asset_library_editor_plugin.{cpp,h}`,
  `editor/script/find_in_files.{cpp,h}` (**+50/-22 lines**)
- **Metric(s) measured**: Godot's tracked peak heap (primary) and peak RSS (secondary)
- **Workload**: empty-project editor startup,
  `--headless --editor --path harness/empty_project --quit-after 1`

## What was tried

Continuing candidate 021's per-call-site direction, using candidate 041's ancestor-chain attribution
to pick owners rather than working through `grep` hits. Three owners each build exactly one
`EditorFileDialog` at startup and pop it from exactly one place:

| owner | member | only reachable from |
|---|---|---|
| `InspectorDock` | `load_resource_dialog` | the Load-resource menu action (`_load_resource()`) |
| `EditorAssetLibrary` | `asset_open` | the "Import..." action (`_asset_open()`) |
| `FindInFilesSearchPanel` | `folder_dialog` | the "Select Folder" button (`_on_folder_button_pressed()`) |

All three are private members with no external references, so the singleton-audit blocker does not
apply. Each was converted to a construct-on-first-call accessor (`_get_load_resource_dialog()`,
`_get_asset_open()`, `_get_folder_dialog()`), with every configure-then-popup line routed through it
per candidate 026's trap (`InspectorDock::_load_resource()` sets a file mode and rebuilds filters
before popping, so all five of its lines had to be routed, not just the popup).

## Result

**Tracked peak heap: -6244224 bytes (-5.95 MB, -1.39%)**, 448282957 -> 442038733 B, byte-identical
across 3 runs per arm.
Tracked live heap at `Main::cleanup()`: -529928 B (-0.51 MB).

**Peak RSS: -8.07 MB (-1.15%)**, 718393.3 -> 710130.0 kB mean (-8.10 MB by median), 6 interleaved
pairs, patched lower in **6/6**. Two of the twelve runs landed in the harness's known low mode (see
candidate 035), but mean and median agree to within 0.03 MB, so the mode does not drive the result.
RSS/tracked ratio 1.36x.

That is ~2.0MB of tracked peak per dialog, matching candidate 036's figure for a *configured*
`EditorFileDialog`.

## Does it change behavior/output?

Verified under Xvfb with the same plugin run against **both** arms
(`bench/verify_plugin.gd`, full transcript in `bench/measurements.txt`):

| | baseline | patched |
|---|---|---|
| startup `EditorFileDialog` count | 64 | **61** |
| after pressing the real "Select Folder" button | 64 | **62** |
| resulting visible dialog title | `Open a Directory` | `Open a Directory` |
| its `file_mode` | 2 (`FILE_MODE_OPEN_DIR`) | 2 |

The startup drop is exactly the three deferred dialogs (and the baseline's 64 independently
reproduces candidate 021's headless counter). The button was located by **accessibility name**
rather than by index, per candidate 034's lesson. Checking `file_mode` as well as the title matters
here: it confirms the deferred dialog is built *with its original configuration*, not merely built -
the failure mode a count alone would miss.

**Full test suite, both arms, identical**: 1427 cases, 1426 passed, 1 failed, 3 skipped (the known
`test_ip.cpp` IPv6 failure).

One deliberate structural difference: `InspectorDock`'s dialog is now parented to the dock itself
rather than to `main_vb`, because `main_vb` is a **constructor-local** `VBoxContainer`, not a member,
so an accessor cannot reach it. For a dialog this is behaviorally equivalent - a dialog is a `Window`
and both parents sit inside the same enclosing `Window`, which is what determines its transient
parent - but the node path differs, which is observable to anything walking the tree.

## Why the result looks the way it does

Nothing subtle: three configured `EditorFileDialog`s at ~2MB of tracked peak each are simply not
built until their one flow runs. The interest is in the selection method - candidate 041's
attribution file named the owners directly, so no per-file `grep` audit of the 66
`memnew(EditorFileDialog)` sites was needed to find three that are single-site and private.

## What's not done / open questions

- **This is a long tail, not a ceiling.** 61 `EditorFileDialog` instances remain at startup after
  this patch. The remaining named owners from candidate 041's attribution are `SpriteFramesEditor`
  (2), `EditorFeatureProfileManager` (2, deliberately ruled out as too risky - see `IDEAS.md`),
  `TileProxiesManagerDialog` (1), plus the `EditorNode`-owned ones candidate 024 left eager on
  purpose (notably `file`, the general scene open/save dialog, which a real session actually uses).
- The other two flows (Inspector "Load resource", Asset Library "Import...") were **not** driven
  interactively - only the Find-in-Files one was, plus the startup count that covers all three. Both
  are the same mechanism as five previously-verified candidates, but that is an argument from
  precedent, not a measurement.
- Not measured on a **real project**: an editor session that opens the Inspector's load-resource
  dialog once pays the ~2MB back at that moment.
