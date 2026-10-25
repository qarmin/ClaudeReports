# index-stdio-last255-hang: a stdio-reader index node with children range ending at 255 makes lookups loop forever

`index_read()` (libkmod/libkmod-index.c, the stdio reader used when the mmap indexes are not loaded, e.g. by `modinfo`) accepts `last` up to 255. The consumers iterate `for (uint8_t ch = node->first; ch <= node->last; ch++)`, which never terminates when `last == 255` (the `uint8_t` wraps). The mm reader rejects `last >= INDEX_CHILDMAX` (128); the stdio reader does not.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Static audit finding (`../___FINDINGS___`) + libFuzzer timeouts on `fuzz_index` | HEAD ASAN | timeouts in `index_read`/`index_dump_node` |
| Real `modinfo -b <root> -k 1.0 foo` with a crafted `modules.alias.bin` (root: prefix "*", children first=last=255, `repro/gen_index.py stdio-last255`) and valid empty other indexes | HEAD ASAN build, `memcap.sh 1500` + 10 s kill, 3 runs | 3/3 the process spins until killed (exit 124); patched: "Module foo not found", exit 1 |

## Status

**Confirmed** - deterministic, 3/3 on HEAD.

**Fully reproducible**: **Yes** - the real `modinfo` binary reading a crafted index through the real stdio path.

## Class

crash (hang / denial of service on a corrupt `modules.*.bin`)

## Root cause

The sanity check on the children range (`first > last`) bounds nothing above `last`; with `last == 255` and a `uint8_t` loop variable the loop condition `ch <= last` is always true.

## Introduced by

47407a2 "libkmod: Fix OOB write with illegal index files" - that commit added the `first > last` range check; the missing upper bound dates from the same code (the loops have used `uint8_t` since the stdio reader was written).

## The fix

`patch.diff` (+2/-1, libkmod/libkmod-index.c): also reject `last >= INDEX_CHILDMAX` in `index_read()`, matching the mm reader. This is the same hunk as in the maintainer's pending working-tree change to this file. Verified: repro 3/3 hang -> 3/3 clean exit. Project test suite (ASAN/UBSAN): 7/7 OK before -> 7/7 OK after.

## What's not done / open questions

- Cycles in child offsets (unbounded recursion / stack overflow in `index_read`, `index_dump_node`, `index_mm_dump_node`) are a different mechanism with a different fix and are queued in `IDEAS.md`.
