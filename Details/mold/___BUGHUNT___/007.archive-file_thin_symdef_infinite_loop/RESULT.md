# archive-file_thin_symdef_infinite_loop

Noticed by code analysis while fixing candidate 005 (both live in the same function's loop body):
`read_thin_archive_members()`'s (`src/archive-file.cc`) `continue` for a member whose resolved
name is exactly `"__.SYMDEF"` or `"__.SYMDEF SORTED"` fires *before* the loop's only `data`
cursor advancement (`data = body;`, at the very end of the loop body, on the code path that
`continue` skips). If a thin archive's long filename resolves to one of those two strings, the
cursor never moves and the loop re-parses the identical header forever.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction | Crafted a `!<thin>\n` archive with a `// ` string-table member containing `__.SYMDEF/\n`, then one member using SysV long name `/0` (offset 0) which resolves to exactly `"__.SYMDEF"` | `timeout 5 mold ... -> exit 124` (killed by timeout) - confirmed hang, not a slow-but-terminating run |
| Process inspection while hanging | `ps -o pid,pcpu,stat` on the running process 2s in | `100% CPU`, state `R` (running/runnable) - confirms an actual busy-spin infinite loop, not a blocked/sleeping wait on I/O or a lock |
| Repeat runs | 3x `timeout 3 mold ...` | 3/3 killed by timeout (exit 124) - deterministic |
| Control input | The project's own `test/thin-archive.sh` (a normal, well-formed thin archive with a long-filename member that does NOT resolve to `__.SYMDEF`) | Passes/exits normally both before and after the fix (part of the full `ctest` run below) - confirms the hang is specific to the `__.SYMDEF` name match, not thin archives in general |

## Status

**Confirmed** - 3/3 runs hang (killed by `timeout`, 100% CPU observed directly via `ps` during one
of the hangs). This is a deterministic infinite loop, not a flaky/rare condition - the loop has no
source of nondeterminism (no I/O, no timing dependency) once the input matches the trigger
condition.

## Fully reproducible

Yes. `repro/run.sh` builds the malformed thin archive with `repro/gen_evil_ar.py` and runs the
real `mold` binary via its real CLI entry point end to end, using `timeout` to detect the hang
(exit code 124) rather than waiting indefinitely.

## Class

security (denial of service via an unbounded/never-terminating loop - CPU-exhaustion hang, not a
memory-safety crash)

## Root cause

`read_thin_archive_members()`'s per-member loop only advances its `data` cursor past the current
member in two places: (1) the `is_strtab()`/`is_symtab()` branches (`data = body + size; continue;`)
and (2) at the very end of the loop body (`data = body;`) for a member that gets pushed into the
result vector. The `__.SYMDEF`/`__.SYMDEF SORTED` name check sits *between* `read_name()` and that
final `data = body;` line, and its own `continue` skips straight back to the top of the loop
without ever reaching it. Since `hdr`/`body`/`size` are recomputed from the same, unchanged `data`
on the next iteration, the loop observes the identical header again, takes the identical branch
again, and spins forever. This requires the member's *resolved* long filename (via the `#1/` or
`/` extended-name convention - a short name can't reach this check at all, since it would already
have hit the earlier `Fatal()` for "filename is not stored as a long filename") to equal one of
those two exact strings - an unusual but entirely attacker-constructible archive, not something a
normal `ar`-produced thin archive would ever contain (BSD `__.SYMDEF` is a *fat*-archive symbol
table convention; thin archives don't carry a `__.SYMDEF` member from any real tool).

## Introduced by

8e709d2b6 "Split mold.h into multiple headers" (2022-01-14) - same commit as candidates 003/006,
this file's oldest attributable point for its current structure per `git blame`.

## The fix

`patch.diff`: moved the `data = body;` cursor advancement from the end of the loop body to
immediately after `read_name()` returns (before the `__.SYMDEF` name check), removing the now-
redundant copy at the end. `body` already reflects the correct next-member position at that point
(read_name's BSD branch already advances `body` past an inline filename via its `u8 *&ptr`
reference parameter before returning), so this doesn't change behavior for any input that doesn't
hit the SYMDEF-skip path - it only guarantees every iteration makes forward progress regardless of
which branch is taken afterward.

Verified: `repro/run.sh` no longer hangs against the patched build - `mold` exits immediately
(status 1, `no input files`, since the crafted archive's only member is legitimately skipped).

- Project test suite (`ctest -j5` in `build-asan/`): 475/476 passed before and after the patch
  (identical single pre-existing, patch-unrelated failure both times - `x86_64-run-clang`, see
  candidates 001/003/005/006's own notes). In particular `x86_64-thin-archive` (a real thin-archive
  test, the same one candidate 005's first fix attempt broke) passed both before and after this
  patch - this fix doesn't touch the size-validation logic candidate 005 already covers.

## What's not done / open questions

- Only the thin-archive path was checked - `read_fat_archive_members()` has the identical
  `__.SYMDEF` name check (`src/archive-file.cc:157` in current `HEAD`), but that function already
  unconditionally advances `data = body + size;` *before* reaching any of its `continue` branches
  (including this one), so it cannot exhibit the same no-progress hang - confirmed by reading the
  code, not separately fuzzed/re-verified with its own repro this session.
- Did not check whether an *empty* resolved name (`atoi()` parsing garbage in a corrupted
  long-name field to `0`, or similar) could trigger a related no-progress condition through some
  other branch - this candidate is scoped to the exact `__.SYMDEF` match that was already
  identified and confirmed.
