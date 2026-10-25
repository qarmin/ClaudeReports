# `godot_unzip_get_current_file_info()` over-reads a too-small heap buffer when the first `unzGetCurrentFileInfo64()` call fails

Found by a mutational fuzz loop in GDScript (byte-mutated `ZIPPacker` output fed to `ZIPReader.get_files()`),
ASan editor build; seed 2, 1500 iterations.

## What was tried

| Approach/tool | Config | Outcome |
|---|---|---|
| Mutational fuzz of ZIP archives -> `ZIPReader.open/get_files/read_file` | ASan editor build `b8420aa3a6`, seed 2 | heap-buffer-overflow READ in `String::append_utf8` called from `zip_io.cpp:48` |
| Debug `fprintf` build of `zip_io.cpp` (reverted) | printed `err`, `size_filename` for both calls | first call: `err=-1, size_filename=2` (stale), second call `err=0, size_filename=92`, buffer size 2 |
| Minimal generated repro (`repro/project/repro.gd`) | 2-entry archive made by `ZIPPacker`, 2nd central-directory name length patched to 92 | same ASan report, 10/10 runs |
| Fix verification | `patch.diff`, rebuilt ASan editor | repro returns `files: ["a/", "a/"]`, no ASan report; 20000-char entry name (the long-path case) still reads back correctly |

## Status

**Confirmed.** Deterministic (no threads), 10/10 identical ASan reports before the fix, none after.

**Fully reproducible**: Yes - the real `ZIPReader.open()`/`get_files()` entry point reads an archive file;
the archive is produced by the engine's own `ZIPPacker` and then has one length field corrupted, as a
malformed/hostile archive would.

## Class

`memory-safety` (heap out-of-bounds read, up to 64K bytes past a small buffer; the over-read bytes end up in
the returned file name or crash the process)

## Root cause

`core/io/zip_io.cpp`:

```cpp
int err = unzGetCurrentFileInfo64(p_zip_file, &r_file_info, short_file_path_buffer, ...);
if (unlikely((err != UNZ_OK) || (r_file_info.size_filename > short_file_path_buffer_size))) {
    long_file_path_buffer.resize(r_file_info.size_filename);   // size taken from r_file_info
    err = unzGetCurrentFileInfo64(..., long_file_path_buffer.ptr(), long_file_path_buffer.size(), ...);
    ...
    r_filepath = String::utf8(long_file_path_buffer.ptr(), r_file_info.size_filename); // size re-read by 2nd call
```

minizip only copies its local file info into `*pfile_info` on success, so when the first call fails (here:
the central directory entry claims a name longer than the remaining file, so the name read comes up short)
`r_file_info.size_filename` is still whatever the caller's `unz_file_info64` held - an uninitialized stack
value in `ZIPReader::get_files()` (here the previous entry's `2`). The buffer is sized from that value. The
second call then re-parses the header, reads at most `buffer size` name bytes (succeeds), but overwrites
`r_file_info.size_filename` with the real, attacker-controlled value (92, or up to 65535), and
`String::utf8()` reads that many bytes from the tiny buffer. With a garbage/huge stale value the `resize()` can
also allocate an arbitrary amount. Same function is used by `godot_unzip_locate_file()` (`read_file`,
`file_exists`) and the zip `FileAccess`/pack loaders.

Not checked: whether other callers of `unzGetCurrentFileInfo64` elsewhere (thirdparty-using code) rely on
`pfile_info` being updated on failure.

## Introduced by

1326b7e04f Fix buffer over-read and memory leaks when using long filepaths in a zip archive and improved robustness of long filepaths and reading files.

## The fix

`patch.diff` (+4/-1): return the error from the first call immediately; only enter the long-path branch when
the first call succeeded and reported `size_filename > 16384`. Applied to the main tree, rebuilt, verified,
reverted.

Tests: fixed build `--headless --test`: 1431 cases, 1430 passed, 1 failed (`[IP] resolve_hostname`,
needs network in this sandbox, unrelated). No clean unfixed full-suite baseline exists (the baseline run
crashed earlier in an environment-dependent mbedtls `CryptoKey` test, see 073's RESULT.md).

## What's not done / open questions

`just test` was not run from a fresh clone (full ASan scons build per variant); `repro/run.sh` was run
against the unfixed and fixed binaries built from the main tree.
