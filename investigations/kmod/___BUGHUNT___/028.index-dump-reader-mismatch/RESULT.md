# index-dump-reader-mismatch: `index_dump()` (stdio) and `index_mm_dump()` differ on some crafted files

The `fuzz_index` testbed component flags `diff:index-dump` (39 artifacts in the 10:48 run): dumping the same index through the stdio reader and the mmap reader produced different bytes.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Replay all 39 artifacts through a small driver (`index_mm_open` + `index_file_open`, both dumps to files) on HEAD | ASAN build | differences present |
| Same replay on a build with the maintainer's pending hardening of `libkmod-index.c` (bounded mm reads, key-length cap, `last` range) | patched ASAN build | 24 of 39 identical (the differences were caused by the mm over-reads / cyclic walks of 003 and 015); 15 remain |
| Smallest remaining case (4544 bytes) | same | identical set of lines, same byte count, one line at a different position: a node holding several values with different priorities. The stdio reader's `add_value()` keeps `node->values` ordered by priority while the mm dump prints values in file order |

## Status

**Not reproducible** as a defect: the remaining difference is value ordering within one node (a presentation difference of two readers with different value containers), and the dump is only used for `modprobe -c` output, where entries of equal key but different priority are not order-sensitive. The other 24 mismatches were symptoms of 003/015.

## Not done

- The fuzz harness' `diff:index-dump` comparison should sort lines before comparing (testbed follow-up; `KNOWN.md` records the signature).
