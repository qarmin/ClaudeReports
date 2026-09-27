// Bare aarch64 no_std program: does real f128 comparisons (which the backend has to lower to
// compiler_builtins' __eqtf2/__lttf2/... soft-float libcalls) and prints the resulting bitmask
// with the raw `write` syscall, so it can be linked static and run under qemu-aarch64.
#![no_std]
#![no_main]
#![feature(f128)]

use core::arch::asm;
use core::hint::black_box;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    unsafe { sys_exit(101) }
}

unsafe fn sys_write(fd: usize, buf: *const u8, len: usize) {
    unsafe {
        asm!("svc #0", in("x8") 64usize, in("x0") fd, in("x1") buf, in("x2") len, options(nostack));
    }
}

unsafe fn sys_exit(code: usize) -> ! {
    unsafe {
        asm!("svc #0", in("x8") 93usize, in("x0") code, options(noreturn, nostack));
    }
}

static mut OUT: [u8; 4096] = [0; 4096];
static mut LEN: usize = 0;

fn emit(b: u8) {
    unsafe {
        OUT[LEN] = b;
        LEN += 1;
    }
}

#[inline(never)]
fn cmp_mask(a: f128, b: f128) -> u32 {
    let a = black_box(a);
    let b = black_box(b);
    let mut m = 0u32;
    if a == b {
        m |= 1;
    }
    if a != b {
        m |= 2;
    }
    if a < b {
        m |= 4;
    }
    if a <= b {
        m |= 8;
    }
    if a > b {
        m |= 16;
    }
    if a >= b {
        m |= 32;
    }
    m
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vals: [f128; 9] = [
        0.0,
        -0.0,
        1.0,
        -1.0,
        3.5,
        1e30,
        -1e30,
        f128::INFINITY,
        f128::NEG_INFINITY,
    ];
    let mut i = 0usize;
    while i < 9 {
        let mut j = 0usize;
        while j < 9 {
            let m = cmp_mask(vals[i], vals[j]);
            emit(b'0' + (i as u8));
            emit(b'0' + (j as u8));
            emit(b':');
            let mut n = m;
            let mut d = [0u8; 4];
            let mut k = 0usize;
            if n == 0 {
                d[0] = b'0';
                k = 1;
            }
            while n > 0 {
                d[k] = b'0' + (n % 10) as u8;
                k += 1;
                n /= 10;
            }
            while k > 0 {
                k -= 1;
                emit(d[k]);
            }
            emit(b'\n');
            j += 1;
        }
        i += 1;
    }
    unsafe {
        sys_write(1, (&raw const OUT) as *const u8, LEN);
        sys_exit(0)
    }
}
