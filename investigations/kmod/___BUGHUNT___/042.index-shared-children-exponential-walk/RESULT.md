# index-shared-children-exponential-walk: index walkers revisit shared child nodes, so a 400-byte modules.alias.bin keeps `modprobe -c` and alias lookups busy forever

The `modules.*.bin` format is a tree, but nothing checks that: a node's child offsets may point at a node that is also reachable another way. The four recursive walkers (`index_dump_node()`/`index_searchwild__all()` for the stdio reader, `index_mm_dump_node()`/`index_mm_searchwild__all()` for the mmapped reader) follow every child of every node they reach, so a chain of nodes whose two children are the same next node has `2**depth` paths in a file of about `10 * depth` bytes. At depth 40 (422 bytes) `modprobe -c` (index dump), `modprobe -n foo` and `modinfo foo` (alias lookup into a `*`-prefixed subtree) do not finish. This is the "branching index cycles stay exponential" follow-up of candidate 015: it needs no cycle, and the same unbounded work also happens for cycles with more than one child.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `repro/gen_index.py 40` as `modules.alias.bin` (other four indexes empty), real `modprobe -C <empty> -d <root> -S 1.0 -c` under `memcap.sh 2048` + `timeout 20` | HEAD ASAN build, 2 runs | 2/2: killed by the 20 s timeout (exit 124); depth 22 (242 bytes) takes about 1 s, depth 5 prints the 32 expected lines |
| Same file with the root prefix `*` (`gen_index.py 40 f wild`): `modprobe -n foo` (mmapped reader) and `modinfo -b <root> -k 1.0 foo` (stdio reader, because `modinfo` does not load the mmapped indexes) | HEAD | 2/2 each: 124 timeout |
| libFuzzer `fuzz_index` on a build with the pending (uncommitted) index hardening of the main tree, 10 minutes | WIP build | two `timeout-*` artifacts, both inside `index_mm_dump_node` / its write loop: the same class (branching nodes); the other crashes in that run were a harness bug (`sorted_lines()` returned an unterminated string for an empty dump, fixed in the testbed) |
| Patched: same three commands, plus a 2-child cycle (root children both point at the root) | patch.diff | all finish at once (`modprobe -c` exit 0, lookups "not found"); the cycle case that overflows the stack on HEAD ends cleanly in `modprobe -c` |
| Real index of 6901 Ubuntu modules (`depmod` output, 55119 dump lines) through `modprobe -c` and seven alias lookups, HEAD vs patched | patch.diff | byte-identical dump, identical lookup results |

## Status

**Confirmed** - deterministic hang, 2/2 on HEAD for each of the three commands (`output.txt`).

**Fully reproducible**: **Yes** - real `modprobe`/`modinfo` binaries reading a crafted index through `-d`/`-b` (the same threat model as candidates 003, 014, 015); no fake participant.

## Class

crash (unbounded CPU time / output)

## Root cause

The walkers assume a tree and keep no record of visited nodes or amount of work. Every legitimate node is visited at most once per dump, and once per (node, key position) in a wildcard lookup.

## Introduced by

e8847fd "Import index handling from module-init-tools" (stdio reader), bf89f70 "index: mmap: add support for seaching with wildcards" (mmapped wildcard lookup)

## The fix

`patch.diff` (+37/-8, libkmod/libkmod-index.c): both readers count node reads per top-level call (`visits`/`visit_limit` in `struct index_file` and `struct index_mm`, reset by `index_readroot()`/`index_mm_readroot()`) and refuse further nodes beyond the budget: the file size for a dump (a valid index has fewer nodes than bytes) and file size times (key length + 1) for a wildcard lookup; plain `index_search()` is unbounded (single path). Verified with the repro (2/2 exit 1 -> 2/2 exit 0), the cycle case above, the real-index comparison above, and the project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before and 7/7 OK after. The patch is against HEAD: the main tree has uncommitted index hardening in the same file (key length cap, bounds checks), which touches neighbouring lines and will need a trivial rebase; the budget is independent of those changes.

## What's not done / open questions

- The budget bounds work, not output size: a valid-looking tree with very many keys still dumps all of them (it is a tree, work is linear in the file size).
- `index_search()` follows one path (at most key length nodes) and was not changed.
- Wildcard lookups on a huge legitimate index with a very long key are allowed up to `size * (strlen(key)+1)` node reads, linear in both.
