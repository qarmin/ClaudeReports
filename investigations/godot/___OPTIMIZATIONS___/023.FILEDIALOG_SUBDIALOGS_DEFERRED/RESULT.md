# Defer `FileDialog`'s 5 nested sub-dialogs - the largest memory win of this workspace

Candidate 021 found 64 `FileDialog` instances built at empty-project editor startup costing
95.24MB, and candidate 022 bisected the near-fixed per-instance cost, finding that `FileDialog`
builds **five nested sub-dialogs of its own** - `confirm_save` (overwrite warning),
`delete_dialog`, `make_dir_dialog` (+ its `new_dir_name` field), `mkdirerr` and `exterr` - worth
0.440MB/29.3% of every instance. Those are rarely-used dialogs *inside* an already-rarely-opened
dialog. This candidate defers all five to first use, from one file, without touching any of the 66
`memnew(EditorFileDialog)` call sites.

`scene/gui/file_dialog.cpp`/`.h`: adds `_ensure_confirm_save()`, `_ensure_delete_dialog()`,
`_ensure_make_dir_dialog()`, `_ensure_mkdirerr()`, `_ensure_exterr()`, each constructing its dialog
(and wiring its signals) on first call, and calls them from the five flows that pop each one:
`_action_pressed()` (empty/invalid filename -> `exterr`; existing file -> `confirm_save`),
`_item_menu_id_pressed()` (delete -> `delete_dialog`), `_make_dir()` (-> `make_dir_dialog`), and
`_make_dir_confirm()`'s failure branch (-> `mkdirerr`). `_make_dir_confirm()` can rely on
`make_dir_dialog`/`new_dir_name` already existing, since it only runs via that dialog's own
`confirmed` signal - noted in a comment at the call site.

## What was tried

Harness: `bin/godot.linuxbsd.editor.x86_64 --headless --editor --path
___OPTIMIZATIONS___/harness/empty_project --quit-after 1`, same as candidates 006/007/013/017/018/019.
Peak RSS via `/proc/<pid>/status`'s `VmHWM` (no `/usr/bin/time` in this container). Every run's exit
code was checked, not just its RSS (see "What's not done" for why that mattered here).

| Variant | Peak RSS (VmHWM) | notes |
|---|---|---|
| baseline (5 sub-dialogs built eagerly) | 730287.2 kB mean (730964/730732/728284/730788/730668, 5 runs, 0.37% spread) | all exit 0 |
| patched (deferred) | 682780.0 kB mean (682116/682060/682924/683028/683772, 5 runs, 0.25% spread) | all exit 0 |

Distributions don't overlap (baseline min 728284 > patched max 683772).

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: peak RSS only** (constructor-phase allocation removal, not a per-call hot
path - matching how candidates 007/013/017/018/019 measured this class of fix).

**Peak RSS drops by 46.39MB (-6.51%)**, from 713.17MB to 666.78MB mean (5 vs 5 runs, no overlap).
This is the largest single improvement recorded in this workspace, and it comes from a 6-line-per-
dialog change in one file.

Candidate 022 predicted ~28MB from the engine-tracked (`Memory::get_mem_usage()`) figure; the
measured RSS delta is 1.66x that, consistent with the same tracked-vs-RSS ratio seen in every other
deferred-construction candidate this session (007: 11.07MB tracked -> 7.85MB RSS; 013: 10.62 ->
15.52; 017: 8.73 -> 13.81; 018: 4.10 -> 7.85; 019: 2.58 -> 3.70). The mechanism also explains why
the win exceeds the tracked prediction: deferring a `ConfirmationDialog`/`AcceptDialog` skips not
just the dialog object but its whole nested widget tree and its `Window`-level allocations.

## Does it change behavior/output?

- **Bit-exact?** N/A (UI/interactive-behavior change) - see verification below.
- **Structural + flow verification with a baseline control** (`bench/scratch_verification_test.cpp`,
  a temporary `TEST_CASE` in `tests/scene/`, reverted before finishing - not part of `patch.diff`).
  It counts `AcceptDialog`-derived children (including internal ones) of a fresh `FileDialog` and
  after each flow runs. Run against **both** builds:

  | Check | baseline (control) | patched |
  |---|---|---|
  | fresh instance, sub-dialogs present | **5** | **0** |
  | `exterr` flow (empty filename in save mode) | 5 -> 5, visible=1 | 0 -> 1, **visible=1** |
  | `confirm_save` flow (save over an existing real file) | 5 -> 5, visible=1 | 0 -> 1, **visible=1** |

  The control matters: it proves the test detects the difference (5 vs 0 at construction) rather
  than the "0" being a measurement artifact, and that the **user-visible outcome is identical** -
  in both builds the correct dialog ends up shown (`visible=1`). Raw output in
  `bench/verification_baseline_control.txt` and `bench/verification_patched.txt`.
- **Is this a breaking change?** No behavioral change was observed. Two honest caveats specific to
  this candidate, which touches **public API used by games**, not editor-only code like every other
  fix this session:
  - The five dialogs are added with `add_child(..., INTERNAL_MODE_FRONT)`, so they are internal
    children excluded from ordinary `get_children()` traversal and scene serialization; code that
    deliberately enumerated internal children of a `FileDialog` (via `get_child_count(true)`) would
    now see 5 fewer until the corresponding flow runs. No such code exists in the engine
    (`confirm_save`/`mkdirerr`/`exterr` have zero references outside `scene/gui/file_dialog.*`;
    `delete_dialog`/`make_dir_dialog` name-collide with unrelated members of other classes but are
    not shared), but a third-party script poking at internal children could notice.
  - `delete_dialog`, `make_dir_dialog`, `mkdirerr` and `exterr` set their text/title once at
    construction via `ETR(...)`, and `FileDialog::_notification()`'s
    `NOTIFICATION_TRANSLATION_CHANGED` handler does **not** re-apply them (it only calls
    `update_filters()` and re-does button icons). So these strings were already "frozen at
    construction time" before this patch; after it they are frozen at *first-use* time instead,
    i.e. they now pick up the locale current when the flow first runs. For a user who changes
    language mid-session this is a change - arguably a fix rather than a regression, but it is a
    difference and is called out rather than glossed.
- Project test suite: 1427 cases, 1426 passed, 1 failed (known pre-existing `test_ip.cpp` IPv6
  failure, no IPv6 in this container), 3 skipped - identical before and after. Note there is **no
  dedicated `tests/scene/test_file_dialog.cpp` suite in this repo at all**, so the suite's coverage
  of `FileDialog` is only indirect; that is exactly why the scratch verification above was written
  with a control rather than relying on the suite.

## Why the result looks the way it does

`FileDialog` is one of the most-instantiated composite widgets in the editor (64 live instances at
startup per candidate 021) and each one carried five complete `Window`-derived sub-dialogs for
flows most users never touch in a session. Deferring them multiplies a modest per-instance saving
by 64. This is the same "pay only when used" pattern as candidates 007/013/017/018/019, but applied
one level down - at the shared widget rather than at each of its 66 users - which is what makes it
an order of magnitude larger for a fraction of the diff.

## What's not done / open questions

- **Only 2 of the 5 flows were exercised end-to-end** (`exterr`, `confirm_save`) - those are the
  two reachable through public API from a test (`emit_signal("confirmed")` routes to
  `_action_pressed()`). The `delete_dialog`, `make_dir_dialog` and `mkdirerr` flows are triggered
  from private handlers (`_item_menu_id_pressed`, `_make_dir`, `_make_dir_confirm`) with no public
  entry point, so they were verified by code reading only: every use of those members is now
  immediately preceded by its `_ensure_*()` call (exhaustive `grep` of all six member names in the
  file, re-checked after the patch). An interactive Xvfb pass clicking the real create-folder /
  delete / create-folder-failure paths in the editor's own file dialog would close this gap.
- **A rare, pre-existing shutdown race was observed once** during this candidate's work and is
  **not** caused by this patch: one empty-project run aborted (exit 134) with
  `ERROR: Parameter "singleton" is null. at: is_cmdline_mode (editor/editor_node.cpp:6706)`.
  `EditorFileSystem::_process_update_pending()` (`editor/file_system/editor_file_system.cpp:2301`)
  calls `EditorNode::is_cmdline_mode()`, whose `ERR_FAIL_NULL_V(singleton, false)` returns `false`
  when `EditorNode` has already been destroyed - so the code then takes the *more* work-doing
  "not cmdline mode" branch during teardown. It did not reproduce in 26 subsequent runs of the same
  patched binary (0/20 in a dedicated loop), so this patch only re-times an existing race rather
  than introducing one. Recorded as an unconfirmed lead in `___BUGHUNT___/IDEAS.md` rather than a
  confirmed `___BUGHUNT___` candidate, since bug-hunt's bar requires a reproduction and this is a
  1-in-~26 event seen once.
- The complementary, larger-ceiling direction (making the ~66 `memnew(EditorFileDialog)` call sites
  lazy, for up to candidate 021's full 95.24MB) is untouched and remains in `IDEAS.md`.
