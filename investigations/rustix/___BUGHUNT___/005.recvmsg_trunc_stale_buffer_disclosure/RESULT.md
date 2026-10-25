# recvmsg_trunc_stale_buffer_disclosure

`recvmsg` with `RecvFlags::TRUNC` on a stream (TCP) socket can silently leave a caller-reused
buffer holding a *previous* call's data while reporting a nonzero byte count for the *current*
call, with no signal in the return value distinguishing this from an ordinary successful read.
Sourced from a follow-up noticed while investigating candidate 001 (`recv`/`recvfrom`'s
uninitialized-memory bug under the same flag).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading | `src/net/send_recv/msg.rs::recvmsg`, both backends' raw `recvmsg` syscall wrappers | Confirmed `recvmsg` is a thin, faithful wrapper - no code inspects or clears the buffer itself |
| Real repro, both backends, real TCP loopback (server thread sends two real messages, client reuses one buffer across two real `recvmsg` calls) | `cargo run --release`, 3 runs per backend (6 total), plus a smaller-buffer variant | Buffer left 100% unchanged after the second (`TRUNC`) call in every run - still holds the first message's bytes |
| `git log --follow -p` | on `src/net/send_recv/msg.rs` | Confirmed this is inherent since `recvmsg` was first added, not a later regression |
| `cargo test --workspace --features "std,all-apis,use-libc"` before/after the doc fix | | No behavioral change (doc-only patch); one pre-existing unrelated failure (`/dev/net/tun` missing) present both before and after |

## Status

**Confirmed**. 6/6 identical runs (3x `libc` backend, 3x `linux_raw` backend), plus a smaller-buffer
variant showing the same unchanged-buffer result.

**Fully reproducible**: Yes. Real `TcpListener`/`TcpStream` server and client threads over a real
loopback socket, sending two real, distinguishable messages and reusing one real buffer across two
real `recvmsg` calls - no hand-constructed bytes/state.

## Class

`logic` / data-integrity (stale-data disclosure across API calls) - **not** memory-safety UB:
`IoSliceMut` requires already-initialized `&mut [u8]`, so no uninitialized memory is ever read; the
"leaked" bytes are legitimately-initialized (but semantically stale) Rust memory belonging to a
previous, unrelated `recvmsg` call.

## Root cause

`recvmsg` (`src/net/send_recv/msg.rs`) is a thin, faithful wrapper: both
`src/backend/libc/net/syscalls.rs::recvmsg` and `src/backend/linux_raw/net/syscalls.rs::recvmsg`
pass the caller's `iov`/`msghdr` straight to the kernel and just report back `(bytes, msg_flags)` -
no code in rustix inspects or clears the buffer. On Linux, passing `MSG_TRUNC` as an *input* flag
to `recvmsg`/`recv` on a **stream** socket does not behave like the documented UDP "report true
datagram size, copy the leading bytes" semantics. Instead the kernel fully discards the dequeued
stream bytes without copying **any** of them into the buffer - confirmed empirically:
- Buffer size == message size (16 bytes): `bytes=16`, buffer completely unchanged from the previous
  call, `ReturnFlags` = `0x0` (no `TRUNC` bit set at all).
- Buffer size (8) < message size (16): `bytes=8` (capped to buffer length, not the true stream
  amount - unlike UDP), buffer still completely unchanged, again `ReturnFlags` = `0x0`.

So on TCP, the `ReturnFlags::TRUNC` output bit that a UDP-experienced caller might check to detect
"this data is incomplete/suspect" is **never set** - the call looks indistinguishable from a normal
successful partial read, while the buffer is 100% untouched. This matches `recv(2)`'s own caveat
that `MSG_TRUNC`'s classic semantics are "not implemented for stream sockets" - Linux instead
reinterprets it as an internal "discard N bytes, don't copy" primitive.

## Introduced by

Not a regression - inherent since `recvmsg` was first added in 2cdbb1d4 "feat: Add sendmsg/recvmsg
(#505)" (2023-04-28). It's a direct consequence of exposing the raw Linux syscall/flag semantics
faithfully; no rustix-side logic error changed this behavior later
(`git log --follow -p -- src/net/send_recv/msg.rs` shows only doc/refactor changes since).

## The fix

**No safe bounded code fix exists** - see `patch.diff` for the documentation-only fix that was
found and verified instead. `recvmsg`'s buffer is caller-owned, pre-initialized memory. Zeroing it
before every call would be a real behavior/performance change many callers don't want, would mask
the exact same hazard when callers invoke the raw syscall or other libraries directly, and doesn't
even make the result "safer" (zeros are just as semantically meaningless as stale bytes if the
caller mistakes them for real data). Synthesizing a stronger flag would require an extra
`getsockopt(SO_TYPE)` per call and would mean rustix inventing behavior the kernel doesn't report -
a deviation from its role as a thin, faithful wrapper, and it can't fully solve the problem since
the caller explicitly opted into `TRUNC` themselves.

The bounded, safe, verified fix applied instead: a documentation-only patch clarifying this exact
hazard on `RecvFlags::TRUNC` (both backends' doc comments) and on `RecvMsg::bytes`, so a careful
caller reusing buffers across `recvmsg` calls with `TRUNC` on stream sockets is warned explicitly
in the API docs. Applied, verified (repro behavior unchanged, as expected for a doc-only change),
then reverted.

- Project test suite: `cargo test --workspace --features "std,all-apis,use-libc"` - identical
  results before and after (doc-only change): one pre-existing unrelated failure
  (`io::ioctl::test_int_setter`, needs `/dev/net/tun`, confirmed present on the unmodified tree
  too via a temporary stash), all other tests and doctests (including the 3 doctests in the
  touched `msg.rs` file) passing both before and after.

## What's not done / open questions

- `RecvFlags::TRUNC`'s doc comment on the sibling `recv`/`recvfrom` functions
  (`src/net/send_recv/mod.rs`) still uses UDP-shaped language ("the returned `length` may be
  greater than the length of the buffer") without the same TCP stale-data caveat - not touched in
  this candidate since the scope was specifically `recvmsg`/`msg.rs`, but the same doc clarification
  would apply there too (candidate 001 already fixed the actual *memory-safety* half of that
  function for `MaybeUninit` buffers; this is the documentation half for the pre-initialized
  `recvmsg` buffer path, which `recv`/`recvfrom` don't share since they don't take pre-initialized
  buffers in the same way).
- A maintainer might instead prefer a stronger code-level mitigation (e.g. an opt-in "clear buffer
  before TRUNC" wrapper, or surfacing a `getsockopt(SO_TYPE)`-derived warning) - this session judged
  that out of bounded scope and potentially against the crate's own thin-wrapper design philosophy,
  but flags it as a legitimate alternative a maintainer could choose over the doc-only fix applied
  here.
