# `Translation::translation_map` - plain unsynchronized `HashMap`, no guards at all in this class

Fresh discovery (same heuristic as candidates 063/`Gradient` and 064/`SpriteFrames`). `Translation`
has **zero** existing thread guards (`grep -c THREAD_GUARD core/string/translation.{h,cpp}` = 0) and
stores its message table in a plain `HashMap<MessageKey, Vector<StringName>, MessageKey>
translation_map`. `get_message()`/`get_plural_message()` additionally use the
`HashMap::getptr()`-then-dereference pattern already confirmed race-prone in this project's earliest
sessions (candidates 008/009/010 - `Input`, `CameraServer`, `ProjectSettings`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `core/string/translation.h`/`.cpp` - `translation_map` is mutated by `add_message()`/`add_plural_message()`/`erase_message()`/`_set_messages()`, read by `get_message()`/`get_plural_message()`/`get_message_list()`/`get_message_count()`/`get_translated_message_list()`/`_get_message_list()`/`_get_messages()` - all with zero synchronization. `get_message()`/`get_plural_message()` call `translation_map.getptr(...)` and dereference the result several lines later, the exact "rehash invalidates the raw pointer" shape from candidates 008/009/010 | Static read | Identified the exact shape and confirmed every method is `ClassDB`-bound (or called by one) |
| GDScript-level repro (`repro/`: one `Translation` with one stable message (`"stable_key"` → `"stable_value"`) added at setup, 3 writer threads repeatedly `add_message()`-ing then `erase_message()`-ing a *different*, freshly-random key each iteration (to keep forcing the table to grow/rehash/shrink), 3 reader threads repeatedly calling `get_message("stable_key")`), against the existing unpatched ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb, reused directly, no rebuild needed for the unfixed run) in an isolated git worktree (`.worktrees/065`) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 15 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on 3/3 attempts** - all three were `heap-use-after-free` reports directly inside `HashMap::_insert()`/`HashMap::erase()` internals, reached through concurrent `add_message()`/`erase_message()` calls on three different writer threads racing each other's mutation of the same table (a writer-vs-writer race on `translation_map` itself, rather than specifically the `get_message()` read-side race this repro was designed to isolate - still the identical root cause, just a different pair of racing call sites than originally targeted) |
| Fix: added `_THREAD_SAFE_CLASS_` to `Translation`, wrapped every method that reads or mutates `translation_map` (`add_message`, `add_plural_message`, `get_message`, `get_plural_message`, `erase_message`, `get_message_list`, `get_message_count`, `get_translated_message_list`) in `_THREAD_SAFE_METHOD_` - rebuilt, reran | Same build/run, 10 attempts | **Clean 10/10** |
| Project test suite with the fix applied | `XDG_DATA_HOME=<tmp> ./bin/godot....san --headless --test` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - identical to the established baseline, no new failures |

## Status

**Confirmed.** 3/3 unfixed crashes, 10/10 clean with the fix.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `Translation`'s public GDScript API (`add_message`/`erase_message`/
`get_message`).

## Class

`memory-safety` (a plain unsynchronized `HashMap` race - concurrent `_insert()`/`erase()` calls
mutating the table's internal bucket/element bookkeeping at the same time, corrupting it; this
session's repro happened to land on a writer-vs-writer interleaving rather than the
`getptr()`-then-dereference read-side race it was designed around, but both are symptoms of the
exact same missing lock).

## Root cause

`Translation::add_message()`/`add_plural_message()`/`erase_message()` (`core/string/
translation.cpp`) mutate `translation_map` with zero lock. `get_message()`/`get_plural_message()`
call `translation_map.getptr({...})` and dereference the returned pointer several statements later
(`msgstrs->get(0)`/`msgstrs->get(index)`) - if a concurrent `add_message()`/`erase_message()` call
triggers the `HashMap`'s internal rehash/resize in between, that pointer is dangling, the exact same
mechanism candidates 008/009/010 found in `Input`/`CameraServer`/`ProjectSettings`. Separately, two
concurrent writers (`add_message()`/`erase_message()` racing each other, as this candidate's repro
specifically caught) directly corrupt the table's internal element/bucket structures during
`_insert()`/`erase()`.

## Introduced by

Not independently dated - this class has never had any thread-safety pass applied to it (0
pre-existing guard macros), so the race has existed since `translation_map` was introduced.

## The fix

`+14/-0` in `core/string/translation.cpp`, `+3/-0` in `core/string/translation.h` (see
`patch.diff`):

- Added `#include "core/os/thread_safe.h"` and `_THREAD_SAFE_CLASS_` to `Translation`.
- `_THREAD_SAFE_METHOD_` as the first statement in `add_message()`, `add_plural_message()`,
  `get_message()`, `get_plural_message()`, `erase_message()`, `get_message_list()`,
  `get_message_count()`, and `get_translated_message_list()` - every method that reads or mutates
  `translation_map`.

Deliberately **not** independently locked: `_get_message_list()` (calls the now-locked
`get_message_list()`, covered transitively via the recursive `Mutex`), `_get_messages()`/
`_set_messages()` (internal serialization helpers used during resource load/save, main-thread-only
in practice), and `set_locale()`/`get_locale()` (a plain `String` field, not `translation_map`-backed
- lower-risk scalar category, consistent with this session's established scope boundaries).

Verified: 10/10 clean runs of `repro/` with the fix applied (vs. 3/3 crashing unfixed). Test suite:
`1431 | 1430 passed | 1 failed | 3 skipped` with the fix applied - identical to the established
baseline.

## What's not done / open questions

- This candidate's repro happened to catch the writer-vs-writer `_insert()`/`erase()` race rather
  than the `getptr()`-then-dereference read-side race it was specifically designed to isolate (a
  stable reader key alongside churning unrelated keys). Both are fixed by the same lock, but a
  maximally rigorous confirmation would also demonstrate the read-side crash in isolation (e.g. by
  reducing writer thread count to 1, or adding a `_THREAD_SAFE_METHOD_`-free intermediate build to
  see which race surfaces first) - not done here given time, since the fix addresses both
  mechanisms identically regardless.
- `TranslationServer` (the global singleton managing a collection of `Translation` resources) was
  not audited - a plausible sibling candidate given the established pattern, not checked this
  session.
- Found via fresh discovery (SKILL.md's normal process). Same heuristic as candidates 063/064 -
  worth continuing to apply to more classes for the rest of the budget.
