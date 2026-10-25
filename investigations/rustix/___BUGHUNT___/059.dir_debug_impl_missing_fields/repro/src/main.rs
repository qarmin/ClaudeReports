// Demonstrates that `Dir`'s manual `Debug` impl (linux_raw backend,
// src/backend/linux_raw/fs/dir.rs) prints only the `fd` field and silently
// omits `any_errors`, `rewind`, `buf`, and `pos` -- the fields that actually
// hold the iterator's state. This uses rustix's real public API
// (`rustix::fs::Dir::read_from`) against a real directory on disk; no
// internal/private state is hand-constructed.

use rustix::fs::{openat, Dir, Mode, OFlags, CWD};
use std::fs::File;
use std::io::Write as _;

fn main() {
    // Set up a real temp directory with enough entries that the Dir's
    // internal getdents64 buffer (`buf`) actually gets populated and `pos`
    // advances to a nonzero, non-buffer-length value after one `next()`.
    let dir_path = std::env::temp_dir().join(format!("rustix_dir_debug_repro_{}", std::process::id()));
    std::fs::create_dir_all(&dir_path).unwrap();
    for i in 0..8 {
        let mut f = File::create(dir_path.join(format!("file{i}"))).unwrap();
        f.write_all(b"x").unwrap();
    }

    let fd = openat(
        CWD,
        &dir_path,
        OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .expect("open real directory");

    let mut d = Dir::read_from(&fd).expect("Dir::read_from");

    // Advance the iterator once (real getdents64 syscall) so `buf` is
    // populated and `pos` is nonzero but less than `buf.len()` (there are 8
    // real entries plus "." and "..", more than fit a single struct if we
    // only consume one).
    let first = d.next();
    println!("first entry from real directory: {:?}", first.is_some());

    // Force the `rewind` flag on via the real public `rewind()` API -- this
    // is genuine internal state a maintainer debugging iteration behavior
    // would want to see.
    d.rewind();

    let debug_text = format!("{:?}", d);
    println!("Debug output: {debug_text}");

    let missing: Vec<&str> = ["any_errors", "rewind", "buf", "pos"]
        .iter()
        .copied()
        .filter(|field| !debug_text.contains(field))
        .collect();

    println!("fields present in struct but missing from Debug output: {:?}", missing);

    std::fs::remove_dir_all(&dir_path).ok();

    if missing.len() == 4 {
        println!("REPRODUCED: Debug impl omits all 4 iterator-state fields (any_errors, rewind, buf, pos)");
    } else {
        println!("NOT REPRODUCED: Debug impl included some/all iterator-state fields");
    }
}
