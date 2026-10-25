# `Object::get_property_list()`'s `is_class(Script)` check vs `cast_to<Script>` - no measurable difference

`Object::get_property_list()` decides whether the synthetic `"script"` property should carry
`PROPERTY_USAGE_NO_EDITOR` by checking `is_class(Script::get_class_static())` - a StringName lookup
into the object's full class-hierarchy-name list (`get_gdtype().get_name_hierarchy().has(p_class)`).
Open PR **godotengine/godot#120054** (`MERGEABLE` against current `HEAD`, single line, single file)
replaces it with `Object::cast_to<Script>(this)`, Godot's own fast typed-cast mechanism - addressing
an unresolved review comment from a previous PR (#94062) that flagged the original as the more
expensive of the two ways to ask "is this a `Script`".

Toolchain: `g++` (GCC) 15, SCons 4.11.1, `scons platform=linuxbsd target=editor tests=yes
debug_symbols=yes`.

## What was tried

Benchmark: new `get_property_list` mode in `harness/gdbench/bench.gd` - calls
`Node2D.new().get_property_list()` 300,000 times via the existing `_timed()` closure helper.
Measured via **5 interleaved baseline/patched pairs**, `uptime` checked before starting (load
average 0.80-1.11 on a 24-core box).

| Pair | baseline us/iter | patched us/iter |
|---|---|---|
| 1 | 31.281 | 31.584 |
| 2 | 30.811 | 32.061 |
| 3 | 30.884 | 30.766 |
| 4 | 31.851 | 31.359 |
| 5 | 31.101 | 31.520 |
| **mean** | **31.186** | **31.458** |

## Result

**No change**: mean delta +0.87% (patched slightly *slower* on average), and the direction is not
consistent - patched is slower in 3/5 pairs, faster in 2/5. This is squarely within measurement
noise, not a real regression or a real win. Time only; peak RSS/allocation count were not measured -
`Object::cast_to<>` and `is_class()` are both pure read-only lookups with no allocation on either
side, so no allocation-count difference is possible by construction.

## Does it change behavior/output?

- **Bit-exact.** Dedicated correctness script (`bench/is_class_correctness.gd`) prints the `"script"`
  property's `usage` flags for a `GDScript` instance, a `Node`, and a `Resource` - all three values
  identical between baseline and patched builds (`Script.script.usage=1048586`,
  `Node.script.usage=1048590`, `Resource.script.usage=1048590` on both arms), confirming
  `is_class(Script)` and `cast_to<Script>()` agree on every tested case, including the one that
  matters (a `Script` object correctly gets the different usage value).
- **Is this a breaking change?** No - internal implementation detail, no observable output change.
- Project test suite: **1427 cases, 1426 passed, 1 failed, 3 skipped both before and after** (same
  known pre-existing `test_ip.cpp` IPv6 failure) - identical.

## Why the result looks the way it does

`Object::get_property_list()` does substantially more work than this one check - it calls
`_get_property_listv()` (which for most classes queries and appends many `PropertyInfo` entries,
each involving `String`/`StringName` construction) before ever reaching the line this patch touches.
One `is_class()` call (itself a small, bounded hierarchy-name lookup, not an expensive operation to
begin with - `get_name_hierarchy()` is a short list for any real class) is too small a fraction of
that total cost to separate from noise, even measured with a tight, low-variance benchmark (baseline
alone spans only 30.811-31.851, a ~3% range). This is the same shape as candidate 053: a
plausible-sounding micro-optimization that doesn't clear the noise floor once actually measured
against the function's real total cost, per `HEURISTICS.md`'s anti-pattern guidance ("is it in the
profiler's top 20").

## What's not done / open questions

- Not measured: peak RSS/allocation count (see "Result" - none expected by construction).
- Did not profile `Object::get_property_list()` as a whole to confirm `is_class()`'s actual
  fraction of its total cost (inferred from the null result rather than measured directly) - a
  `perf` breakdown would make this more precise but wasn't judged worth the time for a change
  already showing no measurable effect.
