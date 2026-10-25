# `InputMap::input_map` has no locking anywhere in the class

`core/input/input_map.h:62` declares `mutable HashMap<StringName, Action> input_map;` with
**zero** `Mutex`/`BinaryMutex`/`_THREAD_SAFE_CLASS_` anywhere in `InputMap` - confirmed by grep
across both `input_map.h` and `input_map.cpp` before this candidate's fix. Every accessor
(`add_action`, `erase_action`, `action_set_deadzone`, `action_add_event`, etc.) reads/writes
`input_map` directly, with no synchronization at all. Unlike candidates 009 (`CameraServer`) and
011 (`AudioServer`), which each had *some* locking elsewhere in the class that one or two accessors
missed, this is the "undocumented singleton with completely unprotected internal state" case in its
purest form. Found via an external 6-thread-bombardment sweep
(`~/Projekty/Godot/Qarminer/tools/thread/SWEEP_RESULTS.md`, 2026-10-05) that already produced a
minimal reproduction project
(`~/Projekty/Godot/Qarminer/problems/crashes/20261005_173454_crashes_InputMap_add_erase_action_race/`),
adapted into this candidate's `gdscript_repro/`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `input_map.h`/`input_map.cpp` lock-usage audit (`grep -n "Mutex"`) | n/a | Zero matches in either file - no synchronization primitive of any kind exists in the class |
| GDScript-level repro (`gdscript_repro/`, adapted from the external sweep's ready-made MRP), against a real, self-built ASan editor binary | `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes` (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb), `--headless --path gdscript_repro`, 6 `Thread`s spamming `InputMap.add_action("race_action")`/`erase_action("race_action")` for 15s each | **Crashed on the first run and all 3 runs tried**, well before the 15s budget elapsed each time - a real, fully-symbolized ASan `heap-use-after-free` (`READ of size 8`) inside `HashMap::has()`'s lookup chain, called from `InputMap::add_action` at `input_map.cpp:119`, from a `CoreBind::Thread` worker |
| Fix (`_THREAD_SAFE_CLASS_`/`_THREAD_SAFE_METHOD_` added to all 18 methods touching the class's 3 maps), same build/run, 8 runs | Same build/run | **Clean 8/8 runs** |
| Project test suite before/after | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` with the fix applied - identical to the established baseline from candidates 002-011 (same pre-existing `test_ip.cpp` IPv6 environment failure) |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (every attempt tried, each crashing well inside the
15-second budget), and 8/8 clean runs with the complete fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`gdscript_repro/`), run against a real,
self-built ASan editor binary, driving the actual `InputMap` singleton through nothing but the
public GDScript API (`InputMap.add_action()`/`.erase_action()`). No second participant is involved
(not a peer/wire-input finding) - this is a same-process data race between GDScript `Thread`s
calling the same two unsynchronized accessors.

## Class

`memory-safety` (concurrent unsynchronized `HashMap` mutation/lookup - same mechanism as
candidate 008's `Input::action_states` race, which is the analogous `Input` singleton bug this
same external sweep's report explicitly compares `InputMap` against)

## Root cause

`InputMap::input_map` is a plain `HashMap<StringName, Action>` with no synchronization primitive
anywhere in the class. `add_action()` does an unlocked `has()` check followed by 3 separate
unlocked `operator[]` writes (`input_map.cpp:118-125`); `erase_action()` does an unlocked `has()`
check followed by an unlocked `erase()` (`:127-131`). When multiple `Thread`s call
`add_action()`/`erase_action()` on the *same* action name concurrently, two threads can race
inside the `HashMap`'s internal bucket array during a concurrent insert/erase (which can trigger a
rehash/reallocation) - one thread's `has()` lookup chain (`_lookup_idx` ->
`_lookup_idx_unchecked` -> `StringName::operator==`) dereferences a bucket that a concurrent
`erase()`/insert on another thread has already freed or relocated, producing the heap
use-after-free ASan caught here. `default_builtin_cache`/`default_builtin_with_overrides_cache`
(the other two maps in the class, populated lazily by `get_builtins()`/
`get_builtins_with_feature_overrides_applied()`) share the identical unguarded shape, though this
candidate's repro only exercises `input_map` directly.

Reachability: `InputMap` is a globally accessible singleton, fully scriptable from any thread - no
special setup beyond calling `add_action`/`erase_action` is needed to reach this race.

## Introduced by

`not determined` (predates tracked history in the meaningful sense). `InputMap`'s source has
existed since 0b806ee0fc "GODOT IS OPEN SOURCE" (2014-02-09, the initial open-source commit) with
no `Mutex` ever added to the class at any point since - this is not a regression from a later
refactor, it is the class's original, unchanged thread-safety posture across the engine's entire
public history.

## The fix

Add `_THREAD_SAFE_CLASS_` (a recursive `Mutex`, `core/os/thread_safe.h`) as a private member and
`_THREAD_SAFE_METHOD_` as the first statement of every public method that touches `input_map`,
`default_builtin_cache`, or `default_builtin_with_overrides_cache` - matching the exact convention
candidates 008/009/010 already established in this workspace (`+59/-9` against
`core/input/input_map.h`/`.cpp`, see `patch.diff`):

```cpp
// input_map.h
private:
	static inline InputMap *singleton = nullptr;

	_THREAD_SAFE_CLASS_

	mutable HashMap<StringName, Action> input_map;
	...

// input_map.cpp
void InputMap::add_action(const StringName &p_action, float p_deadzone) {
	_THREAD_SAFE_METHOD_

	ERR_FAIL_COND_MSG(input_map.has(p_action), ...);
	...
}
```

18 methods instrumented: `suggest_actions`, `add_action`, `erase_action`, `get_actions`,
`has_action`, `get_action_description`, `action_get_deadzone`, `action_set_deadzone`,
`action_add_event`, `action_has_event`, `action_erase_event`, `action_erase_events`,
`action_get_events`, `event_get_action_status`, `get_action_map`, `load_from_project_settings`,
`get_builtins`, `get_builtins_with_feature_overrides_applied`. `event_is_action()`/
`event_get_index()` were deliberately left uninstrumented - they only delegate to
`event_get_action_status()` and never touch `input_map` directly, so no lock is needed at their own
level.

`_THREAD_SAFE_CLASS_`'s `Mutex` is **recursive** (`core/os/mutex.h:97`,
`MutexImpl<std::recursive_mutex>`), which is load-bearing here, not incidental: several
instrumented methods call other instrumented methods on the same thread while already holding the
lock - `erase_action()`/`get_action_description()`/`action_get_deadzone()`/etc. call
`suggest_actions()` on their error path, `load_from_project_settings()`/`load_default()` call
`add_action()`/`action_add_event()` in a loop, and `get_builtins_with_feature_overrides_applied()`
calls `get_builtins()`. A non-recursive `BinaryMutex` here would self-deadlock on the very first
`erase_action()` call for a nonexistent action; the recursive `Mutex` (the same primitive
candidates 008/009/010 already use) makes every one of these nested calls safe.

**A second, cross-class deadlock risk was found and fixed during this candidate's own
verification, before it ever reached `Confirmed`**: the naive version of this patch wrapped
`action_erase_event()`/`action_erase_events()` in `_THREAD_SAFE_METHOD_` for their *entire* body,
including their existing calls to `Input::get_singleton()->is_action_pressed()`/`action_release()`
- meaning both would call into `Input` while still holding `InputMap`'s lock. Separately,
`Input`'s own locked input-parsing path (`core/input/input.cpp`, around line 1087, inside a
function already guarded by `_THREAD_SAFE_METHOD_`/`_THREAD_SAFE_UNLOCK_`/`_THREAD_SAFE_LOCK_`)
calls back into `InputMap::get_action_map()`/`event_is_action()` while holding *`Input`'s* lock.
That is an AB-BA lock-order inversion between two different classes' mutexes: thread A takes
`Input`'s lock then wants `InputMap`'s; thread B takes `InputMap`'s lock then wants `Input`'s. Two
threads hitting both paths concurrently could deadlock. (Note: `Input::is_action_pressed()`/
`action_release()` *themselves* currently lack `_THREAD_SAFE_METHOD_` - this is precisely
candidate 008's still-open, not-yet-applied-for-real finding - so this specific inversion isn't
triggerable in the engine's current unpatched state; but it would become live the moment candidate
008's own fix is applied, since that's the correct, expected fix for that finding. Written here
defensively rather than left as a latent trap for whoever applies both patches together.)

**Fixed** by restructuring both methods so `InputMap`'s lock is scoped to only the `input_map`
mutation, released (via the inner block's `MutexLock` destructor) *before* either method calls into
`Input`:

```cpp
void InputMap::action_erase_event(const StringName &p_action, RequiredParam<InputEvent> p_event) {
	EXTRACT_PARAM_OR_FAIL(event, p_event);

	bool event_erased = false;
	{
		_THREAD_SAFE_METHOD_
		ERR_FAIL_COND_MSG(!input_map.has(p_action), suggest_actions(p_action));
		List<Ref<InputEvent>>::Element *E = _find_event(input_map[p_action], event, true);
		if (E) {
			input_map[p_action].inputs.erase(E);
			event_erased = true;
		}
	}

	if (event_erased && Input::get_singleton()->is_action_pressed(p_action)) {
		Input::get_singleton()->action_release(p_action);
	}
}
```
(and symmetrically for `action_erase_events()`). No behavior change - the early-return semantics of
`ERR_FAIL_COND_MSG` and the order of operations are preserved exactly; only the lock's scope
shrank. Caught by re-reading the patch's own call graph during verification, not by a tool/sanitizer
- TSan/a deadlock detector was not run against this specific interleaving (see "What's not done"
below).

Verified: 8/8 clean runs of `gdscript_repro/` against the corrected fix (vs. crashing on all 3/3
unfixed runs tried, each well inside the 15s budget).

- **Applied for real** to `core/input/input_map.h`/`.cpp`, rebuilt the ASan editor binary (first
  build after the `.h` change triggered a full rebuild of dependents, ~9m26s; the second build,
  after correcting the two methods above with only `.cpp` changed, was a fast incremental rebuild,
  ~53s), ran `gdscript_repro/` and the test suite against both the initial and corrected fix, then
  reverted (`git checkout -- core/input/input_map.h core/input/input_map.cpp`) before finishing
  this candidate - main tree confirmed clean (`git status`) afterward.
- Project test suite (`bin/godot.linuxbsd.editor.x86_64.san --headless --test`, same ASan +
  debug_symbols binary, absolute `XDG_DATA_HOME`): `1431 test cases | 1430 passed | 1 failed | 3
  skipped`, identical to the established baseline (same pre-existing `test_ip.cpp` IPv6 failure,
  unrelated to this change), for both the initial and corrected fix. No new failures.

## What's not done / open questions

- `default_builtin_cache`/`default_builtin_with_overrides_cache` were given the same
  `_THREAD_SAFE_METHOD_` treatment as `input_map` (same class, same shape, same fix), but no
  dedicated repro was built to independently exercise a concurrent-`get_builtins()` race - only
  `add_action()`/`erase_action()` on `input_map` was independently confirmed this session.
- `get_action_map()` returns a direct reference to `input_map` under the lock, but the lock is
  released the instant the method returns - a caller iterating the returned reference while another
  thread concurrently mutates `input_map` is still possible. This is the same known limitation
  already accepted elsewhere in this engine for reference-returning getters under a scoped lock
  (e.g. candidate 010's note on `ProjectSettings`'s sibling `HashMap` getters in `IDEAS.md`) - out
  of scope to redesign the API shape for this candidate.
- The `Input`/`InputMap` lock-ordering fix above was reasoned through by reading the call graph on
  both sides (confirmed via `grep`, not guessed), not verified empirically under a deadlock
  detector/TSan - the interleaving it prevents only becomes reachable once candidate 008's own fix
  is also applied (currently it isn't, in this main tree), so there is no live repro for the
  deadlock itself, only for the race this candidate's own patch fixes. A future session applying
  both candidate 008's and this candidate's fixes together would be the natural point to add a
  dedicated concurrent stress test (`Input.action_press`/`action_release` on one set of threads,
  `InputMap.action_erase_event`/`action_erase_events` on another) to empirically confirm no
  deadlock, rather than relying solely on the code-reading argument made here.
- Did not do a similar call-graph audit for every *other* already-locked singleton `InputMap` calls
  into (only `Input` was checked, since it's the one instrumented method pair that calls into
  another locked class's public API) - `ProjectSettings::get_property_list()`/`GLOBAL_GET`
  (called from `load_from_project_settings()`) were spot-checked and found not to call back into
  `InputMap` while holding `ProjectSettings`'s own lock (its one `InputMap`-calling site,
  `_add_builtin_input_map()`, doesn't hold the lock itself), so no reverse-order risk there - but
  this was not exhaustively re-checked for every other call-out.
