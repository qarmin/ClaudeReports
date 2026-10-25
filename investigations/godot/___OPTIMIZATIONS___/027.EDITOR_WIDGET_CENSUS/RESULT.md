# Widget census: how many of each heavy widget does empty-project editor startup build, and at what cost

Candidate 021's `EditorFileDialog` finding (64 instances, 95.24MB) came from noticing the *same
widget type* dominating two unrelated investigations, then counting it process-wide. This candidate
applies that technique deliberately and in bulk: instance counters plus per-instance
`Memory::get_mem_usage()` deltas on **10** commonly-instantiated, potentially-heavy widget classes,
to find the next `EditorFileDialog`-scale target rather than continuing to bisect dialogs one at a
time.

## What was tried

Harness: `bin/godot.linuxbsd.editor.x86_64 --headless --editor --path
___OPTIMIZATIONS___/harness/empty_project --quit-after 1`. A static counter + constructor scope
guard (`~Guard()` prints the delta, so the whole constructor including base-class and member
construction is captured) added to each class's own constructor - see
`bench/instrumentation.diff` (applied, measured, reverted). Raw final lines in
`bench/census_totals.txt`. Determinism: the counter values were identical across the runs taken
during this candidate's three instrumentation rounds.

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: instance count, and Godot's own engine-tracked heap
(`Memory::get_mem_usage()`) attributed per constructor.**

| Class | Instances at startup | Total | Mean per instance |
|---|---|---|---|
| **PopupMenu** | **520** | **49.02MB** | 96.5KB |
| **AcceptDialog** (incl. all subclasses) | **617** | **28.62MB** | 47.5KB |
| **MenuButton** | **214** | **23.74MB** | 116.4KB |
| **OptionButton** | **158** | **18.16MB** | 117.8KB |
| Tree | 113 | 11.02MB | 99.8KB |
| ItemList | 238 | 3.82MB | 16.4KB |
| TextEdit | 118 | 3.33MB | 28.9KB |
| EditorInspector | 20 | 1.34MB | 68.4KB |
| LineEdit | **1184** | 0.61MB | 0.5KB |
| GraphEdit | 1 | 0.35MB | 359.8KB |

**These totals are inclusive and must not be summed.** Each figure covers everything its
constructor allocates, including nested widgets - so a `MenuButton`'s total contains its
`PopupMenu`, an `AcceptDialog` subclass's total contains its children, and candidate 021's 64
`FileDialog`s are a subset of the 617 `AcceptDialog`s here (`FileDialog` derives from
`ConfirmationDialog` -> `AcceptDialog`). The counts, by contrast, are exact and independent.

**Ownership of the 520 PopupMenus, which is what makes this candidate actionable:** `MenuButton`
(214) and `OptionButton` (158) each construct exactly one in their own constructor, accounting for
**372 of the 520 (72%)**. The remaining ~148 come from the 120 explicit `memnew(PopupMenu)` sites
across the editor plus other internals.

## Why the result looks the way it does - and why the big numbers are mostly *not* addressable

The headline 49MB of `PopupMenu` looks like a bigger `EditorFileDialog`, but it is not the same
kind of opportunity:

- **The 372 MenuButton/OptionButton popups are populated immediately after construction.** The
  canonical usage throughout the editor is `menu_button->get_popup()->add_item(...)` /
  `option_button->add_item(...)` (which forwards to the popup) at the owner's own construction
  time. Making `get_popup()` lazy would therefore construct the popup microseconds later in
  essentially every case - the same "deferral that only re-times an allocation" null result that
  candidate 015 already documented for `VersionControlEditorPlugin`. Not a candidate.
- **1184 `LineEdit`s exist at startup but cost only ~0.5KB each**, because `LineEdit` **already**
  generates its context menu lazily (`LineEdit::_generate_context_menu()`, called from
  `get_menu()` and on right-click) rather than in its constructor. `TextEdit` does the same
  (`TextEdit::_generate_context_menu()`). Had they not, 1184 + 118 controls x 3 popups each would
  have added thousands of `PopupMenu`s - so this is **prior art for the exact pattern candidates
  007/013/017/018/019/023/024/025/026 apply**: lazy sub-widget construction is idiomatic in this
  codebase already, not a novel technique being imposed on it.
- `Tree` (113) and `ItemList` (238) are visible dock/inspector content, not rarely-opened UI, so
  their instances are genuinely needed at startup.

The one clearly-addressable class remains `AcceptDialog` and its subclasses (617 instances,
28.62MB inclusive) - which is precisely the direction candidates 017/018/019/023/024/025/026
already took, one dialog family at a time, and where candidate 021's 66-call-site backlog item
still has ~70MB of headroom.

## Does it change behavior/output?

**N/A - measurement/characterization only, no patch applied to real source** (per SKILL.md's
"patch.diff is mandatory..." section). All instrumentation was reverted before finishing this
candidate; the pristine binary was rebuilt afterwards.

- Project test suite: not re-run for this purely additive counter/`print_line` instrumentation - no
  changes to real source survived this candidate.

## What's not done / open questions

- **Where the other ~148 PopupMenus come from** was not attributed per-site (the 120 explicit
  `memnew(PopupMenu)` sites are known by file - `editor_node.cpp` 10, `script_editor_base.cpp` 8,
  `canvas_item_editor_plugin.cpp` 6, `animation_state_machine_editor.cpp` 6, `scene_tree_dock.cpp`
  5, `filesystem_dock.cpp` 5, ... - but not counted per instance). Context menus among them are
  plausible deferral candidates following `LineEdit`'s own precedent; left in `IDEAS.md`.
- **`Tree`'s 99.8KB mean** was not bisected. 113 instances is a lot, and if a meaningful part of
  that mean is optional/lazy-able sub-structure it would be worth ~11MB; not investigated.
- `GraphEdit`'s single instance costs 359.8KB - the highest per-instance figure in the census, but
  with one instance it is not worth pursuing.
- The census covers 10 hand-picked classes. A fully general version (a counter on `Node` or
  `Control` keyed by class name) is **not** straightforward: `get_class()` inside a base
  constructor returns the *base* class name, since the subclass's class-name machinery is not
  established yet - the same limitation noted in candidate 021.
