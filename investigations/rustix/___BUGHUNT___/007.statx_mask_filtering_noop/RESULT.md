# statx_mask_filtering_noop

The forward-compatibility guard in `statx()` that's supposed to mask off unknown request bits
before invoking the kernel (`mask & StatxFlags::all()`) has been a silent no-op since `StatxFlags`
gained bitflags' "externally defined flags" support, because `StatxFlags::all()` became `u32::MAX`.
Sourced from an open, unmerged GitHub PR on this project, #1656 ("Restore statx request-mask
filtering"), whose own claim was independently re-verified rather than trusted.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading | `src/fs/statx.rs`'s `StatxFlags` bitflags declaration, both backends' `statx` syscall wrappers | Confirmed the `const _ = !0;` externally-defined-flags opt-in and the `mask & StatxFlags::all()` guard at both call sites |
| Real check, `StatxFlags::all().bits()` value + the exact guard expression, real live `statx("/")` call | `cargo run --release`, run multiple times | `StatxFlags::all().bits() == 0xffffffff` confirmed; the guard expression leaves a deliberately-constructed unknown bit (`1<<20`) untouched; the live `statx` call still succeeds (kernel ignores the unknown bit today, consistent with the PR's own "not a vulnerability on current kernels" framing) |
| `git log`/`git show` | on the two historically-referenced commits (guard added, guard neutered) | Both confirmed real and matching the claim |
| Internal `#[cfg(test)]` module (temporary, reverted) calling the real `pub(crate)` masking logic and the real public `statx()` API | `cargo test --lib fs::statx::...` | Confirmed the fix strips the unknown bit while a named flag survives, and the real end-to-end call still succeeds |
| `cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast` before/after fix | | No regressions |

## Status

**Confirmed**. `StatxFlags::all().bits()` is a compile-time-constant expression (not
runtime-dependent), so this is fully deterministic - reproduced identically across every run.

**Fully reproducible**: Yes. This is a pure-function/syscall-argument-construction check with no
timing or environment dependency beyond running on Linux; the repro also makes a real, live
`rustix::fs::statx("/")` call through the actual public API.

## Class

`security` - a forward-compatibility guard (added specifically to prevent a future kernel from
writing outside the `statx` result buffer via unrecognized mask bits) is silently disabled. Matches
the PR's own "restores the future-kernel safety boundary" framing. Not exploitable on any kernel
version rustix currently targets, but the entire point of the guard was to protect against *future*
kernels, and that protection is currently gone.

## Root cause

1. `src/fs/statx.rs`'s `StatxFlags` bitflags declaration contains `const _ = !0;` (bitflags 2.4's
   "externally defined flags" opt-in, per `Cargo.toml`'s `bitflags = "2.4.0"`). This makes
   `StatxFlags::all().bits() == u32::MAX` (`0xffffffff`) - verified empirically, not just the OR of
   the 14 named variants.
2. Both `src/backend/linux_raw/fs/syscalls.rs:840` and `src/backend/libc/fs/syscalls.rs:2091` do
   `let mask = mask & StatxFlags::all();` right after a special-cased `STATX__RESERVED` (bit 31)
   rejection. Since `StatxFlags::all()` is `u32::MAX`, `mask & u32::MAX == mask` - the line is
   mathematically a no-op for every possible input.
3. Demonstrated with an unknown bit (`1 << 20`, chosen because it is above the highest named bit
   `STATX_DIOALIGN = 0x2000`/bit 13, and distinct from the specially-handled `STATX__RESERVED` bit
   31): the "guard" leaves it untouched, while masking against the actual union of named bits
   correctly strips it.

## Introduced by

- Guard added: 2c6db2e7 "Guard against future `statx` fields. (#379)".
- Guard neutered: 7ff96318 "Enable bitflags' support for externally-defined flags. (#787)", which
  added `const _ = !0;` to `StatxFlags` at its then-location
  (`src/backend/{libc,linux_raw}/fs/types.rs`). Commit 7202da6e "Reorganize Stat, Statx, and
  SockaddrStorage (#1282)" later relocated `StatxFlags` to `src/fs/statx.rs`, carrying the same
  `const _ = !0;` (and the no-op-inducing mask line) forward unchanged to its current location.
  Both historical commits were independently confirmed via `git log`/`git show` in this session, not
  just trusted from the PR text.

## The fix

See `patch.diff`: adds a crate-internal `KNOWN_STATX_FLAGS` union of the 14 named bits in
`src/fs/statx.rs`, makes the `statx` submodule `pub(crate)` (`src/fs/mod.rs`) so both backends can
reach it, and replaces `mask & StatxFlags::all()` with
`StatxFlags::from_bits_retain(mask.bits() & KNOWN_STATX_FLAGS)` at both backend call sites. This
was designed and verified independently in this session (not copied blind from PR #1656, though the
same general shape was used as inspiration - the PR's own approach of "a private allowlist of
currently-named bits" was confirmed to be the right fix by testing it, not assumed).

Verified against more than the exact triggering bit: tested with a mix of a named flag
(`StatxFlags::BASIC_STATS`) plus the unknown bit, confirming the named flag survives while the
unknown bit is stripped - not just that the one specific unknown bit used in the repro gets caught.

- Project test suite: `cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast` -
  411 passed / 1 failed before, identical 411 passed / 1 failed after (the one failure,
  `io::ioctl::test_int_setter`, needs `/dev/net/tun`, unrelated, confirmed present with or without
  the patch). Existing statx-specific tests (`tests/fs/statx.rs::test_statx_reserved`,
  `::test_statx_unknown_flags`) passed both before and after.

## What's not done / open questions

- The same `const _ = !0;` "externally defined flags" pattern was applied crate-wide by PR #787
  (44 files touched: epoll, poll, inotify, mm, mount, net send/recv, pipe, rand, time, xattr,
  io_uring, membarrier, pidfd, prctl, procctl, termios, thread/libcap/setns, etc.). Any other call
  site that does `some_mask & SomeFlags::all()` intending it as a filtering/sanitizing guard
  (rather than as a harmless identity/validity check) would have the exact same silent-no-op
  problem now that `all()` is `u32::MAX`/`u64::MAX` for all of these types. This session did not
  have time to audit all 44 files for this specific pattern - added to `IDEAS.md` as a systemic
  follow-up lead (`grep -rn "& .*Flags::all()"` across the backends would be a reasonable starting
  point for a future candidate).
- `STATX_ALL` (a separate named variant, value `4095`/`0xFFF`) was checked and confirmed to be a
  strict subset of the union already covered by `KNOWN_STATX_FLAGS`'s other named bits, so it did
  not need to be added to the constant separately.
