# `Compositor::effects` - plain unsynchronized `LocalVector<Ref<CompositorEffect>>`, no existing thread guards at all in this class

Fresh discovery (same heuristic as candidates 063-069: a `grep` for `scene/resources/*.h` files
with a `Vector`/`HashMap`/`LocalVector` field and zero `Mutex`/`_THREAD_SAFE_CLASS_`). `Compositor`
is small (260 lines, 16 `ClassDB`-bound methods total) and its only container field, `effects`, is
touched by exactly two methods - `set_compositor_effects()`/`get_compositor_effects()` - making it
one of the cleanest, smallest-blast-radius candidates found this session.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/resources/compositor.h`/`.cpp` - `effects` is a `LocalVector<Ref<CompositorEffect>>` (not `CowData`-backed like `Vector`, but still a plain heap-allocated buffer with no internal synchronization), mutated by `set_compositor_effects()`'s `effects.clear()` + per-element `push_back()` loop, read by `get_compositor_effects()`'s `effects[i]` iteration - zero locks anywhere in either method | Static read | Confirmed the exact shape: a `clear()`-then-rebuild writer racing a plain-indexing reader |
| GDScript-level repro v1 (`comp = Compositor.new()`, 3 writer threads each looping `CompositorEffect.new()` ×5 into a fresh array then `comp.compositor_effects = arr`, 3 reader threads looping `comp.compositor_effects` and touching each returned element), against the unpatched ASan editor binary | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 15 ./bin/godot...san --headless --path repro` | **Crashed 2/3 with a clean ASan report** (`heap-use-after-free core/object/object.h:604 in Object::_has_ancestry()`), **1/3 a plain unsymbolized SIGSEGV** - real memory corruption, confirmed |
| Fix: added `_THREAD_SAFE_CLASS_` to `Compositor`, `_THREAD_SAFE_METHOD_` as the first statement in both `set_compositor_effects()`/`get_compositor_effects()`. Rebuilt, reran the same repro 10 times | Same build/run | **10/10 hung** (`timeout`-killed, exit 124) - investigated and found to be a **repro design artifact, not a new bug**: each writer-thread loop iteration created 5 fresh `CompositorEffect.new()` instances, each eagerly allocating a `RenderingServer` RID in its constructor. Once the race no longer crashed the process within milliseconds, the loop ran for the full 3 seconds at a high allocation rate and exhausted the thread-safe RID pool's hard `chunk_limit` (`core/templates/rid_owner.h:116`), spamming `"ERROR: Element limit reached."` forever and never reaching `"survived"` |
| Repro redesigned: pre-allocate a fixed pool of 5 `CompositorEffect` instances once in `_ready()`; writer threads re-assign the *same* instances into a fresh `Array` each iteration (no new RID allocation per loop, isolating the `effects` vector race from RID-pool exhaustion) | Same unfixed binary, 3 runs | **3/3 crashed** again, confirming the redesigned repro still exercises the same bug |
| Same redesigned repro against the fix, 10 runs | Same build/run | **Clean 10/10** - every run printed `"survived"` |
| Project test suite with the fix applied | `XDG_DATA_HOME=<tmp> ./bin/godot....san --headless --test` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - identical to the established baseline, no new failures |

## Status

**Confirmed.** 3/3 unfixed crashes (both the original and the redesigned repro), 10/10 clean runs
with the fix (once the repro itself was corrected to not confound the race with an unrelated
RID-pool exhaustion artifact).

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `Compositor`'s public GDScript API (`compositor_effects` property,
i.e. `set_compositor_effects()`/`get_compositor_effects()`), with no test-only instrumentation.

## Class

`memory-safety` (a plain unsynchronized `LocalVector<Ref<CompositorEffect>>` race - same general
shape as candidates 063-069's confirmed `Vector`/`HashMap`/`RBMap` fixes, just on a `LocalVector`
instead of a `CowData`-backed container, which made no practical difference to the race itself).

## Root cause

`Compositor` has zero pre-existing thread-safety infrastructure (`grep -c THREAD_SAFE
scene/resources/compositor.h` = 0). `set_compositor_effects()` calls `effects.clear()` (freeing the
vector's internal buffer, since `LocalVector::clear()` can shrink/deallocate) and then rebuilds it
via repeated `push_back()` calls (which can reallocate the buffer as it grows) with zero lock, while
`get_compositor_effects()` concurrently reads `effects[i]` by raw index with zero lock - a classic
writer-reallocates/reader-indexes-into-freed-memory race, identical in shape to every other plain
unsynchronized container race this session (just reached through `Compositor` specifically, which
had not been audited before).

## Introduced by

Not independently dated - this class has never had any thread-safety pass applied to it (0
pre-existing guard macros), so the race has existed since `Compositor::effects` was introduced
(it is a relatively new engine feature, the 4.3-era compositor-effects system).

## The fix

`+2/-0` in `scene/resources/compositor.cpp`, `+2/-0` in `scene/resources/compositor.h` (see
`patch.diff`) - the smallest fix of any candidate this session:

- Added `#include "core/os/thread_safe.h"` and `_THREAD_SAFE_CLASS_` to `Compositor`.
- `_THREAD_SAFE_METHOD_` as the first statement in `set_compositor_effects()` and
  `get_compositor_effects()` - the only two methods that touch `effects`.

Verified: 10/10 clean runs of `repro/` with the fix applied (vs. 3/3 crashing unfixed on both the
original and the redesigned repro). Test suite: `1431 | 1430 passed | 1 failed | 3 skipped` with the
fix applied - identical to the established baseline.

## What's not done / open questions

- `CompositorEffect` itself (the element type stored in `effects`) was not audited - its own fields
  (`enabled`, `effect_callback_type`, `access_resolved_color`, etc.) are plain scalars with their own
  setters/getters, each independently forwarding to `RenderingServer` calls gated by
  `rid.is_valid()`. Not part of this candidate's scope (a different class, a different container
  shape - no `Vector`/`HashMap` field at all, same "write-once RID" shape already investigated and
  ruled out safe-by-construction in candidate 059's `material.cpp` work).
- **Lesson worth recording for future repros in this bug-hunt session**: when a "fixed" build's
  verification run hangs instead of crashing or surviving, don't assume it's the same
  `StringName`-intern-table-corruption hang shape as candidates 054/069 - check whether the repro
  itself is doing something resource-exhausting (here, allocating a fresh `RenderingServer` RID per
  loop iteration against a pool with a hard element-count limit) that only becomes visible once the
  race stops crashing the process early. A repro that creates fresh engine objects in a tight loop
  should prefer reusing a small, fixed pool of pre-allocated objects unless churn itself is the thing
  being tested.
- Found via fresh discovery (SKILL.md's normal process), continuing the same
  "zero `_THREAD_SAFE_CLASS_`, container-backed `Resource`, public get/set pair" heuristic that has
  now confirmed 9 distinct classes in a row (063-071, excluding 070's `AudioServer` which used a
  different locking convention entirely).
