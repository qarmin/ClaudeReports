# `PopupMenu::items` has no locking around `set_item_count()`/`get_item_count()`

Found incidentally while verifying candidate 017's `MenuButton::show_popup()` fix: candidate 017's
*original* (unnarrowed) repro still crashed even after that fix was applied and built, but with a
completely different ASan report (a heap `unknown-crash` inside `CowData<PopupMenu::Item>::size()`),
pointing at a second, independent bug reachable via the same repro's `MenuButton.set_item_count()`/
`.get_item_count()` calls.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `popup_menu.h`/`.cpp` lock-usage audit (`grep -n "_THREAD_SAFE" scene/gui/popup_menu.h`) | n/a | Zero matches - `PopupMenu` has no synchronization primitive of any kind, despite `mutable Vector<Item> items` being touched by ~105 public methods | 
| Isolated GDScript repro (`repro/`, 6 worker `Thread`s mixing only `MenuButton.get_popup()`/`.set_item_count()`/`.get_item_count()` - deliberately excludes `show_popup()`/`switch_on_hover` to not depend on candidate 017's separate fix), against the same self-built ASan editor binary used for candidate 017 (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) | `--headless --path repro` under `timeout -k 5 25`, 3 runs | **Crashed 3/3 runs** - a real, symbolized ASan report: `attempting double-free` in `Memory::realloc_static` called from `CowData<PopupMenu::Item>::_realloc_exact`, reached via `PopupMenu::set_item_count()` (`popup_menu.cpp:3064`) called through `MenuButton::set_item_count()` (`menu_button.cpp:135`) |
| Fix (`_THREAD_SAFE_CLASS_` on the class + `_THREAD_SAFE_METHOD_` on `PopupMenu::set_item_count()`/`get_item_count()`), same build/run, 8 repeat runs | Same build/run | **Clean 8/8 runs** |
| Project test suite before/after | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, `XDG_DATA_HOME` set | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` with the fix applied - identical to the established baseline from candidates 002-017 (same pre-existing `test_ip.cpp` IPv6 environment failure) |

## Status

**Confirmed.** Reproduced 3/3 runs (two independent ASan manifestations of the same race were
observed across this and candidate 017's sessions - a `CowData::size()` read `unknown-crash` and
this candidate's own `realloc_static` double-free - both from concurrent, unlocked access to the
same `items` `Vector`), and 8/8 clean runs with the fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `PopupMenu` through nothing but `MenuButton`'s public GDScript API
(`MenuButton.get_popup()`/`.set_item_count()`/`.get_item_count()`). No second participant is
involved - this is a same-process data race between GDScript `Thread`s sharing one `MenuButton`/
`PopupMenu` pair.

## Class

`memory-safety` (concurrent unsynchronized `Vector`/`CowData` resize vs. resize/read - same general
shape as this workspace's other container races, e.g. candidates 009/011/012/016; manifests as
either a double-free or a stale-pointer read depending on the exact interleaving)

## Root cause

`PopupMenu::items` (`mutable Vector<Item> items`, `scene/gui/popup_menu.h:138`) is resized by
`set_item_count()` (`popup_menu.cpp:3043`, among ~20 other item-mutating methods like `add_item`/
`remove_item`/`clear`) and read by `get_item_count()` (`:3081`) and dozens of other accessors, all
with **zero** locking - `PopupMenu` declares no mutex of any kind. `MenuButton::set_item_count()`
(`menu_button.cpp:128`) forwards almost directly into `PopupMenu::set_item_count()` after its own
unlocked `popup->get_item_count()` check. Two threads calling `MenuButton.set_item_count()`
concurrently both reach `Vector<Item>::resize()` -> `CowData::_realloc_exact()` on the same backing
buffer with no mutual exclusion; depending on the exact interleaving this either frees the same
buffer twice (this candidate's `realloc_static` double-free) or lets a concurrent `get_item_count()`
read `items.size()` mid-resize against freed memory (candidate 017's `unknown-crash` observation).

## Introduced by

`not determined` (predates tracked history). `scene/gui/popup_menu.cpp`'s earliest tracked ancestor
is `0b806ee0fc` "GODOT IS OPEN SOURCE" (2014-02-09, the initial open-source commit) - same situation
as candidates 012/016's findings: not a regression from a later refactor, the class's original,
unchanged thread-safety posture across the engine's entire public history. (`set_item_count()`
itself was added later, but the class had no locking at any point either before or after that
addition.)

## The fix

Add `_THREAD_SAFE_CLASS_` (a recursive `Mutex`, `core/os/thread_safe.h`) as a private member right
above `items`, and `_THREAD_SAFE_METHOD_` as the first statement of `set_item_count()` and
`get_item_count()` - the two methods directly in this candidate's confirmed crash path (`+1/-0` in
`popup_menu.h`, `+2/-0` in `popup_menu.cpp`, see `patch.diff`). Verified sufficient against the
isolated repro (8/8 clean runs).

- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before this
  candidate's work and after the fix - no new failures (the 1 failure is the pre-existing
  `test_ip.cpp` IPv6 environment issue already present in the baseline from candidates 002-017).

## What's not done / open questions

- **This fix is deliberately narrow, not a full-class treatment.** Unlike candidates 008/009/010/
  012/016 (9-34 methods each), `PopupMenu` has on the order of **100 public methods** touching
  `items` (every `add_*_item`/`set_item_*`/`get_item_*`/`is_item_*` accessor, `remove_item`,
  `clear`, `scroll_to_item`, `activate_item`, etc.) - instrumenting all of them was out of scope for
  this session's time budget. Only `set_item_count()`/`get_item_count()` (the two methods the
  confirmed repro actually exercises) are locked. **Every other accessor remains unguarded and is a
  real, not-yet-confirmed exposure to the identical race** if called concurrently with
  `set_item_count()`/a sibling mutator - e.g. a thread calling `add_item()` while another calls
  `get_item_text()` would very likely hit the same class of bug. This is flagged as its own,
  larger backlog item in `IDEAS.md` rather than claimed fixed here.
- `MenuButton::set_item_count()` itself still has an unlocked check-then-act
  (`popup->get_item_count() == p_count` read, then a separate `popup->set_item_count()` call) - with
  this fix, each half is now internally consistent, but the two calls aren't atomic together. This
  is a benign logic-level race (could skip or redundantly perform a resize once), not a
  memory-safety issue, since the actual mutation is now safely locked inside `PopupMenu` - left
  unfixed as out of scope.
- Not checked whether `OptionButton` (which also wraps a `PopupMenu`, per `popup_menu.h:119`'s
  `friend class OptionButton`) has the same exposure through its own item-count API - worth
  checking alongside this candidate's open `OptionButton`/`show_popup` follow-up from candidate 017.
