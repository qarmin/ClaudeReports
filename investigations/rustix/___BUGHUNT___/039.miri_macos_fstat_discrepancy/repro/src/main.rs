// Real end-to-end trigger: rustix's own public `fstat` API, called on a
// real file descriptor from a real `std::fs::File`, cross-interpreted by
// Miri with `--target x86_64-apple-darwin`. Miri's own libc shim for
// `fstat` on apple targets doesn't write a defined value into the kernel
// `struct stat`'s reserved `st_lspare`/`st_qspare` fields, so rustix's
// `MaybeUninit::<Stat>::uninit() ... assume_init()` pattern constructs an
// integer field from uninitialized memory, which Miri's validity checker
// correctly flags as UB - matching upstream issue #1668 exactly.
use std::fs::File;

fn main() {
    let path = std::env::temp_dir().join("rustix_miri_fstat_repro.txt");
    let f = File::create(&path).unwrap();

    let stat = rustix::fs::fstat(&f).unwrap();
    println!("fstat ok, st_size = {}", stat.st_size);

    let _ = std::fs::remove_file(&path);
}
