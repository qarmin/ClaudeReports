# recv_recvfrom_trunc_doc_gap

`src/net/send_recv/mod.rs`'s `recv`/`recvfrom` doc comments only say "the returned `length` may be
longer than the buffer length" under `RecvFlags::TRUNC`, with no mention of the more severe,
already-confirmed hazard (candidate 001): on a stream (e.g. TCP) socket, `TRUNC` can make the
kernel write *zero* bytes into the buffer while still reporting a nonzero length, so a reused
buffer silently keeps its previous contents. Candidate 005 added exactly this caveat to
`recvmsg`'s docs but the backlog noted `recv`/`recvfrom` were left out (out of scope for that
candidate). This candidate closes that documentation gap and, since candidate 001's own repro only
exercises the general poison-memory shape rather than isolating a plain `recv()` buffer-reuse
scenario in the same form as 005's `recvmsg` repro, independently re-demonstrates the exact
behavior the new doc text describes using `recv()` specifically.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading | `src/net/send_recv/mod.rs` (`recv`/`recvfrom` doc comments), cross-referenced against candidate 001's `RESULT.md`/`patch.diff` and candidate 005's already-applied (and reverted) doc patch to `recvmsg` | Confirmed the doc gap: `recv`/`recvfrom`'s function-level docs never mention the stream-socket-TRUNC buffer-untouched hazard that candidate 001 found and fixed in the *code*, unlike `recvmsg`'s docs which candidate 005 already updated. |
| Real repro, real TCP loopback (server thread sends two real, distinguishable 16-byte messages; client calls the crate's real public `recv()` twice, reusing one real buffer, second call with `RecvFlags::TRUNC`) | `cargo run --release`, 5 runs | 5/5 identical: after the `TRUNC` call, the buffer still holds the *first* message's bytes, not the second's, while the call reports a full 16-byte count - exactly the behavior the new doc text describes. |

## Status

**Confirmed**. 5/5 runs, release build, real TCP loopback socket. The underlying runtime hazard was
already established by candidate 001 (same root cause, same introducing commit); this candidate's
own repro independently reproduces it specifically for `recv()` (not `recvmsg()`, which 005 already
covered) to ground the doc addition in a fact actually demonstrated by this candidate's own repro,
not merely asserted by analogy to 001/005.

**Fully reproducible**: Yes. Real `TcpListener`/`TcpStream` server and client threads over a real
loopback socket, sending two real, distinguishable messages and reusing one real buffer across two
real `recv()` calls - no hand-constructed bytes/state.

## Class

`other` (documentation) - the runtime hazard itself is candidate 001's already-recorded
`memory-safety`/logic bug; this candidate's own contribution is purely closing a documentation gap
that left `recv`/`recvfrom` callers unwarned about a hazard `recvmsg`'s docs already disclose.

## Root cause

Candidate 005 added a caveat to `recvmsg`'s docs (and to the shared `RecvFlags::TRUNC` flag-level
doc comment, which `recv`/`recvfrom` also use) explaining the stream-socket TRUNC behavior, but
didn't touch `recv`/`recvfrom`'s own function-level doc comments in `src/net/send_recv/mod.rs`
(explicitly out of scope for that candidate, noted in its own "what's not done" section and carried
forward into this workspace's `IDEAS.md`). Those two functions' docs still only mention the
"length may be longer than the buffer" case, not the "buffer may be completely untouched, even
though nonzero bytes are reported" case - which is the more consequential one, since a caller could
easily miss it and trust stale/reused buffer contents as real received data.

## Introduced by

N/A in the strict sense (this is a documentation omission, not a code regression) - the gap has
existed since candidate 005's fix was written this same session; there is no earlier "introducing
commit" to trace beyond that.

## The fix

Added the same caveat candidate 005 wrote for `recvmsg` to `recv`'s and `recvfrom`'s own doc
comments in `src/net/send_recv/mod.rs`, adapted to each function's own return-value shape:

```rust
/// On Linux, when [`RecvFlags::TRUNC`] is used on a stream (e.g. TCP)
/// socket, the kernel discards the data without writing any of it into
/// `buf`, yet the returned byte count is still nonzero. In that case `buf`
/// is left completely unmodified; if it was reused from a previous call, the
/// `Buffer::Output` returned here will still reflect that previous call's
/// data, not the current one. See [`RecvFlags::TRUNC`] for details.
```

Verified: `cargo doc --no-deps --features "std,all-apis,use-libc"` builds cleanly (only a
pre-existing, unrelated `broken_intra_doc_links` warning on `sysproto` in `src/net/types.rs`,
present both before and after this patch); the repro's before/after output is identical, as
expected for a documentation-only change - the runtime hazard itself was never in this candidate's
scope to fix (it's candidate 001's own, already-recorded finding).

- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc"`): identical
  before and after - 51+14+70+16 passed, 1 pre-existing failure both before and after
  (`ioctl::test_int_setter`, unrelated `/dev/net/tun` absence, documented in `README.md`).

## What's not done / open questions

- This only closes the specific gap the backlog flagged (`recv`/`recvfrom`'s function-level docs).
  Not re-audited: whether any other doc comment elsewhere in the crate references `RecvFlags::TRUNC`
  or similar flags without the same caveat.
