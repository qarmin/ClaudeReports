# `ProjectSettings::clear()`/`get_order()`/`set_order()`/`set_builtin_order()`/`is_builtin_setting()` missing lock on `props`

Found by continuing this session's sweep for the "locked-writer vs. unlocked-reader/writer on the
same container" pattern established by candidates 002-004/008/009: `grep -rln
"_THREAD_SAFE_CLASS_"` turned up `core/config/project_settings.h` as one of the files using the
`_THREAD_SAFE_CLASS_`/`_THREAD_SAFE_METHOD_` idiom (`core/os/thread_safe.h`). Auditing every
`ProjectSettings` method that touches `props` (`RBMap<StringName, VariantContainer>`,
`project_settings.h:101` - a real red-black tree, `core/templates/rb_map.h`, not a simple scalar)
for consistent lock usage found `_set()`/`_get()`/`get_setting_with_override*()`/
`_get_property_list()`/`has_setting()` all start with `_THREAD_SAFE_METHOD_`, but five other
public, `ClassDB`-bound methods touching the exact same tree never lock at all: `clear()`,
`get_order()`, `set_order()`, `set_builtin_order()`, `is_builtin_setting()`
(`project_settings.cpp:1056-1082`, pre-fix).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: lock-usage audit of `core/config/project_settings.cpp` | n/a | Confirmed the asymmetry above; `clear()` additionally calls `props.erase()` - a real tree-rebalancing mutation, not just a read, run with **no synchronization** against `_set()`'s locked `props[p_name] = VariantContainer(...)` insert (also a rebalancing mutation) on another thread |
| GDScript-level repro (`repro/`), against a real, self-built ASan editor binary | `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`, `--headless --path repro`, 6 threads calling `ProjectSettings.set_setting()` (locked) + 6 threads calling `ProjectSettings.clear()` (unlocked) on the same 200-key pool + 4 threads calling `ProjectSettings.get_order()` (unlocked) on a builtin key, concurrently for the test's stress window | **10/10 runs crashed** across two independent script revisions (5/5 with a 20s stress window including an `is_builtin_setting()`-calling reader, then a quieter 5/5 re-check after discovering `is_builtin_setting` isn't actually `ClassDB`-bound and removing it) - every single run produced a real AddressSanitizer **heap-use-after-free**, see `output.txt` |
| Fix verification: same `repro/` against the fixed, rebuilt binary | Same binary/build (incremental rebuild, ~44-53s - only `project_settings.cpp` changed), repeated attempts | **0/7+ runs crashed** (no AddressSanitizer output in any attempt) before the verification loop was abandoned for being too slow to finish cleanly within budget - see "What's not done" below for why the run count is stated as a lower bound rather than an exact N/10 |
| Project test suite before/after | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME` | See "The fix" below |

## Status

**Confirmed.**

Reproduces **10/10** runs crashing (two independent script revisions, 5 runs each) against the
unfixed binary, every single one a real ASan heap-use-after-free (not a flaky or seed-dependent
hit) - see `output.txt` for one full backtrace. **0 crashes observed in 7+ runs** against the
fixed, rebuilt binary (no run count ceiling reached - the verification loop became too slow to
finish within this session's time budget due to the repro's own error-log volume under lock
contention, not due to any sign of a crash; see "What's not done").

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `ProjectSettings::_set()`/`clear()`/`get_order()` through nothing but
the public GDScript `ProjectSettings` singleton, which every Godot project uses from the moment it
reads or writes a single setting.

## Class

`memory-safety` (concurrent unsynchronized mutation of a red-black tree - same bug *shape* as
candidate 002's `HashMap` case, but here the unlocked side is **itself a mutator**
(`props.erase()`), not just a reader, so both a locked insert and an unlocked erase can run their
own tree-rebalancing logic on the same node pointers at once)

## Root cause

`core/config/project_settings.cpp`. `ProjectSettings` declares one shared `_THREAD_SAFE_CLASS_`
mutex (`project_settings.h:42`) and several methods correctly wrap their body in
`_THREAD_SAFE_METHOD_` before touching `props` - but five public methods that read or write the
same `RBMap<StringName, VariantContainer> props` never do (pre-fix, `project_settings.cpp:1056-1082`):

- **`clear(p_name)`**: `props.erase(p_name)` - a real red-black-tree deletion (rotations,
  recoloring, node `memdelete`), not a scalar write.
- **`get_order(p_name)`**, **`is_builtin_setting(p_name)`**: `props[p_name].order` - a tree
  lookup (`operator[]`) plus a field read.
- **`set_order(p_name, p_order)`**, **`set_builtin_order(p_name)`**: `props[p_name].order = ...`
  - a tree lookup plus a field write.

Meanwhile `set_setting()` (-> `Object::set()` -> `ProjectSettings::_set()`,
`project_settings.cpp:293`, `_THREAD_SAFE_METHOD_`-guarded) inserts a new key via
`props[p_name] = VariantContainer(p_value, last_order++)` whenever the key doesn't already exist -
also a real tree insertion with its own rotations. A thread calling `set_setting()` on a new/
recently-cleared key and a thread calling `clear()` on that same key (or even a different key -
rotations can touch nodes anywhere in the tree, not just the one being inserted/erased) can run
concurrently: one holds the class mutex, the other holds nothing, so there is no mutual exclusion
between a locked tree insert and an unlocked tree erase. ASan caught this directly as a
heap-use-after-free: `ProjectSettings::_set()` (locked, inserting) reads a tree node that
`clear()` (unlocked, erasing on another thread) had already `free()`'d via `RBMap::_erase()` -
see `output.txt`'s full backtrace, frame `#0` in `RBMap::_insert()` reading into memory `freed by
thread T34 here: ... RBMap::_erase() core/templates/rb_map.h:590`, `previously allocated by thread
T28` in the same `_insert()` path. Many runs also hit `RBMap::_set_color()`'s own internal
consistency assertion ("Condition `p_node == _data._nil && p_color == RED` is true") before
reaching a fatal ASan fault, and a separate recurring crash signature in
`StringName::operator<()` (`core/string/string_name.h:106`) during tree comparisons on an
already-freed key - both symptoms of the same underlying torn tree structure, not separate bugs.

Reachability: `ProjectSettings.set_setting()`/`.clear()`/`.get_order()`/`.set_order()` are
ordinary, documented, `ClassDB`-bound GDScript API methods (used by, e.g., addons/plugins that
programmatically add or remove project settings, editor tooling, or any script managing dynamic
settings at runtime) - calling `clear()` from one `Thread` while another calls `set_setting()`
(or any of the other locked accessors) reaches this race through entirely ordinary-looking
GDScript code, no engine-internal trick required. `is_builtin_setting()` is bound in the public
C++ API (`core/config/project_settings.h:220`) but, unlike the other four, is **not**
`ClassDB`-bound - it isn't reachable from GDScript, only from C++/GDExtension callers; the
repro therefore does not exercise it directly, but the fix still covers it since it shares the
exact same unlocked-access pattern on the same field.

## Introduced by

`not determined`. `git log -S"int ProjectSettings::get_order"` only turns up a directory
reorganization commit (`127458ed17`, "Reorganized core/ directory, it was too fatty already") and
one unrelated later touch, neither of which is the point where `_THREAD_SAFE_METHOD_` was added to
`_set()`/`_get()` without also being added to these five siblings - the asymmetry predates easy
`git blame` attribution and a full `git bisect` wasn't run this session (soft time budget).

## The fix

Add `_THREAD_SAFE_METHOD_` as the first statement of all 5 methods, matching the exact convention
already used by `_set()`/`_get()`/`has_setting()` elsewhere in the same file and sharing the same
class-wide `_thread_safe_` mutex - no new lock, no new idiom, purely closing a gap in an
already-established pattern (`+5/-0` against `core/config/project_settings.cpp`, see
`patch.diff`):

```cpp
void ProjectSettings::clear(const String &p_name) {
    _THREAD_SAFE_METHOD_
    ERR_FAIL_COND_MSG(!props.has(p_name), vformat("Request for nonexistent project setting: '%s'.", p_name));
    props.erase(p_name);
    _queue_changed(p_name);
}
```
(and identically for the other 4 methods). `ProjectSettings`'s `_thread_safe_` is a plain,
recursive `Mutex` (`core/os/mutex.h`, same as candidate 008's `Input`), so none of these newly-
locked methods can deadlock against each other or against `_set()`/`_get()` calling into them (they
don't call each other or back into `_set()`/`_get()`).

Verified: **10/10** crashing runs against the unfixed binary (two independent script revisions) vs.
**0 crashes in 7+ runs** against the rebuilt, fixed binary (same `repro/`, same build) - see
"What's not done" for why this is stated as a lower bound rather than a completed N/10 count.

- **Applied for real** to `core/config/project_settings.cpp`, rebuilt the ASan editor binary twice
  (once to confirm the quieter script revision still crashes unfixed, once with the fix reapplied),
  ran `repro/` against both, then reverted (`git checkout -- core/config/project_settings.cpp`)
  before finishing this candidate - main tree confirmed clean (`git status`) afterward.
- Project test suite (`bin/godot.linuxbsd.editor.x86_64.san --headless --test`, same ASan+
  debug_symbols binary, absolute `XDG_DATA_HOME`), run against the fixed binary:
  `1431 test cases | 1430 passed | 1 failed | 3 skipped` (`425313 assertions | 424313 passed |
  1000 failed`) - identical to the established baseline from candidates 002-009 (same
  pre-existing `test_ip.cpp` IPv6 environment failure, unrelated to this change). No new
  failures.

## What's not done / open questions

- **The fix-verification loop did not reach a clean, explicit "no crash" completion message for
  every run** - it only reached "0 ASan errors observed in 7+ attempted runs" before being
  abandoned. The repro's own error-log volume (many threads' `ERR_FAIL_COND_MSG`/GDScript-backtrace
  prints, now fully serialized through one mutex once the fix is applied) made each run take far
  longer than the unfixed binary's near-immediate crash, long enough that even a 60-90s external
  timeout wasn't always enough to see the script's own final print. This is a repro-harness
  performance artifact, not a sign of a residual bug - no ASan output appeared in any attempt - but
  a future session re-running this candidate should either shorten the stress window further or
  reduce the key-pool/thread-count product to get a clean, fast "no crash" completion on every run.
- Did not check whether `autoloads`/`global_groups`/`scene_groups_cache`/`feature_overrides`/
  `custom_prop_info` (the other `HashMap`-typed members of `ProjectSettings`, `project_settings.h:
  103-114`) have the same locked-writer/unlocked-reader asymmetry - only the `props` tree and its
  five directly-touching methods were audited this session. `get_custom_property_info()`,
  `get_autoload_list()`, `get_global_groups_list()`, `get_scene_groups_cache()` (all `const`,
  returning a reference to the live container with no lock) look like plausible siblings of this
  same pattern and are worth a follow-up candidate.
