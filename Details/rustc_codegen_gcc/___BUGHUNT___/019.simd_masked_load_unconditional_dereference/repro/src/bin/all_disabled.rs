#![feature(portable_simd)]

use std::simd::{Mask, Simd};

fn main() {
    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as usize;
    assert_eq!(page_size, 4096, "test assumes a 4096-byte page");

    // Map a page, then unmap it entirely so the pointer is truly dangling (not just protected).
    let base = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            page_size,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        )
    };
    assert_ne!(base, libc::MAP_FAILED, "mmap failed");
    unsafe {
        libc::munmap(base, page_size);
    }

    // Every lane disabled; the pointer points at fully unmapped memory. Per the documented
    // contract, no lane should ever be touched.
    let mask = Mask::from_array([false, false, false, false]);
    let or = Simd::from_array([-1i32, -2, -3, -4]);

    eprintln!("about to masked-load (all lanes disabled, base pointer fully unmapped)");
    let loaded = unsafe { Simd::<i32, 4>::load_select_ptr(base as *const i32, mask, or) };
    eprintln!("loaded = {:?}", loaded.to_array());
    assert_eq!(loaded.to_array(), [-1, -2, -3, -4]);
    eprintln!("all-disabled masked load ok");
}
