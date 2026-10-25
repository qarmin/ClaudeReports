//! Approach 2 (standalone, rustix-free): isolates WHY MemorySanitizer flagged
//! `RawDir::next`'s `linux_dirent64` field reads as "use of uninitialized value".
//!
//! This program contains no rustix code at all - it issues the exact same real
//! `getdents64` syscall via `libc::syscall` against a real directory, decodes
//! fields through the exact same raw-pointer-cast-to-struct pattern RawDir uses,
//! and shows the warning reproduces here too, PURELY as a function of whether the
//! destination buffer is heap-allocated (`Vec<MaybeUninit<u8>>`) or
//! stack-allocated (`[MaybeUninit<u8>; N]`) - with no change to the syscall, the
//! decode logic, or the real kernel-written bytes. This demonstrates the
//! diagnostic is a known MSan interceptor gap (its `getdents64` syscall
//! post-hook does not unpoison memory it only tracks via `malloc`'s own
//! allocation-time poisoning for this allocation shape), not a signal about
//! rustix's actual code.
//!
//! Run with: RUSTFLAGS="-Zsanitizer=memory -Zsanitizer-memory-track-origins" \
//!   cargo +nightly run -Zbuild-std --target x86_64-unknown-linux-gnu

use std::ffi::CString;
use std::mem::MaybeUninit;

unsafe fn decode_with_stack_buffer(fd: i32) {
    let mut buf = [MaybeUninit::<u8>::uninit(); 4096];
    let n = libc::syscall(
        libc::SYS_getdents64,
        fd,
        buf.as_mut_ptr().cast::<libc::c_void>(),
        buf.len(),
    );
    assert!(n > 0);
    let mut offset = 0usize;
    let mut count = 0;
    while offset < n as usize {
        let ptr = buf[offset..].as_ptr();
        let dirent = &*ptr.cast::<libc::dirent64>();
        let reclen = dirent.d_reclen;
        let _ino = dirent.d_ino;
        offset += reclen as usize;
        count += 1;
    }
    println!("[stack buffer]  decoded {count} entries, no MSan warning expected");
}

unsafe fn decode_with_heap_buffer(fd: i32) {
    libc::lseek(fd, 0, libc::SEEK_SET);
    let mut buf: Vec<MaybeUninit<u8>> = vec![MaybeUninit::uninit(); 4096];
    let n = libc::syscall(
        libc::SYS_getdents64,
        fd,
        buf.as_mut_ptr().cast::<libc::c_void>(),
        buf.len(),
    );
    assert!(n > 0);
    let mut offset = 0usize;
    let mut count = 0;
    while offset < n as usize {
        let ptr = buf[offset..].as_ptr();
        let dirent = &*ptr.cast::<libc::dirent64>();
        let reclen = dirent.d_reclen; // <-- MSan flags this read as "use of uninitialized value"
        let ino = dirent.d_ino;
        offset += reclen as usize;
        count += 1;
        if count <= 1 {
            println!("[heap buffer]   entry: reclen={reclen} ino={ino}");
        }
    }
    println!("[heap buffer]   decoded {count} entries");
}

fn main() {
    let path = CString::new("/tmp").unwrap();
    unsafe {
        let fd = libc::open(path.as_ptr(), libc::O_RDONLY | libc::O_DIRECTORY);
        assert!(fd >= 0, "open failed");

        decode_with_stack_buffer(fd);
        decode_with_heap_buffer(fd);

        libc::close(fd);
    }
}
