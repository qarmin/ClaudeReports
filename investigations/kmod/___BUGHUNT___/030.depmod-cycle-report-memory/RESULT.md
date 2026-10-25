# depmod-cycle-report-memory: the cycle report enumerates every simple path and exhausts RAM on a dense dependency cycle

After 011's fix (`prereq-011.diff`, applied first), `depmod_report_cycles_from_root()` (tools/depmod.c) terminates, but it still enumerates every simple path of a strongly connected component and allocates one `struct vertex` per explored path. For a dense SCC the number of paths is exponential in the number of modules, so `depmod` grows without bound and a tree of about 20 modules in one dense cycle exhausts gigabytes of RAM (on the user's 32 GB machine it freezes the system until the OOM killer acts) instead of printing the cycles.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `repro/gen_tree.py` k minimal ELF modules, module `i` imports `s<j>` for all `j > i`, the last imports `s0` (one dense cycle), real `depmod -b <root> 1.0` | HEAD + prereq-011.diff, ASAN/UBSAN build, run under `memcap.sh 1024` | k=12 finishes in about 1 s; k=20 hits the 1024 MB cap (ASAN `hard rss limit exhausted`), 3/3 runs |
| Same input, patched | HEAD + prereq-011.diff + patch.diff | k=20 prints `Too many dependency paths to explore, cycle report is incomplete` and exits 1 (the normal result for a tree with cycles) within the cap, 3/3 runs |

## Status

**Confirmed** - 3/3 cap hits before, 3/3 clean finishes after (see `output.txt`). The cap is a safety net of the test harness: without it the process keeps allocating.

**Fully reproducible**: **Yes** - real `depmod` on a tree of real (minimal) ELF modules; dependencies are derived by depmod from their symbols.

## Class

crash (unbounded memory growth / denial of service on a module tree with a large dependency cycle)

## Root cause

The DFS in `depmod_report_cycles_from_root()` does not memoize anything: each popped vertex is expanded into one new vertex per dependency, and a vertex is only skipped when its module is already an ancestor. In a dense SCC with `k` modules the number of simple paths is on the order of `(k-1)!`, and every one is kept in `free_list` until the root finishes.

## Introduced by

c7ce9f0 "depmod: handle nested loops" - the path-enumerating cycle search.

## The fix

`patch.diff` (+14/-2, tools/depmod.c, applies on top of `prereq-011.diff`): count the vertices created across the whole report and give up with an explicit `Too many dependency paths to explore, cycle report is incomplete` error after `CYCLE_REPORT_MAX_VERTICES` (2^21, about 100 MB). Verified with the repro (above) and the project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before and 7/7 OK after.

## What's not done / open questions

- The cap truncates the report for huge SCCs rather than reporting it exactly; a complete fix would use a visited set per root (report one cycle per back edge), which changes the output format and was not attempted.
- The limit value is a judgement call; k=12 still reports fully.
- Only the dense-DAG-plus-one-back-edge shape was generated.
