# `PackedSourcePCK::try_open_pack()`: directory file count and path lengths are trusted (hang; `sl + 1` wrap)

Found by the GDScript binary-format fuzz loop (`ProjectSettings.load_resource_pack()` on byte-mutated `PCKPacker` output): the
iteration on a 298-byte file did not finish within the 90 s limit.

## What was tried

| Approach/tool | Config | Outcome |
|---|---|---|
| Mutational fuzz of a 2-file `.pck` | build with 073-082 fixes, seed 1, 300 iterations | one iteration never returns |
| Generated repro (`repro/project/repro.gd`): `PCKPacker` pck with one file, directory file count set to 2147483647 | unfixed build | still running after the 20 s timeout, 10/10 |
| Same, first directory entry path length set to 0xffffffff / 0x7fffffff | unfixed build | 0xffffffff: `load_resource_pack` returns true with an empty path (`sl + 1` wraps to 0, then `cs[sl] = 0` writes at index 0xffffffff of a 0-length buffer); 0x7fffffff: SIGSEGV |
| Fix verification | `patch.diff`, rebuilt | all three variants return false immediately ("Invalid file count / path length in pack directory"); a valid 2-file pck still loads and the file reads back `hello` |

## Status

**Confirmed.** Deterministic: unfixed build does not return within 20 s in every run (loop of ~2^31 iterations); fixed build returns in 0.5 s.

**Fully reproducible**: Yes - real `ProjectSettings.load_resource_pack()` on a pack written by `PCKPacker` with one field patched.

## Class

`logic` (denial of service: unbounded directory loop; plus unchecked 32-bit length arithmetic in the same loop)

## Root cause

`core/io/file_access_pack.cpp`, `PackedSourcePCK::try_open_pack()`:

```cpp
int file_count = f->get_32();
...
for (int i = 0; i < file_count; i++) {
    uint32_t sl = f->get_32();
    CharString cs;
    cs.resize_uninitialized(sl + 1);       // sl == 0xffffffff wraps to 0
    f->get_buffer((uint8_t *)cs.ptr(), sl);
    cs[sl] = 0;
```

`file_count` is never compared with the bytes left in the directory, so a tiny pack makes the loop run up to 2^31 times (each iteration past
the end of file reads zeros and calls `add_path()`); `sl` is not limited either: `sl + 1` wraps for 0xffffffff and huge values request multi-GB buffers.

## Introduced by

5dbf1809c6 A Whole New World (clang-format edition) (oldest commit `git log -S` finds for the loop)

## The fix

`patch.diff` (+4/-0): reject the pack when `file_count` is negative or larger than `remaining_directory_bytes / 40` (smallest possible entry:
4 + 8 + 8 + 16 + 4 bytes), and when a path length exceeds the remaining bytes. Applied to the main tree, rebuilt, verified, reverted.

Tests: fixed build `--headless --test`: 1431 cases, 1430 passed, 1 failed (`[IP] resolve_hostname`,
needs network in this sandbox, unrelated); no clean unfixed full-suite baseline (see 073). Repro on the fixed build: 10/10 clean (exit 0); on the unfixed build 10/10 hung.

## What's not done / open questions

`just test` not run from a fresh clone; `repro/run.sh` run against the unfixed and fixed binaries from the main tree. Encrypted directories
(the check uses the decrypting wrapper's length) were not exercised. Entries already added by the loop before a bad entry is found are kept
in `PackedData`. Other pack/zip loaders (`ZipArchive`, the Android/editor ones) were not audited for the same counts.
