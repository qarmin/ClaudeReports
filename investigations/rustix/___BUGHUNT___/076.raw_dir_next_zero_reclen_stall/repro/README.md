# repro/ for candidate 076

This candidate's bug can only be triggered by a `getdents64` response whose `d_reclen` field is 0
for one directory entry - something the real Linux kernel VFS never emits, but a buggy/malicious
filesystem backend (a broken FUSE or 9p driver, or a corrupted on-disk image surfaced through some
other backend) could. Building a real one of those (actually mounting a FUSE filesystem that serves
a malformed `getdents64` reply) was not attempted as a *separate* exercise for this candidate: this
file/bug-class combination was already investigated for exactly this question by sibling candidates
043 (`src/backend/linux_raw/fs/dir.rs::Dir::read`, the confirmed sibling bug this candidate's
description points at) and 044 (this exact file, `src/fs/raw_dir.rs`, a different concern). Both
concluded independently that a genuine FUSE-backed end-to-end repro is infeasible in this sandboxed
container (no `/dev/fuse` device node, no `CAP_SYS_ADMIN`, `sudo` blocked by "no new privileges",
no package-manager/network access to install `fuse`/`strace` tooling) - see those candidates'
`RESULT.md` files for the full list of things tried. Candidate 044 additionally established that
even a real, working FUSE mount would not help here: the kernel's own VFS (`fs/readdir.c`) enforces
the "never emit a partial/malformed `linux_dirent64` record" invariant generically, above any
individual filesystem driver, so no real (even malicious) filesystem can violate it from userspace
regardless of sandboxing.

This is therefore **not** a standalone public-API crate the way `repro_template.rs` describes.
Instead, `repro_test.patch` adds a `#[cfg(test)]` test directly into `src/fs/raw_dir.rs`, reusing
the exact same "reach inside the private fields of a real iterator populated by one real syscall"
technique candidate 043's own `repro_test.patch` uses for the sibling `Dir::read` bug: the test
drives the real, unmodified `RawDir::new`/`RawDir::next` public entry points against a buffer that
*was* populated by a real `getdents64` syscall on a real directory fd, and only overwrites the two
`d_reclen` bytes of the one record that is about to be parsed next - standing in for the single
field a buggy backend would have gotten wrong. Everything else in the buffer (`d_ino`, `d_off`,
`d_type`, `d_name`, every other record) is exactly what the real kernel returned.

The test fails (panics with a message starting `REPRODUCED:`) if `RawDir::next()` returns the exact
same entry, with `self.offset` unchanged, for 50,000 straight calls - standing in for "forever". It
passes if the implementation instead stops iterating (returns `Some(Err(_))`/`None`) or if
`self.offset` genuinely advances on its own.

Run via `just test` (see `justfile` in this candidate's folder) - it clones the pristine commit,
applies `repro_test.patch`, runs the test (expected: FAILED/panics - bug present), applies the real
fix `../patch.diff`, and runs the same test again (expected: ok - bug fixed).
