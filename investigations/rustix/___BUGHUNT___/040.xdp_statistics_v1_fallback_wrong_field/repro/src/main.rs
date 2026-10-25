//! Repro for candidate 040: `xdp_statistics()`'s v1-struct fallback branch in
//! both rustix backends (`src/backend/linux_raw/net/sockopt.rs` and
//! `src/backend/libc/net/sockopt.rs`, function `xdp_statistics`) assigns
//! `rx_dropped` into both `rx_invalid_descs` and `tx_invalid_descs` instead of
//! the struct's own `rx_invalid_descs`/`tx_invalid_descs` fields.
//!
//! ## Why this isn't a real end-to-end (real kernel) repro
//!
//! The buggy branch is only reached when the *kernel itself* only knows about
//! the smaller, pre-Linux-5.9 `xdp_statistics_v1` layout (commit 77cd0d7b3f25,
//! "xsk: Add new statistics for better diagnostics", which added
//! `rx_ring_full`/`rx_fill_ring_empty_descs`/`tx_ring_empty_descs`). rustix
//! always requests the *full* (post-5.9) struct size from the kernel, so
//! whether the v1 branch is taken is decided entirely by the remote kernel's
//! own build, not by anything a caller can pass in.
//!
//! This sandbox's kernel is `7.0.0-34-generic` (2026, far newer than 5.9), and
//! this sandbox's process additionally cannot create an `AF_XDP` socket at
//! all: `socket(AF_XDP, SOCK_RAW, 0)` fails with `EPERM` because `cap_net_raw`
//! is excluded from this container's capability *bounding set*
//! (`CapBnd` has bit 13 clear), which `sudo`/root cannot restore either (this
//! container also has the `no_new_privs` flag set, so `sudo` itself refuses to
//! run at all). Both the kernel-version gate and the capability gate make a
//! genuine, real-kernel reproduction impossible in this environment - this is
//! the documented exception in `CONFIRMATION.md` section 11 ("needs
//! hardware"/a specific unavailable kernel, not practical this session).
//!
//! What this repro does instead: it (1) empirically demonstrates the `EPERM`
//! above, so the claim "AF_XDP is unusable here" isn't asserted blind, and (2)
//! feeds synthetic bytes - laid out exactly as a real pre-5.9 kernel's
//! `copy_to_sockptr(optval, &stats, sizeof(struct xdp_statistics_v1))` would
//! deposit them (3 sequential `u64`s, rest zeroed, `optlen` reported back as
//! `sizeof(struct xdp_statistics_v1)` = 24) - through the real,
//! `rustix::net::XdpStatistics` type and two functions that are *verbatim*
//! transcriptions of the real (pre-patch / post-patch) backend branch bodies,
//! cited to their exact source location below. This is "manual boundary-value
//! construction" (the tool `HEURISTICS.md` calls for on a pure data-mapping
//! logic bug), not a hand-wavy reimplementation: every line in
//! `xdp_statistics_from_raw_buggy`/`_fixed` below is character-for-character
//! identical to the real source, only the enclosing `fn` signature and the
//! syscall (`getsockopt_raw`) are replaced by directly-supplied bytes.

use linux_raw_sys::xdp::{xdp_statistics, xdp_statistics_v1};
use rustix::net::xdp::XdpStatistics;
use std::mem::{size_of, MaybeUninit};

/// Verbatim copy of the real branch body under test. Without the `fixed`
/// feature this is byte-for-byte:
/// - src/backend/linux_raw/net/sockopt.rs:1056-1064 (as of commit 287214b8)
/// - src/backend/libc/net/sockopt.rs:1259-1267 (as of commit 287214b8, identical)
/// With `--features fixed` this is byte-for-byte this candidate's
/// `patch.diff` applied to both of the same two locations.
fn xdp_statistics_from_raw(value: MaybeUninit<xdp_statistics>) -> XdpStatistics {
    // SAFETY: All members of xdp_statistics are `u64` and thus are
    // correctly initialized by `MaybeUninit::<xdp_statistics>::zeroed()`.
    let xdp_statistics = unsafe { value.assume_init() };
    #[cfg(not(feature = "fixed"))]
    {
        XdpStatistics {
            rx_dropped: xdp_statistics.rx_dropped,
            rx_invalid_descs: xdp_statistics.rx_dropped,
            tx_invalid_descs: xdp_statistics.rx_dropped,
            rx_ring_full: None,
            rx_fill_ring_empty_descs: None,
            tx_ring_empty_descs: None,
        }
    }
    #[cfg(feature = "fixed")]
    {
        XdpStatistics {
            rx_dropped: xdp_statistics.rx_dropped,
            rx_invalid_descs: xdp_statistics.rx_invalid_descs,
            tx_invalid_descs: xdp_statistics.tx_invalid_descs,
            rx_ring_full: None,
            rx_fill_ring_empty_descs: None,
            tx_ring_empty_descs: None,
        }
    }
}

/// Builds a `MaybeUninit<xdp_statistics>` exactly the way rustix's own real
/// code builds it (`MaybeUninit::<xdp_statistics>::zeroed()`), then deposits
/// bytes into only the first `sizeof(xdp_statistics_v1)` of it - exactly what
/// a real `getsockopt(SOL_XDP, XDP_STATISTICS)` syscall on a pre-5.9 kernel
/// writes via `copy_to_sockptr(optval, &stats, len)` with
/// `len == sizeof(struct xdp_statistics_v1)`.
fn synthetic_v1_kernel_response(rx_dropped: u64, rx_invalid_descs: u64, tx_invalid_descs: u64) -> MaybeUninit<xdp_statistics> {
    let mut value = MaybeUninit::<xdp_statistics>::zeroed();
    let v1 = xdp_statistics_v1 {
        rx_dropped,
        rx_invalid_descs,
        tx_invalid_descs,
    };
    assert!(size_of::<xdp_statistics_v1>() <= size_of::<xdp_statistics>());
    // SAFETY: writing `sizeof(xdp_statistics_v1)` bytes into the front of a
    // zeroed, correctly-sized-and-aligned `xdp_statistics` buffer - exactly
    // what the real kernel's `copy_to_sockptr` does over the syscall boundary.
    unsafe {
        std::ptr::copy_nonoverlapping(
            &v1 as *const xdp_statistics_v1 as *const u8,
            value.as_mut_ptr() as *mut u8,
            size_of::<xdp_statistics_v1>(),
        );
    }
    value
}

fn try_real_af_xdp_socket() -> String {
    const AF_XDP: i32 = 44;
    // SAFETY: plain socket() syscall, standard libc FFI usage.
    let fd = unsafe { libc::socket(AF_XDP, libc::SOCK_RAW, 0) };
    if fd < 0 {
        format!(
            "socket(AF_XDP, SOCK_RAW, 0) failed: {}",
            std::io::Error::last_os_error()
        )
    } else {
        unsafe { libc::close(fd) };
        format!("socket(AF_XDP, SOCK_RAW, 0) succeeded: fd={fd}")
    }
}

fn main() {
    println!("--- Step 1: real AF_XDP socket creation attempt in this environment ---");
    println!("{}", try_real_af_xdp_socket());
    println!(
        "(this is why a genuine end-to-end repro against a real AF_XDP socket + a real \
         pre-5.9 kernel is not practical in this sandbox - see the module doc comment)"
    );

    println!();
    println!("--- Step 2: synthetic pre-5.9-kernel XDP_STATISTICS response ---");
    // Three distinct, individually-recognizable values standing in for what
    // a real kernel would report for these three real, distinct counters.
    let (rx_dropped, rx_invalid_descs, tx_invalid_descs) = (111u64, 222u64, 333u64);
    println!(
        "kernel writes (v1 layout): rx_dropped={rx_dropped}, rx_invalid_descs={rx_invalid_descs}, \
         tx_invalid_descs={tx_invalid_descs}"
    );

    let result = xdp_statistics_from_raw(synthetic_v1_kernel_response(
        rx_dropped,
        rx_invalid_descs,
        tx_invalid_descs,
    ));

    println!();
    #[cfg(not(feature = "fixed"))]
    println!("--- Step 3: what rustix's real (pre-patch, base_commit 287214b8) v1-fallback branch returns ---");
    #[cfg(feature = "fixed")]
    println!("--- Step 3: what rustix's real (post-patch, this candidate's patch.diff) v1-fallback branch returns ---");
    println!("{result:?}");

    println!();
    let correct =
        result.rx_invalid_descs == rx_invalid_descs && result.tx_invalid_descs == tx_invalid_descs;

    if correct {
        println!("CORRECT: rx_invalid_descs/tx_invalid_descs match the real kernel-reported values.");
    } else {
        println!(
            "BUG CONFIRMED: rx_invalid_descs={} (expected {rx_invalid_descs}) and \
             tx_invalid_descs={} (expected {tx_invalid_descs}) - both silently replaced by \
             rx_dropped={rx_dropped}.",
            result.rx_invalid_descs, result.tx_invalid_descs
        );
    }

    #[cfg(not(feature = "fixed"))]
    {
        assert!(!correct, "expected the pre-patch branch to reproduce the bug");
        std::process::exit(1);
    }
    #[cfg(feature = "fixed")]
    {
        assert!(correct, "expected the post-patch branch to no longer reproduce the bug");
        std::process::exit(0);
    }
}
