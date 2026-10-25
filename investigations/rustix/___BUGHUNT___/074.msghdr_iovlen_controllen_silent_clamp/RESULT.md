# msghdr_iovlen_controllen_silent_clamp

`src/backend/libc/net/msghdr.rs`'s `msg_iov_len` (lines 32-46, the `c::c_int`-returning variant
active on musl/BSD/solarish/etc., as opposed to the `c::size_t` passthrough active on
glibc-linux/android/redox) and `msg_control_len` (lines 50-66, the `c::socklen_t`-returning variant
active on bsd/solarish/musl/aix/cygwin/emscripten/fuchsia/haiku/hurd/nto) both convert a `usize`
length into the platform's native, narrower `msghdr` field type via
`len.try_into().unwrap_or(T::MAX)` - silently clamping to the field type's max instead of
surfacing an error when the real iovec count or control-buffer byte length doesn't fit, while the
full iov/control buffer is still passed to the kernel by pointer unclamped. Looked suspicious
because, unlike `readv`/`writev`'s analogous clamp (which only ever under-reports a count the
kernel would read less of, safely), a caller here could plausibly end up with a successful
`sendmsg`/`recvmsg` call that silently transmits/receives less data than requested, with no error
surfaced anywhere.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading + `cfg` gate analysis: traced exactly which targets activate the clamped (`c_int`/`socklen_t`) variants of `msg_iov_len`/`msg_control_len` vs. the unclamped passthrough variants, and confirmed real field widths | Read `src/backend/libc/net/msghdr.rs:14-82` in full; cross-compiled a standalone `libc` field-size probe for `x86_64-unknown-linux-musl` (`rustup target add x86_64-unknown-linux-musl`, no musl-gcc needed - Rust's musl target is fully static and runs directly on this Linux host) | Confirmed: on glibc Linux (the default dev/test target), neither clamp is active at all (`msg_iov_len` returns `c::size_t` directly, `msg_control_len` likewise) - the clamp only exists on musl/BSD/solarish/etc. On musl specifically, `size_of_val(&msghdr.msg_iovlen) == 4` and `size_of_val(&msghdr.msg_controllen) == 4`, confirming both clamped variants are genuinely reachable there, and that a real musl binary can be built and run directly on this host to exercise them |
| Live, real-kernel boundary test: built `rustix` for `x86_64-unknown-linux-musl` (the target where both clamps are compiled in) and drove the real public `sendmsg` API over a real `AF_UNIX` `SOCK_DGRAM` `socketpair()`, varying `iov.len()` and the real `SendAncillaryBuffer` control-byte length across the kernel's own actual limits, to see whether a real syscall can ever reach the clamp's `> i32::MAX` (iovcnt) / `> u32::MAX` (control bytes) threshold while still succeeding | `cargo run --target x86_64-unknown-linux-musl --release` against a standalone probe (`investigation/probe_main.rs`) sending 1024 and 1025 1-byte iovec entries, and 1,000 / 32,500 / 33,000 / 100,000 duplicate-fd `SCM_RIGHTS` entries (control buffer 4,024 to 400,024 bytes) over a real loopback `AF_UNIX` socket on this container's real kernel (`net.core.optmem_max = 131072`) | iovcnt: 1024 succeeds (`Ok(1024)`), 1025 fails with real `EMSGSIZE` ("Message too large", errno 90) - i.e. the kernel's own hard-coded `UIO_MAXIOV = 1024` limit (confirmed in `/usr/include/linux/uio.h`, also `getconf IOV_MAX` = 1024) rejects the call roughly **2,097,152x** below the clamp's `c_int::MAX` (2,147,483,647) threshold. Control bytes: fails with real `EINVAL` below ~131,072 bytes (likely `SCM_MAX_FD` exceeded by the duplicate-fd count used to grow the buffer) and real `ENOBUFS` ("No buffer space available", errno 105) at and above ~131,072-132,024 bytes, matching `optmem_max` almost exactly - roughly **32,768x** below the clamp's `socklen_t::MAX` (4,294,967,295) threshold. In both cases the real kernel always produces a real, correct error long before a length anywhere near the clamp's trigger point is reachable |
| Boundary-value construction: isolated the exact `len.try_into().unwrap_or(T::MAX)` logic from both functions into a standalone program and ran it directly on `c_int`/`u32` boundary values (0, a realistic value, `T::MAX`, `T::MAX+1`, `T::MAX+1_000_000`) | `rustc --edition 2021 -O` + run, `investigation/clamp_check.rs` | Confirms the clamp does exactly what the source implies: `msg_iov_len(2147483648) == 2147483647`, `msg_control_len(4294967296) == 4294967295` - i.e. the silent truncation described in the candidate is real *as a pure function*, the open question was only ever whether any real caller can actually reach that input range through a real syscall |

## Status

**Not reproducible** - as a scenario where a real `sendmsg`/`recvmsg` call succeeds while silently
transmitting/receiving less data than the caller asked for. Covered: full `cfg`-gate tracing of
which targets activate the clamp at all (glibc Linux - the common case - never does); a live,
real-syscall boundary sweep on the one target confirmed to compile in both clamped variants
(`x86_64-unknown-linux-musl`, run directly on this host since musl targets are statically linked);
and isolated boundary-value confirmation of the clamp functions' own arithmetic. No combination
produced a real `sendmsg`/`recvmsg` call that both (a) carried a length needing the clamp
(`> i32::MAX` iovec entries, or `> u32::MAX` control bytes) and (b) succeeded - the real kernel's
own, vastly smaller limits (`UIO_MAXIOV = 1024`, a compile-time kernel constant not reachable via
any sysctl; `optmem_max`, a sysctl-tunable `int` whose maximum possible value, `INT_MAX =
2,147,483,647`, is itself still below the `socklen_t::MAX = 4,294,967,295` threshold where
`msg_control_len`'s clamp would matter) always reject the call first, with a real, correct error
(`EMSGSIZE`/`ENOBUFS`/`EINVAL`), before the clamp's own silent-truncation effect could ever change
the outcome of a call that would otherwise have succeeded.

**Fully reproducible**: N/A (no reproduction to report - see Status).

## Root cause

Not applicable - no bug confirmed as actually reachable. The code pattern (`unwrap_or(T::MAX)`
instead of propagating an error) is real and is a legitimate, independent correctness smell: it
*would* silently misbehave if ever fed a length past the field-type boundary, matching the
candidate's hypothesis exactly at the pure-function level (see "Boundary-value construction"
above). But every real path that could grow `iov.len()` or the ancillary control buffer's byte
length up to that boundary is gated by an much smaller, independent OS-level limit that the kernel
enforces *before* any data transfer happens:

- `msg_iov_len`'s clamp triggers only once `iov.len() > i32::MAX` (2,147,483,647 iovec entries,
  which alone would require the `&[IoSlice]`/`&[IoSliceMut]` array itself to occupy >32 GiB of
  contiguous memory on a 64-bit host, since each `IoSlice`/`IoSliceMut` is a 16-byte `(ptr, len)`
  pair). Linux's own `sendmsg`/`recvmsg`/`readv`/`writev` implementation rejects any call with more
  than `UIO_MAXIOV` (1024) iovec entries with `-EMSGSIZE`, a limit that has been a compile-time
  kernel constant (`include/linux/uio.h`) since long before `usize`-vs-`c_int` width even became a
  concern, and is not configurable via any sysctl - so no caller, however the OS is configured, can
  ever present the kernel with an iovec count anywhere close to where this clamp would bite.
- `msg_control_len`'s clamp triggers only once the control buffer exceeds `u32::MAX` (4,294,967,295)
  bytes. The kernel's ancillary-data path (`sock_kmalloc`, used to stage `SCM_RIGHTS`/credentials/
  etc. data) is bounded by `net.core.optmem_max`, a per-socket ancillary/sockopt memory ceiling -
  131,072 bytes by default on this system (and confirmed experimentally: a real `sendmsg` call
  starts failing with real `ENOBUFS` right around that exact value). `optmem_max` is itself a
  plain signed `int` sysctl, so even a maximally, adversarially misconfigured system can never raise
  it above `INT_MAX = 2,147,483,647` - which is still well under the `socklen_t::MAX =
  4,294,967,295` threshold needed to trigger this clamp. (For the symmetric `recvmsg`/
  `RecvAncillaryBuffer` direction: a caller *can* allocate an arbitrarily large local receive
  buffer, since that's plain user-space memory with no kernel ceiling - but the amount of ancillary
  data the kernel could ever actually have queued to copy back out is bounded by the same
  `optmem_max` ceiling on however it was originally enqueued, so the clamp on the "available space"
  field it reports to the kernel is likewise never the limiting factor.)

In short: the clamp functions are real, and are exactly as described in the candidate at the level
of pure `usize -> T` conversion, but the only inputs that could make them actually clamp (rather
than pass the true value through unchanged) are many orders of magnitude past what any real
`sendmsg`/`recvmsg` syscall on Linux can ever carry, because an independent, much tighter kernel
limit always intervenes first and turns the call into a real, correctly-reported error instead.

## Introduced by

N/A - no bug confirmed, nothing to bisect.

## The fix

N/A - no bug found to be practically reachable, so no `patch.diff` in this folder. (The
`unwrap_or(T::MAX)` pattern could still be changed to propagate an error for defense-in-depth /
code-clarity reasons even though it's unreachable in practice on any currently known target+kernel
combination - that's a style judgment call for maintainers, not something this session is treating
as a confirmed bug needing a patch.)

- Project test suite: not run as part of this candidate (no fix to validate - see "Candidate loop"
  step 7, mandatory before/after runs apply only when a fix was applied). N/A.

## What's not done / open questions

- Only `x86_64-unknown-linux-musl` was actually exercised live (the only clamp-affected target this
  container can both cross-compile *and run directly*, since musl binaries are statically linked).
  The BSD/solarish/etc. targets that also activate `msg_control_len`'s clamp were not independently
  verified against a real BSD/Solaris kernel - no such environment was available this session. The
  root-cause argument above (an OS-level ancillary-data ceiling vastly below `socklen_t::MAX`) is
  expected to generalize, since all mainstream Unix kernels impose some such ceiling on socket
  ancillary memory, but this wasn't independently confirmed on non-Linux kernels.
- `optmem_max`'s absolute ceiling being below `socklen_t::MAX` was reasoned from the sysctl's known
  `int` type (standard, long-documented Linux behavior) rather than from an actual write to
  `/proc/sys/net/core/optmem_max` in this session - this container's sandboxed environment has no
  root access (`sudo` is blocked by a "no new privileges" flag), so raising the sysctl to its
  maximum and re-testing directly wasn't possible.
- Did not attempt to drive `recvmsg`'s `RecvAncillaryBuffer` control-length clamp through a real
  syscall (only reasoned about it analytically in "Root cause" above) - a receive-side live test
  would need a sender that can actually enqueue close to `optmem_max` bytes of real ancillary data
  (shown above to itself be rejected well below that point by the same kernel limits), so it was
  judged not to add signal beyond the analytical argument.
- The `readv`/`writev`-side analogue this candidate explicitly contrasts itself with (described as
  "already safe") wasn't independently re-verified this session - taken as given from the candidate
  description.
