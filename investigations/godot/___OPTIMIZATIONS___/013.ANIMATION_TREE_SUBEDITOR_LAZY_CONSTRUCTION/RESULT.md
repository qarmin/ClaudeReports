# Lazily construct `AnimationTreeEditor`'s 4 mutually-exclusive sub-editors

Candidate 012 found that `AnimationTreeEditorPlugin`'s 10.93MB startup cost is 97.2% (10.62MB)
just 4 near-equal sub-editors (`AnimationNodeBlendTreeEditor`, `AnimationNodeBlendSpace1DEditor`,
`AnimationNodeBlendSpace2DEditor`, `AnimationNodeStateMachineEditor`) built unconditionally in
`AnimationTreeEditor::AnimationTreeEditor()` via `add_plugin()`, even though at most one is ever
relevant for a given edited `AnimationNode` (each type's `can_edit()` is a simple, side-effect-free
`Ref<T>` cast check - no existing instance is required to determine which type matches). This
candidate implements lazy per-type construction: build only the one sub-editor actually needed,
the first time a node of that type is edited.

`editor/animation/animation_tree_editor_plugin.cpp`/`.h`: adds `AnimationTreeEditor::
_ensure_editor_for(const Ref<AnimationNode> &)`, which reuses an already-constructed editor if one
exists (also covers non-built-in editors registered externally via `add_plugin()`, e.g. by a
GDExtension) or lazily `memnew()`s + `add_plugin()`s exactly the one matching built-in type
otherwise. `edit_path()` now calls this to find (and construct on demand) the editor to show,
instead of asking each of the 4 pre-built instances `can_edit()`. `AnimationTreeEditor::can_edit()`
(a `const` query used by the sub-editors themselves to check whether a *different*, not-currently-
active node - e.g. one nested inside the currently open tree - is enterable) is fixed to also
consider not-yet-constructed built-in types via the same type checks, since it must stay correct
regardless of construction state. One fragile pre-existing call site was found and fixed:
`_meta_clicked()`'s "Special case for AnimationNodeBlendTree" branch called
`AnimationNodeBlendTreeEditor::get_singleton()` directly (not null-guarded, unlike its sibling call
site in `_animation_list_changed()`) - relying on a preceding `edit_path()` call to have already
constructed it, except that call is itself skipped when already on that path. Fixed to call
`_ensure_editor_for()` directly and null-check the result before use.

## What was tried

Harness: `bin/godot.linuxbsd.editor.x86_64 --headless --editor --path
___OPTIMIZATIONS___/harness/empty_project --quit-after 1`, same as candidates 006/007/010/011/012.
Peak RSS via `/proc/<pid>/status`'s `VmHWM` (no `/usr/bin/time` in this container).

| Variant | Peak RSS (VmHWM) | notes |
|---|---|---|
| baseline (eager construction of all 4 sub-editors) | 730397.6 kB mean (730984/728912/731412/729724/730956, 5 runs, 0.34% spread) | |
| patched (lazy per-type construction) | 714504.0 kB mean (714248/714572/714240/715100/714360, 5 runs, 0.12% spread) | |

Distributions don't overlap (baseline min 728912 > patched max 715100) - a real, clean separation,
not noise.

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: peak RSS only** (this candidate removes constructor-phase allocation of 4
UI widget trees, not a per-call hot path - no separate timing benchmark was built, matching how
candidates 006/007 measured this class of fix).

Peak RSS drops by **15.52MB (-2.18%)**, from 730.4MB to 714.5MB mean (5 vs 5 runs, no overlap). This
is somewhat larger than candidate 012's own 10.62MB `Memory::get_mem_usage()` estimate for the 4
sub-editors - expected, same non-1:1 relationship between the engine's own allocator-wrapper
counter and total process peak RSS already noted in candidate 007's own `RESULT.md` (allocator
fragmentation/reuse, page-level effects).

## Does it change behavior/output?

- **Bit-exact?** N/A (this is a UI/interactive-behavior change, not a computation whose output is
  hashed) - see interactive verification below instead.
- **Interactive verification**: built a scratch test project (not part of `patch.diff` - throwaway
  test scaffolding, not committed anywhere) with a `@tool` `EditorPlugin` autoload that creates an
  `AnimationTree` node, then cycles its `tree_root` through all 4 built-in types in sequence
  (`AnimationNodeBlendTree` -> `AnimationNodeBlendSpace1D` -> `AnimationNodeBlendSpace2D` ->
  `AnimationNodeStateMachine`), calling `get_editor_interface().edit_node(anim_tree)` after each
  swap and saving a screenshot. Ran under `Xvfb` (1280x800x24): **no crash, no script error, no
  `ERR_FAIL_INDEX_MSG`**. All 4 screenshots confirm the correct, visually distinct sub-editor UI
  renders for each type (BlendTree's node graph with "Add Node..."/"Error: Nothing connected to
  output"; BlendSpace1D's 1D axis editor with "Sync"/"Blend" controls; BlendSpace2D's 2D x/y grid
  editor; StateMachine's state graph with a "Start" node) - i.e. `_ensure_editor_for()` fires at the
  right time for each type and produces the same visual result the old eager construction would
  have. Saved screenshots: `bench/screenshot_blend_tree.png`, `bench/screenshot_blend_space_1d.png`,
  `bench/screenshot_blend_space_2d.png`, `bench/screenshot_state_machine.png`.
- **Is this a breaking change?** No - `AnimationTreeEditor`'s public-facing behavior (what the user
  sees when editing any of the 4 node types, in any order, including switching between them
  repeatedly) is unchanged; only *when* each sub-editor is constructed changes (lazily on first use
  of that specific type, instead of eagerly at plugin startup for all 4).
- Project test suite: 1427 cases, 1426 passed, 1 failed (known pre-existing `test_ip.cpp` IPv6
  failure, no IPv6 in this container), 3 skipped - identical before and after.

## Why the result looks the way it does

Same repeated pattern (candidates 006/007/010/011/012): eager construction of UI regardless of
use. This case is the cleanest version of the pattern found so far because the codebase already had
an explicit extension mechanism (`add_plugin()` + `can_edit()` type-check dispatch) designed for
exactly this kind of "one of several alternatives" selection - the fix only had to change *when*
`add_plugin()` is called for the 4 built-in types, not invent a new dispatch mechanism.

## What's not done / open questions

- **The other three of candidate 006's original top-5 plugins** (`ShaderEditorPlugin`,
  `TileSetEditorPlugin`, `SpriteFramesEditorPlugin`) have no equivalent internal sub-editor
  structure to lazily construct - a fix for any of them would mean deferring the entire dock, the
  same higher-risk pattern as this session's ruled-out `ProjectSettingsEditor` attempt. Not
  attempted - see `IDEAS.md`.
- Not verified: switching AWAY from a constructed sub-editor and back again (e.g. BlendTree ->
  StateMachine -> BlendTree) - the interactive test only exercised each type once, in sequence,
  never returning to an already-constructed one. `edit_path()`'s logic reuses the already-existing
  instance via `_ensure_editor_for()`'s first loop in this case (no new construction), which is a
  straightforward, low-risk path - but not explicitly screenshotted.
- Not verified: a GDExtension registering a 5th custom `AnimationTreeNodeEditorPlugin` via
  `add_plugin()` at runtime - reasoned through (the "already-constructed" loop in
  `_ensure_editor_for()`/`can_edit()` checks it before falling through to the built-in type list),
  not tested with an actual extension.
