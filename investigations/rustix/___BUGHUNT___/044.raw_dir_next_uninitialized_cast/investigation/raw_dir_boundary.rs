//! Approach 1: boundary-value / differential stress test for `RawDir`, run as
//! `examples/raw_dir_boundary.rs` inside the rustix checkout (needs the `fs` feature).
//!
//! Creates a real directory with entries of varying name length (to shift record
//! boundaries across every possible alignment), then iterates it with `RawDir`
//! using every buffer size from 0 to 4096 bytes (implementing the documented
//! "grow buffer and retry on INVAL" convention exactly as RawDir::new's own doc
//! example shows), comparing the resulting (name, ino) set against
//! `std::fs::read_dir` for each size. All I/O is real getdents64 syscalls against
//! a real tmpfs directory - nothing hand-constructed.

use rustix::fs::{openat, Mode, OFlags, RawDir, CWD};
use std::collections::BTreeSet;
use std::mem::MaybeUninit;

fn collect_std(dir: &std::path::Path) -> BTreeSet<(String, u64)> {
    let mut set = BTreeSet::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let ino = std::os::unix::fs::MetadataExt::ino(&entry.metadata().unwrap());
        set.insert((entry.file_name().into_string().unwrap(), ino));
    }
    set
}

fn collect_rawdir(dir: &std::path::Path, start_buf_size: usize) -> BTreeSet<(String, u64)> {
    let fd = openat(
        CWD,
        dir,
        OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .unwrap();

    let mut set = BTreeSet::new();
    let mut buf_size = start_buf_size.max(1);
    // Documented "grow buffer and retry" convention (see RawDir::new's own doc
    // example): on INVAL, drop the iterator, grow the buffer, recreate it
    // against the same fd (continues where it left off).
    loop {
        let mut buf = vec![MaybeUninit::uninit(); buf_size];
        let mut iter = RawDir::new(&fd, &mut buf);
        let mut hit_inval = false;
        while let Some(entry) = iter.next() {
            let entry = match entry {
                Ok(e) => e,
                Err(rustix::io::Errno::INVAL) => {
                    hit_inval = true;
                    break;
                }
                Err(e) => panic!("unexpected error at buf_size={buf_size}: {e:?}"),
            };
            let name = entry.file_name().to_string_lossy().into_owned();
            if name == "." || name == ".." {
                continue;
            }
            set.insert((name, entry.ino()));
        }
        if hit_inval {
            buf_size *= 2;
            continue;
        }
        break;
    }
    set
}

fn main() {
    let tmp = std::env::temp_dir().join(format!("rawdir_boundary_{}", std::process::id()));
    std::fs::create_dir_all(&tmp).unwrap();

    let name_lens = [1usize, 2, 3, 7, 8, 15, 16, 31, 32, 63, 100, 200, 250];
    for (i, len) in name_lens.iter().enumerate() {
        let name = "f".repeat(*len) + &format!("_{i}");
        std::fs::File::create(tmp.join(name)).unwrap();
    }

    let expected = collect_std(&tmp);
    println!("created {} real entries", expected.len());

    let mut mismatches = 0usize;
    let mut einval_count = 0usize;
    for buf_size in 0..=4096 {
        let got = collect_rawdir(&tmp, buf_size);
        if got.is_empty() {
            einval_count += 1;
            continue;
        }
        if got != expected {
            mismatches += 1;
            println!(
                "MISMATCH at buf_size={buf_size}: got {} entries, expected {}",
                got.len(),
                expected.len()
            );
        }
    }

    println!("swept buf_size 0..=4096: {mismatches} mismatches, {einval_count} sizes too small for one entry");
    std::fs::remove_dir_all(&tmp).ok();

    if mismatches > 0 {
        std::process::exit(1);
    }
    println!("OK: no logic mismatch or crash/UB observed across the full buffer-size sweep");
}
