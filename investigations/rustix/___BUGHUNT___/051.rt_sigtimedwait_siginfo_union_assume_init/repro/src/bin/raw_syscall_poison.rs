// Deterministic test, independent of stack-reuse luck: heap-allocate the
// 128-byte siginfo_t buffer, explicitly poison every byte with 0xAA, then
// invoke the real `rt_sigtimedwait` syscall directly (same syscall number
// and argument shape rustix's own `kernel_sigtimedwait` uses) and inspect
// exactly which bytes the kernel actually overwrote.
use std::io::Error;

const SIGINFO_SIZE: usize = 128;

fn main() {
    unsafe {
        let sig = libc::SIGRTMIN();
        let mut libc_set: libc::sigset_t = std::mem::zeroed();
        libc::sigemptyset(&mut libc_set);
        libc::sigaddset(&mut libc_set, sig);
        let mut oldset: libc::sigset_t = std::mem::zeroed();
        let r = libc::pthread_sigmask(libc::SIG_BLOCK, &libc_set, &mut oldset);
        assert_eq!(r, 0);

        let mut val: libc::sigval = std::mem::zeroed();
        val.sival_ptr = 0x1234_5678_usize as *mut libc::c_void;
        let pid = libc::getpid();
        let r = libc::sigqueue(pid, sig, val);
        assert_eq!(r, 0, "sigqueue failed: {}", Error::last_os_error());

        // Explicitly poisoned heap buffer - no ambiguity about prior content.
        let mut buf: Vec<u8> = vec![0xAA; SIGINFO_SIZE];
        let buf_ptr = buf.as_mut_ptr();

        // Kernel's own sigset_t size on x86_64 is 8 bytes (64-bit mask).
        let sigsetsize: usize = 8;

        // Direct raw syscall: rt_sigtimedwait(set, info, timeout, sigsetsize)
        // __NR_rt_sigtimedwait == 128 on x86_64.
        let ret: isize;
        std::arch::asm!(
            "syscall",
            inlateout("rax") 128isize => ret,
            in("rdi") &libc_set as *const _ as usize,
            in("rsi") buf_ptr as usize,
            in("rdx") 0usize, // no timeout -> block
            in("r10") sigsetsize,
            out("rcx") _,
            out("r11") _,
            options(nostack)
        );

        println!("rt_sigtimedwait raw syscall returned: {ret}");
        assert!(ret > 0, "syscall failed, errno-ish retval: {ret}");

        let bytes = std::slice::from_raw_parts(buf_ptr, SIGINFO_SIZE);
        let poison_count = bytes.iter().filter(|&&b| b == 0xAA).count();
        println!("poison (0xAA) byte count remaining after real kernel syscall = {poison_count} / {SIGINFO_SIZE}");
        for (i, chunk) in bytes.chunks(16).enumerate() {
            print!("{:3}: ", i * 16);
            for b in chunk {
                print!("{b:02x} ");
            }
            println!();
        }
        if poison_count > 0 {
            println!("CONFIRMED: kernel left {poison_count} bytes of the 128-byte siginfo_t untouched (still our poison pattern).");
            std::process::exit(42);
        } else {
            println!("Kernel overwrote every byte of the 128-byte siginfo_t (no poison survived).");
            std::process::exit(0);
        }
    }
}
