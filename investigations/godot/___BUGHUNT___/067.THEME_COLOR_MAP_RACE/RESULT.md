# `Theme::color_map` - plain unsynchronized nested `HashMap`, no guards at all in this class

Fresh discovery (same heuristic as candidates 063-066). `Theme` has **zero** existing thread guards
(`grep -c THREAD_GUARD scene/resources/theme.{h,cpp}` = 0) and stores its per-type color overrides in
a doubly-nested `HashMap<StringName, HashMap<StringName, Color>> color_map` (outer key: theme type,
inner key: item name). `Theme` is one of the most widely-touched classes in the engine (every
`Control` reads from one), making this a particularly high-value find, though the fix in this
candidate is deliberately scoped to just the `color_map` family - see "what's not done" for why.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/resources/theme.h`/`.cpp` - `Theme` has 7 structurally-identical map families (icon/stylebox/font/font_size/color/constant/sound), each following the same `set_X`/`get_X`/`has_X`/`has_X_nocheck`/`rename_X`/`clear_X`/`get_X_list`/`add_X_type`/`remove_X_type`/`rename_X_type`/`get_X_type_list` pattern (~68 methods total touching some map). Picked `color_map` as a representative, fully-confirmed family rather than attempting all 7 in one pass | Static read | Confirmed the exact shape and that every `color_map`-touching method is `ClassDB`-bound (or called by one) |
| GDScript-level repro (`repro/`: one `Theme` with one stable color (`"stable_name"` in type `"StableType"`) set at setup, 3 writer threads repeatedly `set_color()`-ing then `clear_color()`-ing a *different*, freshly-random theme-type name each iteration (to keep forcing the *outer* `HashMap<StringName, ThemeColorMap>` to grow/rehash/shrink), 3 reader threads repeatedly calling `get_color("stable_name", "StableType")`/`has_color(...)`), against the existing unpatched ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb, reused directly, no rebuild needed for the unfixed run) in an isolated git worktree (`.worktrees/067`) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 15 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on 3/3 attempts** - two clean ASan reports (`heap-buffer-overflow` inside the *outer* `HashMap`'s own `_insert_element()`/`_insert_empty_at()` - confirming the corruption lands on the outer theme-type table, exactly as the repro was designed to stress), and one `Aborted` (exit 134, a `FATAL`-style internal consistency abort) |
| Fix: added `_THREAD_SAFE_CLASS_` to `Theme`, wrapped every method in the `color_map` family (`set_color`, `get_color`, `has_color`, `has_color_nocheck`, `rename_color`, `clear_color`, `get_color_list`, `add_color_type`, `remove_color_type`, `rename_color_type`, `get_color_type_list`) in `_THREAD_SAFE_METHOD_` - rebuilt, reran | Same build/run, 10 attempts | **Clean 10/10** |
| Project test suite with the fix applied | `XDG_DATA_HOME=<tmp> ./bin/godot....san --headless --test` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - identical to the established baseline, no new failures |

## Status

**Confirmed.** 3/3 unfixed crashes (2 clean ASan reports pinpointing the outer table, 1 `FATAL`-style
abort), 10/10 clean with the fix.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `Theme`'s public GDScript API (`set_color`/`clear_color`/`get_color`/
`has_color`).

## Class

`memory-safety` (a plain unsynchronized nested `HashMap` race - concurrent `_insert()`/`erase()`
calls on the *outer* table mutating its internal bucket/element bookkeeping while a reader thread
looks up an unrelated, stable outer key at the same time; this is the second candidate this session
to confirm a nested-container shape, after `SpriteFrames`/064's `HashMap`-of-`Vector`, this time
`HashMap`-of-`HashMap`).

## Root cause

`Theme::set_color()`/`clear_color()`/`add_color_type()`/`remove_color_type()`/`rename_color_type()`
(`scene/resources/theme.cpp`) mutate the *outer* `color_map` (and the inner per-type
`ThemeColorMap`) with zero lock. `get_color()`/`has_color()`/`get_color_list()`/
`get_color_type_list()` read it, also with zero lock. A writer thread repeatedly adding/removing
*different* theme-type keys forces the outer `HashMap`'s internal table to repeatedly grow/rehash/
shrink; a reader thread concurrently looking up an unrelated, stable theme-type key races that
rehash, landing inside the outer table's own `_insert_element()`/`_insert_empty_at()` internals,
exactly as ASan caught.

## Introduced by

Not independently dated - this class has never had any thread-safety pass applied to it (0
pre-existing guard macros), so the race has existed since `color_map` was introduced.

## The fix

`+30/-0` in `scene/resources/theme.cpp`, `+3/-0` in `scene/resources/theme.h` (see `patch.diff`):

- Added `#include "core/os/thread_safe.h"` and `_THREAD_SAFE_CLASS_` to `Theme` (a single mutex
  shared by every map family, not just `color_map` - cheap insurance for whichever family gets
  fixed next, see below).
- `_THREAD_SAFE_METHOD_` as the first statement in `set_color()`, `get_color()`, `has_color()`,
  `has_color_nocheck()`, `rename_color()`, `clear_color()`, `get_color_list()`, `add_color_type()`,
  `remove_color_type()`, `rename_color_type()`, and `get_color_type_list()`.

Deliberately **not** locked in this pass: `_get_color_list()`/`_get_color_type_list()` (internal
`ClassDB`-bound wrappers that call the now-locked `get_color_list()`/`get_color_type_list()`,
covered transitively via the recursive `Mutex`).

Verified: 10/10 clean runs of `repro/` with the fix applied (vs. 3/3 crashing unfixed). Test suite:
`1431 | 1430 passed | 1 failed | 3 skipped` with the fix applied - identical to the established
baseline.

## What's not done / open questions

- **This fix covers only the `color_map` family.** `Theme` has 6 other structurally-identical
  families - `icon_map`, `style_map`, `font_map`, `font_size_map`, `constant_map`, `sound_map` -
  each with the same `set`/`get`/`has`/`has_nocheck`/`rename`/`clear`/`get_list`/`add_type`/
  `remove_type`/`rename_type`/`get_type_list` method group (roughly 68 methods total touch some map,
  per a static count during investigation), all equally unguarded and almost certainly equally
  exploitable by the identical repro shape, just substituting the relevant setter/getter pair. This
  is a strong candidate for the same scripted-sweep treatment candidate 030 (`PopupMenu`) and
  candidate 061 (`Image`) used - not attempted here given the size of the task and the ~20-minute
  rebuild cost every iteration (theme.h is one of the most widely-included headers in the engine).
  The `_THREAD_SAFE_CLASS_` mutex this candidate added is already in place and ready for that future
  sweep to reuse.
- `variation_map`/`variation_base_map` (theme-type-variation bookkeeping, also unguarded) and
  `set_theme_item()`/`get_theme_item()`/`has_theme_item()` (a generic dispatcher that routes to the
  per-type methods by `DataType` enum) were not audited.
- Found via fresh discovery (SKILL.md's normal process). Same heuristic as candidates 063-066 -
  worth continuing to apply to more classes for the rest of the budget, though `Theme`'s own
  remaining 6 map families are themselves a complete, self-contained follow-up task.
