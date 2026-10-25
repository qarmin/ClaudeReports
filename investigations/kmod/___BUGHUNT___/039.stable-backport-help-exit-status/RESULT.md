# stable-backport-help-exit-status: `scripts/stable-backport.sh --help` exits with status 1 and prints the usage to stderr

`usage()` always logs to stderr and calls `exit 1`, and `-h|--help` calls the same function. A successful help request therefore fails (`./scripts/stable-backport.sh --help && echo ok` prints nothing, `set -e` callers abort) and the text cannot be piped to a pager.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `bash scripts/stable-backport.sh --help` (`repro/run.sh`) | HEAD, 5 runs | 5/5: exit status 1, nothing on stdout, usage on stderr |
| Patched | patch.diff, 5 runs | 5/5: exit status 0, usage on stdout; no arguments and unknown option still exit 1 with the usage on stderr |
| Read the rest of the script for logic errors (candidate selection, fix-of-fix chaining, patch-id handling, starting point lookup) | code reading | no defect found beyond the exit status; not exercised against a large history |

## Status

**Confirmed** - deterministic, 5/5 on HEAD (`output.txt`).

**Fully reproducible**: **Yes** - the real script.

## Class

logic

## Root cause

`usage()` hardcodes `exit 1` and stderr for both the error and the help-request callers.

## Introduced by

874a08f "scripts: Add stable-backport.sh"

## The fix

`patch.diff` (+5/-3, scripts/stable-backport.sh): `usage [status]` prints to stdout and exits 0 for `--help`, stderr and 1 otherwise. Verified with the repro (5/5 exit 1 -> 5/5 exit 0) and by hand for the no-argument and unknown-option paths. The project's bats suite (`scripts/test/stable-backport.bats`, 15 tests) was not run: `bats` is not installed in this environment, and the suite has no `--help` test. The meson test suite is unaffected (script only).

## What's not done / open questions

- Whether `--help` output should go to stdout is a convention choice; the exit status is the verified defect.
