# xdp_sockaddr_ifindex_queue_id_byte_swap

`read_sockaddr_xdp` in both backends (`src/backend/linux_raw/net/read_sockaddr.rs:142-143`,
`src/backend/libc/net/read_sockaddr.rs:250-251`) decodes `sockaddr_xdp.sxdp_ifindex`/
`sxdp_queue_id` with `u32::from_be(..)`, treating them as network-byte-order fields, while the
write side (`SocketAddrArg for SocketAddrXdp` / `SocketAddrXdpWithSharedUmem`, `src/net/types.rs`)
stores the identical fields straight through with no byte-swap - an asymmetric encode/decode pair
for the same wire struct.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | Compared write side (`src/net/types.rs:1881-1882`, `:1929-1930`) against read side (both backends' `read_sockaddr_xdp`) | Confirmed the asymmetry: write passes `interface_index()`/`queue_id()` through unchanged, read applies `u32::from_be(..)` to both |
| Upstream kernel source reading | Fetched today's `torvalds/linux` master `include/uapi/linux/if_xdp.h` and `net/xdp/xsk.c` | `struct sockaddr_xdp`'s `sxdp_ifindex`/`sxdp_queue_id` are plain `__u32`, no byte-order annotation; `xsk_bind()` uses `sxdp->sxdp_ifindex` directly with `dev_get_by_index()`, no `ntohl`/`be32_to_cpu` anywhere - confirms host-endian semantics, matching the write side, not the read side |
| Real-kernel end-to-end attempt (`bind()` + `getsockname()`/`recvmsg()`) | Raw C probe: `socket(AF_XDP, SOCK_RAW, 0)` | Failed: `EPERM` - this sandbox has no `CAP_NET_RAW` (confirmed via `/proc/self/status`'s `CapBnd`/`CapEff`, both missing `cap_net_raw`) |
| Upstream kernel source reading (2nd check, to see if a real kernel round trip was possible in principle elsewhere) | `net/xdp/xsk.c`'s `xsk_proto_ops` | `.getname = sock_no_getname` (getsockname/getpeername always `-EOPNOTSUPP` for AF_XDP, unconditionally); `xsk_recvmsg()`/`__xsk_recvmsg()` never touch `msg_name`/`msg_namelen` (AF_XDP delivers data via mmap'd UMEM rings, not `recvmsg`'s buffer); `.accept`/`.connect` are `sock_no_accept`/`sock_no_connect`. So no real Linux syscall currently returns a `sockaddr_xdp` to userspace at all, independent of privilege |
| Direct round-trip via rustix's own public API (`SocketAddrArg::as_any()` then `TryFrom<SocketAddrAny>`) | Debug and release builds, both backends (`net,fs` and `net,fs,use-libc` features), 5 input pairs including boundary values (0, 1, u32::MAX, a mixed-bit pattern, and the motivating 2/5) | Reproduces: every nonzero, non-`u32::MAX` input decodes to a wrong value (e.g. ifindex 2 -> 33554432, queue_id 5 -> 83886080) on this little-endian x86_64 host, on both backends |

## Status

**Confirmed**

Reproduces 3/3 repeat runs (separate process invocations), deterministically, on both the
linux_raw and libc backends, in debug and release builds - this is a pure integer byte-swap bug
with no layout/timing dependence, so determinism is expected and was verified rather than assumed.

**Fully reproducible**: **No**, justified.

A genuine end-to-end kernel round trip (rustix's real `bind()` wrapper, then a real
`getsockname()`/`recvmsg()` call reading back what the kernel actually stored) was attempted and
could not be built, for two independent reasons:

1. Creating an `AF_XDP` socket at all requires `CAP_NET_RAW`, unavailable in this sandbox (a raw
   `socket(AF_XDP, SOCK_RAW, 0)` C probe failed with `EPERM`).
2. More fundamentally, independent of privilege: reading today's upstream `net/xdp/xsk.c` shows
   AF_XDP's own kernel `proto_ops` sets `.getname = sock_no_getname` (`getsockname()`/
   `getpeername()` always fail `-EOPNOTSUPP`), and `xsk_recvmsg()` never populates `msg_name` (data
   moves through mmap'd UMEM rings, not `recvmsg`'s buffer). So there is currently no real Linux
   syscall path at all that hands a `sockaddr_xdp` back to userspace for `read_sockaddr_xdp` to
   parse against genuinely kernel-produced bytes - this isn't a this-session time/hardware
   limitation, it's the real kernel's current AF_XDP semantics.

Given that, `repro/` instead round-trips a `SocketAddrXdp` through rustix's own *public* API pair
that any real caller depends on for this exact struct: `SocketAddrArg::as_any()` (the same encoder
`rustix::net::bind()` uses internally to build the bytes handed to the real `bind(2)` syscall) and
`TryFrom<SocketAddrAny> for SocketAddrXdp` (the same decoder a caller uses on any `SocketAddrAny`
of family XDP, e.g. if a future kernel adds getname support, or via `SocketAddrAny::read` on bytes
obtained some other way). These two functions are supposed to be inverses for the identical
`sockaddr_xdp` C struct/field semantics (confirmed against the real kernel header and `xsk_bind()`
above) - they are not, which is exactly the bug. This does not fake a second participant's
behavior in the sense CONFIRMATION.md section 11 warns about (no bytes are hand-authored to
*imitate* what the kernel would send); it demonstrates that rustix's own encode and decode halves
of the same public API disagree with each other and with the real kernel ABI, which is what's
actually wrong here, via the only entry points genuinely reachable in the current Linux AF_XDP
implementation.

## Class

logic

## Root cause

`read_sockaddr_xdp` (both backends) applies `u32::from_be(..)` to `sxdp_ifindex`/`sxdp_queue_id`
as if they were transmitted in network byte order, matching the pattern used elsewhere in the same
file for genuinely wire-order fields (`sin_port`, `sin6_flowinfo`, etc.). But per
`include/uapi/linux/if_xdp.h` and the real kernel's `xsk_bind()` (`net/xdp/xsk.c`), these two
fields are plain host-endian integers - an interface index and a queue number, never transmitted
over any network wire. The write side (`SocketAddrArg for SocketAddrXdp` /
`SocketAddrXdpWithSharedUmem` in `src/net/types.rs`) correctly stores them straight through with
no transformation, matching the kernel's actual expectation; only the read side wrongly swaps. On
a little-endian host this corrupts any nonzero, non-0xFFFFFFFF value read back - e.g. ifindex 2
becomes 33554432 (0x02000000), queue_id 5 becomes 83886080 (0x05000000).

## Introduced by

5648e33e "Add AF_XDP syscalls, constants and types (#946)" (2024-01-11) - `git log -S
sxdp_ifindex` shows this is the commit that added both the asymmetric write (no swap) and read
(`u32::from_be`) sides simultaneously; the bug has existed since AF_XDP support was first added
and was never present in a different, previously-correct form.

## The fix

Remove the `u32::from_be(..)` wrapping in both backends' `read_sockaddr_xdp`, so the read side
passes `sxdp_ifindex`/`sxdp_queue_id` through unchanged, matching the write side (`patch.diff`).

Verified against `repro/`: before the fix, 3/5 test input pairs mismatch (all nonzero,
non-`u32::MAX` values - the two inputs that happened not to mismatch, 0 and `u32::MAX`, are exactly
the two values a byte-swap can never visibly corrupt, since they're the same forwards and backwards
and the bug only became apparent from the other three); after the fix, all 5/5 match, including
boundary values (0, 1, `u32::MAX`, a mixed-bit pattern, and the motivating 2/5 from the candidate
description) - not just the one exact triggering input.

- Project test suite (`cargo test --verbose --features=all-apis --workspace --no-fail-fast`):
  413 passed, 1 failed before the fix -> 413 passed, 1 failed after (identical). The 1 failure
  (`io::ioctl::test_int_setter`) is pre-existing and unrelated to this change - it fails with
  `ENOENT` on a device file not present in this sandbox, both before and after the fix.

## What's not done / open questions

- `sxdp_shared_umem_fd` (ignored on the read side with a comment explaining why) was not examined
  for the same class of bug - it's intentionally not decoded at all (not returned from
  `acceptfrom`/`recvmsg`), so there's no corresponding read-side code to check.
- This finding is distinct from candidates 040 (`xdp_statistics_v1_fallback_wrong_field`) and 062
  (`socket_addr_any_field_order_sweep`), per the task description; not independently re-checked
  against those here beyond taking the task's word that they cover different code.
- Whether any other rustix `AddressFamily` variant has a similar write/read byte-order mismatch
  for a field that's actually host-endian was not swept here (out of scope for this single
  candidate - every other read-side `u32::from_be`/`u16::from_be` use in both `read_sockaddr.rs`
  files (`sin_addr`, `sin_port`, `sin6_port`, `sin6_flowinfo`) was spot-checked against its
  corresponding write side and each matches: those fields genuinely are network-byte-order on the
  wire, so only the XDP pair is wrong).
- Whether a future kernel version adds `xsk_getname`/populates `recvmsg`'s address for AF_XDP was
  not checked beyond today's upstream `master` - if it ever does, this exact fixed code path would
  then become reachable via a genuine kernel round trip, same as any other socket address type.
