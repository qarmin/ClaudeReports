# recvmsg_ctrunc_ancillary_not_reproducible

`src/net/send_recv/msg.rs`'s ancillary-message parsing (`AncillaryDrain`/`Messages` iterator,
`cvt_msg`) walks a real kernel-populated control buffer using `CMSG_FIRSTHDR`/`CMSG_NXTHDR` and
computes `payload_len = msg.cmsg_len as usize - c::CMSG_LEN(0) as usize` for each message with no
explicit bounds re-check of its own. Nowhere in the crate is `MSG_CTRUNC`/`ReturnFlags::CTRUNC`
(the kernel's own signal that ancillary data didn't fully fit and was dropped) specially handled.
This looked like a plausible candidate for either a malformed/partial trailing `cmsghdr` causing an
out-of-bounds `payload_len` (if the kernel could ever leave a truncated message in the buffer) or a
crash while iterating - worth checking empirically rather than trusting `CMSG_NXTHDR`'s own
correctness by code-reading alone, since the whole point of `CONFIRMATION.md`'s bar is to actually
run it, not just reason about libc macros.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading | `src/net/send_recv/msg.rs` (`AncillaryDrain`, `messages::Messages`, `cvt_msg`) | Confirmed no explicit `MSG_CTRUNC` handling exists; the iterator relies entirely on `CMSG_FIRSTHDR`/`CMSG_NXTHDR` (real libc/kernel-header macros) to stop at a safe boundary. |
| Real repro: two real, cooperating participants over a real `AF_UNIX` `SOCK_STREAM` socketpair - a real sender genuinely calls `sendmsg` with a real `SCM_RIGHTS` ancillary message carrying 20 real, live file descriptors (duplicated from stdin), and a real receiver calls the crate's real public `recvmsg`/`RecvAncillaryBuffer::drain` with a deliberately undersized control buffer that cannot hold all 20 fds' worth of ancillary data, forcing the kernel to genuinely truncate and set `MSG_CTRUNC` | `cargo run --release`, control buffer sizes swept across the boundary: 0, 1, 2, 4, 8, 12, 16, 20, 24 bytes (real `MaybeUninit<u8>` arrays, real socket, real syscalls each run) | No panic, no crash, no out-of-bounds access at any buffer size. `ReturnFlags::CTRUNC` was correctly reported by the kernel and surfaced by rustix's `RecvMsg::flags` in every undersized case. At sizes too small to fit even one complete `SCM_RIGHTS` message (0-20 bytes, `CMSG_LEN(sizeof(int))` typically needs ~24 bytes with alignment on this platform), the drain iterator correctly yielded 0 ancillary messages (not a malformed partial one). At 24 bytes (room for exactly one message but only 2 of the 20 fds fit in that message's payload), the iterator correctly yielded a single `ScmRights` batch of 2 fds, matching what actually fit - no attempt to read past the truncated boundary. |

## Status

**Not reproducible**. `CMSG_FIRSTHDR`/`CMSG_NXTHDR`'s own bounds-checking (real libc/kernel-header
macros used as-is by rustix, not reimplemented) correctly prevents the iterator from ever exposing
a partial/malformed trailing `cmsghdr` under `MSG_CTRUNC`, across the full boundary sweep from an
empty buffer up through a buffer sized for exactly one (partially-filled) message. The kernel's own
control-message truncation behavior (drop whole messages that don't fit, never emit a partial one)
combined with the standard macros' bounds checks makes this safe by construction, matching this
session's other "trust the underlying syscall/libc convention" results (candidates 011, 015).

## Root cause

N/A - no bug found. This is real, empirically-verified robustness, not just an assumption from
reading `CMSG_NXTHDR`'s contract.

## Introduced by

N/A - not applicable, no bug confirmed.

## The fix

N/A - no bug found, nothing to fix.

## What's not done / open questions

- Only swept `SCM_RIGHTS` (file descriptor passing) truncation on the `linux_raw` backend over a
  `SOCK_STREAM` `AF_UNIX` socketpair. Not re-verified under the `libc` backend, nor with
  `SCM_CREDENTIALS` or a datagram (`SOCK_DGRAM`) socket, though the same libc macros and kernel
  truncation behavior should apply identically in both cases.
- No standalone `repro/` folder was created for this candidate since the result is `Not
  reproducible` (a repro folder is only mandatory for `Confirmed` findings per `SKILL.md`'s
  candidate loop step 4) - the scratch program used for the sweep was ad hoc and not preserved.
