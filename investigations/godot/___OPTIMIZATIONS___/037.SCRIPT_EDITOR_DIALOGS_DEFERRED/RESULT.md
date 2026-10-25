# 037. ScriptEditor's user-action-only dialogs deferred to first use

- **Project**: godotengine/godot
- **Base commit**: `9552dfb6859a1aaba1e570b8e0ef5c599b830f19`
- **Files**: `editor/script/script_editor_plugin.cpp`, `editor/script/script_editor_plugin.h`
- **Metric(s) measured**: Godot's own tracked peak heap (`Memory::get_mem_max_usage()`, primary,
  per candidate 035) and whole-process peak RSS (`/proc/<pid>/status`'s `VmHWM`, secondary)
- **Workload**: empty-project editor startup,
  `--headless --editor --path harness/empty_project --quit-after 1`

## What was tried

`ScriptEditor`'s constructor eagerly builds four dialogs that are only reachable through explicit
user actions:

| member | class | reached only from |
|---|---|---|
| `file_dialog` | `EditorFileDialog` | File menu (New Text File / Open / Save As / Import-Export Theme) |
| `help_search_dialog` | `EditorHelpSearch` | Search Help menu item, `_help_search()` |
| `erase_tab_confirm` | `ConfirmationDialog` | closing a tab with unsaved changes |
| `error_dialog` | `AcceptDialog` | "Can't open '%s'" failure path in `_open_recent_script()` |

All four are private members (declared before the class's `protected:` section), so none of the
**69** `ScriptEditor::get_singleton()` call sites elsewhere in the repo can reach them - the
singleton-audit blocker that ruled out deferring `ScriptEditor` as a whole (see `IDEAS.md`) does not
apply to its internals.

The patch replaces the four eager construction blocks with four construct-on-first-call accessors
(`_ensure_file_dialog()`, `_ensure_help_search_dialog()`, `_ensure_erase_tab_confirm()`,
`_ensure_error_dialog()`) and routes every use through them, following candidate 019's
one-accessor-per-member shape. `+86/-59` lines across the two files.

**Why this class is worth more than its per-instance cost suggests**: `ScriptEditor` is constructed
**twice** at editor startup - once by `ScriptEditorPlugin` and once by `ShaderEditorPlugin`
(`memnew(ScriptEditor(config_section, "shader_editor_cache.cfg"))`, candidate 032's finding) - so
every allocation in its constructor is paid twice. `bench/ctor_bisection.txt` has the full
per-section breakdown from `Memory::get_mem_usage()` checkpoints; the four members above sum to
4350714 B over both instances.

`script_create_dialog` (3.15MB) and `shader_create_dialog` (2.37MB) are the two largest items in
that bisection and are **deliberately excluded** - candidate 032 already defers exactly those two
members, and including them here would double-count that candidate's measured win.

`disk_changed` + `disk_changed_list` (473672 B over both instances) are also excluded, for a
different reason: `_test_script_times_on_disk()` calls `disk_changed_list->clear()` and
`create_item()` unconditionally at its top, and is called from `NOTIFICATION_APPLICATION_FOCUS_IN`
and from `edit()`. A naive deferral would therefore show a win in this headless `--quit-after 1`
harness while being a null result in any real editor session (candidate 015's shape). Recorded in
`IDEAS.md` with the restructure that would make it real.

## Result

**Tracked peak heap: -5198706 bytes (-4.96 MB, -1.16%)**, 448283037 -> 443084331 B.
Byte-identical across 5 runs per arm (one distinct value each), all runs exit 0.
Tracked usage at `Main::cleanup()`: -554880 B (-0.53 MB), also byte-identical.

**Peak RSS: +2.90 MB (+0.42%)**, 691.47 -> 694.37MB mean, 691.63 -> 694.39MB median, 15 interleaved
A/B pairs, **patched higher in 15/15 pairs** (no overlap between the arms' ranges: baseline
688.92-692.40MB, patched 692.68-695.40MB). All 30 runs exit 0.

So the two metrics move in **opposite directions**: Godot's own allocator sees ~5MB less peak heap,
while the process's peak RSS is ~3MB higher, reproducibly.

Probe of the mechanism (`MALLOC_ARENA_MAX=1`, 6 interleaved pairs): the RSS regression shrinks to
+1.53 MB (patched higher in 5/6 pairs), i.e. glibc arena behaviour accounts for roughly half of it,
and ~1.5MB persists under a single arena. The same probe also shows something much larger and
independent of this patch, split out as its own candidate: `MALLOC_ARENA_MAX=1` reduces **baseline**
peak RSS by **56.12 MB** (691.47 -> 635.35MB).

## Does it change behavior/output?

Not for the user-visible flows, verified interactively under Xvfb against a **baseline control**
(full transcript in `bench/verification_output.txt`, harness in `bench/verify_plugin.gd`):

| | baseline | patched |
|---|---|---|
| startup `EditorFileDialog` count | 64 | 62 |
| startup `EditorHelpSearch` count | 1 | 0 |
| startup `ConfirmationDialog` count | 400 | 389 |
| after firing the real "New Text File..." menu item | 64 | 63 |
| resulting visible dialog title | `New Text File...` | `New Text File...` |
| after firing the real "Search Help" menu item | 1 | 1 |
| resulting visible dialog title | `Search Help` | `Search Help` |

The startup drop is exactly the 2 deferred file dialogs (one per `ScriptEditor` instance) and the 1
help-search dialog; the baseline arm's 64 independently reproduces candidate 021's headless counter.
The -11 `ConfirmationDialog` figure is fully accounted for: `EditorFileDialog` and `EditorHelpSearch`
are themselves `ConfirmationDialog` subclasses and each `FileDialog` owns 3 more (`confirm_save`,
`delete_dialog`, `make_dir_dialog`), giving -2 -6 -2 -1 = -11.

**Test suite, both arms, identical**: 1427 cases, 1426 passed, 1 failed, 3 skipped; 424066
assertions, 423066 passed, 1000 failed. The single failure is the known `test_ip.cpp` IPv6 check
(this container has no IPv6), present in both arms and unrelated.

Two real behavioral differences that are not visible in the counts above:

1. The four dialogs' `TTRC(...)`/`add_button()` texts are now resolved at first use rather than at
   editor startup, the same translation-timing caveat candidate 023 documented for `FileDialog`'s
   sub-dialogs.
2. `ScriptEditor`'s internal child count is lower until each flow is first used, which is
   observable to anything walking the editor's node tree (e.g. a plugin doing
   `get_children(true)`).

## Why the result looks the way it does

The tracked-heap saving is straightforward: 4350714 B of constructor allocations are not made, and
the measured tracked-peak drop (5198706 B) is somewhat larger than that, consistent with candidate
035's observation that `get_mem_usage()` *checkpoint* deltas undercount transient peaks relative to
`get_mem_max_usage()` (nested widget construction allocates and frees scratch that the checkpoint
pair never sees).

The RSS regression is the interesting part, and it is the **first case in this workspace where a
tracked-heap saving does not reach the process at all, but reverses**. The patched run allocates
strictly less through Godot's allocator, so the extra ~3MB of resident pages is allocator/OS
behaviour rather than engine allocation: removing a set of early, similarly-sized, long-lived
allocations changes the heap's layout and growth pattern, and glibc ends up retaining more (the
`MALLOC_ARENA_MAX=1` probe removing half the effect points at arena multiplication for that half).
This is exactly the class of effect candidate 035 identified as living outside the tracked
allocator - but candidate 035 validated the tracked counter as a *replacement* metric on a case
where the two agreed, and this candidate shows they can disagree in sign.

## What's not done / open questions

- **The ~1.5MB of RSS regression that survives `MALLOC_ARENA_MAX=1` is not attributed.** Worth a
  `pmap`/`/proc/<pid>/smaps` diff between the two arms (which mapping grew?) rather than more
  aggregate RSS runs.
- **Whether this reverses for a real, non-empty project** is unknown: the four dialogs are deferred,
  not deleted, so a session that opens the File menu once pays the allocation back, and the RSS
  effect may well differ once the heap is larger and busier than an empty project's.
- `disk_changed`/`disk_changed_list` restructure (see above), left in `IDEAS.md`.
- The remaining `ScriptEditor` constructor cost is ~1.8MB (inst 1) / ~1.4MB (inst 2) before the
  dialogs, i.e. the tab container, menu bar and buttons - not audited here.
