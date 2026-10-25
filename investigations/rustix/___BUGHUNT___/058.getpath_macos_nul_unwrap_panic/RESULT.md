# getpath_macos_nul_unwrap_panic

On Apple targets, `rustix::fs::getpath` (`src/backend/libc/fs/syscalls.rs`, `getpath`) allocates a
`PATH_MAX`-sized buffer, has the kernel fill it via `fcntl(fd, F_GETPATH, buf.as_mut_ptr())`, then
does `buf.iter().position(|&c| c == 0).unwrap()` followed by `CString::new(buf).unwrap()`, trusting
the kernel to always leave a NUL terminator somewhere within those bytes. Flagged as a candidate
because this is the same "unwrap on syscall-returned bytes whose invariant is trusted rather than
checked" shape as candidate 032 (`readlink`'s `CString::new(buffer).unwrap()`), which explicitly
named this exact `getpath` sibling in its own "what's not done" section as "worth its own
candidate", needing real Apple hardware to confirm.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `src/backend/libc/fs/syscalls.rs:2189-2212` (getpath), `src/fs/getpath.rs` (public wrapper, `cfg(all(apple, feature = "alloc"))`-gated) | Confirmed: no validation of the buffer's content before `position(...).unwrap()`/`CString::new(...).unwrap()`; Apple's own `fcntl(2)` manual page only guarantees the buffer must be `MAXPATHLEN` or greater, never that the kernel leaves room to spare for the terminator |
| Real Apple hardware/execution feasibility re-check (same environment as candidates 038/039) | `rustup target list --installed`, `xcrun --sdk macosx --show-sdk-path`, `sudo -n true`, `cargo build --target x86_64-apple-darwin --features "fs,alloc,std" --lib` | Builds cleanly (rlib only, no link) - confirms `getpath` is real, shipped, reachable code; no macOS SDK/cross-linker exists here (`xcrun`: command not found) and no root, so no way to produce or run a real linked Darwin binary - same conclusion candidates 038/039 already established for this container |
| Miri cross-interpretation of the REAL, unmodified `getpath` (no mock at all) | `MIRIFLAGS="-Zmiri-disable-isolation" cargo miri run --target x86_64-apple-darwin`, real `std::fs::File`'s fd passed through | Dead end: `error: unsupported operation: fcntl: unsupported command 0x32` - Miri has no shim implementing `F_GETPATH`'s path-resolution semantics for any fd (unlike candidate 039's `fstat`, which Miri's libc shim does emulate) - a genuine tool limitation, not evidence of absence |
| Manual boundary-value construction via the real public API, syscall boundary mocked for two magic raw fd values only (`mock.patch`), real `getpath` logic unmodified, run via Miri cross-interpretation for `x86_64-apple-darwin` (Miri interprets MIR directly, so it needs no real linking/execution - this is what makes this approach runnable at all here) | Debug build, `cargo miri run --example` against the worktree, then `just test` from a completely fresh clone | **Reproduced**: panics with `called Option::unwrap() on a None value` at `src/backend/libc/fs/syscalls.rs:2217:46` (the `.position(...).unwrap()` call), 5/5 runs, identical message and location every time; the boundary variant (buffer full except the very last byte, which the kernel does write as NUL - the legitimate maximal path) succeeds cleanly every run, confirming the panic is driven exactly by "no NUL anywhere in the buffer", not by any other incidental factor |

## Status

**Confirmed**, reproduces 5/5 runs (debug build, Miri cross-interpretation for
`x86_64-apple-darwin`, deterministic - no layout/seed dependency; the panic is driven entirely by
the mocked buffer content, not memory layout), plus the boundary-value variant behaving correctly
(no panic) in all 5 of the same runs.

**Fully reproducible**: **No**, justified - the same narrow `CONFIRMATION.md` section 11 exception
already used by candidates 032/038/039. The real second participant for this bug class is a real
XNU kernel's actual `F_GETPATH` response to a real `fcntl` call on a file descriptor whose path is
at or near `PATH_MAX` in length - this session has no macOS hardware, no macOS CI, and this
sandboxed container has no macOS SDK (`xcrun` is absent entirely) and no cross-linker to produce or
execute a real Darwin binary (confirmed fresh in this session, matching candidates 038/039's
already-established conclusion for the same environment). Given that:

- Miri's own cross-interpretation (which sidesteps the "can't link/execute Darwin" problem for
  *pure-computation* code, as it did successfully for candidate 039's `fstat`) hits a hard wall
  here: Miri has no shim for `F_GETPATH` at all (`unsupported operation: fcntl: unsupported command
  0x32`), so it cannot be used to observe a real kernel-shaped response the way it could for
  `fstat`'s `MaybeUninit`/`assume_init()` pattern.
- The repro therefore mocks only the single lowest-level syscall call (`c::fcntl(..., F_GETPATH,
  ...)` inside `getpath`) for two magic raw fd values, in `mock.patch` applied to a freshly cloned
  pristine checkout before building - every other line exercised (rustix's real public
  `rustix::fs::getpath` entry point, the real, unmodified `position(...).unwrap()`/`truncate`/
  `CString::new(...).unwrap()` logic) is the exact, unmodified source being tested, compiled for
  the real `cfg(apple)` target and executed via Miri's real MIR interpreter - not a hand-copied
  re-implementation of the logic (unlike candidate 038's extracted-control-flow approach, which was
  necessary there only because that bug's trigger was a *runtime kernel decision* this session
  could not compute at all; here the trigger condition itself - "no zero byte anywhere in the
  buffer" - is exactly what's being driven, through the real code).

## Class

crash

## Root cause

`getpath` (`src/backend/libc/fs/syscalls.rs:2189-2212`) allocates `buf = vec![0; c::PATH_MAX as
usize]`, has the kernel fill it via `fcntl(fd, F_GETPATH, buf.as_mut_ptr())`, then does:

```rust
let l = buf.iter().position(|&c| c == 0).unwrap();
buf.truncate(l);
buf.shrink_to_fit();

Ok(CString::new(buf).unwrap())
```

This assumes the kernel's `F_GETPATH` response always leaves at least one zero byte somewhere in
the `PATH_MAX`-sized buffer. Apple's own `fcntl(2)` manual page for `F_GETPATH` only documents
"the argument must be a buffer of size `MAXPATHLEN` or greater" - it makes no promise that the
*path itself* is bounded by `MAXPATHLEN - 1` (leaving at least one byte for the NUL terminator), nor
that the kernel always writes a terminator at all in every circumstance (an unusual/exotic
filesystem path construction, a synthetic/firmlink path on APFS, or any other kernel-side edge case
that produces a path at exactly `PATH_MAX` bytes). If the kernel ever returns a full buffer with no
zero byte, `.position(|&c| c == 0)` returns `None`, and `.unwrap()` panics the calling thread. (The
second `.unwrap()`, on `CString::new(buf)`, can never itself fail given a `Some(l)` from
`position()`, since `buf[..l]` by construction contains no zero byte - all the risk is in the first
`unwrap()`.)

## Introduced by

f202bbc58475c6096020ca311834679069b4843a "Split out filesystem system calls in the libc backend."
(2021-11-01) - this commit moved `getpath` into its current file and introduced the exact current
shape of both the `.position(|&c| c == 0).unwrap()` call and the trailing
`CString::new(buf).unwrap()` together (confirmed via `git show f202bbc5 -- src/imp/libc/fs/
syscalls.rs`). An earlier ancestor (85ea2a2c, 2021-06-11) had already introduced the
`position(...).unwrap()` pattern while the function still returned a `PathBuf` (no `CString::new`
involved yet); f202bbc5 is where the function was changed to return `io::Result<CString>` with
today's exact two-unwrap shape.

## The fix

Changed the `position(...).unwrap()` to a `match`, returning `io::Errno::NAMETOOLONG` for the `None`
case instead of panicking - `NAMETOOLONG` matches the project's own existing convention for "path
doesn't fit" conditions (already used elsewhere in the same file, e.g.
`src/backend/libc/fs/syscalls.rs:1043`) and is the semantically closest standard errno to "the
kernel's response didn't leave room for a terminator within the documented buffer size":

```rust
let l = match buf.iter().position(|&c| c == 0) {
    Some(l) => l,
    None => return Err(io::Errno::NAMETOOLONG),
};
buf.truncate(l);
buf.shrink_to_fit();

Ok(CString::new(buf).unwrap())
```

The trailing `CString::new(buf).unwrap()` is left as-is (and documented with a one-line comment
explaining why): once `position()` returns `Some(l)`, `buf[..l]` is guaranteed free of embedded
NULs by construction, so that `unwrap()` can never panic - unlike candidate 032's `readlink` fix,
there is no second failure mode to handle here, only the one `position().unwrap()`.

**Checked against more than the one triggering input (CONFIRMATION.md section 7)**: verified
against both the pathological case (buffer completely full, no NUL anywhere) and a boundary variant
(buffer full except the very last byte, which the kernel does write as NUL - the legitimate maximal
path) - the fix returns a clean `Err(Errno::NAMETOOLONG)` for the former and is a no-op (unaffected,
still succeeds) for the latter, 3/3 runs each. See `patch.diff` (+10/-1 lines against
`src/backend/libc/fs/syscalls.rs`), verified by cross-compiling (`cargo build --target
x86_64-apple-darwin --features "fs,alloc,std" --lib`) and by the mock-driven Miri repro above.

- Project test suite: `cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`
  (host x86_64-unknown-linux-gnu, since `cfg(apple)` code is not compiled into this target at all,
  so this is a regression sanity check on the surrounding file, not an exercise of the actual
  changed code) - **411 passed, 1 failed before the fix; 411 passed, 1 failed after the fix
  (identical)**. The one failure (`ioctl::test_int_setter`) is the same pre-existing,
  environment-caused failure already documented in `projects/rustix.md` and in candidates
  032/038/039's own `RESULT.md` files (`NotFound` opening `/dev/net/tun`, absent in this
  container) - unrelated to this change, present and identical both before and after.

## What's not done / open questions

- Not independently confirmed on real Apple hardware - see "Fully reproducible: No" above for the
  specific, verified reason (no macOS SDK/cross-linker in this environment, and Miri has no
  `F_GETPATH` shim to observe a real kernel-shaped response without one). A maintainer with access
  to real macOS hardware (or CI running on one) could attempt to construct a file descriptor whose
  path is genuinely at or near `PATH_MAX` (e.g. via deeply nested directories or a long-named
  mount) to attempt a fully end-to-end confirmation.
- This repro's `mock.patch`/`mock_and_fix.patch` intercept the syscall by raw fd value rather than
  by path content (unlike candidate 032's mock, which intercepted by path) - `getpath` takes a file
  descriptor, not a path, as its input, so there is no path string available at the mock point to
  switch on; the raw fd value is the only available hook, which is why two reserved sentinel values
  (999999/999998) are used instead.
- Not checked: whether any other Apple-only `fcntl`-based syscall wrapper in this codebase (the
  Apple-only `fcntl_rdadvise`, `fcntl_fulfsync`, or similar immediately following `getpath` in the
  same file) makes a similar unchecked assumption about the shape of bytes the kernel writes back -
  not audited this session, `getpath` was the only one flagged by the candidate description.
- Per candidate 032's own "what's not done", this was the one specific macOS-only sibling flagged
  as needing independent investigation; that is now done. Candidate 032's own readlink bug remains
  unresolved in upstream rustix (both are separate, independently fixable issues).
