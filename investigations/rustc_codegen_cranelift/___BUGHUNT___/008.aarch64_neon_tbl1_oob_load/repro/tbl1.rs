#![no_std]
#![no_main]
#![allow(internal_features)]
#![feature(core_intrinsics)]

use core::arch::aarch64::*;
use core::arch::asm;
use core::mem::transmute;

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    exit(101)
}

fn sys_write(buf: *const u8, len: usize) {
    unsafe {
        asm!("svc #0", in("x8") 64usize, in("x0") 1usize, in("x1") buf, in("x2") len,
             lateout("x0") _, options(nostack));
    }
}

fn exit(code: usize) -> ! {
    unsafe {
        asm!("svc #0", in("x8") 94usize, in("x0") code, options(noreturn, nostack));
    }
}

struct Out {
    buf: [u8; 4096],
    len: usize,
}

impl Out {
    fn s(&mut self, t: &str) {
        for &b in t.as_bytes() {
            self.buf[self.len] = b;
            self.len += 1;
        }
    }
    fn n(&mut self, mut v: u64) {
        let mut tmp = [0u8; 20];
        let mut i = 0;
        if v == 0 {
            self.s("0");
            return;
        }
        while v > 0 {
            tmp[i] = b'0' + (v % 10) as u8;
            v /= 10;
            i += 1;
        }
        while i > 0 {
            i -= 1;
            self.buf[self.len] = tmp[i];
            self.len += 1;
        }
    }
    fn flush(&mut self) {
        sys_write(self.buf.as_ptr(), self.len);
        self.len = 0;
    }
}

static TABLE: [u8; 16] = [
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
];

// index vectors covering: in-range, the 8..15 half-table boundary, exactly 16, and far
// out-of-range values up to 255
static IDX8: [[u8; 8]; 8] = [
    [0, 1, 2, 3, 4, 5, 6, 7],
    [8, 9, 10, 11, 12, 13, 14, 15],
    [15, 16, 17, 8, 7, 0, 31, 32],
    [16, 17, 18, 19, 20, 21, 22, 23],
    [64, 65, 100, 127, 128, 200, 254, 255],
    [255, 255, 255, 255, 255, 255, 255, 255],
    [7, 8, 15, 16, 23, 24, 239, 240],
    [1, 2, 3, 4, 250, 251, 252, 253],
];

static IDX16: [[u8; 16]; 6] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
    [16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31],
    [0, 16, 1, 17, 8, 24, 15, 31, 7, 200, 255, 3, 128, 64, 5, 250],
    [255; 16],
    [
        240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255,
    ],
];

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut o = Out { buf: [0; 4096], len: 0 };
    let t: [u8; 16] = unsafe { core::ptr::read_volatile(&TABLE) };
    let tv: uint8x16_t = unsafe { transmute(t) };

    o.s("vqtbl1_u8 (v8i8)\n");
    let mut k = 0;
    while k < 8 {
        let idx: [u8; 8] = unsafe { core::ptr::read_volatile(&IDX8[k]) };
        let r: [u8; 8] = unsafe { transmute(vqtbl1_u8(tv, transmute(idx))) };
        let mut i = 0;
        while i < 8 {
            o.n(r[i] as u64);
            o.s(" ");
            i += 1;
        }
        o.s("\n");
        k += 1;
    }

    o.s("vqtbl1q_u8 (v16i8)\n");
    let mut k = 0;
    while k < 6 {
        let idx: [u8; 16] = unsafe { core::ptr::read_volatile(&IDX16[k]) };
        let r: [u8; 16] = unsafe { transmute(vqtbl1q_u8(tv, transmute(idx))) };
        let mut i = 0;
        while i < 16 {
            o.n(r[i] as u64);
            o.s(" ");
            i += 1;
        }
        o.s("\n");
        k += 1;
    }

    o.flush();
    exit(0)
}

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
        let x = unsafe { *a.add(i) };
        let y = unsafe { *b.add(i) };
        if x != y {
            return x as i32 - y as i32;
        }
        i += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bcmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    unsafe { memcmp(a, b, n) }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}
