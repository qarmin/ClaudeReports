#![no_std]
#![no_main]

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
    buf: [u8; 65536],
    len: usize,
}

impl Out {
    fn s(&mut self, t: &str) {
        for &b in t.as_bytes() {
            self.buf[self.len] = b;
            self.len += 1;
        }
        self.maybe_flush();
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
        self.maybe_flush();
    }
    fn maybe_flush(&mut self) {
        if self.len > 60000 {
            self.flush();
        }
    }
    fn flush(&mut self) {
        sys_write(self.buf.as_ptr(), self.len);
        self.len = 0;
    }
}

static TABLE: [u8; 16] = [
    0x10, 0x21, 0x32, 0x43, 0x54, 0x65, 0x76, 0x87, 0x98, 0xa9, 0xba, 0xcb, 0xdc, 0xed, 0xfe, 0x0f,
];

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut o = Out { buf: [0; 65536], len: 0 };
    let t: [u8; 16] = unsafe { core::ptr::read_volatile(&TABLE) };
    let tv: uint8x16_t = unsafe { transmute(t) };

    // every index byte value, in every lane position, for both lane counts
    let mut v = 0usize;
    while v < 256 {
        let mut lane = 0usize;
        while lane < 8 {
            let mut idx = [0u8; 8];
            idx[lane] = v as u8;
            let idx: [u8; 8] = unsafe { core::ptr::read_volatile(&idx) };
            let r: [u8; 8] = unsafe { transmute(vqtbl1_u8(tv, transmute(idx))) };
            o.n(v as u64);
            o.s(":");
            o.n(lane as u64);
            o.s(":");
            let mut i = 0;
            while i < 8 {
                o.n(r[i] as u64);
                o.s(",");
                i += 1;
            }
            o.s("\n");
            lane += 1;
        }
        lane = 0;
        while lane < 16 {
            let mut idx = [0u8; 16];
            idx[lane] = v as u8;
            let idx: [u8; 16] = unsafe { core::ptr::read_volatile(&idx) };
            let r: [u8; 16] = unsafe { transmute(vqtbl1q_u8(tv, transmute(idx))) };
            o.n(v as u64);
            o.s("q");
            o.n(lane as u64);
            o.s(":");
            let mut i = 0;
            while i < 16 {
                o.n(r[i] as u64);
                o.s(",");
                i += 1;
            }
            o.s("\n");
            lane += 1;
        }
        // all lanes set to the same value
        let idx8: [u8; 8] = unsafe { core::ptr::read_volatile(&[v as u8; 8]) };
        let r8: [u8; 8] = unsafe { transmute(vqtbl1_u8(tv, transmute(idx8))) };
        let idx16: [u8; 16] = unsafe { core::ptr::read_volatile(&[v as u8; 16]) };
        let r16: [u8; 16] = unsafe { transmute(vqtbl1q_u8(tv, transmute(idx16))) };
        o.n(v as u64);
        o.s("all:");
        let mut i = 0;
        while i < 8 {
            o.n(r8[i] as u64);
            o.s(",");
            i += 1;
        }
        o.s("|");
        let mut i = 0;
        while i < 16 {
            o.n(r16[i] as u64);
            o.s(",");
            i += 1;
        }
        o.s("\n");
        v += 1;
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
