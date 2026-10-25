// Real end-to-end repro for candidate 104: `rustix::net::name_to_index` against the real kernel
// `SIOCGIFINDEX` ioctl, called through the crate's real public API with a real socket fd. The
// "name" passed is "lo\0evil" - a valid Rust `&str` (Rust strings may contain interior NUL bytes),
// crafted the way a caller doing validation like "reject if this isn't a recognized interface
// name" might accidentally let slip through if they only looked at the backend that NODEVs it.
//
// Build twice (two real backends, same kernel, same syscall):
//   cargo run --release                 -> linux_raw backend (default)
//   cargo run --release --features use-libc -> libc backend

use rustix::fd::AsFd;
use rustix::net::netdevice::name_to_index;
use rustix::net::{socket_with, AddressFamily, SocketFlags, SocketType};

fn main() {
    let backend = if cfg!(feature = "use-libc") {
        "libc"
    } else {
        "linux_raw"
    };

    let fd = socket_with(
        AddressFamily::INET,
        SocketType::DGRAM,
        SocketFlags::CLOEXEC,
        None,
    )
    .expect("socket() failed");

    let loopback_index: u32 = std::fs::read_to_string("/sys/class/net/lo/ifindex")
        .expect("read /sys/class/net/lo/ifindex")
        .trim()
        .parse()
        .unwrap();

    let clean = name_to_index(fd.as_fd(), "lo");
    println!("backend={backend} name_to_index(\"lo\") = {clean:?} (real lo ifindex = {loopback_index})");

    let malicious = "lo\0evil";
    let result = name_to_index(fd.as_fd(), malicious);
    println!("backend={backend} name_to_index({malicious:?}) = {result:?}");

    match result {
        Ok(idx) if idx == loopback_index => {
            println!(
                "BUG: backend={backend} resolved an embedded-NUL name {malicious:?} to the real \
                 \"lo\" interface (index {idx}) instead of rejecting it"
            );
            std::process::exit(2);
        }
        Err(e) if e == rustix::io::Errno::NODEV => {
            println!("OK: backend={backend} rejected the embedded-NUL name with NODEV as expected");
        }
        other => {
            println!("UNEXPECTED: backend={backend} produced {other:?}");
            std::process::exit(3);
        }
    }
}
