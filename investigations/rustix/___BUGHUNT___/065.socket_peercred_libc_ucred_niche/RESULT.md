# socket_peercred_libc_ucred_niche

Sibling of candidate 064 (linux_raw backend `socket_peercred`) on the `libc` backend:
`src/backend/libc/net/sockopt.rs`'s `socket_peercred` (lines 1093-1097) routes through that
file's own generic `getsockopt<T: Copy>` (lines 89-111), which zero-initializes a
`MaybeUninit<UCred>`, has libc's `getsockopt()` write into it, and calls `assume_init()` after
only checking `optlen <= size_of::<T>()` - never validating `UCred.pid`'s `NonZeroI32` niche
(`Pid(NonZeroI32)`). Flagged because `SO_PEERCRED` can genuinely return `pid == 0` from the real
kernel (documented Linux behavior: an unconnected socket, or a peer not visible in the caller's
PID namespace), which is exactly the one bit pattern `Pid` promises never to hold.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real end-to-end repro: the actual public `rustix::net::sockopt::socket_peercred` entry point, called on a real AF_UNIX `SOCK_STREAM` socket (`rustix::net::socket`) that was never `connect()`'d/`accept()`'d - a real, single external participant (the kernel itself, genuinely invoked, nothing faked) producing a documented `pid == 0` `SO_PEERCRED` response | Debug + release builds, real `x86_64-unknown-linux-gnu` host kernel `7.0.0-34-generic`, 5/5 release + 1/1 debug runs during investigation; 4/4 further release runs against a fresh pristine clone | Every run: `socket_peercred` returned `Err(Os { code: -1, kind: Uncategorized, message: "Unknown error -1" })` - a nonsensical result (errno `-1` isn't a real errno), not a crash. A parallel raw-libc `getsockopt()` call on the same fd, in the same process, confirmed the real kernel genuinely reports `r=0` (success), `pid=0`, `uid=gid=u32::MAX` - i.e. rustix's own wrapper silently turned a real `Ok` kernel response into garbage |
| Miri, isolating the exact `assume_init()` pattern `getsockopt<T: Copy>` uses, fed the literal byte pattern the real kernel wrote above (`pid=0`, `uid=gid=u32::MAX`) | `cargo +nightly miri run`, 1 run (deterministic) | `error: Undefined Behavior: constructing invalid value of type UCred: at .pid.0.0.0, encountered 0, but expected something greater or equal to 1` - confirms the mechanism directly: this is real, Miri-diagnosed UB (invalid-niche-value construction), not just a coincidentally-wrong result |
| Sanity check: same repro's Case 2, a real connected `socketpair()` (legitimate nonzero pid) | Same builds/runs as above | `socket_peercred` correctly returns `Ok(UCred { pid, uid, gid })` with the real pid every time - confirms the bug is specific to the `pid == 0` kernel response, not a general breakage |

## Status

**Confirmed**. Reproduces 100% (5/5 release + 1/1 debug during investigation, plus 4/4 further
release runs against a fresh pristine clone via `just test` - 9/9 release + 1/1 debug total, no
flakiness, same build/environment each time) via the real end-to-end public API, independently
corroborated by a deterministic Miri UB diagnostic isolating the exact mechanism.

**Fully reproducible**: **Yes**. The repro drives the real, unmodified `rustix::net::sockopt::
socket_peercred` public entry point end-to-end, on a real kernel-produced `SO_PEERCRED` response.
No second participant needed faking: the single real external input here is the Linux kernel's
own, genuinely-invoked `getsockopt()` syscall response to a real (never-connected) socket - the
same "a malformed file handed to a parser's real public entry point IS the genuine external
input" shape `CONFIRMATION.md` section 11 describes as not needing a second participant.

## Class

memory-safety (UB: constructing an invalid niche value via `assume_init()`), manifesting
externally as a logic bug (a real, successful kernel response silently turned into a nonsensical
`Err(-1)`).

## Root cause

`UCred` (`src/net/types.rs:2093`) models the peer's process ID as `pub pid: crate::pid::Pid`,
where `Pid` is `#[repr(transparent)] struct Pid(NonZeroI32)` - the type promises its inner `i32`
is never `0`. `socket_peercred`'s generic `getsockopt<T: Copy>` helper (`src/backend/libc/net/
sockopt.rs:89-111`) zero-initializes a `MaybeUninit<UCred>`, lets the raw C `getsockopt()` call
overwrite it, and then unconditionally calls `value.assume_init()` - the only check performed is
`optlen <= size_of::<T>()` (a size check), never a validity check on the bytes actually written.

Linux's real `SO_PEERCRED` legitimately reports `pid == 0` in (at least) two cases documented in
`unix(7)`/observed directly in this session: an unconnected `AF_UNIX SOCK_STREAM` socket (no
error - `getsockopt` returns success with `pid=0, uid=gid=-1`), and a connected peer whose PID
isn't visible in the caller's PID namespace. Either way, the raw bytes `getsockopt()` writes can
be a bit-for-bit valid `UCred` from C's point of view while violating `Pid`'s own `NonZeroI32`
invariant - exactly the "invalid value" Miri's validity checker flags.

Because `Result<UCred, Errno>` is niche-optimized around `Pid`'s `NonZeroI32` niche (no separate
discriminant byte - `Errno` itself is a plain `#[repr(transparent)] struct Errno(c_int)` with no
niche of its own), writing `pid == 0` into a value that is then read back as `Ok(UCred)` collides
with the bit pattern the compiler reserved to mean `Err(Errno)`. In this environment/build, that
manifests as the caller observing `Err(Errno(-1))` instead of the real, successful `Ok(UCred {
pid: 0, .. })` the kernel actually produced - a concrete, directly observed instance of exactly
the kind of miscompilation the niche-optimization invariant exists to prevent.

## Introduced by

62afd55c "Add wrappers for `SO_PEERCRED` and `SCM_CREDENTIALS` (#851)" (2023-10-04). This commit
added both `UCred` itself (`src/net/types.rs`, `pid: crate::pid::Pid`) and the libc backend's
`get_socket_peercred` (later renamed `socket_peercred`) calling the existing generic
`getsockopt<T: Copy>` with `T = UCred` - the combination of a `NonZeroI32`-backed field in `T`
plus the pre-existing unconditional `assume_init()` is what creates the bug; neither half alone
is new in this commit's own history (the generic `getsockopt` helper predates it), but this is the
first commit where `UCred`'s niche-bearing `pid: Pid` field flows through it.

## The fix

Changed `socket_peercred` (libc backend only; the linux_raw sibling is candidate 064's own scope)
to call `getsockopt` with the raw C `c::ucred` type (plain `pid_t`/`uid_t`/`gid_t`, no niche)
instead of `UCred` directly, then explicitly validate the raw `pid` via the existing, already
fallible `Pid::from_raw` before ever constructing a `UCred`:

```rust
#[cfg(linux_kernel)]
#[inline]
pub(crate) fn socket_peercred(fd: BorrowedFd<'_>) -> io::Result<UCred> {
    let creds: c::ucred = getsockopt(fd, c::SOL_SOCKET, c::SO_PEERCRED)?;
    Ok(UCred {
        pid: crate::pid::Pid::from_raw(creds.pid).ok_or(io::Errno::INVAL)?,
        uid: crate::ugid::Uid::from_raw(creds.uid),
        gid: crate::ugid::Gid::from_raw(creds.gid),
    })
}
```

This never calls `assume_init()` on a `UCred` whose bytes weren't already validated field-by-field
- a `pid == 0` kernel response now surfaces as a clean, well-defined `Err(io::Errno::INVAL)`
instead of the previous nonsensical/UB-derived result. `Uid`/`Gid` have no niche of their own
(plain `u32` wrappers, `from_raw` is infallible), so only `pid` needed the fallible path.

Verified against both the originally-triggering input (`pid=0, uid=gid=u32::MAX`, as the real
kernel wrote for an unconnected socket - now `Err(EINVAL)`, previously `Err(-1)` nonsense) and a
second input in the same "`Pid`-niche" class checked directly under Miri (`pid=1234` legitimate -
still `Ok`, unaffected), plus the real end-to-end repro's own Case 2 (a real connected
`socketpair()`, legitimate nonzero pid) confirming the normal/legitimate path is unaffected before
and after the patch.

- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc" --quiet
  --no-fail-fast`): **411 passed / 1 failed / 5 ignored before, identical 411 passed / 1 failed /
  5 ignored after** - no new failures, no count change. The 1 failure both times is
  `ioctl::test_int_setter` (`NotFound` opening `/dev/net/tun`), a pre-existing environment-only
  gap documented in `projects/rustix.md`, unrelated to this change.

## What's not done / open questions

- **This fix does not change `UCred.pid`'s public type** (`Pid`, not `Option<Pid>`) - it trades
  "silently construct an invalid `Pid` (UB)" for "return `Err(EINVAL)` when the kernel's own
  answer can't be represented", which is sound but means a caller legitimately querying an
  unconnected socket or a cross-namespace peer now gets an error rather than `Ok(UCred { pid:
  None, .. })`. A type-level fix (`UCred.pid: Option<Pid>`, matching how `Pid::from_raw` already
  models "0 means no pid") would preserve more information but is a breaking API change (same
  shape as candidate 019's `child_subreaper` `Option<Pid>` -> `bool` fix) - not attempted this
  session, left as a design decision for whoever reviews this patch.
- **The linux_raw backend's identical-shaped `socket_peercred`** (`src/backend/linux_raw/net/
  sockopt.rs:892-894`) is explicitly out of scope here - see candidate 064, which covers it
  separately (same bug class, different syscall wrapper).
- **The generic `getsockopt<T: Copy>` helper itself** (`src/backend/libc/net/sockopt.rs:89-111`)
  is unchanged and still unconditionally `assume_init()`s whatever bytes `getsockopt()` writes for
  every other `T` it's used with - this fix is narrowly scoped to `socket_peercred`'s own call
  site, not a systemic change to the helper. Candidate 050's prior sweep already covered several
  other `assume_init()` call sites in this backend (and ruled most out as safe/untestable) but did
  not specifically check `UCred`/`socket_peercred` - worth noting in `projects/rustix.md` as a
  confirmed instance of the pattern that sweep was looking for.
- Only tested on this session's real kernel (`7.0.0-34-generic`, x86_64, real hardware, no
  container/VM emulation beyond the usual sandbox) and only via the "unconnected socket" trigger
  for `pid == 0` - the "peer not visible in caller's PID namespace" trigger documented in
  `unix(7)` was not independently exercised (this sandbox has no `CAP_SYS_ADMIN`/unprivileged-
  userns access: `unshare --user --pid` and a raw `unshare(CLONE_NEWUSER)` syscall both fail with
  `EPERM`, and `sudo` is blocked by `no_new_privs`), but both triggers produce the exact same
  `pid == 0` byte pattern at the kernel/libc boundary this candidate's repro already exercises
  directly, so this is not expected to behave differently.
