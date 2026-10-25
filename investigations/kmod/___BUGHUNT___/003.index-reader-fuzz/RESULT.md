# index-reader-fuzz: mmap index reader trusts node contents and reads past the end of the file

`index_mm_read_node()` and the value readers (`read_chars_mm()`/`read_u32_mm()`/`read_value_mm()`, libkmod/libkmod-index.c) only check that a node's *offset* is inside the mapped `modules.*.bin`; the node's prefix string, child table and value list are then read with unbounded `strlen()`/pointer arithmetic. A node that claims more data than the file contains (e.g. `value_count = 0x7fffffff`) makes lookups walk off the end of the mapping.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| libFuzzer `fuzz/fuzz_index.c` on both readers (mm reader over an exact-size heap copy so ASAN sees every byte; stdio reader over `fmemopen`), unpatched | clang ASAN+UBSAN, key + index bytes input, seeds from `fuzz/mkseeds.py`; results in `fuzz/classify_orig.txt` | ~950 artifacts: heap-buffer-overflow in `read_chars_mm` (544), `read_u32_mm` (388), `read_char_mm` (15); plus separate classes (cyclic child offsets: stack-overflow / timeouts in `index_read`, `index_dump_node`, `index_mm_dump_node`; see "Not done") |
| Same harness on the hardened reader (mm hunks + the other hunks of the pending local diff) | `fuzz/classify_fixed.txt` | no crash |
| Real `modprobe -d <dir> --show-depends foo` on a crafted `modules.alias.bin` (`repro/`) | HEAD ASAN build, 10 runs | 10/10 ASAN `unknown-crash` READ at the end of the mapping, from `kmod_lookup_alias_from_alias_bin()` -> `index_mm_searchwild()` -> `read_value_mm()` |
| Real `modprobe` with a node whose prefix is unterminated at EOF, or a node near the end of a page-aligned file | HEAD ASAN build | no crash: the over-read lands in a neighbouring mapping (mmap regions are adjacent), so it only returns garbage - the crash needs a count/length large enough to leave the mapped region |

## Status

**Confirmed** - deterministic, 10/10 runs of `repro/run.sh` against HEAD; reproduced again via the real binary (not a heap-copy harness).

**Fully reproducible**: **Yes** - the input is a single crafted file in the `-d` directory consumed by the real `modprobe`; there is no second participant.

## Class

memory-safety (out-of-bounds read / crash on a malformed `modules.*.bin`)

## Root cause

`index_mm_open()` stores `size`, and `index_mm_read_node()` rejects a node *offset* `>= size`, but nothing bounds what is read after that offset: the prefix via `strlen()`, `child_count` 32-bit offsets, `value_count` and each value's `strlen()`. The consumers (`index_mm_searchwild_allvalues()`, `index_mm_search_node()`, dump) then iterate `value_count` entries unchecked. The files are written by `depmod`, so this only matters for a corrupt/truncated or maliciously prepared index (e.g. a crafted `-d` / `--moduledir` tree, or a truncated write).

## Introduced by

836be9a "index: mmap: create and destroy node" - the mm node reader was introduced there without bounds checks.

## The fix

`patch.diff` (+36/-2, libkmod/libkmod-index.c): the mm-reader hunks of the pending local hardening diff (`mm_left()`/`mm_skip_str()` helpers; `index_mm_read_node()` verifies that the prefix is NUL-terminated inside the file, that the first/last bytes, the child table and the value count fit, and walks and validates every value up front so consumers can read them unchecked). It is a subset of the working-tree diff the maintainer already had pending; the recursion-depth cap and stdio `last` hunks of that diff are intentionally excluded (separate bugs, see below).

Verified: repro 10/10 crash before -> 10/10 clean (`modprobe` prints "Module foo not found", no ASAN) after; the libFuzzer classification of the hardened reader shows no crash. Project test suite (`tools/run_tests.sh`, meson suite under ASAN/UBSAN): 7/7 OK before -> 7/7 OK after.

## What's not done / open questions

- The same file also showed cyclic/over-deep recursion (child offsets pointing back at an ancestor: stack-overflow in `index_read()`/`index_mm_dump_node()`, timeouts in `index_dump_node()`/`add_value()`) and the stdio reader accepting `last == 255` (`uint8_t` loop never terminates). Those are different mechanisms with a different fix (depth/key-length cap, `last >= INDEX_CHILDMAX` check); queued in `IDEAS.md` as their own candidates.
- Only `modules.alias.bin` was crafted for the real-binary repro; the other indexes share the reader.
