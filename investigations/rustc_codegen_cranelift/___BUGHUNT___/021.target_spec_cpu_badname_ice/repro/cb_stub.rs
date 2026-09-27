// Minimal stand-in for the `compiler_builtins` crate, used only by the direct-rustc path in
// run.sh. rustc requires a `compiler_builtins` crate for every no_std target, and the real one
// cannot be handed to `rustc --extern` here (cargo's -Zbuild-std output is a metadata stub).
// The test program only needs the four mem* symbols that `core` references.
#![feature(compiler_builtins)]
#![compiler_builtins]
#![no_builtins]
#![no_std]
#![allow(internal_features)]

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        unsafe { *dst.add(i) = *src.add(i) };
        i += 1;
    }
    dst
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if (dst as usize) < (src as usize) {
        unsafe { memcpy(dst, src, n) }
    } else {
        let mut i = n;
        while i > 0 {
            i -= 1;
            unsafe { *dst.add(i) = *src.add(i) };
        }
        dst
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(dst: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        unsafe { *dst.add(i) = c as u8 };
        i += 1;
    }
    dst
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let (x, y) = unsafe { (*a.add(i), *b.add(i)) };
        if x != y {
            return x as i32 - y as i32;
        }
        i += 1;
    }
    0
}

// cg_clif lowers 128-bit division/multiplication-overflow to libcalls that LLVM inlines, so the
// stub needs them too when `core` itself is compiled by cg_clif.
#[unsafe(no_mangle)]
pub extern "C" fn __udivti3(a: u128, b: u128) -> u128 {
    a / b
}

#[unsafe(no_mangle)]
pub extern "C" fn __umodti3(a: u128, b: u128) -> u128 {
    a % b
}

#[unsafe(no_mangle)]
pub extern "C" fn __divti3(a: i128, b: i128) -> i128 {
    a / b
}

#[unsafe(no_mangle)]
pub extern "C" fn __modti3(a: i128, b: i128) -> i128 {
    a % b
}

#[unsafe(no_mangle)]
pub extern "C" fn __multi3(a: i128, b: i128) -> i128 {
    a.wrapping_mul(b)
}

#[unsafe(no_mangle)]
pub extern "C" fn __rust_u128_mulo(a: u128, b: u128, oflow: &mut i32) -> u128 {
    let (r, o) = a.overflowing_mul(b);
    *oflow = o as i32;
    r
}

#[unsafe(no_mangle)]
pub extern "C" fn __rust_i128_mulo(a: i128, b: i128, oflow: &mut i32) -> i128 {
    let (r, o) = a.overflowing_mul(b);
    *oflow = o as i32;
    r
}

#[unsafe(no_mangle)]
pub extern "C" fn __rust_u128_addo(a: u128, b: u128, oflow: &mut i32) -> u128 {
    let (r, o) = a.overflowing_add(b);
    *oflow = o as i32;
    r
}

#[unsafe(no_mangle)]
pub extern "C" fn __rust_i128_addo(a: i128, b: i128, oflow: &mut i32) -> i128 {
    let (r, o) = a.overflowing_add(b);
    *oflow = o as i32;
    r
}

#[unsafe(no_mangle)]
pub extern "C" fn __rust_u128_subo(a: u128, b: u128, oflow: &mut i32) -> u128 {
    let (r, o) = a.overflowing_sub(b);
    *oflow = o as i32;
    r
}

#[unsafe(no_mangle)]
pub extern "C" fn __rust_i128_subo(a: i128, b: i128, oflow: &mut i32) -> i128 {
    let (r, o) = a.overflowing_sub(b);
    *oflow = o as i32;
    r
}
