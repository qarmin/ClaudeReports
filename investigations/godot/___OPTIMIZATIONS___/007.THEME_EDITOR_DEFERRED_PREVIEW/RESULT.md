# Defer ThemeEditor's default preview construction until the dock is first shown

Candidate 006 root-caused `ThemeEditorPlugin` as the #2 most expensive of 74 built-in editor
plugins at empty-project startup (11.07MB), because `ThemeEditor::ThemeEditor()` unconditionally
built a full `DefaultThemeEditorPreview` widget gallery even though the dock starts closed. This
candidate implements the deferred-construction fix candidate 006 identified but did not attempt,
including the correctness risk candidate 006 flagged (tab-index-to-child-index correspondence)
and the interactive verification candidate 006 said it couldn't do headlessly.

`editor/scene/gui/theme_editor_plugin.cpp`/`.h`: adds `ThemeEditor::_ensure_default_preview()`,
which `memnew()`s `DefaultThemeEditorPreview` and inserts it as child index 0 of
`preview_tabs_content` only the first time it's needed, instead of unconditionally in the
constructor. Called from every path that can make tab 0 visible before an explicit user
interaction: `_add_preview_tab()`, `_change_preview_tab()`, `drop_data_fw()`, and a new
`NOTIFICATION_VISIBILITY_CHANGED` handler (covers the dock simply becoming visible without any of
the other three firing first).

## What was tried

Harness: `bin/godot.linuxbsd.editor.x86_64 --headless --editor --path
___OPTIMIZATIONS___/harness/empty_project --quit-after 1`, same as candidate 006. Peak RSS via
`/proc/<pid>/status`'s `VmHWM` (no `/usr/bin/time` in this container).

| Variant | Peak RSS (VmHWM) | notes |
|---|---|---|
| baseline (eager preview construction) | 716.11MB mean (714852/717672/716488/716316/715228 kB, 5 runs, 0.4% spread) | matches candidate 006's own 716.5MB baseline |
| patched (deferred preview construction) | 708.26MB mean (708612/708932/707264/708240 kB, 4 runs, 0.2% spread) | one additional first-run outlier (737824 kB) excluded as a cold-cache effect (import-cache creation on the very first launch against this harness project after a fresh `.godot/` folder state) - see "What's not done" |

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: peak RSS only** (this candidate removes a one-time constructor-phase
allocation of a UI widget gallery, not a per-call hot path - no separate timing benchmark was
built, matching how candidate 006 itself measured this).

Peak RSS drops by **7.85MB (-1.1%)**, from 716.11MB to 708.26MB mean, comparing 5 baseline runs to
4 stable patched runs (excluding one cold-cache outlier run - see above). This is smaller than
candidate 006's originally attributed 11.07MB for `DefaultThemeEditorPreview`'s construction cost,
which is expected: `Memory::get_mem_usage()` (what 006 used for attribution) tracks Godot's own
allocator wrapper specifically, while `VmHWM` (what's compared here) is total process peak RSS,
subject to allocator fragmentation/reuse and overlap with peak timing elsewhere in startup - the
two are related but not expected to match 1:1. The direction and rough magnitude are consistent
with the fix actually working.

## Does it change behavior/output?

- **Bit-exact?** N/A (this is a UI/interactive-behavior change, not a computation whose output is
  hashed) - see interactive verification below instead.
- **Interactive verification** (candidate 006's own blocker - no display server - is resolved,
  `Xvfb`/`xvfb-run` are available in this container): built a scratch test project with a `.tres`
  Theme resource and a temporary `@tool` `EditorPlugin` autoload
  (`addons/theme_autotest/plugin.gd`, not part of `patch.diff` - throwaway test scaffolding, not
  committed anywhere) that calls `get_editor_interface().edit_resource(theme)` on startup, which
  triggers `ThemeEditorPlugin::edit()` -> `make_visible(true)`, then waits several seconds and
  quits. Ran under `Xvfb` (`:78`, 1280x800x24): **no crash, no error, no `ERR_FAIL_INDEX_MSG`** (the
  guard candidate 006 flagged as the correctness risk if this ever fired stale). A screenshot taken
  mid-run (`bench/theme_dock_after_defer_screenshot.png`) confirms the "Default Preview" tab
  renders its full widget gallery correctly (Label, Button, LineEdit, Disabled LineEdit, Toggle
  Button, TextEdit, Tab 1, etc.) the first time the dock is shown - i.e. `_ensure_default_preview()`
  fires at the right time and produces the same visual result as the old eager construction would
  have.
- **Is this a breaking change?** No - `ThemeEditor`'s public-facing behavior (what the user sees
  when they open a Theme resource) is unchanged; only *when* the preview widget gallery is
  constructed changes (lazily on first show, instead of eagerly at editor startup).
- Project test suite: 1427 cases, 1426 passed, 1 failed (known pre-existing `test_ip.cpp` IPv6
  failure, no IPv6 in this container), 3 skipped - identical before and after. (Also confirmed the
  pre-existing, unrelated `"Drawing is only allowed inside this node's _draw()..."` doctest error
  noise appears identically in an unrelated near-baseline build, i.e. it's not something this patch
  introduced.)

## Why the result looks the way it does

`DefaultThemeEditorPreview`'s constructor builds ~34-41 sample UI controls (buttons, sliders,
trees, etc.) purely to demonstrate how a theme renders - useful only once a user actually opens the
Theme dock, never needed for `handles()`/plugin-registration bookkeeping the way some other eager
plugin construction is. Deferring it to first-show is a straightforward "pay only when used"
change; the interactive check confirms the four call sites (`_add_preview_tab`,
`_change_preview_tab`, `drop_data_fw`, `NOTIFICATION_VISIBILITY_CHANGED`) cover every path that can
make the tab visible, since the real editor session went through `make_visible(true)` ->
visibility-changed -> `_ensure_default_preview()` -> `_change_preview_tab` cleanly with no
index-mismatch error.

## What's not done / open questions

- **The 737824 kB cold-cache outlier run was excluded, not explained.** It only appeared once, on
  the very first patched-binary run against a `.godot/` folder that had just been recreated for
  this session - plausibly one-time shader/import-cache generation, not something a real user would
  hit repeatedly. Not confirmed by re-running with a controlled cold `.godot/` state each time
  (would require deleting `.godot/` before every single run instead of just the first).
- **Tab-switching away from and back to the Default Preview tab was not separately re-tested**
  after the fix (only the initial "dock opens, default preview tab is the visible one" path was
  screenshotted) - `_change_preview_tab()` was exercised as part of the normal
  `edit_resource()` flow (it's called once to select tab 0 initially) but not by manually clicking
  between multiple preview tabs in sequence, which would need real UI input events (no `xdotool` in
  this container) rather than the scripted `edit_resource()` call used here.
- The other 4 of candidate 006's top-5 most expensive plugins (`AnimationTreeEditorPlugin`,
  `ShaderEditorPlugin`, `TileSetEditorPlugin`, `SpriteFramesEditorPlugin`) were not investigated in
  this candidate - still open in `IDEAS.md`.
