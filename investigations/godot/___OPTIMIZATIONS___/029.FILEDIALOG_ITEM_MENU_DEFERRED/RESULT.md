# Defer `FileDialog`'s right-click context menu (`item_menu`)

Candidate 022 bisected `FileDialog`'s per-instance cost and candidate 023 took its largest
deferrable cluster (the 5 nested sub-dialogs, -46.39MB). This candidate takes the next one from the
same file: `item_menu`, the right-click context menu, a `PopupMenu` costing ~96.5KB per candidate
027's census - multiplied by the 64 `FileDialog` instances candidate 021 counted at empty-project
startup.

It is the cleanest deferral in this whole workspace: **every use of `item_menu` lives inside a
single function**, `FileDialog::_popup_menu()` (the right-click handler), which *already*
`clear()`s the menu and rebuilds its items from scratch on every invocation - so there is no state
to preserve by creating it up front. `LineEdit::_generate_context_menu()` and
`TextEdit::_generate_context_menu()` already do exactly this for their own context menus (see
candidate 027), making this idiomatic rather than novel.

`scene/gui/file_dialog.cpp`/`.h`: adds `_ensure_item_menu()` and calls it as the first line of
`_popup_menu()`.

## What was tried

Harness: `bin/godot.linuxbsd.editor.x86_64 --headless --editor --path
___OPTIMIZATIONS___/harness/empty_project --quit-after 1`. Peak RSS via `VmHWM`, exit codes checked.

| Variant | Peak RSS (VmHWM) | notes |
|---|---|---|
| baseline | 712.15MB mean over 8 runs (0.51% spread) | same pristine `HEAD` baseline as candidates 023-026 |
| patched | 703.52MB mean over 5 runs (719060/721208/720380/720892/720476 kB, 0.30% spread) | all exit 0 |

Distributions don't overlap (baseline min 727268 > patched max 721208).

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: peak RSS only.**

**Peak RSS drops by 8.63MB (-1.21%)**, from 712.15MB to 703.52MB (8 baseline vs 5 patched runs, no
overlap) - 1.43x the ~6.03MB predicted from 64 instances x candidate 027's measured 96.5KB mean
`PopupMenu` cost, in line with this workspace's 1.4-1.7x tracked-vs-RSS ratio.

For a 15-line change (`+15/-3` in the `.cpp`, `+2/-0` in the header) this is the best
MB-per-line-of-diff result in the workspace after candidate 023.

## Does it change behavior/output?

- **Bit-exact?** N/A (UI/interactive-behavior change) - see verification below.
- **Verified with a baseline control** via a temporary `TEST_CASE`
  (`bench/scratch_verification_test.cpp`, reverted before finishing - not part of `patch.diff`). It
  counts `PopupMenu` children (including internal ones) of a fresh `FileDialog`, then emits the
  real signal `FileDialog` wires to its right-click handler - `ItemList::empty_clicked` with
  `MouseButton::RIGHT` on the dialog's own file list, obtained through the public
  `get_file_item_list()` - so the actual `_empty_clicked` -> `_popup_menu()` path runs:

  | Check | baseline (control) | patched |
  |---|---|---|
  | direct `PopupMenu` children before right-click | **1** | **0** |
  | after right-click | 1, visible | **1, visible** |

  Identical end state; the menu simply isn't built until a right-click happens. The test's own
  assertions are build-agnostic (menu must exist and be visible *after* the click) and pass in both
  builds. Raw output in `bench/verification.txt`.
- **Is this a breaking change?** No. `item_menu` is private, its only uses are inside
  `_popup_menu()` (verified by `grep` over the whole file - the apparent external hits are other
  classes' own identically-named members, in the debugger tree), and `_popup_menu()` rebuilds the
  menu contents on every call regardless.
- Project test suite: 1427 cases, 1426 passed, 1 failed (known pre-existing `test_ip.cpp` IPv6
  failure), 3 skipped - identical before and after. (As noted in candidate 023, this repo has no
  dedicated `FileDialog` suite, which is why the scratch control above was written.)

## Why the result looks the way it does

A right-click context menu that most users of most file dialogs never open, built 64 times at
startup at ~96.5KB each. Same "pay only when used" reasoning as the rest of this session, with an
unusually small audit surface because the menu's entire lifecycle is already contained in one
rebuild-from-scratch function.

## What's not done / open questions

- `FileDialog` still constructs, per candidate 022's bisection, ~1.0MB of main widget tree per
  instance (nav/drive buttons, layout and sort controls, the file list, filter boxes). The
  `drives` `MenuButton` in particular costs ~116KB per candidate 027's census and is **hidden
  outright on platforms with no drives** (`_update_drives()` calls `drives->hide()` when
  `get_drive_count() == 0`, i.e. always on Linux/macOS) - so ~7.4MB across the 64 instances is
  arguably being spent on a control that can never be shown there. Not attempted: it has ~15
  touch points including two `is_visible()` calls used as *logic* guards and a reparenting step in
  `_update_drives()`, and the win is platform-dependent (zero on Windows). Left in `IDEAS.md`.
- Combined effect of candidates 023 + 029 on the same file was not measured together; each was
  measured independently against the same pristine baseline, and SKILL.md's one-fix-per-candidate
  rule means they are deliberately not bundled (see MEASUREMENT.md section 18 on why summing
  independent deltas is not the same as measuring the bundle).
