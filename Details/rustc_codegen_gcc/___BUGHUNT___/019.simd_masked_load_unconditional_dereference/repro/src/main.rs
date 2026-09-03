#![feature(portable_simd)]

use std::simd::{Mask, Simd};

fn main() {
    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as usize;
    assert_eq!(page_size, 4096, "test assumes a 4096-byte page");

    // Map two adjacent pages, then make the second one inaccessible - a real guard page, not a
    // stand-in. `load_select_ptr`'s documented contract: "When the element is disabled, that
    // memory location is not accessed" - so a disabled lane may safely point into this guard page.
    let total = page_size * 2;
    let base = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            total,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        )
    };
    assert_ne!(base, libc::MAP_FAILED, "mmap failed");
    let page1 = base as *mut u8;
    let page2 = unsafe { page1.add(page_size) };

    unsafe {
        let ret = libc::mprotect(page2 as *mut _, page_size, libc::PROT_NONE);
        assert_eq!(ret, 0, "mprotect failed");
    }

    // Place a 4-lane i32 vector so that lanes 0..=2 sit in the last 12 bytes of page1 (readable)
    // and lane 3 sits exactly at the start of page2 (the guard page, PROT_NONE).
    let vec_start = unsafe { page1.add(page_size - 12) } as *mut i32;
    unsafe {
        vec_start.write(10);
        vec_start.add(1).write(20);
        vec_start.add(2).write(30);
        // lane 3's address is page2 (guard page) - never written, must never be read either.
    }

    let mask = Mask::from_array([true, true, true, false]);
    let or = Simd::from_array([-1i32, -1, -1, -1]);

    eprintln!("about to masked-load (lane 3 disabled, points into a PROT_NONE guard page)");
    let loaded = unsafe { Simd::<i32, 4>::load_select_ptr(vec_start as *const i32, mask, or) };
    eprintln!("loaded = {:?}", loaded.to_array());
    assert_eq!(loaded.to_array(), [10, 20, 30, -1]);
    eprintln!("masked load ok");

    unsafe {
        libc::munmap(base, total);
    }
}
