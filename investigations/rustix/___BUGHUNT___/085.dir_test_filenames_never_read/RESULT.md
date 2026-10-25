# dir_test_filenames_never_read

`tests/fs/dir.rs:83` (`test_dir_seek`) - `cargo clippy ... -W clippy::nursery` flags
`clippy::collection_is_never_read` on `let mut filenames = Vec::<String>::with_capacity(count)`:
the test creates 500 real files, pushes each generated filename into `filenames`, then never reads
`filenames` again. The test's only actual assertion (`assert_eq!(entries, entries2)`) compares two
listings of the same directory, read through `rustix::fs::Dir`'s real `read()`/`seek()`, against
*each other* - never against the real files that were created. A bug in entry/name parsing that
corrupts every parsed name the same way on every read (consistent, deterministic corruption) would
be completely invisible to this test, because both `entries` and `entries2` would be corrupted
identically and still compare equal.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct source read of `tests/fs/dir.rs:60-150` and `cargo clippy --all-targets --features all-apis,try_close,use-explicitly-provided-auxv,use-libc-auxv,linux_latest -- -W clippy::nursery` | confirms the lint and the missing read of `filenames` | Confirmed: `filenames` built (500 entries) and never read anywhere in the function body. |
| Mutation testing: injected a deterministic one-byte-per-name corruption (last byte before the NUL replaced with `X`) into `src/backend/linux_raw/fs/dir.rs`'s dirent-name parsing (the real, actual `Dir::read()` entry-name extraction code), built debug profile, ran the real, currently-shipping `test_dir_seek` via `cargo test --features all-apis --test fs dir::test_dir_seek` | debug build, default/linux_raw backend, mutant applied directly to the main project's working tree (reverted before writing this file) | **The shipping test still passed** (`test dir::test_dir_seek ... ok`) despite every parsed directory-entry name being wrong (e.g. `...-427.txt` read back as `...-427.txX`) - confirms the blind spot is real, not theoretical. Repeated 3/3 runs, identical result every time. Three other, unrelated tests in the same file (`test_dir_new`, `test_dir_read_from`, `readdir::dir_entries`) did fail under the same mutation (they check `.`/`..`/`Cargo.toml` by exact name or count), confirming the mutant is a real, detectable corruption in general - `test_dir_seek` specifically is the one that misses it. |
| Added the missing assertion (comparing `entries`/`entries2` against the real `filenames` set, the fix below) and re-ran against the same mutant | same debug build | The strengthened test now panics: `unexpected directory entry name: "...-427.txt" (not among the real files created)` - 3/3 runs, deterministic. |

No need to escalate to miri/ASAN/a fuzz harness: this is a test-coverage gap (a missing
assertion), not memory-unsafety or arithmetic UB, and the direct source read plus a single
mutation-testing round already gave a clear, deterministic, repeatable signal on the first
approach tried.

## Status

**Confirmed.** Reproduces 3/3 runs (debug build, default `linux_raw` backend, deterministic - no
heap-layout/seed dependency: the mutation and its effect on parsed names are a pure function of
the injected code change, not of memory layout, ASLR, or a hash-map iteration order).

**Fully reproducible**: **Yes**. The repro (see `repro/`) drives the real, public
`rustix::fs::Dir::read_from`/`read`/`seek` API against a real temp directory containing real files
created by the repro itself - there is no second participant anywhere in this finding's trigger
path (no peer, no caller, no wire) to fake: the "bug class" being demonstrated (a name-parsing bug
that corrupts entries consistently) is injected directly into the one real implementation under
test via `repro/mutant.patch`, applied to a pristine clone, not hand-constructed as a stand-in for
some other participant's behavior. `repro/justfile`'s `mutate-again`/`reproduce-after-mutant`
steps additionally re-run the exact same mutation against the **real project test suite** (not
just the standalone repro binaries) with the real fix applied, confirming the fix closes the gap
in the actual shipping test, not only in a parallel reimplementation of its logic.

## Class

logic (test-coverage gap - a missing assertion, not a runtime crash/UB/security issue in
production code; the production `Dir::read()`/`seek()` implementation itself was not found to be
buggy, only the test's ability to detect a plausible bug in it)

## Root cause

`test_dir_seek` (added together with `Dir::seek()` itself) builds `filenames: Vec<String>` by
recording every filename it creates in a temp directory, but the function's only correctness
check afterward is `assert_eq!(entries, entries2)` - two listings of the same (post-seek) portion
of the directory, both produced by calling `dir.read()`, compared only against each other. Neither
`entries` nor `entries2` is ever checked against `filenames`. Since both listings are produced by
the same (potentially buggy) parsing code reading the same underlying kernel bytes, any bug in
name extraction that behaves deterministically (not layout/timing-dependent) produces identically
wrong output on both reads, which still satisfies `entries == entries2`. This was demonstrated
directly: corrupting `Dir::read()`'s name parsing in a fully deterministic way left
`test_dir_seek` green while a strengthened version of the same test (comparing against
`filenames`, the ground truth) caught it immediately.

## Introduced by

75ba0beb "Add support for seekdir(). (#1231)" (2025-01-08) - this commit added both
`Dir::seek()`/`rewind()`-related support and `test_dir_seek` itself (the first and only commit to
add this test function; confirmed via `git log -S'fn test_dir_seek' --oneline -- tests/fs/dir.rs`
returning exactly this one commit). The `filenames` vector and the gap have been present,
unchanged, since the test's introduction - no later commit touching `tests/fs/dir.rs` altered this
function's assertions.

## The fix

Added the missing assertion to `test_dir_seek`: after the existing `assert_eq!(entries,
entries2)`, build a `HashSet` from `filenames` and assert that every name actually read back (in
either `entries` or `entries2`) is either `.`/`..` or a member of that set. This directly closes
the demonstrated gap without changing anything else about the test's existing behavior or timing.

Checked against more than the one originally-tried mutation shape, per `CONFIRMATION.md` section
7: the fix was verified both against the standalone `repro/strong_check` binary (built from a
parallel reimplementation of the same real public API calls) **and** against the real project's
own `tests/fs/dir.rs` with `patch.diff` actually applied and the same mutant re-injected
(`repro/justfile`'s `mutate-again`/`reproduce-after-mutant`) - both independently confirm the fix
catches the same corruption.

- Project test suite (`cargo test --features all-apis --no-fail-fast`, run in the isolated
  worktree used for this candidate): **before fix**: 413 passed, 1 failed
  (`ioctl::test_int_setter` - pre-existing, unrelated: fails identically with or without this
  patch, `ENOENT`/"No such file or directory" from a missing device/resource in this sandbox, not
  touched by this change). **After fix**: identical - 413 passed, 1 failed (same
  `ioctl::test_int_setter`). No new failures introduced, no previously-failing test newly passes
  (expected - this patch only strengthens one test's own assertions, it does not touch any
  production code path).

## What's not done / open questions

- This fix only strengthens `test_dir_seek` on platforms where it already runs (`linux_like,
  solarish, fuchsia, hermit, openbsd, redox`, i.e. wherever `Dir::seek()`/`rewind()` are
  exercised) - it was verified against the default `linux_raw` backend on x86_64 Linux only; the
  `libc` backend's own `src/backend/libc/fs/dir.rs` entry-name parsing was not separately
  mutation-tested this session (its `readdir()` wraps libc's own `readdir_r`/`readdir`, a
  different code shape, so the same specific mutant doesn't directly transfer - it would need its
  own injected corruption to demonstrate the same class of gap there, if that's wanted).
  `test_dir_seek`'s own new assertion still applies equally on that backend since it only tests
  the already-public `Dir`/`DirEntry` API.
  - **Side note, not part of this candidate's own finding**: while confirming this fix doesn't
    regress anything, the same mutation-testing run also incidentally confirmed (as an expected,
    uninteresting side effect, not a new finding) that `test_dir_new`, `test_dir_read_from`, and
    `readdir::dir_entries` *do* already catch this same class of corruption (they check exact
    `.`/`..`/`Cargo.toml` names) - i.e. this gap is specific to `test_dir_seek`'s own assertion
    shape, not a project-wide absence of name-correctness checking.
- Whether other tests in this workspace have a similar "compare two self-produced listings against
  each other, never against ground truth" shape was not swept this session - out of scope for a
  single candidate, noted as a general pattern worth a future look if a similar
  `collection_is_never_read`-shaped clippy hit turns up elsewhere.
