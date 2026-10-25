# Bisecting `FileDialog`'s own constructor - where does the fixed ~1.45MB per instance go?

Candidate 021 found 64 `FileDialog` instances at empty-project editor startup costing 95.24MB, with
a strikingly tight per-instance distribution (median 1.446MB, min 1.445MB) - implying the cost is
inherent to `FileDialog`'s own constructor rather than to per-call-site configuration. This
candidate bisects that constructor, to see whether the fixed cost has a single dominant, deferrable
part. If it did, reducing it once would multiply across all 64 instances.

## What was tried

Same harness as candidate 021. 9 `Memory::get_mem_usage()` checkpoints placed through
`FileDialog::FileDialog()` (`scene/gui/file_dialog.cpp`, a 353-line constructor), at statement
boundaries chosen from the constructor's own `memnew` sites - see `bench/instrumentation.diff`
(applied, measured, reverted). Numbers below are the first constructed instance
(`bench/first_instance_breakdown.txt`); the tight variance candidate 021 measured means it is
representative of all 64.

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: Godot's own engine-tracked heap (`Memory::get_mem_usage()`), for attribution
only.** Whole-instance cost for the measured instance: 1.508MB.

| Segment | Delta | % of instance | What it covers |
|---|---|---|---|
| ctor entry -> before_main_vbox | 0.114MB (incl. tail) | 7.6% | pre-widget setup, plus everything after the last probe (`item_menu` etc.) |
| before_main_vbox -> before_make_dir_container | 0.234MB | 15.5% | root `VBoxContainer`, nav buttons (prev/next/up), drives `MenuButton`, directory `LineEdit`, refresh/favorite buttons |
| before_make_dir_container -> before_show_hidden | 0.212MB | 14.1% | make-dir container + button, and the block between |
| before_show_hidden -> before_file_list | 0.211MB | 14.0% | show-hidden button, layout container, thumbnail/list mode buttons, separators, filename-filter toggle, file sort `MenuButton` |
| before_file_list -> before_file_box | 0.134MB | 8.9% | file `ItemList`, message `Label`, filename filter box + `LineEdit` |
| before_file_box -> before_confirm_save | 0.162MB | 10.8% | file box, filename `LineEdit`, filter `OptionButton`, checkbox/grid option containers |
| **before_confirm_save -> before_make_dir_dialog** | **0.221MB** | **14.7%** | **`confirm_save` + `delete_dialog` (2 nested `ConfirmationDialog`s)** |
| **before_make_dir_dialog -> before_mkdirerr** | **0.113MB** | **7.5%** | **`make_dir_dialog` `ConfirmationDialog` + its `new_dir_name` `LineEdit`** |
| **before_mkdirerr -> before_item_menu** | **0.106MB** | **7.1%** | **`mkdirerr` + `exterr` (2 nested `AcceptDialog`s)** |

**There is no single dominant part - the largest segment is 15.5%.** The ~1.45MB is simply the sum
of ~40 `memnew`'d `Control`s, each carrying Godot's fixed per-`Control`/per-`Node` overhead, spread
almost evenly across the constructor.

**The one structurally interesting cluster**: `FileDialog` builds **five nested sub-dialogs of its
own** - `confirm_save`, `delete_dialog`, `make_dir_dialog` (+ its `new_dir_name` field), `mkdirerr`
and `exterr` - which together account for **0.440MB, 29.3% of each instance**. Those are
rarely-used dialogs *inside* an already-rarely-opened dialog (overwrite confirmation, delete
confirmation, new-folder prompt, and two error popups). Across candidate 021's 64 startup
instances that cluster is **~28MB of the 95.24MB total**, deferrable in principle without touching
any of the 66 call sites - a single change in `scene/gui/file_dialog.cpp` would capture it.

## Does it change behavior/output?

**N/A - measurement/characterization only, no patch applied to real source** (per SKILL.md's
"patch.diff is mandatory..." section). `bench/instrumentation.diff`'s temporary probes were
reverted before finishing this candidate.

- Project test suite: not re-run for this purely additive `print_line` instrumentation - no changes
  to real source survived this candidate.

## Why the result looks the way it does

`FileDialog` is a genuinely large composite widget, and Godot's per-`Control` overhead (theme
cache entries, signal lists, accessibility metadata, `Node` bookkeeping) is what dominates -
roughly 1.45MB / ~40 controls ≈ 36KB per control, which is consistent with the per-`Control` costs
implied by other candidates in this workspace (candidate 007's ~34-41 `memnew`s cost 11.07MB by
`Memory::get_mem_usage()`, though that included heavier widgets). This is why the per-instance cost
is so uniform across all 64: it's structural, not configuration-dependent.

## What's not done / open questions

- **No fix attempted.** The identified 29.3%/~28MB nested-sub-dialog cluster is a real, well-scoped
  lead (one file, five members, all private to `FileDialog` - `confirm_save`/`mkdirerr`/`exterr` have
  no references outside `scene/gui/file_dialog.*` at all, checked this session) but `FileDialog` is
  a **public API class used by games as well as the editor**, not editor-only like everything else
  fixed this session - so it needs the full `tests/scene/test_file_dialog.cpp` suite plus interactive
  verification of the make-dir/delete/overwrite/error flows, which did not fit the remaining session
  budget. Left in `IDEAS.md` as the single highest-value remaining lead in this workspace.
- Not bisected further: the per-`Control` overhead itself (what exactly ~36KB/control consists of).
  That would be an engine-wide memory-architecture question rather than a bounded candidate.
