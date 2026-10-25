# Bisecting the remaining 4 of candidate 006's top-5 most expensive built-in editor plugins

Candidate 006 found the per-plugin cost of all 74 built-in `EditorPlugin`s at empty-project
startup, and candidate 007 root-caused and fixed the #1 (`ThemeEditorPlugin`, 11.07MB). This
candidate root-causes the remaining top-5 entries: `AnimationTreeEditorPlugin` (11.49MB),
`ShaderEditorPlugin` (9.26MB), `TileSetEditorPlugin` (6.32MB), `SpriteFramesEditorPlugin` (5.31MB)
- using the same `Memory::get_mem_usage()` checkpoint technique.

## What was tried

Harness: same as candidates 006/007/010/011 - `bin/godot.linuxbsd.editor.x86_64 --headless
--editor --path ___OPTIMIZATIONS___/harness/empty_project --quit-after 1`. Temporary
`Memory::get_mem_usage()` checkpoints added at the start/end of each of the 4 plugins' own
constructors (`editor/animation/animation_tree_editor_plugin.cpp`,
`editor/shader/shader_editor_plugin.cpp`, `modules/tilemap/editor/tiles_editor_plugin.cpp`,
`editor/scene/sprite_frames_editor_plugin.cpp`), plus 5 additional checkpoints inside
`AnimationTreeEditor::AnimationTreeEditor()` around each of its 4 `add_plugin()` calls (the one
plugin with an obvious internal multi-part structure) - see `bench/instrumentation.diff`. Fully
deterministic across 3 repeat runs (bit-identical byte counts at every checkpoint, 0% spread),
matching the determinism candidates 006/007/010/011 already observed for this counter.

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: Godot's own engine-tracked heap (`Memory::get_mem_usage()`), for attribution
only - same as candidates 006/007/010/011's own breakdowns.**

| Plugin | Total (start->end) | vs candidate 006's estimate | Internal breakdown |
|---|---|---|---|
| `AnimationTreeEditorPlugin` | **10.93MB** | 11.49MB | see below - dominated by 4 near-equal sub-editors |
| `ShaderEditorPlugin` | **8.81MB** | 9.26MB | 100% one `ScriptEditor` instance (generic multi-file code editor, shared machinery with the main Script dock) - no internal sub-parts to bisect |
| `TileSetEditorPlugin` | **6.01MB** | 6.32MB | 100% one `TileSetEditor` instance - no internal sub-parts to bisect |
| `SpriteFramesEditorPlugin` | **5.04MB** | 5.31MB | 100% one `SpriteFramesEditor` instance - no internal sub-parts to bisect |

All 4 totals are close to (slightly under) candidate 006's original per-plugin estimates - expected,
same "measured on a fresh build without other candidates' instrumentation present" effect noted in
candidates 010/011.

**`AnimationTreeEditorPlugin`'s internal breakdown is the standout finding**:

| Segment | Delta | % of AnimationTreeEditorPlugin's 10.93MB |
|---|---|---|
| plugin-level overhead (before `AnimationTreeEditor` construction) | +0.003MB | ~0% |
| `AnimationTreeEditor`'s own UI shell (path bar, split, status bar, error button) | +0.16MB | 1.5% |
| **`AnimationNodeBlendTreeEditor`** | **+2.54MB** | **23.2%** |
| **`AnimationNodeBlendSpace1DEditor`** | **+2.60MB** | **23.8%** |
| **`AnimationNodeBlendSpace2DEditor`** | **+2.78MB** | **25.4%** |
| **`AnimationNodeStateMachineEditor`** | **+2.70MB** | **24.7%** |
| error panel (scroll container + label) + plugin's own trailing setup | +0.16MB | 1.5% |

**The 4 sub-editors together account for 10.62MB - 97.2% of the whole plugin's cost** - and, unlike
`ShaderEditorPlugin`/`TileSetEditorPlugin`/`SpriteFramesEditorPlugin` (one monolithic dock each,
nothing to defer without deferring the entire dock), `AnimationTreeEditor` already has a clean,
existing extension point (`add_plugin()`/`AnimationTreeNodeEditorPlugin`, a `Vector` of sub-editors
selected by an `AnimationNode`-subtype `can_edit()` check) built unconditionally for **all 4 known
`AnimationNode` container types even though at most one type is ever being edited at a time** for a
given `AnimationTree` resource.

## Does it change behavior/output?

**N/A - measurement/characterization only, no patch applied to real source** (per SKILL.md's
"patch.diff is mandatory..." section). `bench/instrumentation.diff`'s temporary
`Memory::get_mem_usage()` probes were reverted before finishing this candidate.

- Project test suite: not re-run separately for this purely additive `print_line`-only
  instrumentation, consistent with candidates 006/010/011 - no changes to real source survived
  this candidate.

## Why the result looks the way it does

Same repeated architectural pattern (candidates 006/007/010/011): Godot's editor builds every
widget it might ever need up front, regardless of whether a given project's session ever needs it.
`AnimationTreeEditorPlugin` is the most extreme case of this pattern found so far in the plugin
loop: it doesn't just eagerly build ONE thing that might not be needed (like candidate 007's single
preview widget) - it eagerly builds all 4 mutually-exclusive alternative sub-editors for a resource
type (`AnimationTree`) that a majority of projects (anything not using animation trees at all) never
even instantiate.

## What's not done / open questions

- **A fix for `AnimationTreeEditorPlugin`'s 4-sub-editor cost is attempted separately as candidate
  013** (lazy per-type construction via the existing `can_edit()` type-check mechanism) - see that
  candidate's `RESULT.md`.
- `ShaderEditorPlugin`, `TileSetEditorPlugin`, `SpriteFramesEditorPlugin` have no internal sub-parts
  to defer - a fix for any of them would need to defer the ENTIRE dock's construction (the same
  higher-risk pattern as this session's ruled-out `ProjectSettingsEditor` attempt - see `IDEAS.md`).
  Not attempted this session; `ShaderEditorPlugin` in particular shares its `ScriptEditor` machinery
  with the main code-editing dock, making it the most deeply cross-referenced of the three and the
  least promising candidate for a quick, safe deferred-construction fix.
