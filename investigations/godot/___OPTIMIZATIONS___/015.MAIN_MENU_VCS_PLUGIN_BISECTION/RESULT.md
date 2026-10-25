# Bisecting candidate 010's "5 main-menu PopupMenus cost 8.65MB" finding - corrects the original guess

Candidate 010 found the 5 main-menu `PopupMenu`s (Scene/Project/Debug/Editor/Help) cost 8.65MB
combined at empty-project startup, and guessed ("surprising for what looks like simple menu
shells") that per-item icon loading or eager submenu population might be the driver, without
bisecting further. This candidate bisects `EditorNode::_update_main_menu_type()`'s 4
`_build_*_menu()` calls individually, and finds the guess was wrong: it isn't icon loading or
submenu population in general - it's one specific unconditional trigger of an unrelated, unused
built-in plugin's construction.

## What was tried

Harness: same as candidates 006/007/010/011/012 - `bin/godot.linuxbsd.editor.x86_64 --headless
--editor --path ___OPTIMIZATIONS___/harness/empty_project --quit-after 1`. Temporary
`Memory::get_mem_usage()` checkpoints added around each of `_update_main_menu_type()`'s 4
`_build_*_menu()` calls, then a second, finer-grained pass of checkpoints inside
`_build_project_menu()` itself once it was identified as the dominant cost - see
`bench/instrumentation.diff`. Fully deterministic across repeat runs (bit-identical byte counts,
0% spread on stable runs), matching prior candidates' own observations for this counter.

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: Godot's own engine-tracked heap (`Memory::get_mem_usage()`), for attribution
only - same as prior bisection candidates.**

| Function | Delta | % of 6.91MB total |
|---|---|---|
| `_build_file_menu()` | +0.28MB | 4.1% |
| **`_build_project_menu()`** | **+6.44MB** | **93.2%** |
| `_build_settings_menu()` | +0.14MB | 2.1% |
| `_build_help_menu()` | +0.04MB | 0.6% |

(Total 6.91MB here vs. candidate 010's 8.65MB - measured on a fresh build without other
candidates' instrumentation present, expected small variance already noted in candidates 010/011.)

**Bisecting `_build_project_menu()` further pinpoints the exact line**: of its 6.44MB, **6.28MB
(97.5%)** happens at a single statement - `vcs_actions_menu = VersionControlEditorPlugin::
get_singleton()->get_version_control_actions_panel();` (`editor/editor_node.cpp`, inside the
`if (!vcs_actions_menu)` block that wires up the Project menu's "Version Control" submenu).
`VersionControlEditorPlugin::get_singleton()` is a **lazy self-constructing singleton** (`return
singleton ? singleton : memnew(VersionControlEditorPlugin);`) - this is the *first* call to it
anywhere in `EditorNode`'s constructor (menu-building happens before the later plugin-instantiation
block), so it triggers full construction of the entire `VersionControlEditorPlugin` right there.

## Why the result looks the way it does

Candidate 010's original guess (per-item icon loading, general eager submenu population) was
**wrong** - every other menu item across all 5 menus combined costs under 0.5MB total. The real
driver is a single unrelated built-in plugin's construction being pulled forward by an incidental
early call to its lazy-singleton accessor.

**This is corrected, useful information, but does not translate into a viable fix**:
`VersionControlEditorPlugin` is *also* unconditionally added via `add_editor_plugin(
VersionControlEditorPlugin::get_singleton())` later in the same constructor (`editor_node.cpp`,
the hand-added "core" plugins section, not the generic `EditorPlugins::create(i)` loop) -
regardless of whether the current project uses version control. Since `get_singleton()` is called
there too, `VersionControlEditorPlugin` gets constructed **somewhere in the startup sequence
either way**; deferring the Project-menu code's own call to it would only move *when* within the
same `--quit-after 1` run the 6.28MB gets allocated (earlier, during menu-building, vs. slightly
later, during the plugin-add block) - both happen before quit, so peak RSS would be unchanged. A
real fix would require making `VersionControlEditorPlugin` itself not unconditionally constructed
for projects that don't use version control - but the Project menu's "Version Control" submenu
(`Create/Override Version Control Metadata...`, `Version Control Settings...`) is *also* the
entry point for **setting up** VCS on a project that doesn't have it yet, so the plugin genuinely
needs to be reachable regardless of whether the current project already uses VCS. This makes
"only construct if already in use" a poor fit for this specific case, unlike candidate 013's
`AnimationTreeEditor` sub-editors (which have no such "always must be reachable" requirement).

## Does it change behavior/output?

**N/A - measurement/characterization only, no patch applied to real source** (per SKILL.md's
"patch.diff is mandatory..." section - a correct "investigated, fix not viable" outcome, not a
"didn't get to it" one). `bench/instrumentation.diff`'s temporary `Memory::get_mem_usage()` probes
were reverted before finishing this candidate.

- Project test suite: not re-run separately for this purely additive `print_line`-only
  instrumentation - no changes to real source survived this candidate.

## What's not done / open questions

- **`VersionControlEditorPlugin`'s own ~6.3MB construction cost itself was not further bisected**
  (what inside its own constructor is expensive) - not investigated this session, since even a
  full internal breakdown wouldn't lead to a viable fix given the "must always be reachable"
  constraint established above. Could still be worth a future look purely for its own sake (is
  6.3MB reasonable for what's ostensibly just a menu/dialog shell before any VCS provider is
  configured?), independent of the menu-timing angle this candidate investigated.
- `_build_project_menu()`'s remaining ~0.16MB (6.44MB - 6.28MB) and `_build_file_menu()`'s 0.28MB
  were not bisected per-item - both small enough that further bisection is low value.
