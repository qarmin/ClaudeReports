# `BoneMap::bone_map` - plain unsynchronized `HashMap<StringName, StringName>`, hangs rather than crashes

Fresh discovery (same heuristic as candidates 063-068). `BoneMap` has **zero** existing thread
guards (`grep -c THREAD_GUARD scene/resources/bone_map.{h,cpp}` = 0) and stores its profile-bone-to-
skeleton-bone mapping in a `HashMap<StringName, StringName> bone_map`. The confirmed symptom here is
notably different from every other candidate this session except 054: the unfixed repro doesn't
crash at all in repeated attempts - it **hangs** deterministically, the same StringName-intern-table
corruption-driven engine-wide freeze shape first documented in candidate 054 (`Node::get_name()`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/resources/bone_map.h`/`.cpp` - `bone_map` is mutated by `_set_skeleton_bone_name()`/`_validate_bone_map()` (called from `set_profile()`/the profile's `profile_updated` signal), read by `get_skeleton_bone_name()`/`find_profile_bone_name()`/`get_skeleton_bone_name_count()` - all with zero synchronization. Values are `StringName`s - the same refcounted-handle type whose `operator=`'s `unref()`-then-reassign sequence candidate 054 already proved races into a heap-use-after-free (or, as this candidate found, a corrupted-intern-table hang) | Static read | Identified the exact shape and confirmed every method is `ClassDB`-bound (or called by one) |
| GDScript-level repro (`repro/`: a `SkeletonProfile` with one bone (`"root"`) assigned to a fresh `BoneMap` at setup, 3 writer threads repeatedly `set_skeleton_bone_name("root", <fresh random StringName>)`, 3 reader threads repeatedly `get_skeleton_bone_name("root")`/`find_profile_bone_name("root")`), against the existing unpatched ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb, reused directly, no rebuild needed for the unfixed run) in an isolated git worktree (`.worktrees/069`). First repro attempt (property-style `prof.bone_size = 1`) silently failed to even start `_ready()` and was replaced with explicit `set_bone_size()`/`set_bone_name()` calls, which worked | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 10 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Hung on 3/3 attempts** - the process printed `"threads started"` and then produced no further output, never reaching `"survived"`, until `timeout` force-killed it. No ASan report, no crash of any kind - a deterministic freeze, same mechanism as candidate 054 (concurrent `StringName::operator=` calls corrupting the global intern table's internal linked-list bookkeeping, causing some subsequent traversal to loop forever while holding the table's own mutex, which then blocks every other thread's own `StringName` operations engine-wide) |
| Fix: added `_THREAD_SAFE_CLASS_` to `BoneMap`, wrapped every method that reads or mutates `bone_map` (`get_skeleton_bone_name`, `_set_skeleton_bone_name`, `find_profile_bone_name`, `get_skeleton_bone_name_count`, `_validate_bone_map`) in `_THREAD_SAFE_METHOD_` - rebuilt, reran | Same build/run, 10 attempts | **Clean 10/10** - every run printed `"survived"` well within the timeout |
| Project test suite with the fix applied | `XDG_DATA_HOME=<tmp> ./bin/godot....san --headless --test` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - identical to the established baseline, no new failures |

## Status

**Confirmed.** 3/3 unfixed hangs (not crashes - a deterministic freeze, consistent across every
attempt), 10/10 clean runs with the fix, each completing well within the same timeout that killed
every unfixed attempt.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `BoneMap`'s public GDScript API (`set_skeleton_bone_name`/
`get_skeleton_bone_name`/`find_profile_bone_name`).

## Class

`memory-safety` (a plain unsynchronized `HashMap<StringName, StringName>` race where the actual
corruption lands on `StringName`'s own global intern table rather than the `HashMap` itself -
identical underlying mechanism to candidate 054, just reached through a different class entirely).

## Root cause

Identical mechanism to candidate 054 - see that candidate's `RESULT.md` for the full
`StringName::operator=`/`unref()` analysis. The new information here: `BoneMap::bone_map`'s value
type is `StringName`, and `_set_skeleton_bone_name()`'s `bone_map.insert(p_profile_bone_name,
p_skeleton_bone_name)` performs an in-place `StringName` assignment on an *existing* map entry's
value (the key count doesn't change, since `set_skeleton_bone_name()` only ever updates a key that's
already present) - meaning the `HashMap`'s own bucket/rehash machinery isn't what's racing here, it's
purely the `StringName` value's own refcount/intern-table bookkeeping, racing exactly as in candidate
054, just via a `HashMap`'s stored value instead of a `Node`'s own field.

## Introduced by

Not independently dated - this class has never had any thread-safety pass applied to it (0
pre-existing guard macros), so the race has existed since `bone_map` was introduced.

## The fix

`+16/-0` in `scene/resources/bone_map.cpp`, `+3/-1` in `scene/resources/bone_map.h` (see
`patch.diff`):

- Added `#include "core/os/thread_safe.h"` and `_THREAD_SAFE_CLASS_` to `BoneMap`.
- `_THREAD_SAFE_METHOD_` as the first statement in `get_skeleton_bone_name()`,
  `_set_skeleton_bone_name()`, `find_profile_bone_name()`, `get_skeleton_bone_name_count()`, and
  `_validate_bone_map()` (the last one not directly `ClassDB`-bound, but mutates `bone_map` whenever
  `set_profile()` or the profile's own `profile_updated` signal fires, so it needed the same
  protection).

Deliberately **not** independently locked: `set_skeleton_bone_name()` itself (a thin wrapper calling
the now-locked `_set_skeleton_bone_name()` then emitting a signal, covered transitively via the
recursive `Mutex`), and `_update_profile()` (calls the now-locked `_validate_bone_map()`, also
covered transitively).

Verified: 10/10 clean runs of `repro/` with the fix applied (vs. 3/3 hanging unfixed). Test suite:
`1431 | 1430 passed | 1 failed | 3 skipped` with the fix applied - identical to the established
baseline.

## What's not done / open questions

- This is the second candidate this session (after 054) to manifest as a hang rather than a crash -
  worth remembering as a distinct symptom class when sweeping for `StringName`-valued `HashMap`/
  `Vector`/plain-field races: "no ASan report across N runs" does not mean "safe," it may mean
  "wrong test duration" (a hang needs `timeout` to even notice, and a short enough timeout to tell
  it apart from the test just being slow).
- `_get`/`_set`/`_get_property_list` (internal serialization helpers using the `"bone_map/<key>"`
  property path convention) call the now-locked public accessors internally - not independently
  locked, covered transitively.
- Found via fresh discovery (SKILL.md's normal process), the seventh candidate this session using
  the same "zero `_THREAD_SAFE_CLASS_`, container-backed `Resource`, public get/set pair" heuristic.
