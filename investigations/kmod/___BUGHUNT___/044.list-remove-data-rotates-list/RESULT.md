# list-remove-data-rotates-list: kmod_list_remove_data() returns the node after the removed one as the list head

`kmod_list_remove_data()` (libkmod/libkmod-list.c) unlinks the element holding `data` and returns `container_of(list_node_remove(...))`, i.e. the removed node's successor. That is the new head only when the head itself was removed; when a middle element is removed, the returned "head" is the element after it, so the circular list is rotated (v1 v2 v3 v4 v5 minus v3 comes back starting at v4: v4 v5 v1 v2). The only caller is depmod's cycle report (`depmod_list_remove_data()` on the `roots` list), so the visible effect is the rotation of which module starts a reported cycle.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of libkmod-list.c, then depmod on a 9-module graph with one cycle (`repro/run.sh`) | HEAD ASAN build, 3 runs | 3/3: `Cycle detected: e -> a -> b -> e` |
| Same with the head-preserving change to `kmod_list_remove_data()` (`candidate_fix_breaks_testsuite.diff`) | wt build | `Cycle detected: b -> e -> a -> b` (same cycle, another starting point) |
| Random graphs (60, 4-9 modules), HEAD vs changed build, compare the cycle lines | both builds | 1 of 60 differ, always by rotation only; the "Found N modules" count is identical |
| Project test suite with the change | wt build | 6/7: `test-depmod` `depmod_detect_loop` fails, its expected output contains the rotated order (`mod_loop_c -> mod_loop_a -> mod_loop_b`) |

## Status

**Inconclusive** - the rotation is real and reproducible, but nothing documents which element should head the list, the only effect is the starting point of a reported cycle, and the project's own expected test output encodes the current behavior. Not offered as a defect without a maintainer decision.

**Fully reproducible**: **Yes** (the rotation itself); the real depmod binary shows it.

## Class

logic (minor, unspecified contract)

## Root cause

`return container_of(node, ...)` where `node` is the removed element's successor, regardless of whether the removed element was the head.

## Introduced by

not determined

## The fix

No `patch.diff`: the change in `candidate_fix_breaks_testsuite.diff` (+8/-3) makes `kmod_list_remove_data()` return the original head unless the head was removed; it passes the repro but needs the expected output of `depmod_detect_loop` updated, which is the maintainers' call.

## What's not done / open questions

- Whether any future caller relies on the head staying put.
