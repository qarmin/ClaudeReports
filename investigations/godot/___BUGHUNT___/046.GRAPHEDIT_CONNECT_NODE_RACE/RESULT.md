# `GraphEdit::connect_node()`/`disconnect_node()` race on unguarded `connections`/`connection_map`

Closes the external thread-safety sweep's `Control` widget lead entirely - `GraphEdit` was the
last of the original 10 widgets (candidates 039 `Label`, 040 `LineEdit`, 041 `ItemList`, 042
`Tree`, 043 `SplitContainer`, 044 `TabBar`, 045 `TextEdit` closed the rest; `OptionButton` was ruled
out, `MenuButton` had an unrelated bug already fixed by candidate 017). `GraphEdit` keeps its
connection data in `Vector<Ref<Connection>> connections` plus `HashMap<StringName,
List<Ref<Connection>>> connection_map`, with zero `Mutex`/`_THREAD_SAFE_CLASS_` anywhere in the
header.

This candidate's repro is noisier than most in this sweep because `connect_node()`/`disconnect_node()`
also perform genuine scene-tree structural operations (`add_child()`/`queue_free()` on an internal
`Line2D`) that are *already* correctly main-thread-guarded by the engine - calling these methods off
the main thread always logs `"Adding children to a node inside the SceneTree is only allowed from
the main thread"`/`"The caller thread can't call ... queue_redraw()"` errors regardless of this
candidate's fix. Those errors are not the bug; the real crash is underneath them, as detailed below.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/gui/graph_edit.h`/`.cpp` - confirmed zero `Mutex` anywhere in the header; `connect_node()`/`disconnect_node()` mutate `connections`/`connection_map` directly with no lock, and `disconnect_node()` additionally calls `conn_to_remove->_cache.line->queue_free()` using a raw `Line2D*` stored by a potentially-concurrent `connect_node()` call | Static read | Confirmed `connect_node`/`disconnect_node`/`is_node_connected`/`get_connection_list` are all `ClassDB`-bound, reachable from GDScript |
| GDScript-level repro (`repro/`: shared `GraphEdit`, added to tree, 3 threads looping `connect_node("A",0,"B",0)` + `disconnect_node("A",0,"B",0)`, 3 threads looping `get_connection_list()`/`is_node_connected(...)` for 3s), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/046`, `bin`/`.sconsign5.dblite` hardlinked from the main tree) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on all 3 attempts** (2 plain SIGSEGV, 1 with a captured real ASan heap-use-after-free, `READ of size 8`) |
| Re-run under `gdb -batch -ex run -ex "thread apply all bt"` to get a symbolized capture of the actual fault (the plain ASan runs hit noisy concurrent log output from the separate, already-guarded structural-operation errors, obscuring the backtrace) | Same binary/repro | SIGSEGV inside `Node::queue_free()` (`scene/main/node.cpp:3506`, reading `data.tree` through what the backtrace shows as an invalid `this`), reached directly from `GraphEdit::disconnect_node()` at `graph_edit.cpp:328` - the exact line `conn_to_remove->_cache.line->queue_free();`. A separate, real ASan heap-use-after-free (`READ of size 8`) was captured in the same run, confirming genuine memory corruption, not just a null-check edge case |
| Fix (`_THREAD_SAFE_CLASS_` on the header + `_THREAD_SAFE_METHOD_` on `connect_node()`, `disconnect_node()`, `is_node_connected()`, `_get_connection_list()`, `_get_connection_list_from_node()` - the methods that directly touch `connections`/`connection_map`), incremental rebuild (ccache-warm hardlinked worktree, ~1m40s) | Same build/run | **Clean 8/8 runs**, "survived" printed every time |
| Project test suite before/after (revert to unfixed source, rebuild ~1m34s, run; reapply fix) | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | Identical both before and after: `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - same pre-existing `test_ip.cpp` IPv6 environment failure already documented in every prior candidate's baseline |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (a real ASan heap-use-after-free in one run, plain
SIGSEGVs in the other two, a gdb-wrapped re-run pinning the exact faulting line), and 8/8 clean
runs with the fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `GraphEdit` class through nothing but the public GDScript
API. No second participant is involved - a same-process crash reachable from GDScript `Thread`s
calling `connect_node()`/`disconnect_node()` concurrently on a shared `GraphEdit`.

## Class

`memory-safety` (heap corruption on `connections`/`connection_map`, manifesting as a dangling
`Line2D*` pointer - concurrent `connect_node()`/`disconnect_node()` calls race each other's
mutation of the shared `Vector`/`HashMap`, corrupting which `Connection` object's `_cache.line`
field is current; a subsequent `disconnect_node()` call dereferences a stale/freed `Line2D*`
through `conn_to_remove->_cache.line->queue_free()`, crashing inside `Node::queue_free()`).

## Root cause

```cpp
Error GraphEdit::connect_node(...) {
	...
	connections.push_back(c);                    // <-- unguarded resize
	connection_map[p_from].push_back(c);          // <-- unguarded HashMap mutation
	connection_map[p_to].push_back(c);
	...
	c->_cache.line = line;                        // <-- stores a raw Line2D* on the new Connection
	...
}

void GraphEdit::disconnect_node(...) {
	...
	if (conn_to_remove.is_valid()) {
		connection_map[p_from].erase(conn_to_remove);   // <-- unguarded HashMap mutation
		connection_map[p_to].erase(conn_to_remove);
		conn_to_remove->_cache.line->queue_free();       // <-- dereferences a raw Line2D*
		connections.erase(conn_to_remove);               // <-- unguarded resize
		...
	}
}
```

`connections` (`Vector<Ref<Connection>>`) and `connection_map` (`HashMap<StringName,
List<Ref<Connection>>>`) are plain members with zero synchronization anywhere in the class. Two
threads calling `connect_node()`/`disconnect_node()` concurrently race each other's mutation of
these shared containers - confirmed via a real ASan `heap-use-after-free` (`READ of size 8`) - and
the corruption surfaces downstream when a `disconnect_node()` call dereferences
`conn_to_remove->_cache.line`, a raw `Line2D*` whose lifetime is entangled with the same race
(confirmed via gdb: `SIGSEGV` inside `Node::queue_free()`, called directly from
`GraphEdit::disconnect_node()` at `graph_edit.cpp:328`).

Separately, and **not itself the crash mechanism**: `connect_node()`'s `connections_layer->
add_child(line)` and both methods' `queue_redraw()` calls are genuine scene-tree structural
operations that Godot's own engine-level thread guard correctly rejects when called off the main
thread (logged as `ERR_FAIL` errors, not crashes) - this is pre-existing, intentional behavior,
unrelated to and not fixed by this candidate.

Reachability: `connect_node`/`disconnect_node`/`is_node_connected`/`get_connection_list` are all
bound, GDScript-callable public methods - `GraphEdit` is an ordinary `Control`-derived node,
reachable from any thread with no special setup once added to the tree (the structural-operation
errors above fire but do not prevent the data-race crash from also occurring).

## Introduced by

not determined - `scene/gui/graph_edit.cpp` has existed since this candidate's earliest ancestor
commit (fbdd925d9be1c4c96d05089d7d5a58cd938b002c "Work in progress visual shader editor *DOES NOT
WORK YET*", 2015-01-03; confirmed via `git log --follow`) and has never had any locking since.

## The fix

`+10/-0` against `scene/gui/graph_edit.cpp`, `+3/-0` against `scene/gui/graph_edit.h` (see
`patch.diff`): add `_THREAD_SAFE_CLASS_` next to the private members, and `_THREAD_SAFE_METHOD_` to
`connect_node()`, `is_node_connected()`, `disconnect_node()`, `_get_connection_list()`, and
`_get_connection_list_from_node()` - the methods that directly touch `connections`/`connection_map`.

Verified: 8/8 clean runs of `repro/` against the fix (vs. 3/3 crashing unfixed). Note: the
engine-level "must be called from the main thread" errors for the structural `add_child`/
`queue_redraw` calls still occur after this fix (correctly, and unrelated to it) - "clean" here
means no crash/corruption, not that every logged error disappears.

- **Applied for real** in an isolated worktree (`.worktrees/046`, created from and removed back
  into the main project's own clean tree - never touched the main working tree directly), rebuilt
  the ASan editor binary, ran `repro/` and the full test suite against both the unfixed and fixed
  source, then discarded the worktree (`git worktree remove --force`) - main tree confirmed clean
  (`git status`) throughout and at the end.
- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before (unfixed
  source, freshly rebuilt) and after the fix - identical to the established baseline (same
  pre-existing `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- `GraphEdit` has 139 public methods; this fix covers only the 5 that touch
  `connections`/`connection_map` directly. The class's much larger surface (node arrangement,
  minimap, panning/zooming, box selection, etc.) remains completely unguarded.
- `get_connections()` (returns `const Vector<Ref<Connection>>&`) was deliberately left unguarded,
  same "caller outlives the lock" reasoning as `Image::get_data()`/candidate 015's `CSGShape3D` -
  locking it would release the lock the instant it returns, while the caller still holds the
  unprotected reference. `_get_connection_list()` (the actually GDScript-bound wrapper, which
  copies into a new `TypedArray` before returning) is locked instead, which is sufficient for the
  bound API surface.
- **This closes the entire `Control`-widget portion of the external thread-safety sweep.** All 10
  originally-flagged widgets are now accounted for: 8 confirmed-and-fixed (`Label`/039,
  `LineEdit`/040, `ItemList`/041, `Tree`/042, `SplitContainer`/043, `TabBar`/044, `TextEdit`/045,
  `GraphEdit`/046), 1 ruled out as delegating to an already-fixed sibling (`OptionButton`), 1
  resolved by a pre-existing, unrelated fix (`MenuButton`/candidate 017), and `CodeEdit` inherits
  `TextEdit`'s fix unchanged (not independently repro'd).
