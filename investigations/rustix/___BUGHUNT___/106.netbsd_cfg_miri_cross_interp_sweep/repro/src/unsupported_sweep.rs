//! Documents (does not "pass" or "fail" in the usual sense) which NetBSD-gated
//! rustix entry points Miri's `--target x86_64-unknown-netbsd` cross-
//! interpretation genuinely cannot execute at all, because NetBSD's own libc
//! uses ABI-versioned symbol names (`__fstat50`, `__stat50`, ...) for exactly
//! the syscalls that candidates 039/050 found interesting on other BSD/Apple
//! targets, and Miri's `shims/unix/netbsd/foreign_items.rs` only implements
//! `__unsetenv13` and `__errno` - nothing else NetBSD-specific.
//!
//! Each call below is expected to abort the whole process with Miri's
//! "unsupported operation: can't call foreign function ... on OS netbsd"
//! diagnostic the first time it's reached - so this binary is run once per
//! probe (see `justfile`), each run expected to fail with that exact message,
//! not a normal "ok" exit.
use std::env;

fn main() {
    let which = env::args().nth(1).unwrap_or_default();
    match which.as_str() {
        "fstat" => {
            let f = std::fs::File::open("/").unwrap();
            let stat = rustix::fs::fstat(&f).unwrap();
            println!("UNEXPECTED SUCCESS: fstat ok, st_size = {}", stat.st_size);
        }
        "kqueue" => {
            let kq = rustix::event::kqueue::kqueue().unwrap();
            println!("UNEXPECTED SUCCESS: kqueue fd = {:?}", kq);
        }
        "uname" => {
            let u = rustix::system::uname();
            println!("UNEXPECTED SUCCESS: uname sysname = {:?}", u.sysname());
        }
        other => {
            eprintln!("usage: unsupported_sweep <fstat|kqueue|uname>, got {:?}", other);
            std::process::exit(2);
        }
    }
}
