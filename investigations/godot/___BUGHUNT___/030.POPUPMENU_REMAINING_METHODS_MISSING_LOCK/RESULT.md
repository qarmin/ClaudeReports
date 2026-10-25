# `PopupMenu`'s ~80 other item-touching public methods are also unlocked, same race as candidate 018

Follow-up to candidate 018 (`POPUPMENU_ITEMS_RACE`), which found and fixed the race in exactly two
methods (`set_item_count()`/`get_item_count()`) and explicitly flagged the other ~100 public
methods touching `mutable Vector<Item> items` (`add_*_item`, every `set_item_*`/`get_item_*`/
`is_item_*` accessor, `remove_item`, `clear`, `scroll_to_item`, `activate_item`, etc.) as a real,
not-yet-confirmed exposure to the identical bug. This candidate confirms that exposure with its own
independent repro (deliberately avoiding `set_item_count()`/`get_item_count()` entirely) and fixes
the whole remaining surface.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: re-confirm current (reverted) tree state has zero locking anywhere in `popup_menu.h`/`.cpp` | `grep -n "_THREAD_SAFE" scene/gui/popup_menu.h scene/gui/popup_menu.cpp` | Zero matches - candidate 018's fix was correctly reverted after its own session, as this workspace's convention requires; the class currently has no synchronization primitive of any kind |
| Isolated GDScript repro (`repro/`, 6 worker `Thread`s calling `add_item`/`add_icon_item`/`clear`/`remove_item`/`set_item_text`/`get_item_text`/`set_item_checked`/`is_item_checked`/`scroll_to_item`/`set_item_disabled` on one shared `PopupMenu` - never `set_item_count`/`get_item_count`), against a self-built ASan editor binary at the current branch tip (b8420aa3a60689572a745c0f8a85c9df62d4cbfb) | `--headless --path repro` under `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 25`, 6 runs | **Crashed 6/6 runs**, exit code 132 (SIGILL - Godot's `FATAL`/`CRASH_COND` abort trap) every time, from a TOCTOU race on `items.size()` between concurrent mutators/readers (see Root cause) |
| Same repro logic (an earlier draft that additionally probed the unbound `activate_item` GDScript binding, which always errored harmlessly and never executed) | Same build/run, 3 runs | **Crashed 3/3 runs** - two independent, fully symbolized AddressSanitizer reports: an `unknown-crash` READ inside `PopupMenu::clear()` -> `Vector<Item>::begin()`/`ptrw()` -> `CowData::_copy_on_write()`, and a separate `CowData<PopupMenu::Item>::get()` OOB read reached through a GDScript-bound item getter's `validated_call` |
| Fix (`_THREAD_SAFE_CLASS_` on the class + `_THREAD_SAFE_METHOD_` as the first statement of all 80 public item-touching methods - see "The fix"), same build/run, 11 repeat runs (8 against an earlier repro draft, 3 against the final shipped `repro/`) | Same build/run | **Clean 11/11 runs** - every run printed `survived` and exited 0 |
| Project test suite before/after | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, `XDG_DATA_HOME` set | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` identically before and after the fix - the 1 failure is the pre-existing `test_ip.cpp` IPv6 environment issue already present in every candidate's baseline since 002 |

## Status

**Confirmed.** Reproduced 6/6 runs with the exact repro shipped in `repro/` (exit 132 every time,
zero "survived" prints), plus 3/3 further crashes (two independently symbolized ASan reports) with
an earlier draft exercising the identical set of methods. 11/11 clean runs with the fix applied (8
against the earlier draft, 3 against the final shipped repro).

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `PopupMenu` through nothing but its own public GDScript API
(`add_item`/`add_icon_item`/`clear`/`remove_item`/`set_item_text`/`get_item_text`/
`set_item_checked`/`is_item_checked`/`scroll_to_item`/`set_item_disabled`). No second participant
is involved - this is a same-process data race between GDScript `Thread`s sharing one `PopupMenu`.

## Class

`memory-safety` (concurrent unsynchronized `Vector`/`CowData` resize vs. resize/read on
`PopupMenu::items` - the identical shape candidate 018 confirmed for `set_item_count`/
`get_item_count`, just reached through a different, much larger set of entry points)

## Root cause

Same field, same missing lock, as candidate 018: `PopupMenu::items` (`mutable Vector<Item> items`,
`scene/gui/popup_menu.h:138` before this fix) has zero synchronization. Candidate 018's own fix only
added `_THREAD_SAFE_METHOD_` to `set_item_count()`/`get_item_count()` - every other public method
that reads or mutates `items` (on the order of 80 of them: all `add_*`/`set_item_*`/`get_item_*`/
`is_item_*` accessors, `toggle_item_checked`/`toggle_item_multistate`, `set_focused_item`/
`get_focused_item`, `scroll_to_item`, `activate_item`/`activate_item_by_event`, `remove_item`,
`add_separator`, `clear`) still does a plain, unlocked `ERR_FAIL_INDEX(p_idx, items.size())`
check-then-act, or a plain `items.resize()`/`items.write[...]`/`items.push_back()` mutation, with no
mutual exclusion against any other thread doing the same. Two threads racing any pair of these
(e.g. one calling `clear()` while another calls `get_item_text()`, or two threads both calling
`add_item()`) hit the exact same `CowData<PopupMenu::Item>` reallocation/bounds-check race as
candidate 018, just via a different pair of call sites. Confirmed directly: this candidate's repro
crashed inside `PopupMenu::clear()` (`Vector<Item>::begin()` -> `ptrw()` -> `CowData::
_copy_on_write()`, an ASan `unknown-crash` READ) and inside a GDScript-bound item getter's
`validated_call` -> `CowData<PopupMenu::Item>::get()` (an OOB read), as well as a plain `FATAL`
bounds-check abort reached through `add_item`/`set_item_text`/`remove_item`/`set_item_disabled`
depending on the exact interleaving of a given run.

## Introduced by

`not determined` (predates tracked history, same situation as candidate 018's own lookup).
`scene/gui/popup_menu.cpp`'s earliest tracked ancestor is `0b806ee0fc` "GODOT IS OPEN SOURCE"
(2014-02-09, the initial open-source commit) - the class has never had any locking at any point in
its public history; the ~80 methods this candidate fixes were each added at different times across
that history (confirmed for a few spot checks via `git log --follow`), but none of them were ever
guarded, consistent with this being the class's original, unchanged thread-safety posture rather
than a regression introduced by any single later commit.

## The fix

Add `_THREAD_SAFE_CLASS_` (`#include "core/os/thread_safe.h"`, a recursive `Mutex` member) as a
private member of `PopupMenu`, and `_THREAD_SAFE_METHOD_` as the first statement of all 80 public
methods that read or mutate `items` - every `add_*`/`set_item_*`/`get_item_*`/`is_item_*` accessor,
`toggle_item_checked`/`toggle_item_multistate`, `set_focused_item`/`get_focused_item`,
`set_item_count`/`get_item_count` (candidate 018's own original scope, now folded into this
candidate's broader, self-contained patch so this `patch.diff` stands on its own against a plain
checkout), `scroll_to_item`, `activate_item`/`activate_item_by_event`, `remove_item`,
`add_separator`, and `clear` (`+83/-0` across `popup_menu.h`/`popup_menu.cpp`, see `patch.diff`).
`Mutex` (as opposed to `BinaryMutex`) is recursive, so a method that internally calls another
locked method on the same object (e.g. several `add_*_item` variants ultimately call through to a
shared private helper, and some setters call `notify_property_list_changed()`/emit signals that
could re-enter the object on the same thread) does not deadlock.

Verified sufficient against the repro: 11/11 clean runs (8 against an earlier repro draft exercising
the same method set, 3 against the final shipped `repro/`), after 6/6 (+3/3 on the earlier draft)
unpatched crashes with the identical repro content.

- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before this
  candidate's fix and after - no new failures (the 1 failure is the pre-existing `test_ip.cpp`
  IPv6 environment issue already present in the baseline from candidates 002-018).

## What's not done / open questions

- **Private/internal methods that also touch `items` without going through the locked public API
  are out of scope here.** `_draw_items()`, `_get_item_height()`, `_shape_item()`,
  `_get_mouse_over()`, `_highlight_first_available_item()`, `_filter_items()`, and similar `private:`
  helpers still read `items` without their own lock - safe today only because every caller of these
  is either (a) one of the now-locked public methods above (the recursive mutex covers that case
  correctly) or (b) internal UI/rendering code that only ever runs on the main thread (`_notification`,
  `_draw_items`, input handling). Not independently audited line-by-line for a caller that reaches
  one of these helpers from a non-main thread without going through a locked public entry point
  first; flagged for a future session rather than assumed clean.
- `OptionButton`/`MenuButton` are both declared `friend class` of `PopupMenu::Item` and reach into
  `PopupMenu` internals directly in a few spots (per candidate 018's own open question) - not
  re-checked this session now that the broader `PopupMenu` surface is locked; worth a follow-up to
  confirm neither bypasses the lock via the friend relationship.
- Only `PopupMenu`'s own `items` field was in scope. `PopupMenuItems` (the inner `Control` that
  actually renders/handles input for the menu, `scene/gui/popup_menu.h:475-486`) reads `popup->items`
  via the friend relationship from its own `gui_input()`/`get_tooltip()` - these run on the main
  thread only (standard `Control` input/rendering contract) so weren't included in this candidate's
  fix, consistent with 018's own scoping.
