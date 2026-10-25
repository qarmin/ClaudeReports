# socket_peercred_linux_raw_ucred_niche

`socket_peercred` (`src/backend/linux_raw/net/sockopt.rs:892-894`, mirrored in
`src/backend/libc/net/sockopt.rs:1095`) instantiates the shared generic `getsockopt<T: Copy>`
helper (same file, lines 44-61: `let mut value = MaybeUninit::<T>::uninit(); ...
unsafe { Ok(value.assume_init()) }`) with `T = UCred`, which embeds `pid: crate::pid::Pid` -
`Pid` being `#[repr(transparent)]` over `NonZeroI32`. This looked suspicious per `HEURISTICS.md`'s
"`unsafe` blocks... check the invariant the safety comment claims and whether every call site
actually upholds it": every *other* `getsockopt<T>` caller in this file either uses plain-data `T`
or post-processes through a fallible constructor (e.g. `socket_protocol` uses `RawProtocol::new`);
`UCred` was the only niche-sensitive type left unchecked.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value reasoning: does the real kernel ever actually return `ucred.pid == 0` for `SO_PEERCRED`? | Read `net/core/sock.c::cred_to_ucred()` kernel source reasoning + a raw C program (`getsockopt` on a plain, never-connected `AF_UNIX`/`SOCK_STREAM` socket) | **Yes, trivially** - no PID-namespace trickery needed at all. `cred_to_ucred()` sets `ucred->pid = pid ? pid_vnr(pid) : 0` and `sk->sk_peer_pid` is `NULL` for any unix socket that was never `connect()`/`accept()`/`socketpair()`'d. Verified directly: `getsockopt(SOL_SOCKET, SO_PEERCRED)` on a fresh unconnected socket returns `pid=0, uid=4294967295, gid=4294967295` (the "overflow"/"nobody" placeholders), confirmed 100% reproducible across multiple runs. |
| Attempted real two-PID-namespace repro (the scenario originally named in this candidate's description: a peer in a non-nested PID namespace) | `unshare --pid`/`unshare --user --pid`, raw `clone(CLONE_NEWPID)`/`clone(CLONE_NEWUSER)` via Python `ctypes` | **Infeasible in this sandbox**: every attempt returned `EPERM`, even for `CLONE_NEWUSER` alone (`unprivileged_userns_clone=1` but the sandbox still blocks it - no `CAP_SYS_ADMIN`, bounding set lacks `cap_sys_admin`). Abandoned in favor of the much simpler, equally-real unconnected-socket trigger above, which needs no privilege at all and is a *more direct* path to the same `pid==0` kernel value. |
| Real end-to-end repro via rustix's own public API (`rustix::net::socket` + `rustix::net::sockopt::socket_peercred`) on a genuine unconnected `AF_UNIX` socket, debug + release builds, 5 runs each | `cargo run` / `cargo run --release`, `repro/src/bin/unconnected_peercred_public_api.rs` | **Reproduces, 10/10 runs** (5 debug + 5 release): `socket_peercred()` returns `Err(Os { code: 1, kind: PermissionDenied, ... })` even though the underlying `getsockopt(2)` syscall actually succeeded. |
| Independent cross-check that the underlying syscall genuinely succeeds (rules out an actual permission problem) | Raw `libc::getsockopt` call on the *exact same fd* rustix's `socket()` returned, immediately before the failing rustix call | `libc::getsockopt` returns `r=0, errno=0`, with the real kernel bytes `[0,0,0,0,255,255,255,255,255,255,255,255]` (pid=0, uid=gid=u32::MAX) - confirming the syscall itself never fails; only rustix's own `Result` construction turns it into a reported error. |
| Temporary `eprintln!` instrumentation inside `getsockopt_raw`/`getsockopt<T>` (added, observed, then `git checkout --` reverted before this write-up - the main tree was never left dirty) | Same debug build | Traced the exact point of corruption: `getsockopt_raw` returns `Ok(())` with the correct raw bytes; the very next line, `unsafe { Ok(value.assume_init()) }`, evaluates to a `Result` whose `.is_ok()` is already `false`. The corruption happens at that single expression, not anywhere else in the call chain. |

## Status

**Confirmed** - reproduces **10/10 runs** (5 debug-build + 5 release-build runs of the real,
unmodified public API on a genuinely unconnected socket), and additionally **3/3 further runs**
on a fresh `rm -rf target` rebuild, and again **3/3 runs** via the standalone `repro/` against a
freshly cloned pristine checkout (`just test`'s `reproduce-before` step). No flakiness observed
across any of debug/release/rebuild - this is fully deterministic given the trigger condition
(any never-connected `AF_UNIX` socket), not seed- or layout-dependent in the way `CONFIRMATION.md`
section 3 warns about.

**Fully reproducible**: **Yes**. The repro runs the real, unmodified `rustix::net::socket` +
`rustix::net::sockopt::socket_peercred` public API end to end against a real kernel object (a
genuine `AF_UNIX`/`SOCK_STREAM` socket). There is no second participant being faked here: an
unconnected socket *is* the real external input - the kernel itself, not this repro, decides what
`SO_PEERCRED` reports for it (see `CONFIRMATION.md` section 11's own "malformed file handed to a
parser's real public entry point" exception - this is the same shape, just with a socket instead
of a file).

## Class

`logic` (a successful syscall is silently, deterministically reported to the caller as a spurious
error - no crash, no sanitizer needed to see it; the underlying mechanism is the same class of
"producing an invalid value for a niche-having type is UB" that `miri`/UBSan would also flag, but
here it has a directly observable, tool-free symptom).

## Root cause

`UCred.pid` is typed as `crate::pid::Pid`, a `#[repr(transparent)]` wrapper around `NonZeroI32`.
`socket_peercred`'s generic `getsockopt<T: Copy>` helper builds its return value with
`unsafe { Ok(value.assume_init()) }`, directly turning raw `getsockopt(2)` kernel bytes into a
`Result<UCred, Errno>` in one step, without validating that the `pid` field is actually non-zero.

The Linux kernel's own `cred_to_ucred()` (`net/core/sock.c`) legitimately sets `ucred.pid = 0`
whenever it has no peer PID to report - the simplest and most common case being a `SO_PEERCRED`
query on a unix socket that has never had a peer established via `connect()`/`accept()`/
`socketpair()` (`sk->sk_peer_pid == NULL`). This is not a namespace corner case at all; it is the
default state of any freshly created, unconnected unix socket.

Because `Pid`'s `NonZeroI32` niche makes the all-zero bit pattern a value the type system
considers impossible, the Rust compiler is free to (and, confirmed by direct instrumentation,
does) reuse that exact all-zero-`pid` bit pattern as the niche encoding for `Result<UCred,
Errno>`'s `Err` discriminant, rather than emitting a separate explicit tag. When the real kernel
value happens to be `pid=0` (a legitimate, common, non-exotic case), it is bit-for-bit
indistinguishable from the compiler's chosen `Err` encoding: the *successful* syscall's `Ok(UCred)`
silently becomes `Err(Errno)`, with the real `uid`/`gid` bytes (`0xFFFF_FFFF` each, the kernel's
documented "overflow"/"nobody" placeholders for this case) reinterpreted as the `Err` payload's raw
bits - which is why the printed error (`Os { code: 1, kind: PermissionDenied, ... }`) has nothing
to do with any real permission problem.

This is the generic-UB-helper-misuse pattern `HEURISTICS.md` and this project's own
`projects/rustix.md` risk map already flagged as a live, unswept lead ("Systemic lead... every
other `getsockopt<T>` caller... either uses plain-data `T` or post-processes through a fallible
constructor... `UCred` is the only niche-sensitive type left unchecked" - candidate 050's own
sweep of other `assume_init()` call sites did not cover this one, since `UCred` has no raw
pointer/FFI call site of its own in that sweep's scope).

## Introduced by

62afd55c "Add wrappers for `SO_PEERCRED` and `SCM_CREDENTIALS` (#851)" (2023-10-04). This commit
added `UCred { pid: crate::pid::Pid, ... }` and both backends' `socket_peercred`/
`get_socket_peercred` functions, routing them through the pre-existing generic `getsockopt<T:
Copy>` helper. `Pid` had already been changed to wrap `NonZeroI32` four months earlier in
eb68ba30 "Reconsider guarantees of Pid and refactor (#657)" (2023-06-16) - so by the time 62afd55c
chose `Pid` (rather than a plain `RawPid`/`i32`) as `UCred.pid`'s type, the niche hazard already
existed in `Pid` itself; 62afd55c is what first exposed it through an `assume_init()` path fed by
real, attacker/environment-influenced kernel data. Found via `git log -S"struct UCred"` /
`git log -S"struct Pid(NonZeroI32)"` cross-referenced by commit date.

## The fix

Changed `UCred.pid`'s public type from `Pid` to `Option<Pid>` (`src/net/types.rs`). This is a
minimal, surgical change: no call-site logic needed to change in either backend's
`socket_peercred`, because `Option<Pid>` has the exact same size and bit layout as `Pid`/`i32`
(`0 => None`, nonzero `=> Some(Pid(n))`) - a guarantee this project's own `src/pid.rs` test suite
already asserts and relies on (`assert_eq_size!(RawPid, Option<Pid>)`, plus an explicit transmute
round-trip test). With this type, *every* possible raw kernel bit pattern - including `pid=0` - is
now a valid value of the type, so `assume_init()` can never construct an invalid value, and there
is no niche left for `Result<UCred, Errno>` to collide with. This also transparently fixes the
sibling raw-bytes-to-`UCred` conversion in `src/net/send_recv/msg.rs:572`
(`payload.as_ptr().cast::<UCred>().read_unaligned()`, used for `SCM_CREDENTIALS` ancillary
messages), which had the identical hazard and is now equally safe, with no code change needed
there either.

Two existing tests asserted `ucred.pid == getpid()`; updated both to `Some(getpid())`
(`tests/net/unix.rs`, `tests/net/unix_alloc.rs`) since `getpid()` returns a plain `Pid` and the
field is now `Option<Pid>`.

Verified against the repro (checked beyond just the one triggering input, per `CONFIRMATION.md`
section 7 - both an unconnected socket producing `pid=0` *and* the pre-existing
`test_unix_peercred`/`test_unix_peercred_implicit` tests exercising the *normal*, nonzero-pid,
connected-socketpair case still pass and now correctly wrap in `Some`):

- Before (pristine `HEAD` 287214b8): repro prints `Err(Os { code: 1, ... })`, 3/3 fresh-clone runs.
- After (`patch.diff` applied): repro prints `Ok(UCred { pid: None, uid: Uid(4294967295), gid:
  Gid(4294967295) })`, 3/3 fresh-clone runs.
- `net::types::tests::test_sizes`'s `assert_eq_size!(UCred, libc::ucred)` still passes after the
  patch, confirming the ABI-visible size genuinely didn't change.
- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`):
  **411 passed, 1 failed, before and after** - identical counts, identical single failure
  (`ioctl::test_int_setter`, the pre-existing environment-caused failure documented in
  `___BUGHUNT___/projects/rustix.md`, "sandbox has no `/dev/net/tun`"). No new failures introduced,
  no pre-existing failure incidentally fixed.

## What's not done / open questions

- This is a breaking API change (`UCred.pid: Pid` -> `Option<Pid>`), same shape as candidate 019's
  `child_subreaper` fix - needs a semver-major release, a decision for the maintainers, not this
  session.
- **Cross-candidate note**: `___BUGHUNT___/063.ucred_niche_violation_scm_credentials/` (same
  session batch, written independently) targets the exact sibling site named below
  (`src/net/send_recv/msg.rs:572`'s `SCM_CREDENTIALS` parsing) and - based on its `patch.diff`,
  present in that candidate's folder at the time this was written, though its own `RESULT.md` had
  not yet been written - takes a different, non-breaking fix shape: it keeps `UCred.pid: Pid` and
  instead makes that one parsing site read through a plain niche-free `RawUCred` struct, discarding
  the whole ancillary message (returning `None` from that match arm) when the raw pid is `0`,
  rather than widening the public type. That is a real, mutually-inconsistent alternative to this
  candidate's fix (one keeps `Pid`, the other widens to `Option<Pid>`) for the *same* underlying
  type - whoever reconciles the two should pick one modeling choice for `UCred.pid` project-wide,
  not apply both patches as-is.
- `src/net/send_recv/msg.rs:572`'s `SCM_CREDENTIALS` ancillary-message parsing
  (`payload.as_ptr().cast::<UCred>().read_unaligned()`) shares the exact same raw-bytes-to-`UCred`
  hazard and is fixed for free by this same type change, but was not independently exercised by a
  dedicated repro this session (a sending peer can set an ancillary `ucred.pid` of `0` too, at
  least for an unprivileged sender whose passed credentials the kernel doesn't override - not
  verified end-to-end here; noted as a sibling site, not a separate finding).
- The originally-suspected PID-namespace-based trigger (a peer in a non-nested PID namespace) was
  not independently confirmed in this sandbox (no `CAP_SYS_ADMIN`/unprivileged userns available -
  see "What was tried"); it is very likely a second real trigger path for the identical bug
  (same kernel `cred_to_ucred()` function, same `pid=0` fallback), but this session's `Confirmed`
  status rests entirely on the unconnected-socket trigger, which needed no such privilege and was
  fully verified.
- Did not check other BSD/Apple/Solaris `SOCKCRED`/`LOCAL_PEERCRED`-style peer-credential APIs in
  this codebase (if any exist) for the same pattern - out of scope for a Linux-only sandbox.
