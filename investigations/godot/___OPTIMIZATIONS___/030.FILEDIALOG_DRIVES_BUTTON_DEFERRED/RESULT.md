# Defer `FileDialog`'s `drives` MenuButton until the dialog is actually used

Third and final `FileDialog`-internal deferral of this session, after candidate 023 (5 nested
sub-dialogs, -46.39MB) and candidate 029 (right-click context menu, -8.63MB). `FileDialog`
constructs a `drives` `MenuButton` - ~116KB including its own `PopupMenu`, per candidate 027's
census - in its constructor, multiplied by the 64 `FileDialog` instances candidate 021 counted at
empty-project startup.

`drives` is only ever *shown* by `_update_drives()`, which runs when the dialog is actually used (a
directory change or an invalidate), not at construction. This candidate creates the button there
instead, and null-guards the handful of other places that read it.

`scene/gui/file_dialog.cpp`/`.h`: adds `_ensure_drives()`, called from the one branch of
`_update_drives()` that shows the button; the two `drives->hide()` paths become no-ops when it
doesn't exist yet; and the two places that use `drives->is_visible()` as a *logic* guard
(`_change_dir`'s path resolution and the drive-selection sync) become `drives && drives->is_visible()`.
`drives_container` (a cheap `HBoxContainer`) is left eager, and `_ensure_drives()` parents the
button into it directly rather than relying on `_update_drives()`'s later reparenting.

## What was tried

Harness: `bin/godot.linuxbsd.editor.x86_64 --headless --editor --path
___OPTIMIZATIONS___/harness/empty_project --quit-after 1`. Peak RSS via `VmHWM`, exit codes checked.

| Variant | Peak RSS (VmHWM) | notes |
|---|---|---|
| baseline | 712.15MB mean over 8 runs (0.51% spread) | same pristine `HEAD` baseline as candidates 023-026, 029 |
| patched | 705.14MB mean over 5 runs (722080/722328/722140/721180/722584 kB, 0.19% spread) | all exit 0 |

Distributions don't overlap (baseline min 727268 > patched max 722584).

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: peak RSS only.**

**Peak RSS drops by 7.01MB (-0.98%)**, from 712.15MB to 705.14MB (8 baseline vs 5 patched runs, no
overlap). That is 0.97x the ~7.25MB predicted from 64 instances x candidate 027's 116KB mean
`MenuButton` cost - a ratio near 1.0 rather than this workspace's usual 1.4-1.7x, which makes sense
here because the census figure for `MenuButton` was already *inclusive* of its nested `PopupMenu`,
whereas the earlier predictions were built from single-widget figures.

## Does it change behavior/output?

- **Bit-exact?** N/A (UI/interactive-behavior change) - see verification below.
- **Verified with a baseline control** via a temporary `TEST_CASE`
  (`bench/scratch_verification_test.cpp`, reverted before finishing - not part of `patch.diff`),
  counting `MenuButton`s in a fresh `FileDialog` and then again after `set_access(ACCESS_FILESYSTEM)`
  + `set_current_dir()`, which is what actually runs `_update_drives()`:

  | Check | baseline (control) | patched |
  |---|---|---|
  | `MenuButton`s in a fresh `FileDialog` | **2** (file-sort + drives) | **1** (file-sort only) |
  | after `set_access` + `set_current_dir` | 2 | **2** (drives created on demand) |
  | resulting `get_current_dir()` | `/home/rafal/.cache` | `/home/rafal/.cache` (identical) |

  The third row matters as much as the first: it exercises the `_change_dir` path whose guard I
  changed to `drives && drives->is_visible()`, and the resulting directory is identical.
- **A wrong premise, caught by this verification - worth recording.** This candidate was started on
  the assumption that the drives button is *permanently hidden* on Unix-like systems (so could be
  skipped outright there, not merely deferred). The test printed
  `platform drive_count=19`: Godot's Unix `DirAccess::get_drive_count()` returns **mounted
  filesystems**, not Windows-style drive letters, so the button genuinely is shown on Linux once a
  dialog is used. The measured win is unaffected (it comes from the 64 dialogs that are never
  opened at startup), but the *mechanism* is ordinary deferral-until-first-use, and the code
  comments were corrected before capturing `patch.diff` - they had claimed the Unix-never-shown
  story, which would have misled a reviewer.
- **Is this a breaking change?** No behavioral difference observed. `drives` is private; the
  apparent external references are other classes' own identically-named members.
- Project test suite: 1427 cases, 1426 passed, 1 failed (known pre-existing `test_ip.cpp` IPv6
  failure), 3 skipped - identical before and after.

## Why the result looks the way it does

Same "pay only when used" pattern as the rest of the session. The reason it works here despite the
button being genuinely reachable is that `_update_drives()` is driven by *use* of the dialog, and
most of the 64 `FileDialog`s alive at editor startup are never opened at all.

## What's not done / open questions

- Only the `set_access` + `set_current_dir` path was exercised. `_update_drives()`'s
  `drives_are_shortcuts()` branch (which reparents the button to `shortcuts_container` rather than
  `drives_container`) is only taken on platforms where drives are shortcuts, so on this container
  the reparenting logic after lazy creation was not exercised - worth checking on Windows/macOS
  before this would be upstream-ready.
- Per candidate 022's bisection, `FileDialog` still constructs ~0.9MB of main widget tree per
  instance after candidates 023/029/030. What remains is genuinely-visible UI (nav buttons, file
  list, filter boxes, layout/sort controls), so further gains would need a different approach than
  deferral - e.g. reducing Godot's per-`Control` overhead itself, which candidate 022 measured at
  roughly 36KB per control and is an engine-architecture question rather than a bounded candidate.
