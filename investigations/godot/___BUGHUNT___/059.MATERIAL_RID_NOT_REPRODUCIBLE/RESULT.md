# `material.cpp`'s `ShaderMaterial`/`BaseMaterial3D` RID race - re-tested, now empirically confirmed not reproducible

From IDEAS.md's oldest-standing open backlog row (flagged since the project's earliest sessions,
"still never empirically tested"), explicitly listed by the coordinator as a priority-3 item to
revisit with a real repro rather than the paper-only re-analysis done near the end of the first
360-minute session.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/resources/material.cpp` - `ShaderMaterial::_check_material_rid()`/`BaseMaterial3D::_check_material_rid()` both lock `material_rid_mutex` around their entire lazy-create-and-populate body, gated by `_get_material().is_null()`. Traced every call site that mutates `material` (`_set_material()`) - **exactly 2**, both inside the locked `_check_material_rid()` bodies, both only reached when `material` is currently null. `material` is **never reset to a null/invalid RID anywhere else** in this file (no `_set_material(RID())` call exists outside those 2 sites) | Static read | Confirmed `material` is effectively **write-once per instance**: the lock correctly prevents the one dangerous operation (two threads both calling `RS::material_create_from_shader()` for the same instance), and every unguarded read site (`get_rid()` returning `_get_material()` right after an unlocked call to the now-already-completed `_check_material_rid()`; `_update_shader()`'s `if (_get_material().is_valid())` branches) either reads a value that's already fully-initialized by the time any thread can observe it (the lock guarantees this), or safely no-ops on a still-null value (skipping an update that will be superseded once the lazy-create eventually runs) |
| GDScript-level repro (`repro/`: ~7,800 trials per run, each trial creating a **fresh** `ShaderMaterial` + `Shader` and racing 4 threads all calling `.get_rid()` and `.render_priority = 1` simultaneously on it - deliberately using a fresh instance every trial to repeatedly exercise the *first-ever-access* creation race window, since a long-lived single instance only has that window once), against the existing unpatched ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb, reused directly, no rebuild needed) in an isolated git worktree (`.worktrees/059`) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 20 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **0/3 runs crashed** - ~7,800 fresh-instance creation races per run (≈23,400 total trials, ≈93,600 individual racing threads), all clean |

## Status

**Not reproducible.** Re-confirms and *strengthens* this session's earlier paper-only re-analysis
with an actual empirical test: ~23,400 trials of the specific "multiple threads racing a brand-new
instance's first `get_rid()` call" scenario, 0 crashes.

## Why it looks safe (and why it isn't just "didn't get lucky")

Unlike most of this session's confirmed candidates, this isn't a case of "didn't trigger it yet" -
the code structure itself rules out the obvious race mechanism: `material_rid_mutex` is taken for
the *entire* lazy-create-and-populate sequence, and `material` has no reset path once set. The only
way this could still be unsafe is a torn (non-atomic) read of the `RID` struct itself on some
platform/ABI where `RID`'s internal representation isn't naturally atomic to read/write - not
something a userspace GDScript repro on x86-64 can meaningfully probe for, and not the kind of thing
ASan's instrumentation would catch as a heap-safety violation even if it did occur (a torn read of a
stack/member value isn't a memory-safety bug in ASan's sense, just a potential logic bug - and even
that would require `RID`'s size to exceed a single atomic-width load, which it doesn't on any
mainstream 64-bit target).

## What's not done / open questions

- Did not test `_update_shader()`'s unguarded `_get_material().is_valid()` branch under a scenario
  where `material` transitions from valid to invalid mid-flight, because no such transition exists
  in this codebase today (confirmed by the exhaustive `_set_material()` call-site search above) -
  if a future refactor ever adds a "clear/reset material" path, this reasoning would need
  re-verification.
- This closes out `material.cpp`'s RID race as a backlog item - it has now been tested twice
  (paper-only in the first session, empirically in this one) and both times judged safe. Not worth
  further time barring a specific new lead (e.g., an actual bug report or a code change that
  introduces a `material` reset path).
- Next per the coordinator's priority-3 list: `TextParagraph`'s 13 remaining unlocked methods (038
  follow-up, low priority), then `Image`'s ~95 remaining unlocked methods via a scripted sweep
  (candidate 030's approach), then the `Tree`/`TreeItem` cross-object lock gap, then the ~22
  remaining unaudited `_THREAD_SAFE_CLASS_` files.
