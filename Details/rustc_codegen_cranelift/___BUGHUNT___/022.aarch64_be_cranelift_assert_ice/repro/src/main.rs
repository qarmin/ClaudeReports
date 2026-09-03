// Bare-metal aarch64 program: no_std, no_main, raw Linux syscalls, so it can be linked as a
// freestanding static ELF and executed by `qemu-aarch64` (user-mode emulation).
//
// It does real integer codegen (checksum) and real decimal formatting, then writes the result with
// the `write` syscall, so the run-time output is a genuine end-to-end check of what the backend
// emitted - not just "the compiler exited 0".
#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    unsafe { sys_exit(101) }
}

unsafe fn sys_write(fd: usize, buf: *const u8, len: usize) {
    unsafe {
        asm!(
            "svc #0",
            in("x8") 64usize,
            in("x0") fd,
            in("x1") buf,
            in("x2") len,
            options(nostack),
        );
    }
}

unsafe fn sys_exit(code: usize) -> ! {
    unsafe {
        asm!("svc #0", in("x8") 93usize, in("x0") code, options(noreturn, nostack));
    }
}

#[inline(never)]
fn checksum(n: u64) -> u64 {
    let mut acc: u64 = 0;
    let mut i: u64 = 0;
    while i < n {
        acc = acc.wrapping_mul(31).wrapping_add(i ^ 0xa5a5);
        i += 1;
    }
    acc
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let v = checksum(1000);
    let mut buf = [0u8; 32];
    let mut n = v;
    let mut pos = 31usize;
    if n == 0 {
        pos -= 1;
        buf[pos] = b'0';
    }
    while n > 0 {
        pos -= 1;
        buf[pos] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    buf[31] = b'\n';
    unsafe {
        sys_write(1, buf.as_ptr().add(pos), 32 - pos);
        sys_exit(0)
    }
}
