# `AnimationLibrary::animations` - plain unsynchronized `RBMap`, no guards at all in this class

Fresh discovery (same heuristic as candidates 063/064/065). `AnimationLibrary` has **zero** existing
thread guards (`grep -c THREAD_GUARD scene/resources/animation_library.{h,cpp}` = 0) and stores its
animations in a plain `RBMap<StringName, Ref<Animation>, StringName::AlphCompare> animations` - a
red-black tree map rather than a hash table or vector, the first candidate this session to confirm
the same unsynchronized-container shape applies to `RBMap` too, not just `HashMap`/`Vector`/
`LocalVector`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/resources/animation_library.h`/`.cpp` - `animations` is mutated by `add_animation()`/`remove_animation()`/`rename_animation()`/`_set_data()`, read by `has_animation()`/`get_animation()`/`get_animation_list()`/`get_animation_list_size()`/`_get_animation_list()`/`_get_data()` - all with zero synchronization | Static read | Identified the exact shape and confirmed every method is `ClassDB`-bound (or called by one) |
| GDScript-level repro (`repro/`: one `AnimationLibrary` with one stable animation (`"stable"`) added at setup, 3 writer threads repeatedly `add_animation()`-ing then `remove_animation()`-ing a *different*, freshly-random key each iteration, 3 reader threads repeatedly calling `get_animation("stable")`/`has_animation("stable")`/`get_animation_list_size()`), against the existing unpatched ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb, reused directly, no rebuild needed for the unfixed run) in an isolated git worktree (`.worktrees/066`) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 15 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on 3/3 attempts** - a clean ASan report (`heap-use-after-free core/templates/rb_map.h:293 in RBMap::_insert()`), a `FATAL: Condition "!e" is true` internal-consistency abort (the tree's own node-pointer invariants violated by concurrent mutation), and a plain segfault (exit 139, no ASan report captured before the process died) |
| Fix: added `_THREAD_SAFE_CLASS_` to `AnimationLibrary`, wrapped every method that reads or mutates `animations` (`add_animation`, `remove_animation`, `rename_animation`, `has_animation`, `get_animation`, `get_animation_list`, `get_animation_list_size`, `_set_data`, `_get_data`) in `_THREAD_SAFE_METHOD_` - rebuilt, reran | Same build/run, 10 attempts | **Clean 10/10** |
| Project test suite with the fix applied | `XDG_DATA_HOME=<tmp> ./bin/godot....san --headless --test` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - identical to the established baseline, no new failures |

## Status

**Confirmed.** 3/3 unfixed crashes (3 distinct manifestations of the same underlying race), 10/10
clean with the fix.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `AnimationLibrary`'s public GDScript API (`add_animation`/
`remove_animation`/`get_animation`/`has_animation`/`get_animation_list_size`).

## Class

`memory-safety` (a plain unsynchronized `RBMap` (red-black tree) race - concurrent `_insert()`/
`erase()` calls mutating the tree's internal node pointers/rebalancing state at the same time,
corrupting it; the first candidate this session to confirm this same bug shape extends to `RBMap`,
not just `HashMap`/`Vector`/`LocalVector`).

## Root cause

`AnimationLibrary::add_animation()`/`remove_animation()`/`rename_animation()` (`scene/resources/
animation_library.cpp`) mutate `animations` (an `RBMap`) with zero lock - each does a
check-then-act sequence (`has_animation()` then `insert()`/`erase()`) that is itself racy even
ignoring the underlying tree-structure corruption. `get_animation()`/`has_animation()`/
`get_animation_list_size()` read it, also with zero lock. Two threads racing `add_animation()`/
`remove_animation()` on different keys corrupt the tree's internal rebalancing structure directly
(confirmed by ASan inside `RBMap::_insert()`), and a reader's concurrent traversal/lookup during
that corruption produces either a `FATAL` internal-consistency abort or a raw segfault depending on
exactly which node pointer gets corrupted first.

## Introduced by

Not independently dated - this class has never had any thread-safety pass applied to it (0
pre-existing guard macros), so the race has existed since `animations` was introduced.

## The fix

`+22/-0` in `scene/resources/animation_library.cpp`, `+3/-0` in
`scene/resources/animation_library.h` (see `patch.diff`):

- Added `#include "core/os/thread_safe.h"` and `_THREAD_SAFE_CLASS_` to `AnimationLibrary`.
- `_THREAD_SAFE_METHOD_` as the first statement in `add_animation()`, `remove_animation()`,
  `rename_animation()`, `has_animation()`, `get_animation()`, `get_animation_list()`,
  `get_animation_list_size()`, `_set_data()`, and `_get_data()` - every method that reads or
  mutates `animations`.

Deliberately **not** independently locked: `_get_animation_list()` (calls the now-locked
`get_animation_list()`, covered transitively via the recursive `Mutex`) and `get_argument_options()`
(editor-only tooling, calls the now-locked `get_animation_list()`, also covered transitively).

Verified: 10/10 clean runs of `repro/` with the fix applied (vs. 3/3 crashing unfixed, 3 distinct
manifestations). Test suite: `1431 | 1430 passed | 1 failed | 3 skipped` with the fix applied -
identical to the established baseline.

## What's not done / open questions

- `AnimationMixer` is declared as a `friend class` of `AnimationLibrary` "for faster access" - worth
  checking whether `AnimationMixer` reaches into `animations` directly (bypassing the now-locked
  public accessors) anywhere, which would reintroduce the same race through a different, friend-only
  path. Not checked this session.
- Found via fresh discovery (SKILL.md's normal process). Same heuristic as candidates 063/064/065,
  now confirmed to generalize across `LocalVector`/`HashMap`/`RBMap` alike - worth continuing to
  apply to more classes for the rest of the budget.
