// Candidate 099: read_sockaddr_netlink_socketaddrany_precondition_gap
//
// `SocketAddrAny::new` (rustix's own public, documented `unsafe fn`) only requires, per its own
// written AND enforced (via `assert!`) precondition:
//   - `len >= size_of::<sockaddr_header>()` (2 bytes: just the `sa_family` field)
//   - `len <= size_of::<SocketAddrStorage>()`
//   - the first `len` bytes of `storage` are initialized
//
// A caller of this real public API who follows that documented contract to the letter - nothing
// hand-waved, nothing internal-only - can construct a `SocketAddrAny` tagged `AF_NETLINK` with
// `len` as small as 2. But the safe `TryFrom<SocketAddrAny> for SocketAddrNetlink` conversion
// (`read_sockaddr_netlink` in both backends) asserts `addr.addr_len() >= size_of::<sockaddr_nl>()`
// (12 bytes on Linux) before it will proceed - so a value that is "valid" per the constructor's own
// contract panics a safe, infallible-looking `TryFrom` instead of returning
// `Err(Errno::AFNOSUPPORT)` the way a mismatched-family input does.
//
// This repro is a direct, single-process call into rustix's own real public API - not a stand-in
// for a second participant's wire input. There is no "peer" being faked here: the external caller
// of `SocketAddrAny::new` *is* the genuine external input this finding is about (see
// CONFIRMATION.md section 11's "a malformed file handed to a parser's real public entry point"
// exception - this is the same shape: a direct caller of a real public unsafe constructor,
// supplying input that satisfies its own documented contract).
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero/panic = reproduced.

use rustix::net::addr::SocketAddrStorage;
use rustix::net::netlink::SocketAddrNetlink;
use rustix::net::{AddressFamily, SocketAddrAny};
use std::mem::MaybeUninit;

fn main() {
    // Build the smallest storage that satisfies `SocketAddrAny::new`'s own documented and
    // enforced precondition: only the 2-byte `sa_family` header is initialized, tagged
    // `AF_NETLINK`. The rest of `SocketAddrStorage` is zeroed (not left uninitialized) so this
    // repro isolates exactly the panic-vs-contract mismatch being claimed, without also relying
    // on reading uninitialized memory to make the point.
    let mut storage = MaybeUninit::<SocketAddrStorage>::zeroed();
    unsafe {
        let family_ptr = storage.as_mut_ptr().cast::<u16>();
        *family_ptr = AddressFamily::NETLINK.as_raw();
    }

    // `len = 2` is exactly `size_of::<sockaddr_header>()` - the documented minimum
    // `SocketAddrAny::new` accepts. This call must NOT panic: it satisfies the constructor's own
    // contract to the letter.
    let any: SocketAddrAny = unsafe { SocketAddrAny::new(storage, 2) };

    println!(
        "constructed SocketAddrAny via the public API: family={:?} addr_len={}",
        any.address_family(),
        any.addr_len()
    );
    assert_eq!(any.address_family(), AddressFamily::NETLINK);
    assert_eq!(any.addr_len(), 2);

    // Now hand it to the safe `TryFrom` conversion a normal (non-unsafe) caller would use. A
    // family mismatch would correctly return `Err(AFNOSUPPORT)` with no panic; here the family
    // matches, so the only remaining question is whether the safe conversion panics instead of
    // either succeeding or returning a clean `Err`.
    let result = std::panic::catch_unwind(|| SocketAddrNetlink::try_from(any));

    match result {
        Ok(Ok(addr)) => {
            println!(
                "did not reproduce - try_from succeeded cleanly: {addr:?}"
            );
        }
        Ok(Err(errno)) => {
            println!("did not reproduce - try_from returned a clean Err: {errno:?}");
        }
        Err(panic_payload) => {
            let msg = panic_payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| panic_payload.downcast_ref::<&str>().copied())
                .unwrap_or("<non-string panic payload>");
            eprintln!("REPRODUCED - safe TryFrom<SocketAddrAny> for SocketAddrNetlink panicked: {msg}");
            std::process::exit(1);
        }
    }
}
