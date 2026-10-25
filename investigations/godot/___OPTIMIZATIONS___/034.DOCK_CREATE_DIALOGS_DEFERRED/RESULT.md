# Defer the 4 dock-owned `CreateDialog`s built at editor startup

Candidate 032 removed 4 of the 9 `CreateDialog`s alive at empty-project startup as a side effect of
deferring the `ScriptCreateDialog`s that owned them, and left "measure the other 5" as an open
question. This candidate takes the 4 of those 5 whose owners are docks/editors with an explicit
"add something" user action:

| Owner | Member | Entry point |
|---|---|---|
| `SceneTreeDock` | `create_dialog` | scene tree dock's "Add/Create a New Node." button (`TOOL_NEW`), and `TOOL_CHANGE_TYPE` |
| `FileSystemDock` | `new_resource_dialog` | filesystem context menu -> "New -> Resource..." (`FILE_MENU_NEW_RESOURCE`) |
| `InspectorDock` | `new_resource_dialog` | inspector's "New Resource..." (`InspectorDock::_new_resource()`) |
| `CanvasItemEditor` | `add_node_dialog` | 2D editor's Add Node toolbar menu (`ADD_NODE`) |

All four have the identical shape: **one `popup_create()` site**, plus one or two dereferences
inside the handler of the dialog's own `create` signal (`_do_create()`, `_resource_created()`,
`_create_node()`), which therefore cannot run before the dialog exists. Each gets an
`_ensure_*()`/`_get_*()` accessor and its popup site routed through it; the signal-handler
dereferences are left alone with a one-line comment recording that invariant.

The 5th, `SceneCreateDialog::select_node_dialog`, was deliberately left eager: its
`select_node_dialog` is touched from `SceneCreateDialog::_notification()` (`select_base()`), which
is candidate 025's "a notification handler configures the dialog" blocking shape and would need the
same rework rather than a plain deferral.

**In-tree prior art, again:** `EditorInterface::popup_create_dialog()`
(`editor/editor_interface.cpp:589`) already constructs *its* `CreateDialog` lazily with exactly this
`if (!create_dialog) { memnew ... }` shape.

## What was tried

Harness: `bin/godot.linuxbsd.editor.x86_64 --headless --editor --path
___OPTIMIZATIONS___/harness/empty_project --quit-after 1`, same as every startup-RSS candidate here.
Peak RSS via `/proc/<pid>/status`'s `VmHWM`, every run's exit code checked (all 24 runs exit 0).

**Measured interleaved A/B/A/B, 12 runs per arm** (`bench/rss_interleaved_ab_round1.txt`,
`bench/rss_interleaved_ab_round2.txt`) - deliberately more runs than this workspace's other
candidates, because the effect turned out to be about the same size as a bimodal mode in the
harness itself (see below).

| Variant | Peak RSS (VmHWM) | notes |
|---|---|---|
| baseline (all 4 eager) | 718123.3 kB mean = **701.29MB**, median 719040 kB, 12 runs | 10 runs in 718820-720128, 2 in 712288-712632 |
| patched (all 4 deferred) | 711131.0 kB mean = **694.46MB**, median 712010 kB, 12 runs | 10 runs in 711428-712924, 2 in 705348-706380 |

**Both arms are bimodal in the same way**: ~10/12 runs land in a "main" mode and ~2/12 in a mode
~6.5MB lower. That is a property of the harness, not of the patch - it appears in the pristine
baseline too - and it is roughly the same size as this candidate's effect, so the usual
"distributions don't overlap" test **fails** here (baseline min 712288 < patched max 712924) even
though the effect is clear. Three analyses that do not depend on the modes:

- **Mode-matched means**: main mode 702.40MB -> 695.49MB (**-6.91MB**); low mode 695.76MB ->
  689.32MB (**-6.44MB**).
- **Medians**: 702.19MB -> 695.32MB (**-6.87MB**).
- **Pairwise**: in **138 of 144** baseline-vs-patched run pairs (95.8%) the baseline run is higher.

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: peak RSS only.**

**Peak RSS drops by 6.83MB (-0.97%)** on the pooled means (701.29MB -> 694.46MB, 12 runs per arm),
and by 6.91MB / 6.44MB when the two modes are compared like-for-like. **~1.7MB per deferred
`CreateDialog`** - which also answers candidate 032's open question about what a `CreateDialog`
costs, from the fix rather than from instrumentation.

## Does it change behavior/output?

- **Bit-exact?** N/A (UI/interactive-behavior change) - see verification below.
- **Interactive verification with a baseline control**, under `Xvfb` (1280x800x24), using a
  throwaway `@tool` `EditorPlugin` (`bench/scratch_verification_plugin.gd`, not part of
  `patch.diff`) that counts `CreateDialog` nodes across the whole scene tree, then finds the scene
  tree dock's **real** "Add/Create a New Node." `Button` (matched by tooltip, not by index) and
  emits its `pressed` signal:

  | Check | baseline (control) | patched |
  |---|---|---|
  | `CreateDialog`s in tree at startup | **9** | **5** |
  | press "Add/Create a New Node." | dialog shown, title `Create New Node` | dialog shown, title `Create New Node` |
  | count after the press | 9 (already existed) | 5 -> **6** (created on demand) |

  Exactly **4 fewer** at startup, matching the 4 deferrals, with an identical user-visible outcome.
  Raw output in `bench/verification_{patched,baseline_control}.txt`. The baseline's 9 reproduces
  candidate 032's count.
- **Is this a breaking change?** No. All four members are private, none is exposed through a
  `ClassDB`-bound getter (unlike candidate 032's `SceneTreeDock::get_script_create_dialog()`), and
  every use is either routed through the accessor or inside the dialog's own `create`-signal
  handler. `grep` after patching confirms no raw `create_dialog->` / `new_resource_dialog->` /
  `add_node_dialog->` uses remain outside the accessors and those handlers.
- One deliberate behavioral nuance: `SceneTreeDock`'s `favorites_updated` connection to
  `_update_create_root_dialog()` is now made when the dialog is first opened rather than at startup.
  The signal can only be emitted *by that dialog*, so nothing is missed while it does not exist.
- Project test suite: 1427 cases, 1426 passed, 1 failed (known pre-existing `test_ip.cpp` IPv6
  failure, no IPv6 in this container), 3 skipped - identical before and after
  (`bench/test_suite_patched.txt`).

## Why the result looks the way it does

`CreateDialog` is the class-picker dialog: a `Tree`, a search `LineEdit`, favorites/recent
`ItemList`s and an `EditorHelpBit`, ~1.7MB per instance. Four docks each built one at startup for a
"create a node/resource" action that only happens when the user asks. It is the same pattern as
candidates 017/018/019/023/024/025/026/031/032, applied to the last widget family this workspace
found with more than two startup instances.

The win is smaller per dialog than the `EditorFileDialog` work (1.45MB tracked / ~2.3MB RSS each)
because only 4 instances exist, not 64 - which is exactly why this candidate was worth doing only
after the attribution in candidate 032 said there was nothing bigger left in this family.

## What's not done / open questions

- **The bimodal ~6.5MB startup-RSS mode in the harness is unexplained.** It shows up in ~2/12 runs
  of *both* arms (and, retrospectively, is a plausible contributor to the ~10MB baseline drift
  candidate 032 saw between XDG states). Worth attributing before any future candidate whose
  expected effect is under ~7MB, since it makes the min/max non-overlap test useless at that scale.
  Recorded in `IDEAS.md`.
- Only the `SceneTreeDock` flow was exercised interactively. The other three (`FileSystemDock`
  "New -> Resource...", `InspectorDock` "New Resource...", `CanvasItemEditor` Add Node) were
  verified by code reading plus the accessor-routing `grep`; all three are the same one-popup-site
  shape and their `create`-signal handlers are the only other dereference.
- `SceneCreateDialog::select_node_dialog` (the 5th startup instance) is left eager - see above. The
  4 remaining after this patch are that one plus 4 not attributed per-owner
  (`bench/verification_*` counts the class, not its owners; candidate 032's
  `attribution_64_file_dialogs.txt` technique would name them).
- Not attempted: `CreateDialog`'s own per-instance cost breakdown. The ~1.7MB figure here is the
  end-to-end RSS delta divided by 4, not a bisection of what inside `CreateDialog` is expensive
  (candidate 028 showed its `Tree`'s inline cell editor alone is ~74KB of it).
