// Standalone reproduction for candidate 032: `fs::readlink`'s success path does
// `CString::new(buffer).unwrap()` on the raw bytes the `readlink(2)` syscall wrote
// (src/fs/abs.rs, `_readlink`). A normal `symlink(2)`-created target can never contain an
// embedded NUL (symlink creation takes a NUL-terminated C string), but `readlink(2)`'s *return*
// value is not required to respect that for every filesystem - a FUSE daemon's own `readlink`
// callback, or a corrupted/crafted on-disk symlink inode, can hand the kernel arbitrary bytes
// (including an embedded NUL) to copy back into the caller's buffer, which makes
// `CString::new` return `Err` and `.unwrap()` panic.
//
// Exercising this via a REAL FUSE daemon (the genuine second participant that can make this
// happen) was attempted first and found infeasible in this sandboxed container:
//   - `/dev/fuse` does not exist here (`ls /dev/fuse` -> No such file or directory)
//   - no root / CAP_SYS_ADMIN (`sudo -n true` fails: "no new privileges" blocks sudo)
//   - no loop devices (`/dev/loop*` do not exist) - rules out mounting a corrupted raw image too
//   - unprivileged user namespaces are blocked (`unshare -rm` -> "Operation not permitted")
// See output.txt / RESULT.md for the exact commands and output that established this.
//
// Given that, this repro mocks only the single lowest-level syscall wrapper
// (`backend::fs::syscalls::readlink` in the `linux_raw` backend) for one magic path, via
// `mock.patch` applied to the cloned `rustix` source before building (see this folder's
// `justfile`). Every other path still takes the real, unmodified `__NR_readlinkat` syscall.
// Everything *above* that one mocked line - `rustix::fs::readlink`'s real public API, the real
// `_readlink` loop, the real `assert!`/`resize`/`CString::new(...).unwrap()` - is the exact,
// unmodified source being tested; only the leaf syscall (standing in for the unavailable FUSE
// daemon / corrupted on-disk symlink) is simulated.

fn describe_payload(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else {
        "<non-string panic payload>".to_string()
    }
}

fn main() {
    // This magic path is recognized only by mock.patch's injected branch in the cloned source;
    // every other path goes through the real, unmodified syscall.
    let result =
        std::panic::catch_unwind(|| rustix::fs::readlink("/__bughunt_fake_fuse_target__", Vec::new()));

    match result {
        Ok(Ok(cstr)) => {
            println!("did not reproduce - readlink succeeded with {:?}", cstr);
        }
        Ok(Err(e)) => {
            println!("did not reproduce - readlink returned a clean error: {:?}", e);
        }
        Err(payload) => {
            eprintln!("REPRODUCED - panicked: {}", describe_payload(&*payload));
            std::process::exit(1);
        }
    }
}
