# `LabelSettings::stacked_outline_data`/`stacked_shadow_data` - two plain unsynchronized `Vector`s, no existing thread guards at all in this class

Fresh discovery (same heuristic as candidates 063-071: a `grep` for `scene/resources/*.h` files with
a `Vector`/`HashMap`/`LocalVector` field and zero `Mutex`/`_THREAD_SAFE_CLASS_`). `LabelSettings` has
two independent `CowData`-backed `Vector<T>` fields - `stacked_outline_data` and
`stacked_shadow_data` - each with its own full `count`/`add`/`move`/`remove`/per-field `set`/`get`
family (11 methods per field, 22 total), all equally unguarded.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/resources/label_settings.h`/`.cpp` - confirmed zero `Mutex`/`_THREAD_SAFE_CLASS_` anywhere in the class, and that `add_stacked_outline()`/`remove_stacked_outline()`/`move_stacked_outline()`/`set_stacked_outline_count()` mutate `stacked_outline_data` (`.insert()`/`.remove_at()`/`.resize()`, each able to reallocate the `CowData` buffer) with zero lock, while `set_stacked_outline_size()`/`get_stacked_outline_size()`/`set_stacked_outline_color()`/`get_stacked_outline_color()` index into it (`.write[p_index]`/`[p_index]`) with zero lock - the identical shape duplicated for `stacked_shadow_data` | Static read | Identified the exact shape, duplicated across two independent fields in the same class |
| GDScript-level repro (`repro/`: a `LabelSettings` pre-seeded with 5 outline + 5 shadow entries, 3 writer threads each looping `add_stacked_outline()`/`remove_stacked_outline(0)`/`add_stacked_shadow()`/`remove_stacked_shadow(0)`, 3 reader threads looping `get_stacked_outline_count()` then `set_stacked_outline_size()`/`get_stacked_outline_color()` over every index, same for shadow, 3 seconds), against the unpatched ASan editor binary | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 15 ./bin/godot...san --headless --path repro` | **3/3 crashed** - `unknown-crash core/templates/cowdata.h:185 in CowData<LabelSettings::StackedOutlineData>::size()`, the same for `StackedShadowData`, and a `heap-buffer-overflow` in `memmove` |
| Fix: added `_THREAD_SAFE_CLASS_` to `LabelSettings`, used the same scripted regex-insertion approach as candidates 030/061/067 to add `_THREAD_SAFE_METHOD_` as the first statement in all 22 methods touching either vector, rebuilt, reran | Same build/run, 10 attempts | **Clean 10/10** - every run printed `"survived"` |
| Project test suite with the fix applied | `XDG_DATA_HOME=<tmp> ./bin/godot....san --headless --test` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - identical to the established baseline, no new failures |

## Status

**Confirmed.** 3/3 unfixed crashes (three distinct ASan signatures across the two vectors), 10/10
clean runs with the fix.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `LabelSettings`'s public GDScript API (`add_stacked_outline`/
`remove_stacked_outline`/`set_stacked_outline_size`/`get_stacked_outline_color`/the shadow
equivalents), with no test-only instrumentation.

## Class

`memory-safety` - two independent, identically-shaped `CowData`-backed `Vector<T>` races in the same
class (same general shape as candidates 063/072's siblings: a writer that can reallocate the backing
buffer via `insert`/`remove_at`/`resize` racing readers that index by raw position).

## Root cause

`LabelSettings` has zero pre-existing thread-safety infrastructure (`grep -c THREAD_SAFE
scene/resources/label_settings.h` = 0 before this fix). Both `stacked_outline_data` and
`stacked_shadow_data` are plain `Vector<T>` fields (`CowData`-backed, so `insert()`/`remove_at()`/
`resize()` can detach/reallocate the shared buffer) mutated and read by 11 methods each, none of
which take any lock - a textbook instance of this session's most common bug shape, just duplicated
twice in the same class because the two "stacked" features (outlines and shadows) were implemented
as parallel, independently-unguarded arrays.

## Introduced by

Not independently dated - this class has never had any thread-safety pass applied to it (0
pre-existing guard macros), so the race has existed since the stacked-outline/stacked-shadow feature
was added to `LabelSettings` (a relatively recent engine feature).

## The fix

`+22/-0` in `scene/resources/label_settings.cpp`, `+2/-0` in `scene/resources/label_settings.h` (see
`patch.diff`):

- Added `#include "core/os/thread_safe.h"` and `_THREAD_SAFE_CLASS_` to `LabelSettings`.
- `_THREAD_SAFE_METHOD_` as the first statement in all 22 methods that touch either vector:
  `get_stacked_outline_data`, `get_stacked_outline_count`, `set_stacked_outline_count`,
  `add_stacked_outline`, `move_stacked_outline`, `remove_stacked_outline`,
  `set_stacked_outline_size`, `get_stacked_outline_size`, `set_stacked_outline_color`,
  `get_stacked_outline_color`, and the 12 shadow equivalents (`get_stacked_shadow_data` through
  `get_stacked_shadow_outline_size`).

A single class-wide mutex is used (not one per vector) since `_THREAD_SAFE_CLASS_` only provides one
- acceptable here since the two vectors are never both touched by the same method, so there's no
extra contention beyond what a single `LabelSettings` instance would already serialize through its
own API surface.

Verified: 10/10 clean runs of `repro/` with the fix applied (vs. 3/3 crashing unfixed). Test suite:
`1431 | 1430 passed | 1 failed | 3 skipped` with the fix applied - identical to the established
baseline.

## What's not done / open questions

- `_set`/`_get`/`_get_property_list`/`_property_can_revert`/`_property_get_revert` (the
  `PropertyListHelper`-based serialization path used for the `stacked_outline_0/size` etc. property
  naming convention) internally call the now-locked public accessors - not independently locked,
  covered transitively by the recursive `Mutex`.
- Found via fresh discovery (SKILL.md's normal process), continuing the same
  "zero `_THREAD_SAFE_CLASS_`, container-backed `Resource`, public get/set pair" heuristic that has
  now confirmed 10 distinct classes in a row (063-072, excluding 070's `AudioServer` which used a
  different locking convention entirely) - this class in particular demonstrates the pattern can
  recur *twice* within a single class when a feature is duplicated as parallel arrays.
