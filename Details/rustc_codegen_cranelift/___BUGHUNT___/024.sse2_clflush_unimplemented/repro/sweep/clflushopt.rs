#![feature(simd_x86_clflushopt, clflushopt_target_feature)]
use std::arch::x86_64::_mm_clflushopt;
use std::sync::atomic::{Ordering, fence};

#[target_feature(enable = "clflushopt")]
unsafe fn flush(p: *const u8) {
    _mm_clflushopt(p);
}

fn main() {
    let mut buf = [0u8; 256];
    for (i, b) in buf.iter_mut().enumerate() {
        *b = i as u8;
    }
    unsafe {
        flush(buf.as_ptr());
        flush(buf.as_ptr().add(64));
    }
    fence(Ordering::SeqCst);
    let s: u32 = buf.iter().map(|&b| b as u32).sum();
    println!("clflushopt ok sum={s}");
}
