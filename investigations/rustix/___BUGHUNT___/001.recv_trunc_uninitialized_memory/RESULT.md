# recv_trunc_uninitialized_memory

`rustix::net::recv()`/`recvfrom()` can certify uninitialized caller memory as valid received data
when called with `RecvFlags::TRUNC` on a stream (TCP) socket. Sourced from GitHub issue #1654
("net::recv with TRUNC flag is unsound"), which already included a proof-of-concept.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading | `src/net/send_recv/mod.rs` `recv`/`recvfrom`, `src/buffer.rs`'s `Buffer::assume_init` contract, both backends' raw `recv`/`recvfrom` syscall wrappers | Confirmed the `min(len, recv_len)`-then-`assume_init` shape and that neither backend can know how many bytes the kernel actually wrote when `TRUNC` is set |
| Standalone repro, real TCP loopback (real server thread + real client thread, real socket) | `cargo run --release`, 4 cases: equal-size+TRUNC, larger-buffer+TRUNC, smaller-buffer+TRUNC, no-TRUNC control | Poison bytes (`0xAA`) returned as "real received data" in all 3 TRUNC cases; control case correctly returns the real sent bytes |
| `git log -S` | on `"min_len = min(len, recv_len)"` | Pinned the introducing commit |
| `cargo test --workspace` (broadened features) before/after fix | `--features "std,net,fs,event,process,thread,time,mm,pipe,rand,stdio,system,termios,pty,shm,mount,use-libc"` | Revealed a real behavioral tradeoff - see "What's not done" below |

## Status

**Confirmed**. 21/21 individual TRUNC sub-case observations across 7 runs of the unpatched repro
(5 initial single-case runs + 2 full four-case runs) all showed pure poison bytes falsely
certified as initialized; 0 misses. Post-fix: 3/3 clean runs showing the bug is gone (0 bytes
claimed under `TRUNC`, `recv_len` still reported correctly).

**Fully reproducible**: Yes. The repro runs a real server thread (`TcpListener`/`TcpStream`,
writing a real, recognizable 16-byte pattern) and a real client thread calling
`rustix::net::recv()` with `RecvFlags::TRUNC` over a real loopback TCP connection - the actual
public entry point, driven by a real second participant, not a hand-constructed `recv_len` or
fake syscall return.

## Class

`memory-safety` / `security` - uninitialized-memory disclosure. The caller-supplied
`MaybeUninit<u8>` buffer's leftover contents (stack/heap poison, in a real attack scenario:
potentially sensitive leftover data from a previous allocation) are certified via
`Buffer::assume_init` as valid, freshly-received socket data, even though the kernel wrote zero
bytes into the buffer when `MSG_TRUNC` discards a stream socket's data.

## Root cause

In `src/net/send_recv/mod.rs`, both `recv()` and `recvfrom()` compute `min_len = min(len,
recv_len)` and then call `buf.assume_init(min_len)`, asserting that `min_len` bytes of the
caller's buffer are now initialized. This assumption holds for ordinary `recv()`, but breaks under
`RecvFlags::TRUNC`: per `man 7 tcp`, "This flag causes the received bytes of data to be discarded,
rather than passed back in a caller-supplied buffer" - for TCP (stream) sockets, `MSG_TRUNC` makes
the kernel write **zero** bytes into the buffer while still returning the full/consumed byte count
as the syscall's return value (`recv_len`). Both backends (`src/backend/libc/net/syscalls.rs` and
`src/backend/linux_raw/net/syscalls.rs`) just forward the raw kernel return value as `recv_len`
with no way to know how many real bytes were written - so `min(len, recv_len)` bytes of
leftover/poison caller memory get incorrectly certified as initialized via `Buffer::assume_init`
(`src/buffer.rs`, whose contract requires "At least `len` elements of the buffer must now be
initialized").

**Important nuance**: this discard-everything behavior is TCP(stream)-specific. For datagram
sockets (UDP, Unix `SOCK_DGRAM` - as covered by rustix's own existing test
`tests/net/recv_trunc.rs::net_recv_uninit_trunc`), `MSG_TRUNC` legitimately copies real data (up
to buffer length) into the buffer while reporting the true, possibly-larger, datagram size in
`recv_len` - this is the intended, safe, commonly-used idiom for datagram protocols. rustix's
generic `recv()`/`recvfrom()` (parameterized only over `AsFd`) cannot distinguish socket type
without an extra `getsockopt(SO_TYPE)` syscall, so no fix that avoids adding a syscall can be
exactly correct for both stream and datagram sockets simultaneously.

## Introduced by

45ee8d25 "Introduce a new `Buffer` trait. (#1290)" (Dan Gohman, 2025-02-26), which introduced the
`Buffer`/`assume_init` abstraction and the `min(len, recv_len)`-then-`assume_init` shape in
`recv`/`recvfrom`. Found via `git log -S "min_len = min(len, recv_len)"`.

## The fix

See `patch.diff`: when `RecvFlags::TRUNC` is set, treat 0 bytes as guaranteed initialized instead
of `min(len, recv_len)`, while still returning the real, unmodified `recv_len` to the caller.
Justification: rustix's generic `recv`/`recvfrom` don't know the socket type, and TCP's documented
`MSG_TRUNC` behavior (discard everything, write nothing) is impossible to distinguish from
datagram-socket `MSG_TRUNC` behavior (write a real prefix) without an extra `getsockopt(SO_TYPE)`
call. Zero is the only value safe to assume in the worst case (stream sockets) while preserving
the syscall-calling convention (no added syscall, no signature change).

Verified against more than the single triggering input: buffer equal to, larger than, and smaller
than the sent size, all under `TRUNC`, plus a non-`TRUNC` control case confirming ordinary `recv`
is unaffected.

- Project test suite: `cargo test --workspace --no-fail-fast --features "std,net,fs,event,process,thread,time,mm,pipe,rand,stdio,system,termios,pty,shm,mount,use-libc"` (the default `cargo test --workspace` only enables `std` and doesn't exercise `net` at all). Before: 1 pre-existing unrelated failure (`ioctl::test_int_setter`, sandbox/environment-specific), all 84 `net` tests passing. After: same pre-existing failure, **plus one new failure**: `tests/net/recv_trunc.rs::net_recv_uninit_trunc` (83/84 net tests passing) - this is a direct, expected consequence of the fix's conservative choice, not a sign of an incorrect patch; see below.

## What's not done / open questions

- **The one new test failure is a real, load-bearing design tradeoff, not a fix defect.** The
  existing test `net_recv_uninit_trunc` uses a Unix-domain `SOCK_DGRAM` socket and asserts that
  `MSG_TRUNC` returns the real first N bytes of a datagram into a smaller buffer - legitimate,
  safe behavior for datagram sockets that this session's conservative fix zeroes out too, because
  it can't generically distinguish socket type. A maintainer finalizing this needs to choose
  between: (a) accepting the conservative "0 for all TRUNC" fix and updating/removing that
  datagram-socket test's expectation (documenting that `recv`/`recvfrom` with `TRUNC` on a
  `MaybeUninit` buffer no longer guarantee any real data prefix, even for datagram sockets), or (b)
  adding a `getsockopt(SO_TYPE)` check (one extra syscall) to preserve the legitimate datagram
  behavior while still zeroing for stream sockets. This session implemented (a) since it needed no
  new syscalls or signature changes and is unconditionally memory-safe; (b) is a viable, more
  precise alternative left for the maintainers.
- **`recvmsg` (`src/net/send_recv/msg.rs`) has a related but different-severity issue, not fixed
  here**: it takes `IoSliceMut` (pre-initialized `&mut [u8]`), so there's no `assume_init`/
  uninitialized-memory unsoundness, but with `MSG_TRUNC` on a TCP socket it returns
  `ReturnFlags::TRUNC` while leaving the caller's buffer completely unchanged - meaning the caller
  can observe *stale bytes from a previous `recvmsg` call on the same buffer*, misattributed to
  the current call. This is a data-integrity/security-adjacent logic bug (not UB) worth a
  dedicated future candidate with its own repro (reuse one buffer across two `recvmsg` calls to
  prove stale-data leakage) - added to `IDEAS.md`.
- Other `Buffer`-based `assume_init` call sites introduced by the same commit (45ee8d25) that
  accept flags capable of "kernel reports more/other than what it actually wrote" were not swept
  for the same shape this session (e.g. other `MSG_TRUNC`-consuming APIs, `epoll`/`kqueue`/`readv`
  if any share this pattern) - flagged in `IDEAS.md` as a follow-up grep, not investigated.
