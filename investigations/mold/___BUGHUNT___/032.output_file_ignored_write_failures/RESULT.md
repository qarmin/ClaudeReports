# 032. output_file_ignored_write_failures

`MallocOutputFile<E>::close()` (`src/mold.h:1701-1725` at c340272e) writes the entire linked image
with `fwrite(this->buf, this->filesize, 1, fp); fclose(fp);` and checks neither result. Flagged by
the 2026-08-13 `code-audit-structured` pass as the last unclaimed high/easy finding; it is also the
first candidate in this project on the *output* side rather than the input-parsing side.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `src/mold.h` + `src/output-file-unix.cc`, establish which subclass is selected when | source reading, c340272e | `OutputFile<E>::open()` picks `MallocOutputFile` iff `path == "-"` **or** `std::filesystem::is_regular_file(path)` is false *without* an error (i.e. an already-existing non-regular file: device, FIFO, socket). Everything else - including a not-yet-existing path, where `is_regular_file` sets `ENOENT` - goes to `MemoryMappedOutputFile`. No caller of `close()` (`main.cc:750`, `relocatable.cc:180`, `passes.cc:3633`) inspects anything afterwards, and there is no later `ferror`/`fflush`/exit-status check anywhere in the chain. |
| `mold -o /dev/full` (ENOSPC on every write), plain Debug build | `build-plain`, 5/5 runs | mold exits **0**, prints nothing, writes nothing. GNU ld 2.46 on the identical link: `final link failed: No space left on device`, exit 1. |
| `RLIMIT_FSIZE` + `SIGXFSZ` ignored, `mold -o -` with stdout redirected to a regular file | `build-plain`, limit 1024 B, 5/5 runs | mold exits **0**, output file is 1024 B instead of 3360 B - a silently truncated ELF. |
| Same, but through the real `gcc -B<mold> -o -` driver on a full glibc-linked PIE | `build-plain`, limit 4096 B, output 8280 B | mold and `collect2` both exit **0**; the 4096-byte artifact is already recognised as corrupt by `file` ("missing section headers at 8088"). `gcc -fuse-ld=bfd` under the identical limit: exit 1, nothing written. |
| gdb breakpoint on `mold::MallocOutputFile<mold::X86_64>::close`, `follow-fork-mode child` | `build-plain` | Confirms the `/dev/full` case really does reach `MallocOutputFile::close` (backtrace through `mold_main` at `main.cc:750`), rather than some other path. |
| Control: `RLIMIT_FSIZE` on a **regular** output file (the `MemoryMappedOutputFile` path) | `build-plain`, limit 1024 B | mold correctly reports `fatal: ftruncate failed: File too large`, exit 1 - the mmap path's `ftruncate`/`mmap` calls *are* checked, so the silent-failure shape is specific to `MallocOutputFile`. |
| FIFO output with a reader that stops reading early | `build-plain` | Not a useful trigger: the whole 3360-byte image fits in the 64 KiB pipe buffer, so the write genuinely succeeds and no error is lost. Dropped from the repro. |
| Audit of the mmap sibling paths for the same shape | source reading | `MemoryMappedOutputFile::close()` and `LockingOutputFile::close()` both call `::close(this->fd)` unchecked and never `msync`. Real but not locally reproducible (a `close()` error after a `MAP_SHARED` write needs a network/quota filesystem); fixed defensively in the same patch, see below. |

## Status

**Confirmed** - 5/5 runs of `repro/run.sh` against the pristine Debug build print
`VERDICT: BUG PRESENT`, and each individual scenario was additionally run 5/5 on its own. Fully
deterministic: no timing, layout or seed dependence. Debug build (`-DCMAKE_BUILD_TYPE=Debug`,
gcc); this is a plain missing-error-check, so no sanitizer is involved and a Release build behaves
identically (nothing about the mechanism depends on optimisation level).

**Fully reproducible**: **Yes**. The repro runs the real `mold` binary end-to-end (and, in
`output.txt` section 4, the real `gcc` driver using mold as its linker), on a real object file
produced by a real compiler, writing to a real kernel-provided failing sink (`/dev/full`) or under
a real `RLIMIT_FSIZE`. Nothing is hand-constructed or faked - the "second participant" here is the
kernel's write path, which is genuinely exercised. GNU ld is run on the identical inputs as the
reference behaviour.

## Class

logic (error handling / silent data loss)

## Root cause

`MallocOutputFile<E>::close()` is the only place the malloc-backed output image ever reaches the
filesystem, and it discards both error signals available to it:

```cpp
fwrite(this->buf, this->filesize, 1, fp);
fclose(fp);
```

`fwrite` returns the number of *complete items* written (0 on a short or failed write, since
`nmemb == 1`), and `fclose` returns `EOF` when the final flush fails - both are dropped. `close()`
returns `void`, its three call sites ignore it, and `mold_main` goes on to `exit(0)`. So any
`ENOSPC`/`EFBIG`/`EDQUOT`/`EIO` while writing the output produces a truncated or entirely missing
binary together with a success exit status: a build system sees a successful link and ships or
links against the corrupt artifact.

Which of the two dropped results *would* have caught the failure depends on the output size versus
stdio's buffer (verified with temporarily instrumented branch markers, `output.txt` section 5):
an image smaller than the stdio buffer is fully absorbed by `fwrite`, which then returns success,
and the error only appears from `fclose`'s flush. Checking `fwrite` alone would therefore have left
every small-output case silent - both checks are needed.

Reachability: `MallocOutputFile` is chosen for `-o -` and for any existing non-regular output path
(`/dev/full`, `/dev/null`, a device, a FIFO). It is not the common `-o some/file.exe` path - that
one uses `MemoryMappedOutputFile`, whose `ftruncate`/`mmap` failures *are* checked - so this is a
narrower configuration than "any full disk". `-o -` piping a linked image into another tool, and
`-o /dev/null`-style link-only checks, are the realistic ways in.

## Introduced by

cd68e02d "temporary" (2021-01-09) - the commit that created `MallocOutputFile` in the then-new
`output_file.cc`, already with the exact unchecked

```cpp
FILE *fp = fdopen(fd, "w");
fwrite(buf, filesize, 1, fp);
fclose(fp);
```

The two lines have been carried through every later rename/refactor (`1a1a5587`, `0692db34`,
`5b859497`, `2e326cc0`) unchanged, so the defect has been present for the class's entire lifetime.

## The fix

`patch.diff` (against c340272e):

- `src/mold.h`, `MallocOutputFile<E>::close()`: check `fwrite`'s item count (guarded by
  `this->filesize` so a legitimate zero-byte output isn't misread as a failure), saving `errno_string()`
  *before* the `fclose` that would clobber `errno`; then check `fclose(fp) == EOF`. Both report via
  the codebase's `Fatal(ctx) << ...` idiom and name both the output path and the underlying reason.
  Also adds the missing null check on `fdopen`, which sits in the same block and would otherwise
  hand a null `FILE *` straight to `fwrite`.
- `src/output-file-unix.cc`: check `::close(this->fd)` in `MemoryMappedOutputFile::close()` and
  `LockingOutputFile::close()` - the same ignored-I/O-failure shape in the sibling mmap paths.
  Defensive only: not independently reproduced (see "What's not done").

After the patch:

- `mold -o /dev/full ...` -> `mold: fatal: cannot write to /dev/full: No space left on device`, exit 1
  (matching GNU ld's `final link failed: No space left on device`, exit 1).
- `mold -o -` under `RLIMIT_FSIZE` -> `mold: fatal: cannot write to -: File too large`, exit 1.
- Through the `gcc` driver -> mold exits 1 and `collect2` reports `ld returned 1 exit status`.
- `repro/run.sh` flips from `VERDICT: BUG PRESENT` to `VERDICT: BUG ABSENT`.

Checked against more than the one triggering input (CONFIRMATION section 7): three distinct failure
injections (ENOSPC on a character device; EFBIG via `RLIMIT_FSIZE` on a 3360-byte output that fails
only at `fclose`; EFBIG on a 203480-byte output that fails at `fwrite`), plus the control case on the
mmap path, plus normal-link sanity (`mold -o file`, `mold -o -`, and running the resulting binary -
prints `hello`).

- Project test suite: `ctest --test-dir build-plain -j4`, **476/476 passed before, 476/476 passed
  after** (16 tests skipped by ctest's own feature gating in both runs - apx, ifunc-export,
  incompatible-libs, sframe-unwind, z-rewrite-endbr3, zstd, compressed-debug-info,
  dead-debug-sections, hwasan). No new failures. The "before" run was verified to be against a
  genuinely pristine binary (mold built 15:26:57, first source edit 15:30:29).

## What's not done / open questions

- The two `::close()` checks added to the mmap paths are code-reading-only. Making `close()` on a
  `MAP_SHARED` mapping actually fail needs a network/quota filesystem (NFS `EDQUOT`/`ENOSPC` at
  close time), which wasn't available here. A maintainer may prefer to drop those two hunks and
  take only the `MallocOutputFile` fix.
- Neither mmap path calls `msync` before `close`, so a genuine writeback error can still be lost
  there entirely (the kernel reports it to whoever calls `fsync`/`close` *first*, and mold calls
  neither meaningfully). Out of scope for this candidate but the same bug class; adding an
  `fsync` would have a performance cost the maintainers should weigh.
- Three more ignored-write-failure sites of the same class exist and were not touched:
  `lib/tar.cc:96/99/109/112` (four unchecked `fwrite`s, used by `--repro`), `src/mapfile.cc`
  (`--print-map` `std::ofstream`, checked only at `open`, never after writing or at close), and
  `src/passes.cc::write_dependency_file` (`--dependency-file`, same `std::ofstream` shape). Each
  would silently produce a truncated auxiliary file on a full disk.
- The patch leaves the partially-written bytes on disk (mold has nothing to unlink for `-o -`);
  GNU ld leaves a zero-length file. Only the exit status was aligned, not the cleanup behaviour.
- Not checked: the Windows path (`src/output-file-win32.cc`) selects `MallocOutputFile` under its
  own conditions and shares the same `close()` body, so the fix covers it, but nothing was built or
  run on Windows.
