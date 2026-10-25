# ancillary_drain_cmsg_len_underflow

`src/net/send_recv/msg.rs`'s `AncillaryDrain::advance`/`cvt_msg` trust a received `cmsghdr`'s own
`cmsg_len` field unconditionally (`**length -= msg_len` and `payload_len = msg.cmsg_len as usize -
CMSG_LEN(0)`), with no check against how much control-buffer space is actually left. On macOS,
`recvmsg` truncating `SCM_RIGHTS` ancillary data to fit a too-small `RecvAncillaryBuffer` leaves the
*message's own* `cmsg_len` field holding the untruncated length instead of reducing it to match
(unlike Linux, which keeps it consistent - see candidate 017 in this same workspace), so `advance`
underflows a `usize` subtraction, panics, and `RecvAncillaryBuffer`'s `Drop` impl drains again on
the now-corrupted bookkeeping, panicking a second time mid-unwind and aborting the whole process.
Source: GitHub issue #1683 / PR #1684 (an AI-authored fix PR, open at investigation time, flagged
as only possibly covering "this one path" per the candidate brief this session started from).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading | `src/net/send_recv/msg.rs` (`AncillaryDrain::advance`/`cvt_msg`, `messages::Messages`), plus PR #1684's own diff/description (fetched via `gh pr view`/`gh pr diff`) | Confirmed the unchecked subtraction (`**length -= msg_len`) and the unchecked `payload_len` computation; confirmed PR #1684's own fix clamps inside the single shared `advance`/`cvt_msg`/`Messages::next` machinery used by every drain call site (`next`/`fold`/`count`/`last`/`collect`, and the public `AncillaryDrain::parse` entry point) - the candidate brief's concern that the PR "only clamps this one path" did not hold up under direct reading of the diff: the clamp is generic over all `cmsghdr` types and all iterator methods, not SCM_RIGHTS-specific or call-site-specific. |
| Real end-to-end harness: real `AF_UNIX` `SOCK_STREAM` socketpair, a real sender thread doing a real `sendmsg` of 16 real, live `SCM_RIGHTS` file descriptors, a real receiver doing a real `recvmsg` (rustix's actual public API) into a real `RecvAncillaryBuffer` sized for only 5 descriptors, forcing the real Linux kernel to genuinely truncate - **with one field directly overwritten afterward** (see "Fully reproducible" below) to stand in for macOS's documented, not-reproducible-on-Linux kernel quirk, since no macOS hardware/VM is available in this environment | `cargo run` (debug, overflow checks on) and `cargo run --release`, against the pristine tree, repeated 5x (debug) | Debug: 5/5 runs produced the identical two-panic-then-abort sequence (`attempt to subtract with overflow` at msg.rs:544, then a second panic at msg.rs:484 while unwinding, then `panic in a destructor during cleanup` / SIGABRT, exit 134). Release: no debug-only overflow panic; instead `cvt_msg`'s payload slice genuinely read 16 bytes past the end of the real 48-byte buffer and interpreted them as 8 extra file descriptors (real memory-safety UB, not just a crash), before a later single panic (exit 101, no abort since no destructor-during-unwind this time). |
| Negative control (same harness, no field overwrite - i.e. trusting the real Linux kernel's own, correctly-truncated `cmsg_len` as-is) | Same build/config, no simulated byte | No panic, debug or release - 8 fds received cleanly. Confirms the bug is reachable only with the macOS-specific stale `cmsg_len`, not a general Linux-reachable issue (consistent with candidate 017's own finding). |
| Boundary variation on the patched tree (CONFIRMATION.md section 7): `NUM_FDS=7`/`NUM_SLOTS=3` instead of `16`/`5` | Debug build, patched tree | No panic; 6 fds received cleanly - fix holds at a different boundary, not just the exact numbers the original repro uses. |
| Project test suite, before and after the fix | `cargo test --workspace --features=all-apis --no-fail-fast` | Identical both times: 413 passed, 1 failed (`ioctl::test_int_setter`, pre-existing/unrelated - needs `/dev/net/tun`, not present in this sandbox container; same failure confirmed present on the untouched pristine tree). No new failures, no regressions. |

## Status

**Confirmed**. Reproduces 5/5 runs (debug build, `cargo run`, no fixed seed needed - deterministic
given the simulated byte pattern) via a real socketpair/real `sendmsg`/real `recvmsg`/real
`RecvAncillaryBuffer::drain()` call chain. Also independently confirmed in a release build, where
the effect is a genuine out-of-bounds memory read (not just a debug-only assertion).

**Fully reproducible**: **No** - with an explicit, narrow justification per `CONFIRMATION.md`
section 11. The real trigger for this bug is macOS-specific kernel behavior (macOS's real
`recvmsg` leaving `cmsg_len` untruncated after truncating the control data it writes) - no macOS
hardware or VM is available in this environment to make a real `recvmsg` syscall actually produce
that mismatch, and this project's own candidate 017 already empirically confirmed the real Linux
kernel does **not** produce it (Linux correctly keeps `cmsg_len` consistent with what it truncated
to). Every other part of the repro is genuinely real and exercises the actual public API end to
end: a real `AF_UNIX` socketpair, a real independent sender thread performing a real `sendmsg` of
real, live file descriptors as a real `SCM_RIGHTS` message, a real `recvmsg` call, and the real
`RecvAncillaryBuffer`/`AncillaryDrain::drain()` code under test. The only simulated element is one
`usize` field (`cmsg_len`) of the resulting, really-kernel-populated `cmsghdr`, overwritten in place
to the exact value PR #1684's own author reports real macOS 15.6 leaves behind - standing in for
the one real participant (the macOS kernel) this environment cannot invoke. See `repro/src/main.rs`'s
own module doc comment for the same disclosure inline.

## Class

memory-safety (release build: genuine out-of-bounds read, fds materialized from adjacent memory)
and crash (debug build: double panic -> process abort).

## Root cause

`AncillaryDrain::advance` (`src/net/send_recv/msg.rs`) computes `msg_len` directly from the
kernel-reported `cmsghdr.cmsg_len` field with no upper bound against the buffer's own remaining
space, then does `**length -= msg_len` (a plain, non-saturating `usize` subtraction) and, in
`cvt_msg`, builds a payload slice of `msg.cmsg_len as usize - CMSG_LEN(0)` bytes starting at the
message's data pointer - also with no bound against the buffer's actual end. Both computations
implicitly assume `cmsg_len` can never exceed how much space is actually left in the buffer, which
is true for Linux (the kernel adjusts `cmsg_len` to match whatever it truncates) but is not a
universal POSIX guarantee - macOS's own `recvmsg` truncates the `SCM_RIGHTS` payload to fit the
caller's buffer while leaving the header's `cmsg_len` field at the size it would have been without
truncation. A `usize` subtraction with `length < msg_len` underflows (panics with overflow checks
on, wraps to a huge value with them off); either way, the next `drain()` call (including the one
`RecvAncillaryBuffer`'s `Drop` impl makes via `clear()`) indexes `self.buffer[self.read..][..self.length]`
with that corrupted `length`, which is itself a slice-index panic - during unwinding from the first
panic, which Rust escalates to a hard process abort rather than a normal unwind.

## Introduced by

2cdbb1d4 "feat: Add sendmsg/recvmsg (#505)" (2023-04-28) - the original commit that added this
whole `sendmsg`/`recvmsg`/ancillary-message module, including the unguarded `**length -= msg_len`
subtraction and the unguarded `payload_len` computation, both unchanged in shape (only refactored
cosmetically) in every later revision up to the commit this session investigated (287214b8).
Confirmed via `git log -S "length -= msg_len" -- src/net/send_recv/msg.rs`, which returns only this
one commit.

## The fix

Applied directly (matching the approach already proposed, independently, in the project's own open
PR #1684 - reviewed, verified against this session's own real repro, not merely assumed correct
from reading the PR):

- `messages::Messages`'s iterator item changed from `&mut cmsghdr` to `(&mut cmsghdr, usize)`,
  where the `usize` is the number of buffer bytes actually remaining at and after that header
  (computed from `msghdr.msg_control`/`msg_controllen`, i.e. the real buffer bounds the caller gave
  the OS - not from the untrusted `cmsg_len` field itself).
- `AncillaryDrain::advance` now clamps `msg_len = (msg.cmsg_len as usize).min(space)` before using
  it for both the `length`/`read` bookkeeping and the payload-length computation in `cvt_msg`
  (changed to take the already-clamped `msg_len` instead of recomputing from `msg.cmsg_len`
  directly, and to use `checked_sub` for the `CMSG_LEN(0)` subtraction, returning `None` instead of
  underflowing if there isn't even a whole header's worth of clamped space left).
- `cvt_msg`'s `SCM_RIGHTS` branch additionally rounds `payload_len` down to a whole number of
  `OwnedFd`s, so a clamp that lands mid-descriptor doesn't read a partial one.
- `SendAncillaryBuffer`'s own unrelated use of `Messages::new(..).last()` (for *sending*, not
  parsing untrusted kernel data) updated to destructure the new `(header, space)` tuple item and
  discard `space`, since that call site has nothing to clamp against (the buffer there is sized and
  filled entirely by this crate itself, not the kernel).

This clamp lives inside the single `advance`/`cvt_msg`/`Messages::next` implementation shared by
every drain call site (`next`, `fold`, `count`, `last`, `collect`) and by the public
`AncillaryDrain::parse` entry point - not duplicated per call site and not specific to `SCM_RIGHTS`
(the `#[cfg(linux_kernel)]` `SCM_CREDENTIALS` branch benefits from the same clamped `payload_len`
too). Re-verified against the real repro (`just test` in this candidate's own folder, from a fresh
clone): **before** the fix, `reproduce-before` aborts (SIGABRT, exit 134); **after** applying
`patch.diff`, `reproduce-after` exits 0, having drained exactly 8 fds (the real number that fits in
the real 48-byte buffer) without panicking. Also re-checked against a different boundary
(`NUM_FDS=7`/`NUM_SLOTS=3`, not just the repro's own `16`/`5`) - still no panic, 6 fds received,
per `CONFIRMATION.md` section 7.

- Project test suite: 413 passed / 1 failed before -> 413 passed / 1 failed after (same single
  pre-existing, unrelated failure both times: `ioctl::test_int_setter`, which needs `/dev/net/tun`,
  not present in this sandbox container). No new failures introduced by the fix.

## What's not done / open questions

- Not verified on real macOS or FreeBSD hardware/VM (none available in this environment) - the fix
  itself was taken from and cross-checked against PR #1684, whose own author did run it on real
  macOS 15.6 (`cargo test --features=all-apis --test net`, 45 pass; the PR's own new test aborts
  with SIGABRT against unpatched `main` there) and reasoned explicitly about FreeBSD's differing
  truncation behavior (delivers none of a truncated message's payload, rather than Linux's "fill
  what fits" or macOS's "fill what fits but lie about cmsg_len") - this session's own contribution
  is independently confirming the underlying Rust-level logic bug is real (via the simulated-byte
  harness) and independently re-verifying the fix against a real repro and the full test suite in
  this environment, not re-deriving the fix from scratch.
- PR #1684 also adds its own test (`tests/net/cmsg.rs::test_truncated_scm_rights`) which was read
  but not additionally run as part of this candidate's own verification, since (per the PR's own
  description and candidate 017's prior finding in this workspace) it cannot actually trigger the
  bug on Linux - the real Linux kernel never produces the mismatched `cmsg_len` the bug depends on,
  so that test would pass identically on the pristine and patched tree here and adds no additional
  signal beyond what this candidate's own repro already demonstrates.
- Did not check `SCM_CREDENTIALS` (Linux-only, `#[cfg(linux_kernel)]`) under an actual truncation
  scenario specifically - its `payload_len >= size_of::<UCred>()` check does benefit from the same
  clamp by construction (it shares `cvt_msg`'s now-clamped `payload_len`), but no dedicated
  real-syscall harness was built to truncate an `SCM_CREDENTIALS` message the way the `SCM_RIGHTS`
  one was.
