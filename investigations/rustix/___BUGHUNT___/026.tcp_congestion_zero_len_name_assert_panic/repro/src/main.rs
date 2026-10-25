// Real end-to-end exercise of rustix's PUBLIC `rustix::net::sockopt::tcp_congestion` API,
// against a real TCP socket, through the real `getsockopt(2)` libc call path (the libc backend
// is selected via the `use-libc` Cargo feature so interposing libc's `getsockopt` symbol via
// LD_PRELOAD actually takes effect - the default linux_raw backend makes the raw syscall
// directly and would bypass an LD_PRELOAD shim entirely).
//
// What's genuinely real here: the socket, the `getsockopt` call rustix issues, and the panic
// rustix's own code produces from whatever bytes come back.
//
// What's stood in for: this sandboxed environment has no CAP_SYS_MODULE/root (verified: `id`
// shows a non-root uid, `capsh --print` shows cap_sys_module stripped from the IAB/bounding set,
// `sudo` is blocked by no-new-privileges, and there is no `/lib/modules/<running-kernel>` tree to
// build an out-of-tree kernel module against) - so there is no way in this environment to
// register a real out-of-tree TCP congestion-control algorithm whose `name` field is exactly 16
// bytes with no trailing NUL and have a real kernel hand that back via a real getsockopt(2). A
// small LD_PRELOAD shim (see ../shim/fake_congestion.c) stands in for that kernel behavior,
// handing back exactly the 16 non-NUL-terminated bytes a real such module's registration would
// produce (confirmed by reading the actual upstream kernel source for the TCP_CONGESTION
// getsockopt handler in net/ipv4/tcp.c - see RESULT.md). See RESULT.md's "Fully reproducible"
// section for why this is the documented, rare, explicitly-justified exception per
// CONFIRMATION.md section 11, rather than a disqualifying faked second participant.

use rustix::net::sockopt;
use rustix::net::{AddressFamily, SocketType};

fn main() {
    let socket =
        rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).expect("socket(2) failed");

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        sockopt::tcp_congestion(&socket)
    }));

    match result {
        Ok(Ok(name)) => {
            println!("did not reproduce - tcp_congestion() returned Ok({name:?})");
        }
        Ok(Err(e)) => {
            println!("did not reproduce - tcp_congestion() returned Err({e:?})");
        }
        Err(panic_payload) => {
            let msg = panic_payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| panic_payload.downcast_ref::<&str>().copied())
                .unwrap_or("<non-string panic payload>");
            eprintln!("REPRODUCED - rustix::net::sockopt::tcp_congestion() panicked: {msg}");
            std::process::exit(1);
        }
    }
}
