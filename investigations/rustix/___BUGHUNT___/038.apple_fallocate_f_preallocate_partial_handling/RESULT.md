# apple_fallocate_f_preallocate_partial_handling

Upstream bytecodealliance/rustix#1682: on Apple platforms, `rustix::fs::fallocate` only checks
whether `fcntl(F_PREALLOCATE)` returned `-1`; it never inspects `fstore_t.fst_bytesalloc`, so a
real XNU kernel partial allocation (`F_ALLOCATECONTIG` without `F_ALLOCATEALL` can succeed while
reserving less than requested, which Apple's own `fcntl(2)` documents) is silently accepted and
`ftruncate` extends the file to the full logical size anyway - leaving a sparse file whose later
writes are not actually backed by reserved disk space. Carried over from candidate
028.cross_platform_github_issues_untestable_env, which separated this out explicitly as needing
real Apple hardware rather than carrying it to its own verdict.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: compare current HEAD's exact source against the issue's own quoted permalink | `src/backend/libc/fs/syscalls.rs:1797-1825` at HEAD 287214b8 (the exact commit the issue cites) | Byte-for-byte identical to what the issue quotes - confirms the reported code is still present verbatim, not already fixed or changed since the report |
| `git log -S`/`git show` to find the introducing commit | `git log --reverse --all -S "F_ALLOCATECONTIG" -- .` | Found origin commit 5f3540967a631413dc02548f4959b4b54a8a77f5 (2020-12-07): the Apple `fallocate` implementation was written this way from its very first commit - the unconditional `ftruncate` after only checking `== -1` is not a regression, it's present since the function's introduction |
| Real cross-compile check: `cargo build --target x86_64-apple-darwin --features fs,std` against the actual unmodified rustix source | `rustup target add x86_64-apple-darwin` (prebuilt std available, no macOS SDK needed - the vendored `libc` crate's Apple definitions are plain Rust) | Builds cleanly - confirms the vulnerable `cfg(apple)` code path is real, shipped, reachable code, not dead/unreachable in this crate's actual build matrix |
| Attempt a real end-to-end execution: link a minimal binary calling `rustix::fs::fallocate` for `x86_64-apple-darwin`, then look for a way to actually run it | `cargo build --target x86_64-apple-darwin` on a trivial caller crate | **Linking fails outright**: `cc: error: unrecognized command-line option '-arch'` / `'-mmacosx-version-min=...'` - this container's `cc` is plain Linux `gcc`, there is no macOS SDK (`xcrun --sdk macosx --show-sdk-path` fails: "No such file or directory"), and no Darwin cross-linker package exists here. Separately confirmed `qemu-user`'s installed binaries (`qemu-x86_64`, `qemu-aarch64`, etc.) translate Linux-ABI syscalls for a foreign CPU, not XNU/Darwin's distinct syscall ABI or Mach-O loading - not a substitute for real hardware even if linking somehow succeeded |
| Manual boundary-value construction: extract rustix's own control flow (pre-fix and patched) verbatim into a standalone repro, driven by a mock `fcntl`/`ftruncate` whose return values are the exact real numbers the issue's own author reported from an actual macOS 15.7.4/Darwin 24.6.0 arm64 run against real APFS volumes (not invented) | host x86_64, debug extraction run in release mode, 3 repeat runs each of 4 before/after x plenty/scarce combinations | **Reproduces 3/3 runs** for both scenarios (see `output.txt`): pre-fix logic reports `Ok(())` while under-allocating (3,088,384 of 134,217,728 requested bytes in the "plenty" scenario; 104,857,600 of 209,715,200 in the "scarce" scenario, both numbers quoted directly from the issue body). Patched logic holds 3/3 runs for both scenarios too: "plenty" now retries with `F_ALLOCATEALL` and reaches full allocation before `ftruncate`; "scarce" now returns `Err` (ENOSPC) without ever calling `ftruncate`, leaving the file untouched, matching the issue's own documented `F_ALLOCATEALL` control experiment |

## Status

**Confirmed.** Reproduces 3/3 runs (both the "plenty" and "scarce" scenarios), via the logic
extraction described above, using real kernel-response numbers quoted from the upstream issue's
own actual macOS/APFS hardware run - not invented values. Fix verified 3/3 runs for both scenarios
with the correct, documented all-or-nothing behavior restored.

**Fully reproducible: No** - a genuine, stated hardware exception per `CONFIRMATION.md` section
11, verified directly rather than assumed:

- This session has no macOS hardware, no macOS CI, and confirmed this container can neither link
  (`cc` here is plain Linux `gcc`; `xcrun --sdk macosx --show-sdk-path` fails outright, no Darwin
  SDK/cross-linker package is installed or available) nor execute (`qemu-user`'s installed
  binaries translate foreign-CPU Linux-ABI syscalls, not XNU's distinct syscall ABI or Mach-O
  loading) a real macOS binary.
- Unlike candidate 028's sparc `NCCS` bug (a static, compile-time-constant mismatch that could be
  reproduced by substituting the real per-architecture *constants* into rustix's own extracted
  logic and running it natively), this bug's trigger is the real XNU kernel's *runtime* decision
  of how many bytes to actually reserve for a given `F_PREALLOCATE` request on a given
  filesystem - not a value this session can compute or independently observe without the real
  kernel.
- The repro instead uses the real, externally-reported numbers from the issue's own actual
  macOS/APFS hardware run (both the "plenty" and "scarce" scenarios' exact byte counts are quoted
  verbatim from the issue body) as the mock's return values, so the only thing faked is *which
  physical machine ran the kernel call* - not the magnitude or shape of its result, which comes
  from a real, independently-published hardware observation rather than a guess. See
  `repro/src/main.rs`'s header comment for the exact citations and reasoning.

## Class

logic

## Root cause

`src/backend/libc/fs/syscalls.rs`'s `cfg(apple)` `fallocate` first calls
`fcntl(fd, F_PREALLOCATE, &store)` with `store.fst_flags = F_ALLOCATECONTIG` and only checks
whether the call returned `-1`. Apple's own `fcntl(2)` documents that `F_PREALLOCATE` without
`F_ALLOCATEALL` is explicitly **not** all-or-nothing: "upon success, the space that is allocated
can be the size requested, larger than the size requested, or (if the `F_ALLOCATEALL` flag is not
provided) smaller than the space requested" - the kernel reports the real amount via
`store.fst_bytesalloc`, which the pre-fix code never reads. Because the first call can return `0`
(success) while reserving far less than `fst_length`, the code falls straight through to an
unconditional `ftruncate(fd, new_len)`, extending the file's logical size to the full requested
length regardless of how much space was actually reserved. The result: `fallocate` returns
`Ok(())` for a request that was only partially honored, leaving a sparse file whose later writes
can hit `ENOSPC` that `fallocate`'s whole purpose is to preempt.

## Introduced by

5f3540967a631413dc02548f4959b4b54a8a77f5 "Implement posix_fadvise and refine the return type of
isatty (#1)" (2020-12-07) - this commit introduced the Apple `fallocate` implementation in its
original form, including the unconditional `ftruncate` after only checking the first `fcntl`
call's return value for `-1`. The bug has been present, unchanged in this respect, since the
function's first commit (confirmed via `git log --reverse -S "F_ALLOCATECONTIG"` - later commits
reorganized/moved the file but did not touch this logic until this session's own patch).

## The fix

Extend the existing `if` condition that decides whether to retry with `F_ALLOCATEALL` to also
fire when the first call "succeeded" but under-allocated
(`store.fst_bytesalloc < new_len`), not only when it returned `-1`:

```rust
if c::fcntl(borrowed_fd(fd), c::F_PREALLOCATE, &store) == -1 || store.fst_bytesalloc < new_len {
    store.fst_flags = c::F_ALLOCATEALL;
    let _ = ret_c_int(c::fcntl(borrowed_fd(fd), c::F_PREALLOCATE, &store))?;
}
ret(c::ftruncate(borrowed_fd(fd), new_len))
```

This directly implements the issue's own suggested direction ("Use `F_ALLOCATEALL` for the Apple
emulation and validate `fst_bytesalloc` before calling `ftruncate`") while keeping
`F_ALLOCATECONTIG` as a genuine fast path for the common case where it already reserves the full
range (no behavior change, no extra syscall, when the first call fully succeeds) - it only adds
the retry-with-`F_ALLOCATEALL` fallback (which already existed for the `-1` case) to the
previously-unhandled "succeeded but partial" case. Because `F_ALLOCATEALL` is documented
all-or-nothing, the retry either fully reserves the range (so the subsequent `ftruncate` is now
safe) or fails outright with `ENOSPC` via the existing `ret_c_int(...)?`, which returns before
`ftruncate` is ever reached - matching the issue's own expected behavior ("An oversized request
should return `ENOSPC` without being converted into a successful sparse logical extension").

**Checked against more than the one triggering input (CONFIRMATION.md section 7)**: verified
against both of the issue's own reported scenarios, not just one - the "plenty" case (enough real
disk space, first call under-allocates but a full reservation is achievable) and the "scarce" case
(not enough real disk space, no reservation can ever satisfy the request) exercise the fix's two
different outcomes (retry-then-succeed vs. retry-then-fail-cleanly) - see `output.txt`.

**Build-verified** (patched tree, real source, real target): `cargo build --target
x86_64-apple-darwin --features fs,std` - succeeds, both via the worktree's own cross-compile check
and via `just test`'s `cross-check-before`/`cross-check-after` recipes run against a fresh clone
(confirmed by actually running `just test` end-to-end in a scratch directory, not just reading the
recipe). `cargo clippy --target x86_64-apple-darwin --features fs,std` on the patched tree
produces no new warnings attributable to this change (two pre-existing, unrelated
`clippy::wrong_self_convention`/`clippy::manual_filter` warnings elsewhere in the crate are present
before and after).

- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`,
  host x86_64, since `cfg(apple)` code is not compiled into this target at all so this is a
  regression sanity check on the surrounding file, not an exercise of the actual changed code):
  **411 passed, 1 failed, identical before and after** - the one failure
  (`ioctl::test_int_setter`, `NotFound` opening `/dev/net/tun`) is the same pre-existing,
  environment-caused failure already documented in `projects/rustix.md` and in candidate 028's own
  `RESULT.md`; verified by running the suite against both the pristine and patched tree in this
  session (not assumed). No new failures introduced by the fix.

## What's not done / open questions

- Not independently confirmed on real Apple hardware - see "Fully reproducible: No" above for the
  specific, verified reason (no macOS hardware/CI/SDK/cross-linker in this environment, and
  qemu-user cannot emulate XNU's syscall ABI or Mach-O loading). A maintainer with access to real
  macOS hardware (or CI running on one) could re-run the issue's own minimal reproduction against
  the patched binary for a fully end-to-end confirmation.
- `#1678` (`tcgetpgrp` unsound on macOS/BSD) and `#1668` (Miri+macOS `fstat`) remain in the same
  "needs real Apple hardware" category this candidate's parent (028) flagged them in - not
  investigated in this session, still open backlog items for a future session with macOS access.
- Not checked: whether `F_ALLOCATECONTIG`'s partial-allocation behavior can also be observed (and
  thus this same class of bug independently confirmed) via a different documented Apple API or a
  BSD relative with similar `fcntl`-based preallocation semantics that might be testable without
  real Apple hardware - not pursued this session, the issue is specific to the Apple backend only.
- The fix changes behavior only for the "succeeded but under-allocated" case; it does not change
  how `F_ALLOCATECONTIG` failures (`-1`) are handled, which already retried with `F_ALLOCATEALL`
  before this patch.
