# `SceneState`: indices read from a binary `.scn` are never validated (`node_paths[..]`, `names[..]`, `variants[..]` out of bounds in `instantiate()`), and the node/connection int arrays are read past their end

Found by the GDScript binary-format fuzz loop (`ResourceLoader.load()` + `PackedScene.instantiate()` on byte-mutated `.scn`),
ASan editor build, seed 1: `FATAL: Index p_index = 16711680 is out of bounds (size() = 0)` from `CowData<NodePath>::get`
inside `PackedScene::instantiate()`.

## What was tried

| Approach/tool | Config | Outcome |
|---|---|---|
| Mutational fuzz of a binary `.scn` (Node3D with MeshInstance3D + Skeleton3D) | ASan editor build `b8420aa3a6`, seed 1, 1000 iterations | crash in `SceneState::instantiate` (`node_paths[id & FLAG_MASK]`, `NODE_FROM_ID`) |
| Generated repro (`repro/project/repro.gd`): pack a 2-node scene, set one field of the child's record in the `_bundled` dictionary, save with `ResourceSaver`, load the `.scn` and instantiate | unfixed build | field 9 (owner, path flag + huge index): crash 10/10; fields 2, 3, 10 (type / name indices) crash too, the others fail gracefully |
| Code reading of `set_bundled_scene()` | n/a | `r[idx++]` reads of the `nodes`/`conns` int arrays have no bounds check at all (only `size() >= node_count` is checked); element counts (`properties.resize(r[idx++])`) are unchecked too |
| Fix verification | `patch.diff`, rebuilt | all 14 mutated fields -> "Invalid bundled scene" error + null instantiate, exit 0 (10/10 for the repro); a scene with an instanced sub-scene, group, property and a persistent signal connection round-trips identically on both builds |

## Status

**Confirmed.** Deterministic, 10/10 crashes before, 0/10 after.

**Fully reproducible**: Yes - a real `.scn` file written by `ResourceSaver` and loaded by `ResourceLoader`, then `PackedScene.instantiate()`.
The corruption is applied through the `_bundled` dictionary before saving (the same data the binary loader hands to `set_bundled_scene()`);
the fuzz loop also reached the identical crash by flipping bytes of a saved `.scn`.

## Class

`memory-safety` (out-of-bounds index into `node_paths` / `names` / `variants` -> crash; unchecked reads past the int arrays)

## Root cause

`scene/resources/packed_scene.cpp`:

- `SceneState::instantiate()` uses `node_paths[p_id & FLAG_MASK]` (`NODE_FROM_ID` for parent/owner/connection ends, plus the "vanished parent"
  warning), `snames[n.name]`, `snames[n.type]`, `props[n.instance & FLAG_MASK]`, `snames[c.signal]`, `snames[c.method]` and `props[bind]` with no
  index validation (properties and groups, in contrast, do have `ERR_FAIL_INDEX` checks). In `NODE_FROM_ID` the path flag is tested with
  `p_id & FLAG_ID_IS_PATH`, so even a plain `-1` id (all bits set) takes the `node_paths[0xffffff]` branch.
- `SceneState::set_bundled_scene()` copies the arrays with `r[idx++]` while only checking `snodes.size() >= node_count`
  (not that each node's variable-length property/group lists fit), so truncated/inconsistent data reads past the array; negative or
  huge counts are passed straight to `resize()`.

## Introduced by

5dbf1809c6 A Whole New World (clang-format edition) (the oldest commit `git log -S` finds for the unchecked reads; the instantiate-side
index uses are of similar age)

## The fix

`patch.diff` (+70/-20, `scene/resources/packed_scene.cpp` only): bounds-checked reads (and sane element counts) in `set_bundled_scene()`, and a
validation pass at its end that rejects the scene (clears nodes/connections, `ERR_FAIL_MSG`) when a node name/type/instance index, an
is-path parent/owner/connection id, a connection signal/method or a bind refers outside `names`/`variants`/`node_paths`. `instantiate()` then
fails through its existing `node count is 0` check. Applied to the main tree, rebuilt, verified, reverted.

Tests: fixed build `--headless --test`: 1431 cases, 1430 passed, 1 failed (`[IP] resolve_hostname`,
needs network in this sandbox, unrelated); no clean unfixed full-suite baseline (see 073).

## What's not done / open questions

`just test` not run from a fresh clone; `repro/run.sh` run against the unfixed and fixed binaries from the main tree.
The fix is larger than the other candidates' fixes (validation instead of one guard); an alternative with fewer lines is adding
`ERR_FAIL_INDEX` checks at each use in `instantiate()`. Other readers of `SceneState` data (`get_node_*` accessors, `_recover_node_path_index`,
`SceneState::get_connection_*`, editor-side functions) were not audited for the same unchecked indices; they now see only validated data when it comes from a
loaded file. `editable_instances`, `ids`, `id_paths`, `base_scene_idx` are not validated. Candidate 080 is the separate string-length bug found in the same run.
