# `GLTFDocument::_parse_scenes()`: scene root node indices are not checked against the node array (`generate_scene()` aborts)

Found by the GDScript binary-format fuzz loop (`GLTFDocument.append_from_buffer()` + `generate_scene()` on a byte-mutated `.gltf`), seed 5: a
mutation renamed the `"nodes"` key, leaving `"scenes":[{"nodes":[0]}]` pointing at a node that no longer exists.

## What was tried

| Approach/tool | Config | Outcome |
|---|---|---|
| Mutational fuzz of a minimal triangle glTF (embedded base64 buffer) | build with 073-085 fixes, seed 5 | `FATAL: Index p_index = 0 is out of bounds (size() = 0)` in `GLTFDocument::generate_scene()` |
| Minimal hand-written JSON (`repro/project/repro.gd`) | unfixed build | `{"asset":{"version":"2.0"},"scene":0,"scenes":[{"nodes":[0]}]}`: `append_from_buffer` returns OK, `generate_scene` aborts, exit 134, 10/10. `"nodes":[5]` with a 1-node array crashes the same way; a bad `children` index is already rejected at parse time |
| Fix verification | `patch.diff`, rebuilt | both inputs: "glTF: A scene refers to a node that does not exist.", `append_from_buffer` returns Parse error, exit 0, 10/10; a valid one-node glTF with a mesh still loads and generates a Node3D |

## Status

**Confirmed.** Deterministic, 10/10 aborts before, 0/10 after.

**Fully reproducible**: Yes - real `GLTFDocument.append_from_buffer()` / `generate_scene()` on glTF JSON text (the importer uses the same calls).

## Class

`crash` (unchecked index into `GLTFState.nodes`)

## Root cause

`modules/gltf/gltf_document.cpp`, `_parse_scenes()` copies the `nodes` list of the selected scene into `p_state->root_nodes` without comparing the values with the number of entries in the file's top-level
`nodes` array (which is optional: `_parse_nodes()` returns OK when it is absent). `_generate_scene_node_tree()` then does `p_state->nodes[root_nodes[i]]` and trips the vector bounds trap. (For `_naming_version < 2` the roots are recomputed in
`_compute_node_heights()`, which hides the problem; the default newer naming version uses the file's list as is.)

## Introduced by

5d492e1aed GLTF: Allow parsing glTF files without nodes (made a missing `nodes` array legal; the unchecked `root_nodes` copy is older)

## The fix

`patch.diff` (+3/-0): in `_parse_scenes()` require each scene root entry to be a number in `[0, node_count)` where `node_count` is the size of the top-level `nodes` array (0 if absent), else `ERR_FILE_CORRUPT`. Applied to the main tree, rebuilt, verified, reverted.

Tests: fixed build `--headless --test`: 1431 cases, 1430 passed, 1 failed (`[IP] resolve_hostname`,
needs network in this sandbox, unrelated); no clean unfixed full-suite baseline (see 073).

## What's not done / open questions

`just test` not run from a fresh clone; `repro/run.sh` run against the unfixed and fixed binaries from the main tree. Other cross-references in the glTF JSON (mesh/skin/camera/light indices on nodes, accessor
and buffer-view indices, animation channel targets) were not systematically checked; the fuzz run only mutated one small file for 1500 iterations.
