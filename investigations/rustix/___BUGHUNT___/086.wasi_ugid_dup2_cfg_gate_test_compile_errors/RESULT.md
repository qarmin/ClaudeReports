# wasi_ugid_dup2_cfg_gate_test_compile_errors

Follow-up to candidate 081, which fixed the `st_atime`/`st_mtime`/`st_ctime` field-name mismatch
for `cargo test --no-run --target wasm32-wasip1` but explicitly left two other, unrelated compile
errors unfixed and flagged them as "a different bug class ... worth a follow-up candidate":
`cannot find \`ugid\` in the crate root` (`src/fs/constants.rs`) and `cannot find function
\`dup2\`` (`src/backend/libc/fs/dir.rs:496`). Both are `#[cfg(test)]`-only code referencing an item
that is unconditionally `#[cfg]`'d off for `target_os = "wasi"` elsewhere in the crate.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct cross-target compile check (no Miri needed - same technique as candidate 081's own 4th approach), reading the two flagged error messages back to their definition sites | `cargo test --no-run --target wasm32-wasip1 --features "std,fs" --lib`, stable toolchain (`rustup target add wasm32-wasip1`, prebuilt std) | **Reproduces deterministically** (a compile error over fixed source + a fixed `libc` crate version, not runtime behavior - repeated 3+ times, byte-identical error set every time): 24 total errors, of which 2 are `error[E0433]: cannot find \`ugid\` in \`crate\`` (`src/fs/constants.rs:100` and `:101`, referencing `crate::ugid::RawUid`/`RawGid`) and 1 is `error[E0425]: cannot find function \`dup2\` in module \`crate::io\`` (`src/backend/libc/fs/dir.rs:496`, in test `dir_iterator_handles_io_errors`) - rustc's own "found an item that was configured out" note points directly at the gating `#[cfg]` in each case, confirming the mechanism without further tooling needed. The remaining 21 errors are candidate 081's already-confirmed, separately-tracked `st_atime`/`st_mtime`/`st_ctime` bug |
| `git log -S`/`git log --diff-filter=A` on the two affected lines/files, to find the introducing commit for each half independently | `git log --oneline -S "crate::ugid::RawUid" -- src/fs/constants.rs`; `git log --oneline -S "dir_iterator_handles_io_errors" -- src/backend/libc/fs/dir.rs`; `git log --diff-filter=A -- src/io/dup.rs` | Found both introducing commits (see "Introduced by" below) - no ambiguity, single hit each |

A 3rd escalating approach (e.g. a sanitizer or fuzzer) was not needed - this is a `rustc`
compile-time error over fixed, deterministic inputs (source + a pinned `libc` crate version), not
a runtime/input-dependent bug, so the first approach already meets `CONFIRMATION.md`'s
repeatability bar on its own.

## Status

**Confirmed.** Reproduces 3/3+ runs, byte-identical every time (inherently deterministic, same
reasoning as candidate 081's identical bug class). Verified both before and after the fix via a
real, fresh `git clone` + `just test` run (see `output.txt`), not just against the working copy
used to investigate it.

**Fully reproducible**: **Yes**. Same category as candidate 081 (and this workspace's own
candidates 004/039): a single-participant finding - rustix's own `#[cfg(test)]` code failing to
compile against the project's own real, unmodified, vendored `libc` dependency for a real Rust
target. There is no adversarial peer/caller to simulate, so `CONFIRMATION.md` section 11's
two-participant bar doesn't apply. `repro/check.sh` runs the real, unmodified `cargo test --no-run
--target wasm32-wasip1` command against a real, freshly-cloned checkout of the project at the exact
commit this candidate was found on - nothing about the trigger path is hand-constructed or mocked.

## Class

build-failure

## Root cause

Two independent instances of the same underlying pattern: `#[cfg(test)]` code in one file
references an item that a `#[cfg]` attribute elsewhere in the crate makes unconditionally
unavailable for `target_os = "wasi"`, and nothing accounted for that when the test code was
written (or, for the `ugid` case, when the test code was later rewritten).

1. **`ugid`**: `src/lib.rs:411` gates `mod ugid;` with `#[cfg(not(any(windows, target_os =
   "wasi")))]` (plus feature flags) - this gate has existed since 2023-05-24 (commit 2b7bca79,
   "Add a \`pipe\` module."), long before the offending test lines existed. `src/fs/constants.rs`'s
   `#[cfg(test)] fn test_layouts()` asserts `some_stat.st_uid`/`st_gid` against
   `crate::ugid::RawUid`/`RawGid` with no corresponding `#[cfg]` guard, so referencing
   `crate::ugid` at all fails to resolve under `--target wasm32-wasip1` (`rustc`'s own "found an
   item that was configured out" note confirms this directly, pointing at the `mod ugid;` gate).
2. **`dup2`**: `src/io/dup.rs` gates `pub fn dup2` with `#[cfg(not(target_os = "wasi"))]` - this
   gate has existed since the file itself was created in 2021-11-02 (commit cfc08c0e, "Split
   fd.rs into multiple files."), long before the offending test existed.
   `src/backend/libc/fs/dir.rs`'s `#[cfg(test)] fn dir_iterator_handles_io_errors()` calls
   `crate::io::dup2(&file_fd, &mut owned_fd)` unconditionally to swap the directory's open fd for
   a file's fd (simulating the directory being deleted/replaced out from under an open `Dir`
   iterator), with no corresponding `#[cfg]` guard.

In both cases the mismatch is purely in test-only code - no shipped, non-test code path is
affected (the crate's real public API already correctly omits `ugid`/`dup2` for wasi; nothing in
non-test code calls either of them unconditionally). Practical impact is the same as candidate
081: `cargo test --target wasm32-wasip1` has never been able to compile for this crate, which is
why this has gone unnoticed - the project's own CI only ever runs `cargo check` (not `--tests`) for
wasi targets.

## Introduced by

Two separate commits, one per error (both are the commit that added the offending test code,
*after* the referenced item was already wasi-gated elsewhere - verified via `git log -S`/
`git log --diff-filter=A`, not `git bisect`, since the single hit in each case already pins it down
unambiguously):

- `ugid`: 912db929 "Work around a negative timestamps bug on macOS. (#1305)" (2025-02-01) - this
  commit's "More stat layout checks." portion added the
  `assert_eq!(some_stat.st_uid, 0 as crate::ugid::RawUid)` / `st_gid` lines in their present form
  (`git log -S "crate::ugid::RawUid" -- src/fs/constants.rs`, single hit). `mod ugid`'s wasi gate
  in `lib.rs` already existed since 2023-05-24, ~20 months earlier - consistent with candidate
  081's own finding that this same reorganization-era commit cluster (sibling commit 7202da6e,
  two days earlier) introduced that candidate's bug too.
- `dup2`: 31fd98ca "Merge pull request from GHSA-c827-hfw6-qwvm" (2023-10-12) - this is the commit
  that introduced `dir_iterator_handles_io_errors` itself (`git log -S
  "dir_iterator_handles_io_errors" -- src/backend/libc/fs/dir.rs`, single hit), including its
  `crate::io::dup2` call. `dup2`'s wasi gate in `src/io/dup.rs` already existed since the file was
  created on 2021-11-02 (`git log --diff-filter=A -- src/io/dup.rs`), about 2 years earlier.

## The fix

Gated both offending spots behind `#[cfg(not(target_os = "wasi"))]`, mirroring the style candidate
081 already used for the sibling `st_atime` bug and the existing `#[cfg(not(target_os = "wasi"))]`
precedent already used elsewhere in both of these same files (e.g. `FileType::is_fifo`/
`is_socket` in `constants.rs`, several `use` gates in `dir.rs`):

- `src/fs/constants.rs`: wrapped the `st_uid`/`st_gid` assertions in a block gated
  `#[cfg(not(target_os = "wasi"))]`. Safe because this only skips a types-match assertion for an
  item (`crate::ugid`) that is itself entirely absent on this target - there is nothing left to
  assert about.
- `src/backend/libc/fs/dir.rs`: added `#[cfg(not(target_os = "wasi"))]` directly on the
  `dir_iterator_handles_io_errors` test function. Safe for the same reason - the test exists
  specifically to exercise `dup2`-based fd substitution, which has no wasi equivalent available to
  substitute; there is no safe/weaker variant of this test to keep running on wasi.

**Checked against more than the one triggering symptom (CONFIRMATION.md section 7)**: fixed both
of the two distinct code locations (not just the first one hit), and re-ran
`cargo test --no-run --target wasm32-wasip1 --features "std,fs" --lib` 3+ times after the fix to
confirm both specific error signatures (`ugid`, `dup2`) are gone deterministically, not just
absent on one lucky run; also re-ran the "before" state in between to confirm it's still present
there (ruling out a stale-binary false negative).

- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`,
  host `x86_64-unknown-linux-gnu`): **411 passed, 1 failed, identical before and after** - the one
  failure is the pre-existing, environment-caused `ioctl::test_int_setter` (`/dev/net/tun` missing
  in this sandbox, already documented in `projects/rustix.md` and in candidate 081's own
  `RESULT.md`), not a regression from this patch.
- `wasm32-wasip1` compile check (`cargo test --no-run --target wasm32-wasip1 --features "std,fs"
  --lib`): 24 errors before (2 `ugid` + 1 `dup2` + 21 candidate-081-owned `st_atime`/`st_mtime`/
  `st_ctime` errors) -> 21 errors after (only the candidate-081-owned errors remain, since that
  candidate's own fix is not applied in this session's patch - see "What's not done" below),
  reproduced 3+/3+ runs each way, including via a full fresh `git clone` + `just test` run (see
  `output.txt`).

## What's not done / open questions

- This candidate's `patch.diff` does **not** include candidate 081's own fix - the 21 remaining
  errors after this patch are exactly candidate 081's already-`Confirmed`, already-patched bug,
  deliberately left untouched here per this workspace's one-bug-per-candidate convention. Applying
  both candidates' patches together is required to get `cargo test --no-run --target
  wasm32-wasip1 --features "std,fs" --lib` to compile cleanly for wasi.
- Not checked: whether `wasm32-wasip2`/`wasm32-wasip3`/`wasm32-wasip1-threads` have the identical
  two errors or something additionally different - only `wasm32-wasip1` was used for this
  candidate's repro (same target candidate 081 used, for direct comparability).
- Per "Critical pitfalls" #7 (and candidate 081's own identical caveat): the same "test code
  references an item that's `#[cfg]`'d off for a target the test wasn't written with in mind"
  class could in principle recur elsewhere in the crate for wasi or any other narrowly-gated
  target; a full `grep` of every `#[cfg(test)]` block against every narrowly-scoped `#[cfg]` gate
  in the crate was not done this session - only the two spots candidate 081 had already flagged
  were investigated.
