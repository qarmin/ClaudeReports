# repro/ for candidate 043

This candidate's bug can only be triggered by a `getdents64` response whose `d_reclen`
field is 0 for one directory entry - something the real Linux kernel VFS never emits, but a
buggy/malicious filesystem backend (a broken FUSE or 9p driver, or a corrupted on-disk image
surfaced through some other backend) could. Building a real one of those (actually mounting a
FUSE filesystem that serves a malformed `getdents64` reply) was not possible in the sandboxed
container this candidate was investigated in: no `/dev/fuse` device node exists, there is no
`CAP_SYS_ADMIN` (`capsh --print` lists it explicitly excluded from the bounding set), `sudo` is
blocked by the kernel's "no new privileges" flag, and there is no package-manager access to
install `fuse`/`strace` tooling or load the `fuse` kernel module. See RESULT.md's "Fully
reproducible" section for the full list of things that were tried.

This is therefore **not** a standalone public-API crate the way `repro_template.rs` describes.
Instead, `repro_test.patch` adds a `#[cfg(test)]` test directly into
`src/backend/linux_raw/fs/dir.rs`, reusing the exact same "reach inside `Dir`'s private fields"
technique the file's own pre-existing `dir_iterator_handles_io_errors` test already uses for a
different scenario (replacing the real directory fd with a file fd to force a `getdents64`
error). This is the real `Dir::read_from`/`Iterator::next` public entry point, driven by a `buf`
that *was* populated by one real `getdents64` syscall on a real directory - the test only
corrupts the two `d_reclen` bytes of the one record that is about to be parsed next, standing in
for the single field a buggy backend would have gotten wrong, exactly matching this candidate's
own stated caveat.

The test fails (panics with a message starting `REPRODUCED:`) if `Dir::read()`/`.next()` returns
the exact same entry, with `self.pos` unchanged and `any_errors` never set, for 50,000 straight
calls - standing in for "forever". It passes if the implementation instead stops iterating
(returns `None` or `Some(Err(_))`) or if `self.pos` genuinely advances on its own.

Run via `just test` (see `justfile` in this candidate's folder) - it clones the pristine commit,
applies `repro_test.patch`, runs the test (expected: FAILED/panics - bug present), applies the
real fix `../patch.diff`, and runs the same test again (expected: ok - bug fixed).
