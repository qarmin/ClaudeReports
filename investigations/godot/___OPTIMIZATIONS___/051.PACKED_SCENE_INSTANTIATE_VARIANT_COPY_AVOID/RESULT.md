# `PackedScene::instantiate()` avoids redundant Array/Dictionary Variant copies for scripted properties

When a node's attached script overrides `_get_property_list()` to expose `Array`/`Dictionary`
properties (a common `@tool` plugin/editor-property pattern, and the shape `SceneState::instantiate()`
takes for *any* scripted `Array`/`Dictionary` property, tool or not), the pre-patch code copies the
property's `Variant` into a local `Array`/`Dictionary` by value (`Array set_array = value;`), does
the same again for the node's *current* value via `node->get(...)` to check type compatibility, then
- for `Dictionary` specifically - rebuilds the whole dictionary from scratch via two more full
`Array` copies (`keys()`/`values()`) and a duplicate-then-clear-then-reinsert loop, and finally (in
editor `GEN_EDIT_STATE_INSTANCE` mode) does one more blanket `value.duplicate(true)` regardless of
whether anything needed duplicating. Open PR **godotengine/godot#120100** (`MERGEABLE` against
current `HEAD`, applies clean) replaces the `Array set_array = value` copies with
`VariantInternal::get_array()`/`get_dictionary()` raw pointers (mutating the existing `Variant`
in place instead of copying it out and back in), rewrites `setup_resources_in_array`/
`setup_resources_in_dictionary` to mutate their argument in place via `Array`/`Dictionary`'s own
iterators instead of rebuilding via `keys()`/`values()`, and drops the final blanket
`duplicate(true)` in `GEN_EDIT_STATE_INSTANCE` mode.

Toolchain: `g++` (GCC) 15, SCons 4.11.1, `scons platform=linuxbsd target=editor tests=yes
debug_symbols=yes`.

## What was tried

Benchmark: new dedicated probe `harness/gdbench/packed_scene_instantiate_probe.gd` +
`harness/gdbench/prop_list_node.gd` (a `@tool` `Node` script overriding `_get_property_list()` with
100 `Dictionary` properties, matching PR #120100's own "Scenario B" test case). The probe populates
all 100 properties with non-empty dictionaries, `pack()`s the node into a `PackedScene`, then times
5000 `instantiate()` + `free()` calls (10% warmup discarded). Measured via **5 interleaved
baseline/patched pairs** using two saved binaries (no rebuild between measurements), `uptime`
checked before starting.

| Pair | baseline us/iter | patched us/iter |
|---|---|---|
| 1 | 31.7376 | 23.7768 |
| 2 | 32.4614 | 23.9150 |
| 3 | 32.4108 | 23.6608 |
| 4 | 32.2486 | 24.4028 |
| 5 | 32.9402 | 23.9364 |
| **mean** | **32.359** | **23.938** |

## Result

**-26.0% wall time per `PackedScene::instantiate()` call** (32.359us -> 23.938us/iter mean, on a
node with 100 scripted `Dictionary` properties), **5/5 interleaved pairs non-overlapping** (min
baseline 31.7376 > max patched 24.4028) - a real, solid, reproducible win on `instantiate()`, one of
the hottest paths in any Godot game (every scene load, every spawned instance). Time only; peak
RSS/allocation count were not measured - the mechanism (avoiding redundant `Variant` copies of
Array/Dictionary, which are refcounted/COW handles, not deep-copied by a bare assignment) plausibly
reduces allocation count too (fewer COW-triggering mutations), but this session did not quantify it.

## Does it change behavior/output?

- **Bit-exact for the tested cases.** Two correctness scripts (both saved in `bench/`), run against
  both arms:
  - `packed_scene_correctness_test.gd`: instantiates the 100-Dictionary-property scene, checks every
    property's value matches what was packed exactly (`{"k": i, "extra": "value_%d" % i}` for each
    `i`), then instantiates two more instances and mutates one's `dict_prop_0` to confirm the other
    instance's copy is unaffected (no accidental aliasing from the raw-pointer/in-place-mutation
    rewrite) - `ALL_TESTS_PASS` on both baseline and patched.
  - `packed_scene_correctness_test2.gd`: specifically exercises `GEN_EDIT_STATE_INSTANCE` (the
    editor "instance child scene" mode, the *only* place the removed blanket `duplicate(true)` could
    matter - confirmed via `PackedScene::instantiate()`'s own `ERR_FAIL_COND_V_MSG(p_edit_state !=
    GEN_EDIT_STATE_DISABLED, ..., "does not work without tools compiled")`, i.e. unreachable from a
    normal game's runtime `instantiate()` call, only from an editor build under tools). Two instances
    created in this mode, one's `dict_prop_0` mutated, the other's checked for independence -
    `ALL_TESTS_PASS` on both arms, so the removed blanket duplicate does not appear to have been the
    only thing preventing aliasing for this property shape (the array/dict-specific duplication logic
    already elsewhere in the function covers it).
- **Is this a breaking change?** No public API/file format change - internal instantiation-path
  rewrite only.
- Project test suite: **1427 cases, 1426 passed, 1 failed, 3 skipped both before and after** (same
  known pre-existing `test_ip.cpp` IPv6 failure) - identical.
- **Testing-methodology note**: correctly ran the full suite from `bin/`, not a `/tmp` copy, per the
  gotcha candidate 050 re-confirmed earlier this session.

## Why the result looks the way it does

`Array`/`Dictionary` are refcounted COW handles in Godot - a bare `Array set_array = value;` copy
doesn't deep-copy the underlying data, but it does an atomic refcount increment/decrement pair and,
critically, the *next* mutation through that new handle (`set_array.duplicate()`, or any write)
triggers a real COW split if the refcount is >1 (which it is here, since `value` itself also holds a
reference). The pre-patch `setup_resources_in_dictionary` compounds this: it extracts `keys()`/
`values()` as two more full `Array` copies, builds two *more* duplicated arrays, clears the whole
dictionary, then reinserts every key/value pair one at a time - a lot of intermediate Array/Dictionary
churn for what the patched version does with one pass over the existing dictionary's own iterator,
mutating values in place via `Dictionary::operator[]`. With 100 such properties per instantiated
node, this compounds directly into the ~26% measured here.

## What's not done / open questions

- Only the `Dictionary` shape (PR's "Scenario B") was benchmarked this session; the PR's diff
  equally affects the `Array` shape (its "Scenario C") and the general Array/Dictionary-typed-property
  path used by any scripted property, not just `@tool`-exposed ones - not separately measured here.
- Peak RSS/allocation count not measured (see "Result").
- Did not measure the PR's own claimed "1.6x -> 1.3x @tool vs non-tool discrepancy" framing
  directly - this session measured absolute per-instantiate time, not the tool/non-tool ratio the PR
  itself frames its numbers around.
