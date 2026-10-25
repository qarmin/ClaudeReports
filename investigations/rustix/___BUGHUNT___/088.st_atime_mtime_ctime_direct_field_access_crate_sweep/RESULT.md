# st_atime_mtime_ctime_direct_field_access_crate_sweep

Follow-up on candidate 081's own "What's not done" note: it confirmed `src/fs/constants.rs`'s
`#[cfg(test)]` self-check assumes every `libc`-backend target exposes flat `st_atime`/`st_mtime`/
`st_ctime` integer fields, when wasi's vendored `libc::stat` nests them inside `timespec`
sub-structs instead - but explicitly left "a grep for `st_atime`/`st_mtime`/`st_ctime` across
`src/` beyond `constants.rs`" undone. This candidate did that grep, widened it to `tests/` (the
real integration-test directory, not just `#[cfg(test)]` unit tests), and found the same bug class
recurs in two integration test files that were never given the `target_os = "wasi"` guard their
sibling test files already carry.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `grep -rn '\.st_atime\b\|\.st_mtime\b\|\.st_ctime\b\|_nsec\b'` across `src/` | static analysis, no build | Every non-test hit is gated `linux_kernel` (both `libc`- and `linux_raw`-backend `stat64`/`statx` conversion helpers in `src/backend/{libc,linux_raw}/fs/syscalls.rs`, plus custom-struct field *definitions* in `src/backend/{libc,linux_raw}/fs/types.rs`) or `#[cfg(apple)]` (`fix_negative_stat_nsecs` in `src/backend/libc/fs/syscalls.rs:2688`) - none reachable on wasi. The only non-Linux/non-Apple-gated hit is the already-`Confirmed` candidate 081 instance in `src/fs/constants.rs`. **No new recurrence in `src/` shipped code or `src/`'s own test module.** |
| Same grep widened to `tests/` (rustix's integration-test harness, a separate binary per subdirectory, still "test-only" in the sense of not shipping to users, but a distinct, previously-unchecked location from candidate 081's `#[cfg(test)]` scope) | static analysis, no build | Found 4 files: `tests/fs/futimens.rs`, `tests/fs/negative_timestamp.rs`, `tests/fs/utimensat.rs`, `tests/fs/y2038.rs`. `futimens.rs`/`utimensat.rs` already have `#[cfg(not(any(target_os = "redox", target_os = "wasi")))]`-equivalent guards per test function. `negative_timestamp.rs` (1 function) and `y2038.rs` (3 functions) do **not** - a direct, real lead |
| Direct cross-target compile probe (no Miri - a real compile error, not runtime behavior; same technique candidate 081 used), `cargo test --no-run --target wasm32-wasip1 --features "std,fs"` (no `--lib` this time, so the real `tests/fs` integration binary is reached too), stable toolchain, 3 repeat runs before the fix + 3 after | real `wasm32-wasip1` target (`rustup target add`, prebuilt std, no `-Zbuild-std`) | **Reproduces 3/3 runs, byte-identical error set every time, both before and after.** Before: 46 errors specifically attributed (on the error's own `-->` line) to `tests/fs/negative_timestamp.rs` (16) and `tests/fs/y2038.rs` (30), all `E0609`/`E0026` "no field `st_atime`/`st_mtime`/`st_ctime`(`_nsec`) on type `libc::stat`" - plus the already-known, separate 23 lib-test errors in `src/fs/constants.rs` (candidate 081, unaffected by this fix) and a handful of other unrelated, out-of-scope wasi gaps (`readlinkat.rs`'s `OsStr::from_bytes`, `crate::ugid`, `crate::io::dup2`/`dup`). After the fix: the 46 errors in `negative_timestamp.rs`/`y2038.rs` are gone; every other pre-existing error (the 081 instance plus the unrelated gaps) remains unchanged, confirming the fix is scoped to exactly its own two files |

## Status

**Confirmed.** Reproduces 3/3 runs (fresh `git clone` each time) before the fix, 0/3 after - this is
a `rustc` compile error over fixed source code and a fixed `libc` crate version, not runtime
behavior, so it is inherently deterministic, not merely "observed 3 times."

**Fully reproducible**: **Yes**. Single-participant finding (rustix's own test code failing to
compile against the project's own real, unmodified, vendored `libc` dependency for a real Rust
target) - no adversarial peer/caller to simulate, so `CONFIRMATION.md` section 11's two-participant
bar doesn't apply (same category as candidate 081/039/004). `repro/check.sh` runs the real `cargo
test --no-run --target wasm32-wasip1 --features "std,fs"` command (not `--lib` - the real
integration-test binary, not just unit tests) against a real, freshly-cloned checkout of the
project at the exact commit this candidate was found on - nothing about the trigger path is
hand-constructed or mocked.

**Note on a build-system subtlety found while writing the standalone `repro/`**: without
`--keep-going` (which isn't even a valid `cargo test` flag - it's `cargo build`'s), `cargo test
--no-run`'s own job scheduler stops queuing new rustc invocations after the first target fails to
compile; which other independent test-binary targets actually get attempted in the same invocation
can vary with build-cache warmth and job-scheduling order. This only matters for observing the full
error set in one invocation - it does **not** affect the bug itself (verified the `tests/fs`
integration binary alone, via `cargo test --no-run --target wasm32-wasip1 --features "std,fs" --test
fs`, in a fully pristine clone, reaches and fails on these same two files independently of
whatever else is being built in parallel).

## Class

build-failure

## Root cause

Same mechanism as candidate 081: `src/backend/libc/fs/types.rs` defines `pub type Stat = c::stat;`
(a direct type alias) for every `libc`-backend target except `linux_like`/`hurd`/`netbsd` -
including wasi. The `libc` crate's own WASI binding (`libc-0.2.189`'s `src/wasi/mod.rs`) represents
timestamps as nested `st_atim`/`st_mtim`/`st_ctim: timespec` sub-structs (matching WASI's own
`__wasi_filestat_t` naming) rather than the flat `st_atime`/`st_atime_nsec` etc. integer fields
every other tested `libc`-backend target (Apple, BSDs, Solaris/illumos, Haiku, ...) exposes. Since
`Stat` is a direct alias for `c::stat` on wasi, naming `.st_atime`/`.st_mtime`/`.st_ctime` on a
`Stat`/`libc::stat` value is a compile-time error (`E0609`/`E0026`) on this target, not a runtime
bug.

`tests/fs/negative_timestamp.rs`'s `negative_file_timestamp` test and all three test functions in
`tests/fs/y2038.rs` (`test_y2038_with_utimensat`, `test_y2038_with_futimens`,
`test_y2038_with_futimens_and_stat`) call `rustix::fs::{stat, lstat, fstat, statat}` (none of which
are wasi-gated - they compile and, per the crate's own public API, are meant to run on wasi) and
then read the returned `Stat`'s `.st_mtime`/`.st_mtime_nsec`/`.st_atime`/`.st_atime_nsec` fields
directly by name - the exact same flat-field assumption as candidate 081's unit-test instance, just
in a different (integration-test) file. Unlike `futimens.rs`/`utimensat.rs` - whose own single test
function in each file already carries a `target_os = "wasi"` exclusion (added together with these
functions' very first implementation, commit 59ea6383, 2022-04-12, which predates wasi support in
the crate) - these two files were never given the equivalent guard, so they fail to compile under
`cargo test --target wasm32-wasip1` (or `wasm32-wasip2`/`wasm32-wasip3`) exactly like candidate
081's `src/fs/constants.rs` instance does, just reached by `--tests`/plain `cargo test` rather than
only `--lib`.

This only affects test code - no shipped, non-test code path is affected (same scope note as
candidate 081): the real `fstat`/`stat`/`lstat`/`statat` implementations operate on `Stat`/`c::stat`
through its real (nested, for wasi) field layout via raw pointer casts, never naming `st_atime`
directly outside test code. Practical impact is identical to 081's: this project's own CI
(`.github/workflows/main.yml`) only ever runs `cargo check` (not `--tests`, not `cargo test`) for
`wasm32-wasip1`/`wasm32-wasip2`, so this has never been exercised by CI either.

## Introduced by

Two separate commits (the bug has two independent origins, one per file):

- `tests/fs/negative_timestamp.rs`: 912db92919f62802ba01f8e64e4baae708115af2 "Work around a
  negative timestamps bug on macOS. (#1305)" (2025-02-01) - this commit created the file. By this
  date, wasi support already existed in the crate for over two years (wasi support added
  2022-11-28, commit aa08658b), and the exact same `target_os = "wasi"` guard this file needed
  already existed on its sibling test functions in `futimens.rs`/`utimensat.rs` (added 2022-04-12,
  commit 59ea6383) - so this is a straightforward omission at the time of writing, not something
  that only later became wrong.
- `tests/fs/y2038.rs`: a5afe5a384f8643e185eddf0540f13a7082be246 "Portability fixes for Redox,
  Fuchsia, WASI, and Windows." (2023-05-24) - the file itself predates wasi support (created
  2022-05-08, before wasi support existed in the crate at all, so no bug existed yet at creation).
  This later commit is titled, explicitly, as a portability fix for (among others) WASI, and in the
  same diff added a `target_os = "redox"` exclusion to this file's first two test functions - but
  did not add the equivalent `wasi` exclusion, despite the commit's own stated purpose. This is the
  best-effort "introducing" commit for this file: the one that should have added the guard (and did
  add the analogous one for Redox) but didn't.

Verified via `git log --follow -p`/`git log --diff-filter=A`/`git log -S'target_os = "wasi"'` on
both files and their siblings - not a `git bisect` (a cheap go/no-go check wasn't needed; the
history itself directly shows the guard being added to sibling files and omitted from these two).

## The fix

Added `#[cfg(not(target_os = "wasi"))]` as an additional attribute (matching the existing per-file
style of separate `#[cfg(not(...))]` lines rather than merging into one `any(...)`) to:

- `tests/fs/negative_timestamp.rs`: the one `#[test] fn negative_file_timestamp`.
- `tests/fs/y2038.rs`: all three `#[test] fn test_y2038_with_utimensat` /
  `test_y2038_with_futimens` / `test_y2038_with_futimens_and_stat`.

This is sufficient and safe for the same reason candidate 081's fix was: `Stat` is a direct type
alias for `c::stat` on wasi (not a separately-defined struct rustix controls the layout of), so
there is no loss of real test coverage for a renaming/offset check that was tautological to begin
with - skipping the whole test on wasi (rather than trying to special-case the field names) matches
the precedent already set by the untouched sibling files `futimens.rs`/`utimensat.rs`.

**Checked against more than the one triggering symptom (CONFIRMATION.md section 7)**: fixed all 4
affected test functions across both files (not just the first one hit), and re-ran `cargo test
--no-run --target wasm32-wasip1 --features "std,fs"` 3 times after the fix to confirm the specific
46-error signature (all `st_atime`/`st_mtime`/`st_ctime` field errors attributed to these two files)
is gone deterministically, not just absent on one lucky run, while every other pre-existing,
out-of-scope error (candidate 081's own `src/fs/constants.rs` instance, `readlinkat.rs`, `ugid`,
`dup2`) is unchanged.

- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`,
  host `x86_64-unknown-linux-gnu`): **411 passed, 1 failed, identical before and after** - the one
  failure is the pre-existing, environment-caused `ioctl::test_int_setter` (`/dev/net/tun` missing
  in this sandbox, documented in `projects/rustix.md`), not a regression from this patch. The patch
  only adds a `target_os = "wasi"` exclusion, which is a no-op on the host target
  (`x86_64-unknown-linux-gnu`), so this identical-before/after result is expected, not merely
  observed.
- `wasm32-wasip1` compile check (`cargo test --no-run --target wasm32-wasip1 --features "std,fs"`):
  the 46 `st_atime`/`st_mtime`/`st_ctime` errors attributed to `tests/fs/negative_timestamp.rs`/
  `tests/fs/y2038.rs` are gone after the fix, reproduced 3/3 runs each way (fresh clone each time).

## What's not done / open questions

- Per Critical pitfalls #7 (again): this sweep covered `src/` (full recursive grep, all non-test
  hits accounted for and all Linux-kernel/Apple-gated) and `tests/` (all 4 files that reference
  these field names by name). Not checked: `examples/` (grepped, zero hits, so nothing to report
  there) and whether any out-of-tree consumer of rustix's public `Stat` type does the same thing -
  out of scope for this crate's own repository.
- Not checked: `wasm32-wasip2`/`wasm32-wasip3`/`wasm32-wasip1-threads` specifically (same
  arbitrary-first-available-target choice as candidate 081) - the fix (`target_os = "wasi"`) covers
  all of them identically since `cfg(target_os = "wasi")` doesn't distinguish the wasip1/wasip2/wasi
  preview versions.
- The other, unrelated pre-existing wasi compile gaps surfaced incidentally by this candidate's own
  `cargo test --no-run` runs (`tests/fs/readlinkat.rs`'s `OsStr::from_bytes` not in scope without an
  explicit `use std::os::wasi::ffi::OsStrExt`, `tests/fs/file.rs`'s `chown`/`fchown` not found,
  `tests/io/dup.rs`'s private/not-found `dup`/`dup2`) are a **different** bug class (narrower
  `#[cfg]` gates on the *function being tested* than what the test code assumes, or a missing
  explicit trait import) - not fixed here, same "one bug per candidate" boundary candidate 081 drew
  around its own very similar `ugid`/`dup2` leftovers. Worth a follow-up candidate, now with two
  independent sessions (081, this one) pointing at the same unaddressed pile.
