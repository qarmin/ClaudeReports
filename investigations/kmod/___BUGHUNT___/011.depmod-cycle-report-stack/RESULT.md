# depmod-cycle-report-stack: cycle reporting walks cycles that do not pass through its root and overflows its stack

`depmod_report_cycles_from_root()` (tools/depmod.c) runs an explicit-stack DFS from each module that is still "in use" after the topological sort. The stack is sized for `n_r` entries (the number of such modules), the DFS never checks whether a module is already on its own ancestor path unless it is the root, and an `assert(is < stack_size)` guards the push. With overlapping cycles the DFS keeps following a cycle that does not contain the root (each loop pushes another vertex), and with dense dependency graphs the stack legitimately needs more than `n_r` slots. Either way `depmod` aborts on the assert (no `-DNDEBUG`) or, with asserts compiled out, writes past the `malloc`ed stack.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Synthetic tree of `k` minimal ELF modules (`repro/gen_tree.py`): module `i` exports `s<i>` and imports `s<j>` for all `j > i`, the last imports `s0` | real `depmod -b <root> 1.0`, HEAD ASAN build, under `memcap.sh 2048` | k=2: normal cycle report. k=3 (m0->{m1,m2}, m1->m2, m2->m0, root m1 reaches the cycle m0<->m2 not through itself) and k=8, 12: `Assertion 'is < stack_size' failed`, SIGABRT (134) |
| Fix only the stack (grow it dynamically), k=3 | patched build, 30 s timeout | no assert but depmod never finishes: the DFS follows `m0 <-> m2` forever (one new vertex per iteration, memory grows without bound, killed by the cap/timeout) |
| Fix only the ancestor check, k=3 and k=8 | patched build | k=3 is fixed; k=8 still hits the assert (fan-out of the dense graph exceeds `n_r`) |
| Both changes, k=3, 8, 10, 12 | patched build | each prints the cycles and `Found k modules in dependency cycles!` in about 1 s; `depmod` exit code 1 is the normal result for a tree with cycles |

## Status

**Confirmed** - deterministic, 5/5 on HEAD at k=3 (exit 134). Both halves of the fix are required (checked by applying each alone).

**Fully reproducible**: **Yes** - real `depmod` on a tree of real (minimal) ELF modules; dependencies are derived by depmod from their symbols, as for any module tree.

## Class

crash (assertion abort; heap overflow with asserts disabled; unbounded memory growth if the stack is merely enlarged)

## Root cause

1. The DFS marks `m->visited` but only uses it for the root; a vertex whose module already appears among its ancestors (a cycle not containing the root) is expanded again, so the search never terminates by itself. The assert on the stack size happened to stop it.
2. Even for terminating searches the stack bound `n_r` is wrong: a path can leave `outdegree - 1` pending siblings per level, so up to roughly the number of edges is needed.

The case is reachable whenever a module tree contains overlapping dependency cycles (e.g. a broken/third-party module set); the regular `depmod` run then aborts instead of reporting the cycles, so no `modules.dep` is written at all.

## Introduced by

c7ce9f0 "depmod: handle nested loops" - the stack assert and the root-only cycle check come from this commit (`git log -S"assert(is < stack_size)"`).

## The fix

`patch.diff` (+24/-4, tools/depmod.c): (a) skip a popped vertex whose module is already one of its ancestors (a cycle not through the root is reported when that cycle's own module is the root); (b) grow the DFS stack with `realloc` (doubling) instead of asserting. Verified on k=3, 8, 10, 12 (all report cycles in about 1 s; k=2 and the testsuite's cycle cases unchanged). Project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before -> 7/7 OK after.

## What's not done / open questions

- The search still enumerates simple paths, so a very dense SCC (k well above 12) is exponential in time; not measured beyond k=12 and not part of this fix.
- Only the dense-DAG-plus-one-back-edge shape was generated; random graphs are a natural testbed addition.
