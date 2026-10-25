// Repro for: rustix's AF_XDP sockaddr decode (`read_sockaddr_xdp`, both backends) wrongly
// byte-swaps `sxdp_ifindex`/`sxdp_queue_id` via `u32::from_be(..)`, while the encode side
// (`SocketAddrArg for SocketAddrXdp` / `SocketAddrXdpWithSharedUmem`, in `src/net/types.rs`)
// stores the identical fields straight through with no byte-swap.
//
// Per `include/uapi/linux/if_xdp.h`, `sxdp_ifindex`/`sxdp_queue_id` are plain `__u32` fields with
// no byte-order annotation (unlike e.g. `sin_port`, which the kernel headers document as network
// byte order) - confirmed by reading the real kernel source: `net/xdp/xsk.c`'s `xsk_bind()` uses
// `sxdp->sxdp_ifindex` directly (`dev_get_by_index(sock_net(sk), sxdp->sxdp_ifindex)`), with no
// `ntohl`/`be32_to_cpu` applied anywhere. So the two fields are host-endian integers end to end,
// and rustix's own encode and decode functions for the exact same wire struct must be exact
// inverses of each other - they are not.
//
// WHY THIS REPRO DOES NOT GO THROUGH A REAL KERNEL ROUND TRIP (bind + getsockname/recvmsg):
// AF_XDP's own kernel-side `proto_ops` (`net/xdp/xsk.c`) sets `.getname = sock_no_getname`
// (verified by reading today's upstream `net/xdp/xsk.c`), so `getsockname()`/`getpeername()` on
// an AF_XDP socket always fail with -EOPNOTSUPP in current Linux - no syscall ever returns a
// `sockaddr_xdp` back to userspace that way. `xsk_recvmsg()`/`__xsk_recvmsg()` in the same file
// never touch `msg_name`/`msg_namelen` either (AF_XDP delivers data through the mmap'd UMEM
// rings, not through `recvmsg`'s buffer) - so `recvfrom`/`recvmsg` never populate one either.
// `.accept`/`.connect` are `sock_no_accept`/`sock_no_connect`. In short: there is currently no
// real kernel syscall path that hands a `sockaddr_xdp` back to userspace for rustix's decode side
// to ever parse against genuine kernel-produced bytes - this is independent of any privilege
// issue (creating the AF_XDP socket at all also requires CAP_NET_RAW, unavailable in the sandbox
// this was investigated in; confirmed separately with a raw `socket(AF_XDP, SOCK_RAW, 0)` C probe
// failing with EPERM here).
//
// So instead this repro demonstrates the asymmetry directly through rustix's own *public* API
// surface, using the exact same two functions real callers depend on:
//   - `SocketAddrArg::as_any()` on `SocketAddrXdp`, the same public encoder `rustix::net::bind()`
//     would use internally to build the raw bytes handed to the real `bind(2)` syscall.
//   - `TryFrom<SocketAddrAny> for SocketAddrXdp`, the same public decoder any caller uses to turn
//     a kernel-returned sockaddr back into a `SocketAddrXdp` (e.g. after a hypothetical future
//     kernel that does support getname, or any other SocketAddrAny obtained by a caller).
// If these two public functions were consistent (as they must be, both operating on the identical
// `sockaddr_xdp` C struct and field semantics), round-tripping a value through them would be the
// identity. It isn't.

use rustix::net::addr::SocketAddrArg;
use rustix::net::xdp::{SocketAddrXdp, SocketAddrXdpFlags};

fn round_trip(ifindex: u32, queue_id: u32) -> SocketAddrXdp {
    let original = SocketAddrXdp::new(SocketAddrXdpFlags::empty(), ifindex, queue_id);
    // Encode: the same path `rustix::net::bind(&sock, &original)` uses internally.
    let any = original.as_any();
    // Decode: the same path a caller uses to interpret a kernel-returned sockaddr_xdp.
    any.try_into().expect("decode of a just-encoded SocketAddrXdp failed")
}

fn main() {
    let mut any_mismatch = false;

    // The exact scenario from the finding: a plausible real ifindex (2) and queue_id (5).
    let cases = [(2u32, 5u32), (0, 0), (1, 1), (u32::MAX, u32::MAX), (0x1234_5678, 0x9abc_def0)];

    for (ifindex, queue_id) in cases {
        let original = SocketAddrXdp::new(SocketAddrXdpFlags::empty(), ifindex, queue_id);
        let decoded = round_trip(ifindex, queue_id);
        let ok = original == decoded;
        println!(
            "ifindex={ifindex:>10} queue_id={queue_id:>10}  ->  decoded ifindex={:>10} queue_id={:>10}  {}",
            decoded.interface_index(),
            decoded.queue_id(),
            if ok { "OK" } else { "MISMATCH" }
        );
        if !ok {
            any_mismatch = true;
        }
    }

    println!();
    if any_mismatch {
        println!("BUG CONFIRMED: SocketAddrXdp round-trip through rustix's own public encode (SocketAddrArg::as_any) and decode (TryFrom<SocketAddrAny>) functions is not lossless.");
        std::process::exit(1);
    } else {
        println!("round-trip OK: no mismatch (bug not present / fixed).");
    }
}
