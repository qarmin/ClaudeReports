// Direct exercise of SocketAddrUnix construction + addr.rs methods (libc backend),
// boundary-value lengths, plus a real kernel round-trip sanity check.
use rustix::net::{bind, getsockname, socket, AddressFamily, SocketAddrUnix, SocketType};
use std::collections::hash_map::DefaultHasher;
use std::convert::TryFrom;
use std::hash::Hash;

fn exercise(label: &str, a: &SocketAddrUnix) {
    // Exercises exactly the methods candidate 056 suspects: path_bytes() (does
    // self.len() - offsetof_sun_path() internally), PartialEq::eq, Ord::cmp, Hash::hash.
    let pb = a.path_bytes();
    let mut h = DefaultHasher::new();
    a.hash(&mut h);
    let eq = a == a;
    let ord = a.cmp(a);
    println!("{label}: path_bytes={pb:?} eq={eq} ord={ord:?} (no panic/underflow)");
}

fn main() {
    // Boundary-value construction through the crate's own public constructors,
    // exactly the paths read_sockaddr_unix funnels real kernel data through.
    let empty = SocketAddrUnix::new(&[][..]).unwrap();
    exercise("empty path", &empty);

    #[cfg(target_os = "linux")]
    {
        let unnamed = SocketAddrUnix::new_unnamed();
        exercise("unnamed", &unnamed);

        let abstract_empty = SocketAddrUnix::new_abstract_name(&[]).unwrap();
        exercise("abstract empty", &abstract_empty);

        let abstract_max = SocketAddrUnix::new_abstract_name(&[b'x'; 107]).unwrap();
        exercise("abstract max (107)", &abstract_max);

        assert!(SocketAddrUnix::new_abstract_name(&[b'x'; 108]).is_err());
    }

    let max_path: String = "a".repeat(107);
    let max = SocketAddrUnix::new(max_path.as_str()).unwrap();
    exercise("fs path max (107, with NUL)", &max);

    // 108 'a's fits exactly in sun_path with no NUL terminator stored at all
    // (the `bytes.len() > unix.sun_path.len()` check in `_new` is `108 > 108` == false).
    let max_no_nul = SocketAddrUnix::new("a".repeat(108)).unwrap();
    exercise("fs path max (108, no NUL)", &max_no_nul);

    assert!(SocketAddrUnix::new("a".repeat(109)).is_err());

    // Real kernel round trip: bind + getsockname, then run the exact addr.rs
    // methods under scrutiny on the address the kernel handed back.
    let dir = std::env::temp_dir().join(format!("rustix-056-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("s");
    let sock = socket(AddressFamily::UNIX, SocketType::STREAM, None).unwrap();
    let addr = SocketAddrUnix::new(path.as_path()).unwrap();
    bind(&sock, &addr).unwrap();
    let got_any = getsockname(&sock).unwrap();
    let got = SocketAddrUnix::try_from(got_any).unwrap();
    exercise("real kernel round-trip (bind+getsockname)", &got);
    std::fs::remove_dir_all(&dir).ok();

    println!("ALL BOUNDARY/ROUND-TRIP CHECKS PASSED, NO PANIC, NO UNDERFLOW");
}
