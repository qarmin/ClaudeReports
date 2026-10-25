# `ProjectSettings::set_custom_property_info()`/`get_custom_property_info()` missing lock on `custom_prop_info`

Follow-up to confirmed candidate 010 (`ProjectSettings` `props` RBMap missing lock), continuing
that candidate's own flagged "what's not done" item: check whether
`custom_prop_info`/`autoloads`/`global_groups`/`scene_groups_cache`/`feature_overrides` (the other
`HashMap`-typed members of `ProjectSettings`, `project_settings.h:103-114`) share the same
locked-side/unlocked-side asymmetry its `props` fix closed. Audited all five; one
(`custom_prop_info`) reproduces with the exact same shape and is fixed here. The other four are
addressed below under "What's not done" rather than batch-fixed without an independent repro each.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: lock-usage audit of all 5 `HashMap`-typed members across `core/config/project_settings.cpp` | n/a | Found `custom_prop_info`'s writer (`set_custom_property_info()`, line 1389) and its only public raw-reference getter (`get_custom_property_info()`, line 1395) both fully unlocked, while the container's other reader (`_get_property_list()`, line 455) **does** hold `_THREAD_SAFE_METHOD_` - the same asymmetric shape candidate 010 fixed for `props`. Also found `set_custom_property_info()`'s own `ERR_FAIL_COND(!props.has(prop_name))` line reads `props` unlocked, a second, narrower gap candidate 010's fix didn't close (that fix only touched `clear()`/`get_order()`/`set_order()`/`set_builtin_order()`/`is_builtin_setting()`). Found `feature_overrides` is **already fully guarded** on both sides (write inside `_set()`, reads inside `get_setting_with_override()`/`get_setting_with_override_and_custom_features()`, all three already `_THREAD_SAFE_METHOD_`-guarded) - not a real gap despite matching the candidate description. Found `autoloads`/`global_groups`/`scene_groups_cache` have the same unlocked-getter/unlocked-direct-writer shape by reading alone, but their getters (`get_autoload_list()`, `get_global_groups_list()`, `get_scene_groups_cache()`) are not `ClassDB`-bound, so no *pure*-GDScript repro reaches them the same direct way (see "What's not done") |
| GDScript-level repro (`repro/`), against a real, self-built ASan editor binary | `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`, `--headless --path repro`, 6 threads calling `ProjectSettings.add_property_info({...})` (unlocked write into `custom_prop_info`) + 4 threads calling `Object.get_property_list()` (-> locked `_get_property_list()`, reading the same `custom_prop_info`) concurrently on a pre-populated 200-key pool for an 8s stress window | **5/5 runs crashed** - every single run a real AddressSanitizer heap-use-after-free inside `HashMap<StringName, PropertyInfo>` (`custom_prop_info`'s own type), reached directly through `ProjectSettings::set_custom_property_info()`/`_get_property_list()`; see `output.txt` for a full backtrace |
| Same repro, varied input (pool_size 200 -> 50, same bug class) | Same binary, same build | **3/3 runs crashed**, same ASan heap-use-after-free shape - confirms the finding isn't an artifact of the specific pool size first tried |
| Fix verification: same `repro/` (both pool sizes) against the fixed, rebuilt binary | Same binary/build (incremental rebuild, ~50-65s each - only `project_settings.cpp` changed) | **0/7 runs crashed** (pool_size=200) and **0/3 runs crashed** (pool_size=50) - clean "No crash within the time limit" exit every single attempt |
| Project test suite before/after | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME` | See "The fix" below |

## Status

**Confirmed** (scoped to `custom_prop_info`'s `set_custom_property_info()`/`get_custom_property_info()`).

Reproduces **5/5** runs crashing with the original input (pool_size=200) and **3/3** with a varied
input in the same bug class (pool_size=50) against the unfixed binary - every single one a real
ASan heap-use-after-free inside `HashMap<StringName, PropertyInfo>`, not a flaky or seed-dependent
hit. **0 crashes observed across 10 total runs** (7 + 3) against the fixed, rebuilt binary, for
both inputs.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `ProjectSettings::set_custom_property_info()` (via the public,
`ClassDB`-bound `ProjectSettings.add_property_info()`) and `ProjectSettings::_get_property_list()`
(via the public, `ClassDB`-bound `Object.get_property_list()`) through nothing but ordinary
GDScript `Thread`s - no engine-internal trick, no hand-constructed internal state.

## Class

`memory-safety` (concurrent unsynchronized access to a `HashMap` - same bug *shape* as candidate
002's `ResourceUID` case and candidate 010's `props` RBMap case: one side (`_get_property_list()`)
takes the class's `_THREAD_SAFE_METHOD_` lock, the other side (`set_custom_property_info()`) does
not, so the lock provides no actual mutual exclusion between them)

## Root cause

`core/config/project_settings.cpp`. `ProjectSettings` declares one shared `_THREAD_SAFE_CLASS_`
mutex (`project_settings.h:42`). `_set()`, `_get()`, `get_setting_with_override()`,
`get_setting_with_override_and_custom_features()`, `_get_property_list()`, and `has_setting()` all
correctly start with `_THREAD_SAFE_METHOD_` before touching their respective containers - but
`set_custom_property_info()` (project_settings.cpp:1389, pre-fix) and
`get_custom_property_info()` (project_settings.cpp:1395, pre-fix) touch `custom_prop_info`
(`HashMap<StringName, PropertyInfo>`, `project_settings.h:103`) with **no lock at all**:

- **`set_custom_property_info(p_info)`**: `custom_prop_info[prop_name] = p_info` - a real
  `HashMap::operator[]` insert, which can trigger `_resize_and_rehash()` (reallocating and
  freeing the backing bucket array) exactly like candidate 002's `ResourceUID` case.
- **`get_custom_property_info()`**: returns `const HashMap<StringName, PropertyInfo> &` - a live
  reference to the same container, handed back with no lock held during the call and none implied
  for however long the caller keeps using the reference afterward (same caveat already flagged in
  this workspace's `projects/godot.md` risk map as a systemic pattern).

Meanwhile `_get_property_list()` (project_settings.cpp:455, `_THREAD_SAFE_METHOD_`-guarded) reads
the exact same map twice per base property (`custom_prop_info.has(base.name)` then
`custom_prop_info[base.name]`, lines 516-517, repeated for `setting_overrides` at 539-546) while
iterating. A thread calling `ProjectSettings.add_property_info()` (GDScript -> `_add_property_info_bind()`
-> `set_custom_property_info()`, unlocked) and a thread calling `Object.get_property_list()`
(GDScript -> `_get_property_list()`, locked) can run concurrently: the locked side's lock does
nothing to stop the unlocked side from mutating/rehashing the map mid-iteration. ASan caught this
directly as a heap-use-after-free: `set_custom_property_info()`'s `custom_prop_info[prop_name] = p_info`
(frame `#4`, `project_settings.cpp:1392`) writes into a `HashMapElement<StringName, PropertyInfo>`
slot that had just been freed by a *different* `HashMap` instance's own rehash on the same
physical memory (an unrelated `HashMap<StringName, ClassDB::ClassInfo>` - expected: ASan tracks
freed/allocated history per address, not per C++ type, so the "freed by" side of a UAF report often
names whichever allocation last occupied that reused memory, not literally the same container -
see `output.txt`'s note). Other runs crashed inside different internal `HashMap` functions
(`_lookup_idx_unchecked`, `_insert_element`), consistent with a genuine race rather than one fixed
crash site.

As a smaller, secondary finding: `set_custom_property_info()`'s own `ERR_FAIL_COND(!props.has(prop_name))`
check (project_settings.cpp:1391) reads the *other* container, `props`, with no lock either - a
narrower version of candidate 010's own bug, on a line that candidate's fix didn't touch (010 only
added locks to `clear()`/`get_order()`/`set_order()`/`set_builtin_order()`/`is_builtin_setting()`,
not to this call site). Adding `_THREAD_SAFE_METHOD_` to `set_custom_property_info()` as a whole
closes this too, as a side effect of the same single lock acquisition - not an extra line.

Reachability: `ProjectSettings.add_property_info()` is an ordinary, documented, `ClassDB`-bound
GDScript API method (`core/config/project_settings.cpp:1648`); `Object.get_property_list()` is
likewise a universally available, `ClassDB`-bound method every `Object`-derived instance exposes
(`core/object/object.cpp:1913`). Calling `ProjectSettings.add_property_info()` from one `Thread`
while another calls `get_property_list()` on the `ProjectSettings` singleton (or on any other
object, while `ProjectSettings`'s own `_get_property_list()` runs as part of property reflection)
reaches this race through entirely ordinary-looking GDScript code - e.g. any addon/plugin that
registers custom property info from a background thread while something else (an inspector
refresh, serialization, another script) queries the property list concurrently.

## Introduced by

`not determined` - `custom_prop_info`/`set_custom_property_info()` predate the public tracked
history: `git log -S"custom_prop_info" --all -- core/globals.cpp` (the file's pre-rename name)
bottoms out at 0b806ee0fc ("GODOT IS OPEN SOURCE", 2014-02-09, the initial public squash commit),
where `custom_prop_info[...]` assignments and `set_custom_property_info()` already exist, already
unlocked. By contrast, `_set()`/`_get()`/`_get_property_list()`/`has()` already carried
`_THREAD_SAFE_METHOD_` in that same initial commit (confirmed via `git log -p -S"_THREAD_SAFE_METHOD_"`),
so the asymmetry itself - not just the unlocked code - predates tracked history too; a full `git
bisect` wasn't run this session (no earlier history to bisect into, and no cheap go/no-go check
would add information beyond what's already established).

## The fix

Add `_THREAD_SAFE_METHOD_` as the first statement of `set_custom_property_info()` and
`get_custom_property_info()`, matching the exact convention already used by `_set()`/`_get()`/
`_get_property_list()`/`has_setting()` elsewhere in the same file and sharing the same class-wide
`_thread_safe_` mutex - no new lock, no new idiom, purely closing a gap in an already-established
pattern (`+2/-0` against `core/config/project_settings.cpp`, see `patch.diff`):

```cpp
void ProjectSettings::set_custom_property_info(const PropertyInfo &p_info) {
	_THREAD_SAFE_METHOD_
	const String &prop_name = p_info.name;
	ERR_FAIL_COND(!props.has(prop_name));
	custom_prop_info[prop_name] = p_info;
}

const HashMap<StringName, PropertyInfo> &ProjectSettings::get_custom_property_info() const {
	_THREAD_SAFE_METHOD_
	return custom_prop_info;
}
```

`ProjectSettings`'s `_thread_safe_` is a plain, recursive `Mutex` (`core/os/mutex.h`, same as
candidates 008/010), so neither newly-locked method can deadlock against `_set()`/`_get()`/
`_get_property_list()` calling into them (none of them call back into `set_custom_property_info()`/
`get_custom_property_info()`, and vice versa).

Verified: **5/5** (pool_size=200) and **3/3** (pool_size=50) crashing runs against the unfixed
binary vs. **0/7** and **0/3** against the rebuilt, fixed binary (same `repro/`, same build,
same two inputs) - see `output.txt`.

- **Applied for real** to `core/config/project_settings.cpp`, rebuilt the ASan editor binary twice
  (once to reconfirm the unfixed binary still crashes with the varied input, once with the fix
  reapplied), ran `repro/` against both with both inputs, then reverted
  (`git checkout -- core/config/project_settings.cpp`) before finishing this candidate - main
  worktree confirmed clean (`git status`) afterward.
- Project test suite (`bin/godot.linuxbsd.editor.x86_64.san --headless --test`, same ASan +
  debug_symbols binary, absolute `XDG_DATA_HOME`): **identical before and after** -
  `1431 test cases | 1430 passed | 1 failed | 3 skipped` (`425313 assertions | 424313 passed |
  1000 failed`), the same pre-existing `tests/core/io/test_ip.cpp` IPv6-environment failure
  already the established baseline from candidates 002-010/027. No new failures introduced.

## What's not done / open questions

- **`autoloads`/`global_groups`/`scene_groups_cache`** (`project_settings.h:112-114`) show the
  same shape by code reading - unlocked getters (`get_autoload_list()`, `get_global_groups_list()`,
  `get_scene_groups_cache()`) returning live references, racing against unlocked direct writers
  (`add_autoload()`/`remove_autoload()`, `add_global_group()`/`remove_global_group()`,
  `add_scene_groups_cache()`/`remove_scene_groups_cache()`) - **but none of these getters are
  `ClassDB`-bound**, so no pure-GDScript repro reaches the unlocked-reader side the same direct
  way `custom_prop_info`'s `get_property_list()` did. The only real engine-internal call sites
  found this session that read them unlocked at runtime (not editor-only `TOOLS_ENABLED`
  autocompletion) are `modules/gdscript/gdscript_compiler.cpp:423` (`get_autoload_list()`, read
  during GDScript compilation - potentially reachable from a background script-loading thread) and
  similar compiler/doc-gen call sites - plausible but not independently built and run as a repro
  this session (would need a real threaded-script-load scenario, not just a `Thread` calling a
  bound method directly). Left as a sharper, code-grounded lead rather than batch-fixing three
  more containers without a run-verified reproduction each, per this workspace's own confirmation
  discipline (see `CONFIRMATION.md`).
- **`feature_overrides`** (`project_settings.h:109`) was checked and found to **already be fully
  guarded**: its only writer (inside `_set()`) and its only readers (`get_setting_with_override()`,
  `get_setting_with_override_and_custom_features()`) are all three already
  `_THREAD_SAFE_METHOD_`-guarded. Despite matching the candidate description's listed containers,
  this one is a negative result, not a gap - no further action needed here.
- Did not check whether other `ProjectSettings` members beyond the five named in this candidate's
  description (`global_class_list`, `custom_features`, `hidden_prefixes`, `input_presets`) have
  similar gaps - out of scope for this candidate.
