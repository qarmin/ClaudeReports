# `Node::get_name()` has zero thread guard, races `set_name()`'s renames - and the obvious fix recurses into a stack overflow

From the coordinator's priority list ("`Node` (the base class itself)! worth checking directly, it's
in the sweep's Group A list"). Unlike `Window` (candidate 049, ruled not reproducible - 147 existing
guards), `Node` turned out to have a genuine, zero-guard gap on one of its most commonly used public
methods, **and** the textbook fix for it (the same macro used by a dozen other methods in this exact
file) turned out to recurse into a stack overflow due to an undocumented dependency cycle
(`get_name()`'s error path → `get_description()` → `get_path()` → `get_name()` on every ancestor).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/main/node.cpp` - `Node::get_name()` is `return data.name;` with **zero** guard of any kind, while `Node::set_name()` has a manual `ERR_FAIL_COND_MSG(data.tree && !Thread::is_main_thread(), ...)` restricting *writes* to the main thread once the node is in a tree. `StringName`'s `operator=`/copy-constructor (`core/string/string_name.cpp`) manage a global intern table (`Table::table`) under `Table::mutex`, but only around the table *mutation* when a refcount hits zero - the field-level read/write of `data.name` itself (a plain `StringName` member) has no synchronization at all | Static read | Identified `get_name()` as a genuinely new, unguarded gap: the intended, documented-as-supported shape (main thread renames via `set_name()`, worker threads read via `get_name()`) races a `StringName::operator=`'s `unref()`-then-reassign sequence against a concurrent reader's copy-construction of the same field |
| First repro attempt: node *not* added to the tree, 3 writer + 3 reader threads both off the main thread (an out-of-contract scenario per `is_accessible_from_caller_thread()`'s own documented design, but still worth empirically checking per this session's established practice, same as candidate 049's "before add_child" approach) | GDScript-level repro, self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb), `--headless --path repro`, `timeout -k 5 30` | **Hung/deadlocked instead of crashing** - never reached "survived" even at a 30s timeout. One run surfaced `ERROR: BUG: Unreferenced static string to 0: servers` from `StringName::unref()` - an *unrelated* static string's refcount reaching zero, strong circumstantial evidence the race had already corrupted the global intern table, consistent with some traversal looping forever while holding `Table::mutex` (which would explain the apparent total engine hang, since practically everything touches `StringName`). Not pursued further as the primary finding since it falls outside the documented-supported shape and a clean crash report wasn't obtained - recorded here as corroborating evidence, not the confirmed repro |
| **Real repro** (`repro/`): node added to the tree; *all* renames come from the **main thread** (`_ready()`'s own loop, satisfying `set_name()`'s existing guard) using fresh never-before-seen names (`"name_%d" % randi()`) so each rename's old value's refcount actually reaches zero; 6 reader threads calling `n.name` (`get_name()`) in a tight loop for 6s | Same build/run | **Crashed on 4/4 attempts** - `heap-use-after-free core/templates/safe_refcount.h:87 in SafeNumeric<unsigned long>::decrement()` / `Object::_predelete()`, reached through `String::operator=(String&&)` inside the GDScript VM's implicit `StringName`→`String` conversion (`var s: String = n.name`) - a reader thread copy-reading `data.name` exactly as the main thread's `set_name()` was freeing the old `_Data` out from under it |
| First fix attempt: added the standard `ERR_READ_THREAD_GUARD_V(StringName())` (the exact macro `get_child_count()`/`get_child()`/`get_children()` already use in this same file) to the top of `get_name()` - rebuilt, reran | Same build/run, 10 attempts | **Still "crashed" 10/10**, but with a *completely different*, much worse failure mode: a **silent segfault with zero output** - no ASan report, no GDScript backtrace, not even the guard's own error message. Investigated and found the cause: `ERR_READ_THREAD_GUARD_V`'s generated error message calls `get_description()`, which (since the node is inside the tree) calls `get_path()`, which calls `get_name()` on **every ancestor in the chain** - recursing straight back into the same guard, on the same (still-disallowed) thread, over and over, stack-overflowing before a single byte of the error message could be formatted or printed |
| Complete fix: replaced the macro with an equivalent manual guard that uses `get_class()` (a plain static type-name lookup, touches no `Node` state) instead of `get_description()` in the error message - rebuilt, reran | Same build/run, 10 attempts | **Clean 10/10**, "survived" every time - and the rejection message (`"...get_name() ... can only be accessed from either the main thread or a thread group..."`) fired **564,947 times** in one run's log, confirming the guard is actually doing real, continuous work rejecting the reader threads' now-disallowed calls, not just a lucky absence of the race |
| Project test suite with the complete fix applied | `XDG_DATA_HOME=<tmp> ./bin/godot....san --headless --test` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - identical to the established baseline, no new failures |

## Status

**Confirmed.** 4/4 unfixed crashes in the documented-supported "main thread renames, worker threads
read" shape, 10/10 crashing (differently and more dangerously) with the obvious textbook fix, 10/10
clean with the corrected fix.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `Node`'s public GDScript API (`.name` property / `get_name()`/
`set_name()`), in exactly the shape the engine's own existing `set_name()` guard implies is meant to
be supported.

## Class

`memory-safety` (`StringName`'s own `_Data` refcount/table-entry lifetime race, structurally the
same shape as this session's `CowData` races even though `StringName` isn't `CowData`-backed itself
- it's a separate hand-rolled refcounted interning scheme with the identical "unsynchronized field
holding a refcounted handle, read and written from different threads" defect).

## Root cause

`Node::get_name()` (`scene/main/node.cpp:1431`, before this fix) is a bare `return data.name;` with
no thread guard of any kind - unlike nearly every other public accessor in this class.
`Node::set_name()` has its own guard restricting *writes* to the main thread once the node is inside
a tree, implying the engine's own design intends "main thread renames, other threads/thread-groups
read" to be a supported pattern (consistent with `get_child_count()`/`get_child()`/`get_children()`
all being `ERR_THREAD_GUARD`-protected the same way). But `get_name()` was simply never given the
matching read-side guard.

`StringName::operator=` (`core/string/string_name.cpp:196`) does, in order: (1) `unref()` the
current value (decrementing its `_Data`'s refcount, and - critically - **freeing `_Data` without
resetting this instance's own `_data` pointer to null** if the count hits zero), then (2) ref and
assign the new value. A concurrent reader copy-constructing from the same field (`StringName(const
StringName&)`, used implicitly for `return data.name;` and again for the GDScript VM's `StringName`→
`String` conversion) reads `_data` and calls `_data->refcount.ref()` - if this read lands in the
window after step (1)'s free but before step (2)'s reassignment, it's a heap-use-after-free on
already-freed memory, exactly what ASan caught.

**The fix's own complication is itself worth recording as a lesson**: `scene/main/node.h`'s
`ERR_READ_THREAD_GUARD_V` macro formats its error message using `get_description()`, which (for an
in-tree node) calls `get_path()`, which calls `get_name()` on every node from `this` up to the root.
Adding the standard guard to `get_name()` therefore creates a direct recursive cycle the moment the
guard actually *fires* on an in-tree node - `get_name()`'s failure path calls a function that calls
`get_name()` again, on the same disallowed thread, infinitely. This is **not** a pre-existing bug
(no code path reaches it today, since nothing currently guards `get_name()`) - it is a landmine that
only the *fix itself* would have stepped on, had it used the copy-paste-obvious macro instead of a
manually-written equivalent.

## Introduced by

Not independently dated - `Node::get_name()` has had this exact unguarded shape since thread-safety
guards were first introduced to this class (the surrounding methods have guards; this one never
did). The `get_description()`/`get_path()`/`get_name()` recursive dependency is simply a pre-existing
structural fact about the class, not a regression.

## The fix

`+7/-0` in `scene/main/node.cpp` (see `patch.diff`):

```cpp
StringName Node::get_name() const {
	if (unlikely(!is_readable_from_caller_thread())) {
		ERR_FAIL_V_MSG(StringName(), vformat("%s: The function `get_name()` on this node can only be accessed from either the main thread or a thread group. Use `call_deferred()` instead.", get_class()));
	}
	return data.name;
}
```

Deliberately **not** `ERR_READ_THREAD_GUARD_V(StringName())` - see above. `get_class()` is a plain,
non-recursive static type-name lookup, safe to call from any thread regardless of guard state.

Verified: 10/10 clean runs of `repro/` with the fix applied (vs. 4/4 crashing unfixed, and 10/10
*worse*-crashing - a silent stack-overflow segfault - with the naive macro-based fix). Test suite:
`1431 | 1430 passed | 1 failed | 3 skipped` with the fix applied - identical to the established
baseline.

## What's not done / open questions

- The "node not yet added to the tree, multiple threads racing on the very same instance" scenario
  (my first repro attempt) is out of scope for this fix (and arguably out of the engine's own
  documented contract per `is_accessible_from_caller_thread()`'s design) - not independently
  resolved, just noted as corroborating evidence that the underlying `StringName` field race is real
  and severe enough to produce an apparent engine-wide hang, not just a clean crash.
- `set_name()`'s own guard was not touched - it already correctly restricts writes to the main
  thread once `data.tree` is set; only the missing read-side guard was added.
- `get_path()`/`get_path_to()` and anything else that walks `get_name()` across multiple nodes were
  not independently audited for the same "error-path recursion" landmine - worth a quick grep across
  the rest of `node.cpp`'s guarded methods before anyone adds a new guard there using the standard
  macros, given how easily this one bit back.
- `MultiplayerSpawner` is the next item on the coordinator's priority list.
