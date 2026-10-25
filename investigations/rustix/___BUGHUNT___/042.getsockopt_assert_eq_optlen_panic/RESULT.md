# getsockopt_assert_eq_optlen_panic

The generic `getsockopt<T>` helper used by most of `rustix::net::sockopt`'s getters
(`src/backend/linux_raw/net/sockopt.rs:55`, and the analogous XDP-specific fallbacks at
`src/backend/linux_raw/net/sockopt.rs:1005`/`:1066` and `src/backend/libc/net/sockopt.rs:1208`/
`:1269`) enforces "the kernel always returns exactly `size_of::<T>()` (or, for the two XDP structs,
one of exactly two known sizes) bytes" via `assert_eq!(optlen, ..., "unexpected getsockopt size")`,
which **panics** rather than returning `io::Result::Err` if a future/different kernel ever returns
a third size - the same kind of transition that already happened once for `xdp_statistics`
(`xdp_statistics_v1` -> `xdp_statistics`, handled in commit 5648e33e "Add AF_XDP syscalls, constants
and types (#946)"). Flagged as a forward-compatibility concern distinct from candidate 012 (which
only swept level/optname literal mismatches, not this assert_eq size-invariant).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Exhaustive real-syscall sweep of every generic `getsockopt<T>` call site reachable without elevated privileges (`socket_type`, `socket_linger`, `socket_error`, `socket_domain`, `socket_protocol`, `socket_cookie`, `socket_incoming_cpu`, `socket_timeout`, `ip_ttl`, `ipv6_v6only`, `tcp_nodelay`, `tcp_keepcnt`, `tcp_keepidle`, `tcp_keepintvl`, `tcp_user_timeout`, `socket_peercred`) against real TCP/UDP/Unix sockets, `linux_raw` backend (default), this session's kernel (`7.0.0-34-generic`, x86_64) | Standalone crate, `cargo run` (debug build, so `debug_assert!` also active) | Zero panics - every call's real kernel-returned `optlen` matched `size_of::<T>()` exactly. See `output.txt` approach 1. |
| Same sweep, forced onto the `libc` backend (`RUSTFLAGS="--cfg rustix_use_libc"`) | Same standalone crate, same real sockets | Zero panics - same result under the libc backend's own generic `getsockopt<T>` (`src/backend/libc/net/sockopt.rs:90`, which actually uses a more lenient `assert!(optlen <= size_of::<T>())`, not `assert_eq!`, to tolerate Windows' documented 1-byte `TCP_NODELAY` quirk). See `output.txt` approach 2. |
| Attempted to exercise the one place a dual-struct-size fallback already exists for real (`xdp_statistics`/`xdp_mmap_offsets`, `SOL_XDP`) via a genuine `AF_XDP` socket - both directly through `rustix::net::socket(AddressFamily::from_raw(44), ...)` and via a standalone C program (`socket(AF_XDP, SOCK_RAW, 0)`), then via `unshare -Ur` (new user+net namespace) to try to self-grant `CAP_NET_RAW` | This sandboxed container, uid 1000, `CapEff: 0000000000000000`, `NoNewPrivs: 1` | `EPERM` on every attempt - creating an `AF_XDP` socket at all requires `CAP_NET_RAW`, which this sandbox has no way to obtain (`sudo` and `unshare` both refuse because of `no_new_privs`). The privileged XDP-specific fallback code paths can't be reached here at all, let alone with a kernel returning a third, not-yet-invented struct size. See `output.txt` approach 3. |

## Status

**Not reproducible.** No currently-reachable `getsockopt` call site, on the real kernel available in
this environment, ever returns an `optlen` that doesn't exactly match the Rust type expecting it -
confirmed across both backends via a real-syscall sweep (not a single run: both backend variants
were run, each deterministically producing the same all-`Ok`/no-panic result since there is no
kernel-version skew to hit on a single fixed kernel). The one call path already built to expect two
different kernel-returned sizes (`xdp_statistics`/`xdp_mmap_offsets`) could not be reached at all in
this sandbox (no `CAP_NET_RAW`, and no way to self-grant it here), so even the already-known,
already-handled two-size case was not directly exercised this session, let alone a hypothetical
third size. The underlying concern (a kernel that one day adds a third XDP statistics struct
revision, or a future growing struct elsewhere, panicking instead of erroring) is a real,
inspectable-from-the-code forward-compatibility risk, but it is not something any currently
obtainable kernel/environment combination will actually trigger - there is no real "second
participant" (a real kernel) that produces the mismatched condition today for this code to
mishandle.

**Fully reproducible**: N/A - Status is Not reproducible, not Confirmed.

## Root cause

N/A - not a presently-triggerable bug. The `assert_eq!`/`assert!` calls are real and really would
panic (not return `io::Errno`) if a future kernel ever handed back a size outside what's currently
coded for, but:

- Every non-XDP call site in this codebase passes a `T` whose size is a long-stable, architecture-
  fixed kernel ABI fact (plain `c_int`/`u32`/`u64`, or small fixed structs like `linger`/`ucred`/
  `sock_txtime`) - there is no "struct that grows release over release" (the `TCP_INFO`-style case)
  among rustix's actual `getsockopt<T>` callers today, so no existing call can observe a mismatch on
  any real Linux kernel.
- The one struct that *did* already grow this way (`xdp_statistics`, `xdp_statistics_v1` ->
  current) already has its own two-size fallback (not the generic `getsockopt<T>` helper at all),
  predating this candidate - this candidate is about what happens if a *third* revision is ever
  added, which no shipped kernel has done.
- That fallback path can't even be exercised in this sandbox regardless, since creating any
  `AF_XDP` socket needs `CAP_NET_RAW`, unavailable here (`CapEff` all-zero, `NoNewPrivs=1` blocking
  both `sudo` and `unshare`).

## Introduced by

N/A - not applicable, no bug confirmed. For context: the `assert_eq!("unexpected getsockopt size")`
pattern itself was introduced across 77c4aaea "Reorganize sockopts code and add several more socket
options (#855)" and a74f97f0 "Implement more socket options. (#858)"; the `xdp_statistics_v1`/
`xdp_statistics` two-size fallback that already handles the one real-world precedent for this class
of kernel skew was added in 5648e33e "Add AF_XDP syscalls, constants and types (#946)".

## The fix

No fix attempted. A real, in-scope fix (if the project's maintainers decide the forward-compat risk
is worth hardening against today, before any kernel actually ships a third size) would turn
`getsockopt<T>`'s `assert_eq!` into a real `io::Result::Err` (e.g. `Err(io::Errno::INVAL)` or a
dedicated error variant) when `optlen != size_of::<T>()`, and extend the two XDP-specific fallbacks'
final `assert_eq!` the same way instead of assuming the `else` branch is always the larger, current
size - but since no real kernel today ever takes that `else`-mismatched branch, there was no
reproducing input to verify such a change against this session, so no `patch.diff` is included
(a change made without a failing case to verify against isn't a verified fix, per this skill's own
confirmation bar).

- Project test suite: not run before/after, since no fix was applied - N/A.

## What's not done / open questions

- This sandbox's lack of `CAP_NET_RAW` means the existing `xdp_statistics_v1`/`xdp_statistics`
  fallback branch itself was never actually exercised against a real kernel this session (only
  read, not run) - a future session with real `AF_XDP` access (bare metal or a container with
  `--cap-add=NET_RAW` and a kernel built with `CONFIG_XDP_SOCKETS`) could at least confirm the
  *existing* two-size fallback still works against today's kernel, even though a third size still
  can't be manufactured.
- If the project's maintainers want this hardened preemptively, the fix sketch above would need
  someone to decide the right error to return (a new `io::Errno` variant vs. reusing an existing
  one) - not a decision this candidate makes unilaterally.
- Not checked whether any other `getsockopt<T>`-style helper exists outside `net::sockopt` (e.g. any
  other place `assert_eq!` gates a kernel-returned size against a fixed Rust type) - this candidate
  was scoped to the four locations named in the task.
