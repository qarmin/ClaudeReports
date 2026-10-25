# Defer `EditorBuildProfileManager`'s construction until first opened

Candidate 016 found `EditorBuildProfileManager` (4.10MB) has no external `get_singleton()`
callers, matching `EditorAbout`'s clean profile. This candidate confirms it has exactly **one**
call site in the entire codebase (`EditorNode::_tool_menu_option()`'s `TOOLS_BUILD_PROFILE_MANAGER`
case, reached from Project -> Tools -> "Engine Compilation Configuration Editor...") and implements
the same deferred-construction fix as candidates 007/013/017.

`editor/editor_node.cpp`/`.h`: adds `EditorNode::_ensure_build_profile_manager()`, which
`memnew()`s `EditorBuildProfileManager` and adds it to `gui_base` only the first time it's needed.

## What was tried

Harness: `bin/godot.linuxbsd.editor.x86_64 --headless --editor --path
___OPTIMIZATIONS___/harness/empty_project --quit-after 1`, same as candidates 006/007/013/017. Peak
RSS via `/proc/<pid>/status`'s `VmHWM` (no `/usr/bin/time` in this container).

| Variant | Peak RSS (VmHWM) | notes |
|---|---|---|
| baseline (eager construction) | 729932.0 kB mean (730292/730736/730360/728664/729608, 5 runs, 0.28% spread) | |
| patched (deferred construction) | 721894.4 kB mean (721900/722388/722588/721656/720940, 5 runs, 0.23% spread) | |

Distributions don't overlap (baseline min 728664 > patched max 722588) - a real, clean separation.

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: peak RSS only** (constructor-phase allocation removal, not a per-call hot
path - matching how candidates 007/013/017 measured this class of fix).

Peak RSS drops by **7.85MB (-1.10%)**, from 729.9MB to 721.9MB mean (5 vs 5 runs, no overlap).
Smaller than candidate 016's 4.10MB `Memory::get_mem_usage()` estimate would suggest at first
glance, but consistent with the same non-1:1 relationship between the engine's own allocator
counter and total process peak RSS already noted repeatedly this session - the measured RSS delta
(7.85MB) is actually *larger* than the attributed figure, same direction as candidates 007/013/017.

## Does it change behavior/output?

- **Bit-exact?** N/A (UI/interactive-behavior change) - see interactive verification below.
- **Interactive verification**: built a scratch test project (not part of `patch.diff`) with a
  `@tool` `EditorPlugin` autoload that finds the Project menu's "Tools" submenu, then the "Engine
  Compilation Configuration Editor..." item inside it, and emits the tool submenu's own
  `index_pressed` signal (the same signal path a real click goes through -
  `PopupMenu::index_pressed` -> `EditorNode::_tool_menu_option` -> `TOOLS_BUILD_PROFILE_MANAGER`).
  Ran under `Xvfb` (1280x800x24): **no crash, no script error**. The resulting
  `EditorBuildProfileManager` window was found in the scene tree, visible, titled "Edit Compilation
  Configuration Profile" - screenshot (`bench/screenshot_build_profile.png`) confirms the full
  profile-editing UI (feature checkboxes, Save/Load/Reset/Detect actions) renders correctly.
- **Is this a breaking change?** No - only *when* the dialog's widgets are constructed changes.
- Project test suite: 1427 cases, 1426 passed, 1 failed (known pre-existing `test_ip.cpp` IPv6
  failure, no IPv6 in this container), 3 skipped - identical before and after.

## Why the result looks the way it does

Same repeated pattern this session: a single, unambiguous call site with no external singleton or
cross-file dependency, following the exact heuristic recorded in `projects/godot.md` after
candidates 007/013/017.

## What's not done / open questions

- **`ExportTemplateManager` (2.58MB)**, the last untried member of candidate 016's group, has
  ~12 internal call sites in `editor_node.cpp` itself (all Android-build-template-setup related:
  `_android_build_source_selected`, install/remove/validate template flows,
  `PROJECT_SETUP_ANDROID_BUILD`/`EDITOR_MANAGE_EXPORT_TEMPLATES` menu handlers) rather than
  `EditorAbout`/`EditorBuildProfileManager`'s single call site - needs auditing that all of them
  are only reachable from genuine user-initiated Android-export actions (not any automatic/startup
  path) before attempting the same fix. Not attempted this session; left in `IDEAS.md`.

## Independent reproduction from a fresh clone (2026-09-10)

This candidate's `just test` was run end-to-end for real (not just read) in a scratch location, as
the session's designated `just test` validation candidate. The full pipeline - `clone` (HTTPS,
full history, `git checkout` of the pinned `base_commit`), `build-baseline` (4m56s), `bench-baseline`,
`apply`, `build-patched` (0m56s), `bench-patched`, `clean` - completed successfully and printed a
real before/after:

| Variant | Peak RSS (VmHWM), fresh clone | 
|---|---|
| baseline | 713468 / 717460 / 717484 kB, mean 716137 kB (699.35MB) |
| patched | 711028 / 704236 / 709780 kB, mean 708348 kB (691.75MB) |

**Delta -7.61MB (-1.09%)**, independently reproducing this candidate's own in-tree measurement of
-7.85MB (-1.10%) - agreement within ~3% on the absolute delta and within 0.01pp on the percentage.
(Absolute values are ~14MB lower than the in-tree run because the fresh clone starts from a
pristine `.godot`/editor-config state; the *delta* is what reproduces, which is the claim.)

**Three real justfile bugs were found and fixed by this run** - see `README.md`'s "Reproducibility
bugs found by actually running `just test`" section. They affected this and most other candidates'
justfiles, and would have made every one of them fail for any reader who tried them.
