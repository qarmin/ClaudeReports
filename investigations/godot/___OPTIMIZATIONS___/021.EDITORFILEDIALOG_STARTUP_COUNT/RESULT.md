# 64 `FileDialog` instances are built at empty-project editor startup, costing 95.24MB

The largest single finding of this session, and the largest attributed block found so far in the
whole workspace. Reached by accident: while auditing `SpriteFramesEditor` (5.04MB, candidate 012's
4th-largest plugin) as a deferred-construction candidate, bisecting its constructor showed its two
dominant chunks were both `EditorFileDialog` instances at ~1.7MB each - and candidate 011 had
*independently* measured an `EditorFileDialog` at 1.60MB inside `ProjectExportDialog`. Two
independent ~1.6MB sightings of the same widget prompted counting them editor-wide.

`grep -rn "memnew(EditorFileDialog)"` finds **66 instantiation sites across 47 editor files**. This
candidate measures how many actually run during an empty-project startup, and what they cost in
total.

## What was tried

Harness: same as candidates 006/007/010-019 - `bin/godot.linuxbsd.editor.x86_64 --headless --editor
--path ___OPTIMIZATIONS___/harness/empty_project --quit-after 1`. Instrumentation
(`bench/instrumentation_filedialog_counter.diff`, applied/measured/reverted): a static instance
counter plus a scope-guard in `FileDialog`'s constructor (`scene/gui/file_dialog.cpp` -
`EditorFileDialog` subclasses `FileDialog` and has no constructor of its own, so the base
constructor is where the cost and the count both live) that records
`Memory::get_mem_usage()` before/after each instance's construction and prints the per-instance
delta plus a running total. Full log in `bench/startup_filedialog_log.txt` (64 lines).
`bench/instrumentation_spriteframes_bisection.diff` is the `SpriteFramesEditor` constructor
bisection that led here.

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: Godot's own engine-tracked heap (`Memory::get_mem_usage()`), attributed
per-instance.**

**64 `FileDialog`/`EditorFileDialog` instances are constructed during a bare empty-project editor
startup, totalling 99,861,708 bytes = 95.24MB.**

| Statistic | Value |
|---|---|
| instances | 64 |
| total | **95.24MB** |
| mean | 1.488MB |
| median | 1.446MB |
| min | 1.445MB |
| max | 1.945MB |

For scale: candidate 006 measured the whole empty-project startup at ~308.2MB of engine-tracked
heap (716.5MB peak RSS). **These file dialogs alone are ~31% of that tracked total** - larger than
candidate 006's entire 74-plugin creation loop (83.4MB), larger than the "+83.7MB" menu/dialog
block candidates 010/015/016 spent three candidates bisecting, and about 10x the largest single
dialog fixed this session (`EditorAbout`, 8.73MB).

**The variance is remarkably tight** (median 1.446MB, min 1.445MB, 90% of instances within 1.44-1.7MB).
That matters for interpretation: the cost is essentially **fixed per instance and inherent to
`FileDialog`'s own constructor**, not a function of what each call site configures afterwards
(filters, access mode, title). Candidate 022 bisects that constructor.

## Does it change behavior/output?

**N/A - measurement/characterization only, no patch applied to real source** (per SKILL.md's
"patch.diff is mandatory..." section). The temporary counter/probe instrumentation was reverted
before finishing this candidate.

- Project test suite: not re-run for this purely additive counter/`print_line` instrumentation - no
  changes to real source survived this candidate.

## Why the result looks the way it does

Same architectural pattern this whole session, but at a scale that dwarfs the individual cases:
every editor dialog, dock and plugin that *might* ever need to pick a file constructs its own
private `EditorFileDialog` up front. Nothing shares one, and nothing defers one. A single
`EditorFileDialog` is not a small widget - it's a full file browser (path bar, drive menu, favorites,
file list, filter box, plus five nested `ConfirmationDialog`/`AcceptDialog` sub-dialogs of its own,
per candidate 022) - so 64 copies of it is the biggest single lever on editor startup memory found
in this workspace so far.

## What's not done / open questions

- **`FileDialog`'s own constructor is bisected in candidate 022** - to see whether the ~1.45MB
  fixed cost has a dominant, deferrable part (which would multiply across all 64 instances).
- **No fix attempted here.** Two distinct fix directions, both larger than anything attempted this
  session and both left in `IDEAS.md`: (a) make the ~66 `memnew(EditorFileDialog)` call sites lazy
  individually (mechanical but very broad - 47 files, and each needs its own audit of the kind
  candidates 017/018/019 did for one dialog); (b) reduce `FileDialog`'s own per-instance cost, which
  would benefit all 64 at once without touching any call site - the higher-leverage option, and the
  one candidate 022 is scoped to inform.
- Not measured: how many of the 64 are `EditorFileDialog` vs plain `FileDialog`. `get_class()`
  inside the base constructor returns `"FileDialog"` for every instance (the subclass's class name
  isn't established yet during base construction), so the log can't distinguish them. Given all 66
  known `memnew` sites are `EditorFileDialog` and this is an editor-only run, the practical
  assumption is that nearly all 64 are `EditorFileDialog`, but it isn't proven by this measurement.
- Not measured: the same count for a *non-empty* project, or for the runtime (game) case rather
  than the editor - the 64 figure is specific to empty-project editor startup.
