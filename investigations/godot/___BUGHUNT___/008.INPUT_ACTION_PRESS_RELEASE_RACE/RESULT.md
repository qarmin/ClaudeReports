# `Input::action_press()`/`action_release()` and friends missing `_THREAD_SAFE_METHOD_`

Found during this session's sweep for more instances of the "locked-writer vs. unlocked-reader"
pattern established by candidates 002-004: `grep -rl "_THREAD_SAFE_METHOD_\|_THREAD_SAFE_CLASS_"`
turned up `core/input/input.h`/`.cpp` as one of 44 files using the `_THREAD_SAFE_CLASS_`/
`_THREAD_SAFE_METHOD_` macros (`core/os/thread_safe.h`) - a `mutable Mutex`-backed RAII lock
idiom used throughout the engine. Auditing `Input`'s own methods for consistent lock usage found
a cluster of 9 public, widely-used methods that never lock at all, despite sharing state with
methods elsewhere in the same file that do.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: lock-usage audit of `core/input/input.cpp` | n/a | `is_key_pressed`, `is_joy_button_pressed`, `get_joy_axis`, and others all start with `_THREAD_SAFE_METHOD_` - but `is_action_pressed`, `is_action_just_pressed`, `is_action_just_pressed_by_event`, `is_action_just_released`, `is_action_just_released_by_event`, `get_action_strength`, `get_action_raw_strength` (`:415-580`) and `action_press`/`action_release` (`:1515-1543`) all read/write `HashMap<StringName, ActionState> action_states` with **no lock at all**. `_parse_input_event_impl` (the engine's own internal writer, `:883`) is guarded by `DEV_ASSERT(Thread::get_caller_id() == Thread::get_main_id())` - documented main-thread-only - but `action_press()`/`action_release()` are plain public, `ClassDB`-bound methods (`core/input/input.cpp:205-206`) with no such restriction, callable from any GDScript `Thread` |
| GDScript-level repro (`gdscript_repro/`), against a real, self-built ASan editor binary | `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`, `--headless --path gdscript_repro`, 16 `Thread`s calling `Input.action_press()`/`.action_release()` on 10 built-in actions concurrently, 14 total runs across two batches (~15-20s each) | **4/14 runs crashed** (SIGSEGV) - but unlike every other candidate this session, with **no usable backtrace**, from either ASan's own handler or `gdb` on a core dump (see "Status" below for why that's itself meaningful) |
| Fix verification: same `gdscript_repro/` against the fixed, rebuilt binary | Same binary/build (incremental rebuild, ~44s - only `input.cpp` changed), 10 runs | **Clean 10/10 runs**, "No crash within the time limit" every time |
| Project test suite before/after | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` with the fix applied - identical to the established baseline from candidates 002-007 (same pre-existing `test_ip.cpp` IPv6 environment failure) |

## Status

**Confirmed.**

Reproduces **4/14** runs crashing against the unfixed binary; **10/10** clean runs with the fix
applied. This is a narrower hit rate than candidates 002-004/006 (which were consistently
reproducible or near-100%), closer in character to candidate 005's timing-dependent race - except
here the underlying bug (concurrent, fully-unsynchronized `HashMap::operator[]` mutation from 16
threads at once, not just a reader-vs-writer pair) is a *more* severe corruption shape than any
other candidate this session, evidenced by every crash producing a completely unsymbolizable
stack (neither ASan's signal handler nor `gdb` on a core dump could unwind past the fault frame) -
see `gdscript_repro/README.md` for the full gdb output and reasoning.

**Fully reproducible**: **Yes** - a real Godot project (`gdscript_repro/`), run against a real,
self-built ASan editor binary, driving the actual `Input::action_press`/`action_release`/
`is_action_pressed`/etc. methods through nothing but the public GDScript `Input` singleton,
which every Godot game uses. No standalone C++ mirror was built for this candidate (per this
project's current repro policy) - this bug has a trivial, extremely common GDScript entry point.

## Class

`memory-safety` (concurrent unsynchronized `HashMap` mutation - same mechanism as candidate 003,
but with *more* concurrent writers racing each other directly, rather than one lazy-init race)

## Root cause

`core/input/input.cpp`. `Input` declares a single shared `_THREAD_SAFE_CLASS_` mutex
(`core/input/input.h:80`) and most of its methods correctly wrap their body in
`_THREAD_SAFE_METHOD_` (`core/os/thread_safe.h:36`, a `MutexLock` RAII guard on that shared
mutex) - but 9 methods that read or write `HashMap<StringName, ActionState> action_states` never
do:

- **Writers**: `action_press()`, `action_release()` (`:1515-1543`) - public, `ClassDB`-bound,
  callable from any thread via `Input.action_press()`/`Input.action_release()`.
- **Readers**: `is_action_pressed()`, `is_action_just_pressed()`,
  `is_action_just_pressed_by_event()`, `is_action_just_released()`,
  `is_action_just_released_by_event()`, `get_action_strength()`, `get_action_raw_strength()`
  (`:415-580`) - also public, `ClassDB`-bound, and additionally called internally by `get_axis()`/
  `get_vector()`.

Both `action_press`/`action_release` *and* the real engine-internal input pipeline
(`_parse_input_event_impl`, which writes the same map on the main thread and is itself correctly
guarded) mutate `action_states` via `HashMap::operator[]`, which can insert a new key and trigger
a `_resize_and_rehash()` (same mechanism as candidate 002/003). With no lock on the
`action_press`/`action_release` side, two or more threads calling either function concurrently -
this session's repro used only `action_press`/`action_release` themselves, no interaction with
`_parse_input_event_impl` needed - can both observe the map in a pre-rehash state and both
proceed to insert/mutate concurrently, corrupting the map's internal arrays. Because this is
*mutual* writer-vs-writer corruption (not a single lazy-init race, and not a single reader
dereferencing a buffer another thread is freeing), the resulting corruption is more severe and
less localized - consistent with this candidate's signature crashes being unsymbolizable rather
than landing cleanly in `HashMap::_insert_empty_at` or similar, as seen in candidates 002/003.

Reachability: `Input.action_press()`/`.action_release()` are ordinary, documented, commonly-used
GDScript API methods (used for simulating input, e.g. in automated testing, accessibility
features, or virtual on-screen controls) - calling either from more than one `Thread` at once, or
from a `Thread` while another is reading via `is_action_pressed()`/`get_action_strength()` (which
every game calls every frame, often many times, from `_process`/`_physics_process`), reaches this
race through entirely ordinary, supported-looking GDScript code.

## Introduced by

`not determined`. The asymmetry (most `Input` methods lock, these 9 specifically don't) suggests
these were added, or last touched, separately from the methods around them without the
`_THREAD_SAFE_METHOD_` convention being followed - not bisected this session.

## The fix

Add `_THREAD_SAFE_METHOD_` as the first statement of all 9 methods, matching the exact convention
already used by their sibling methods elsewhere in the same file (e.g. `is_key_pressed`,
`get_joy_axis`) and sharing the same class-wide `_thread_safe_` mutex - no new lock, no new
idiom, purely closing a gap in an already-established pattern (`+18/-0` against
`core/input/input.cpp`, see `patch.diff`):

```cpp
bool Input::is_action_pressed(const StringName &p_action, bool p_exact) const {
    _THREAD_SAFE_METHOD_

    ERR_FAIL_COND_V_MSG(!InputMap::get_singleton()->has_action(p_action), false, ...);
    ...
}
```
(and identically for the other 8 methods). `Input`'s `_thread_safe_` is a plain `Mutex`
(recursive, per `core/os/mutex.h:97`), so `get_axis()`/`get_vector()` calling
`get_action_strength()`/`get_action_raw_strength()` (each independently taking the lock) remains
safe.

Verified: 10/10 clean runs of `gdscript_repro/` against the rebuilt binary (vs. 4/14 crashing
before), identical test-suite results (`1431 | 1430 | 1 | 3`, same baseline).

- **Applied for real** to `core/input/input.cpp`, rebuilt the ASan editor binary (incremental,
  ~44s - single translation unit, no header touched), ran `gdscript_repro/` and the test suite
  against it, then reverted (`git checkout -- core/input/input.cpp`) before finishing this
  candidate - main tree confirmed clean (`git status`) afterward.
- Project test suite (`bin/godot.linuxbsd.editor.x86_64.san --headless --test`, same ASan+
  debug_symbols binary, absolute `XDG_DATA_HOME`): `1431 test cases | 1430 passed | 1 failed |
  3 skipped`, identical to the established baseline (same pre-existing `test_ip.cpp` IPv6
  failure, unrelated to this change). No new failures.

## What's not done / open questions

- **Crashes from the unfixed binary could not be symbolized** (neither ASan's handler nor `gdb`
  on a core dump produced a usable backtrace) - the fix is justified entirely by the code-reading
  argument (a genuinely unlocked, concurrently-reachable `HashMap::operator[]` on both sides) plus
  the repeatable crash/no-crash before/after pattern, not by a stack trace pointing at the exact
  corrupting line the way every other candidate this session has. A future session with more time
  could try to get a cleaner repro (e.g. ThreadSanitizer instead of ASan, which is designed to
  catch the *race* itself rather than wait for the resulting corruption to crash) to pin down the
  exact interleaving.
- Did not check whether `InputMap` (a separate singleton, referenced via
  `InputMap::get_singleton()->has_action(p_action)` in every one of these functions) has its own,
  similar lock gaps - only `Input`'s own `action_states` map was audited this session.
- Did not determine whether `_update_action_cache()` (called from `action_press`/`action_release`,
  and presumably from the real input pipeline too) has any locking requirements of its own beyond
  what the now-added `_THREAD_SAFE_METHOD_` already provides via the shared recursive mutex - not
  audited independently, but no new issue was observed in testing.
