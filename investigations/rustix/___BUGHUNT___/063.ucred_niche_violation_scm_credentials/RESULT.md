# ucred_niche_violation_scm_credentials

`RecvAncillaryBuffer`/`AncillaryDrain::cvt_msg` (`src/net/send_recv/msg.rs:572`) parses a received
`SCM_CREDENTIALS` ancillary payload by casting the raw bytes to `*const UCred` and calling
`.read_unaligned()`. `UCred`'s first field is `pid: Pid`, and `Pid` wraps a `NonZeroI32`. Linux
documents (and this crate's own candidate 027/035 history already shows awareness of similar
kernel-vs-Rust-niche edge cases) that the kernel can legitimately deliver `pid == 0` in
`SCM_CREDENTIALS` - when the sender's PID isn't representable in the receiving process's PID
namespace. Reading such a payload via `read_unaligned::<UCred>()` constructs an invalid
`NonZeroI32`, which is immediate Undefined Behavior per Rust's validity invariants for
niche-optimized types, reachable through the safe public `recvmsg`/`RecvAncillaryBuffer::drain()`
API.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Debug-assertions build, real `AF_UNIX` socketpair, real `sendmsg`/`recvmsg` round trip via rustix's own public API (`sockopt::set_socket_passcred`, `SendAncillaryMessage::ScmCredentials`, `RecvAncillaryBuffer::drain()`), then re-parsing the *genuine* captured bytes with only the `pid` field patched to 0 via the public `AncillaryDrain::parse` | `cargo run`, debug, x86_64-unknown-linux-gnu | No crash/panic - silently constructs and prints `UCred { pid: Pid(0), .. }`. Confirms the bug is real but invisible without stricter tooling in a plain build (expected per CONFIRMATION.md section 2) |
| `cargo +nightly miri run`, libc backend (`RUSTFLAGS="--cfg rustix_no_linux_raw"`, `linux_raw`'s inline-`asm!` syscalls can't run under Miri at all, per this workspace's `README.md`), same patched-pid=0 buffer fed through the real public `AncillaryDrain::parse` entry point, with `MIRIFLAGS="-Zmiri-disable-stacked-borrows"` | Miri (nightly-x86_64-unknown-linux-gnu), 3 separate runs | **Confirmed**: `error: Undefined Behavior: constructing invalid value of type std::ptr::Unaligned<rustix::net::UCred>: at .0.pid.0.0.0, encountered 0, but expected something greater or equal to 1` at `src/net/send_recv/msg.rs:572:37`, identical on all 3 runs |
| Attempted to construct the real, unmodified trigger condition (two processes in unrelated PID namespaces sharing an `AF_UNIX` socketpair) via `unshare(CLONE_NEWUSER)`/`unshare(CLONE_NEWPID)`, both the `unshare` CLI and a raw C syscall test | This sandbox, as root-less `uid=1000` with `no_new_privs` set and `apparmor_restrict_unprivileged_userns=1` | Both return `EPERM` (`Operation not permitted`) - confirmed via a direct syscall test (`unshare(CLONE_NEWUSER) = -1 errno=1`, `unshare(CLONE_NEWPID) = -1 errno=1`), not just the CLI wrapper's own message. PID-namespace creation is genuinely blocked in this environment; see "Fully reproducible" below |

2 escalating approaches were used (debug build, then Miri), plus the one environment-capability
check above that determined the scope of what a fully genuine repro could demonstrate here. A
third independent tool (ASAN/UBSAN) wasn't needed - Miri already produced an unambiguous,
specific, correctly-located diagnostic (CONFIRMATION.md section 5), and this bug class
(constructing an invalid niche value) is exactly what Miri's validity-invariant checker is
designed to catch deterministically, so a third tool would not add information.

Note: reaching the target diagnostic under Miri required `-Zmiri-disable-stacked-borrows`. Without
it, Miri aborts *earlier*, on every `SCM_CREDENTIALS` message (valid or not), with an unrelated
Stacked-Borrows violation in the same `cvt_msg` function's `CMSG_DATA`/`slice::from_raw_parts_mut`
reborrow (`retag ... Unique permission ... tag does not exist in the borrow stack`,
`src/net/send_recv/msg.rs:558`). That is a distinct, separate potential soundness issue, out of
scope for this candidate (one bug per candidate folder) - see "New suspects" below.

## Status

**Confirmed**

Reproduces deterministically under Miri: 3/3 runs, nightly Miri, libc backend
(`--cfg rustix_no_linux_raw`), `-Zmiri-disable-stacked-borrows`, same exact diagnostic and source
location every time. Does not produce an externally visible crash in a plain (non-Miri) debug or
release build - the invalid `Pid(0)` is silently constructed and printed instead; this is expected
and does not weaken the finding (CONFIRMATION.md section 2: a bug needing instrumentation to
manifest is not less real for needing it).

**Fully reproducible**: **No**, with justification (per CONFIRMATION.md section 11's narrow,
explicitly-stated exception).

The finding is about how `cvt_msg` reacts to a `pid == 0` `SCM_CREDENTIALS` payload - input that,
in reality, only a second participant (the Linux kernel, mediating between two processes in
unrelated PID namespaces) actually produces, not something a hostile peer can simply choose to
send (Linux validates/overwrites unprivileged `SCM_CREDENTIALS` sender-supplied `pid`/`uid`/`gid`
against the sender's own real credentials; `pid == 0` is specifically the kernel's own translation
of an *unrepresentable* sender PID across a PID-namespace boundary, not an attacker-chosen value).
Building the genuine, real two-participant trigger (one process sending real `SCM_CREDENTIALS`
credentials from inside a PID namespace that doesn't contain the receiver, over a real socket)
requires `unshare(CLONE_NEWUSER)`/`CLONE_NEWPID`, and this sandbox's AppArmor policy
(`apparmor_restrict_unprivileged_userns=1`) plus `no_new_privs` (which also blocks the
`sudo`/setuid-helper workaround) block both, confirmed via a direct syscall test (`output.txt`
section 1), not just inferred from the `unshare` CLI's own error message.

Given that, the repro instead: (a) performs a REAL end-to-end `AF_UNIX` `socketpair` +
`sendmsg`/`recvmsg` `SCM_CREDENTIALS` round trip through rustix's own public API to capture the
*genuine* kernel-produced `cmsghdr`+`ucred` byte layout (not a guessed one), and (b) patches only
the 4-byte `pid` field of that genuine, captured buffer to 0 - the one field that would require the
blocked namespace-crossing to produce for real - before re-parsing it through the same real public
`AncillaryDrain::parse` entry point. Every other byte (header length/level/type, alignment padding,
`uid`, `gid`) is exactly what the real kernel wrote. `repro/src/bin/end_to_end.rs` and
`repro/src/bin/miri_check.rs` both document this in their own module doc comments.

## Class

UB, memory-safety (construction of an invalid niche-optimized value via `read_unaligned`)

## Root cause

`UCred { pid: Pid, uid: Uid, gid: Gid }` is read directly out of attacker/kernel-controlled memory
with `payload.as_ptr().cast::<UCred>().read_unaligned()`. `Pid` is `#[repr(transparent)]`-like over
`NonZeroI32` (actually a plain tuple struct wrapping it), so the *only* valid bit patterns for its
first 4 bytes are nonzero `i32`s - the Rust compiler is entitled to assume this unconditionally
anywhere a `Pid`/`UCred` value exists, including doing nothing in particular when `read_unaligned`
reads a zero into it, which is exactly what Miri's validity-invariant checker exists to catch.
Every other read of a `Pid` from an external source in this crate goes through the fallible
`Pid::from_raw(raw: i32) -> Option<Pid>` (e.g. `socket_peercred`'s backend implementations,
`waitpid`, etc.) specifically because 0 is a real, reachable value for a raw pid in several kernel
APIs; `cvt_msg` is the one place that bypassed that fallible path and read the whole `UCred`
(including its `Pid` field) directly via a raw pointer cast instead.

## Introduced by

62afd55c "Add wrappers for `SO_PEERCRED` and `SCM_CREDENTIALS` (#851)" (2023-10-04) - this commit
added the `(c::SOL_SOCKET, c::SCM_CREDENTIALS)` arm to `cvt_msg` with the `read_unaligned::<UCred>()`
call exactly as it stood before this candidate's fix (`git log -S "cast::<UCred>"` on
`src/net/send_recv/msg.rs` returns only this one commit).

## The fix

Changed the `SCM_CREDENTIALS` arm of `cvt_msg` to read the raw `pid`/`uid`/`gid` fields through a
plain, niche-free `#[repr(C)] struct RawUCred { pid: RawPid, uid: RawUid, gid: RawGid }` (all plain
integers, no validity invariant to violate) instead of reading a `UCred` directly, then constructs
the real `UCred` only via the crate's own existing fallible `Pid::from_raw`. If the kernel delivered
`pid == 0` (unrepresentable), `cvt_msg` now returns `None` for that ancillary message - the
`SCM_CREDENTIALS` message is silently skipped rather than ever constructing an invalid `Pid` - the
same precedent the function already uses one branch earlier for a too-short payload. `uid`/`gid`
have no niche (`Uid`/`Gid` wrap plain `u32`s) and are unaffected; they're still returned via
`Uid::from_raw`/`Gid::from_raw` for consistency with the raw-field read.

This is a deliberate behavior change worth flagging explicitly for the reader: a caller that used
to receive `UCred { pid: Pid(0), .. }` (itself already an invalid, UB-tainted value before this
fix) will now see the `ScmCredentials` message simply absent from `RecvAncillaryBuffer::drain()`'s
iteration for that one kernel-delivered message, with `uid`/`gid` lost along with it. Changing
`UCred::pid`'s type to something that can represent an unrepresentable PID (e.g. `Option<Pid>` or
a raw `i32`) would preserve `uid`/`gid` in this case, but is a public API break out of scope for a
bounded fix.

Verified against two inputs in the same bug class (CONFIRMATION.md section 7): the original
genuine-captured-buffer-with-pid-patched-to-0 (both via `end_to_end.rs`'s real socketpair capture
and via `miri_check.rs`'s embedded-bytes variant), and the normal non-zero-pid case (`end_to_end.rs`
step 1, before any patching) continuing to parse correctly and round-trip the real credentials
unchanged - re-run against the patched tree, still passes.

Before -> after, same `repro/`:
- Miri (`miri_check`): `error: Undefined Behavior: constructing invalid value ...` (abort) ->
  `FIXED: pid=0 credentials message was safely skipped` (clean exit, 0)
- Real end-to-end (`end_to_end`): `BUG: constructed UCred with invalid Pid: UCred { pid: Pid(0), .. }`
  -> `FIXED: pid=0 credentials message was safely skipped`

- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`):
  **411 passed, 1 failed, both before and after** the patch - identical counts, identical single
  failure (`ioctl::test_int_setter`, pre-existing and unrelated: `/dev/net/tun` doesn't exist in
  this sandbox, already noted in this workspace's `README.md`). No new failures introduced by the
  fix.

## What's not done / open questions

- The unrelated Stacked-Borrows violation in the same function's `CMSG_DATA`/
  `slice::from_raw_parts_mut` reborrow (hit on every `SCM_CREDENTIALS` message under strict Miri
  aliasing checking, worked around here with `-Zmiri-disable-stacked-borrows`) was not
  investigated - it's a separate potential soundness issue, noted as a new suspect, not fixed or
  further characterized here.
- The same "kernel can legitimately deliver values a safe Rust type can't represent" risk pattern
  might apply to other ancillary-message/credential-passing code paths in this crate (e.g.
  `socket_peercred`'s `SO_PEERCRED` backends construct `UCred` the same way from `getsockopt`
  output - not checked this session; `uid`/`gid` there have no niche, but worth a quick look to
  confirm `pid` is handled consistently, or left with the same latent issue).
- Because a genuine cross-PID-namespace two-participant repro couldn't be built in this sandbox
  (see "Fully reproducible" above), the exact *rate* at which real-world namespace-crossing
  `SCM_CREDENTIALS` traffic would hit this (vs. this being a rare/theoretical edge in most
  deployments) wasn't established - a maintainer with access to an unprivileged-userns-capable
  environment could close that gap with a true two-namespace repro if wanted.
