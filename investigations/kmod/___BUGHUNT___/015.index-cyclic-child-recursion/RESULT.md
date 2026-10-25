# index-cyclic-child-recursion: a child offset pointing back at an ancestor recurses until the stack overflows

The recursive index walkers (`index_mm_dump_node`, `index_mm_searchwild_all`, `index_dump_node`, `index_searchwild__all` in libkmod/libkmod-index.c) follow child offsets without any cycle or depth protection. An index file where a node's child offset points at itself or an ancestor makes them recurse forever: ASAN reports a stack overflow (and a release build would segfault). Reachable through `modprobe -c` (dumps `modules.alias.bin`/`modules.symbols.bin` via the mm reader) and `modinfo`/alias lookups (wildcard walk through the stdio reader).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| libFuzzer `fuzz_index` (testbed) | HEAD ASAN | stack-overflow in `index_read`/`index_mm_dump_node` frames, timeouts in `index_dump_node` |
| Real `modprobe -c -d <root>` with `modules.alias.bin` root node child `a` -> root (`repro/gen_index.py cycle`), other indexes empty and valid | HEAD ASAN, `memcap.sh 2048`, 3 runs | 3/3 `AddressSanitizer: stack-overflow` in `index_mm_dump_node` |
| Real `modinfo -b <root> -k 1.0 foo` with root prefix `*` and child `a` -> root (`cycle-wild`) | HEAD ASAN, 3 runs | 3/3 `stack-overflow` in the stdio wildcard walk |
| Monotonic-offset check (reject child offsets <= parent offset) as the fix | patched build | breaks 3 of the 7 suite tests: the pre-generated indexes in `testsuite/rootfs-pristine` (older writer layout) have children before parents, so offsets are not monotonic in valid files - discarded |
| Key-length cap (`INDEX_MAX_KEY_LEN` 8192) in the four walkers as the fix | patched build | both repros 3/3 clean, suite 7/7 |
| Cycle with two children (`a` and `b` both -> root) after the key-length cap | patched build, 15 s | still never finishes (exponential: 2^8192 paths); on HEAD it dies earlier of the stack overflow |

## Status

**Confirmed** - deterministic, 3/3 on HEAD for both readers.

**Fully reproducible**: **Yes** - real `modprobe` and `modinfo` binaries on a crafted index file.

## Class

crash (stack exhaustion / denial of service on a corrupt `modules.*.bin`)

## Root cause

Trie traversal assumes the index is a tree; nothing checks that a child offset leads to a node that is not an ancestor. Each recursion level pushes at least one character into the key buffer, so the key length bounds the depth.

## Introduced by

e8847fd "Import index handling from module-init-tools" - the recursive walkers came in with the imported index code and have never had cycle protection (`git log -S"index_dump_node"`).

## The fix

`patch.diff` (+19/-0, libkmod/libkmod-index.c): `INDEX_MAX_KEY_LEN` (8192) and an early return (closing the node in the stdio variants, which own it) at the top of the four walkers. This is the same shape as the maintainer's pending working-tree change. It removes the stack overflow for every cycle shape, but it is a partial fix: a cycle with several children per node can still cost exponential time (only the depth is bounded, not the number of paths). A complete fix needs a visited set / per-walk node budget; the obvious "children come after parents" check is invalid because legacy indexes are laid out the other way.

Verified: both repros 3/3 crash -> 3/3 clean; project test suite (ASAN/UBSAN) 7/7 OK before -> 7/7 OK after.

## What's not done / open questions

- Branching cycles (exponential time) remain after the patch; not fixed here.
- The testbed's `diff:index-dump` signature (mm and stdio dumps differ) was not analyzed; it may just be this cycle class.
