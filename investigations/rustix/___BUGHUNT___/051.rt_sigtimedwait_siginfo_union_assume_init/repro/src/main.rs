use std::hint::black_box;
use std::mem::size_of;

use rustix::runtime_448b8ad740e2a26f as runtime;

// Poison a chunk of stack memory, roughly the size of `MaybeUninit<Siginfo>`,
// in hopes the same stack slot gets reused by `kernel_sigtimedwait`'s own
// local `MaybeUninit<Siginfo>` (common, not guaranteed, reproducibility
// technique for proving a real uninitialized-stack-memory read).
#[inline(never)]
fn poison_stack() {
    let mut buf = [0xAAu8; 4096];
    black_box(&mut buf);
}

fn main() {
    unsafe {
        // Block SIGRTMIN so it queues instead of running a default handler.
        let sig = libc::SIGRTMIN();
        let mut set: runtime::KernelSigSet = std::mem::zeroed();
        // Build a sigset containing only `sig` using libc, then hand it to
        // rustix's own raw type via a byte copy (both are the kernel's
        // native sigset_t ABI).
        let mut libc_set: libc::sigset_t = std::mem::zeroed();
        libc::sigemptyset(&mut libc_set);
        libc::sigaddset(&mut libc_set, sig);
        assert_eq!(size_of::<libc::sigset_t>() >= size_of_val(&set), true);
        std::ptr::copy_nonoverlapping(
            &libc_set as *const _ as *const u8,
            &mut set as *mut _ as *mut u8,
            size_of_val(&set),
        );
        let mut oldset: libc::sigset_t = std::mem::zeroed();
        let r = libc::pthread_sigmask(libc::SIG_BLOCK, &libc_set, &mut oldset);
        assert_eq!(r, 0, "pthread_sigmask failed");

        // Queue a real-time signal to ourselves with `sigqueue`, which fills
        // the kernel's `_rt` union arm (pid, uid, sigval) and sets
        // `si_code == SI_QUEUE`. This is a real, well-defined Linux
        // mechanism, not a hand-authored siginfo_t.
        let mut val: libc::sigval = std::mem::zeroed();
        val.sival_ptr = 0x1234_5678 as *mut libc::c_void;
        let pid = libc::getpid();
        let r = libc::sigqueue(pid, sig, val);
        assert_eq!(r, 0, "sigqueue failed: errno={}", std::io::Error::last_os_error());

        // Poison the stack right before the real call, several times with
        // different call depths to improve the odds the slot overlaps.
        poison_stack();
        poison_stack();

        let info = runtime::kernel_sigtimedwait(&set, None).expect("sigtimedwait failed");

        // Reinterpret the whole struct as raw bytes - this is exactly what
        // `assume_init()` already implicitly certified as "fully
        // initialized" before we ever got here.
        let bytes: [u8; size_of::<runtime::Siginfo>()] = std::mem::transmute_copy(&info);

        println!("sizeof(Siginfo) = {}", bytes.len());
        println!("si_signo = {}", info.__bindgen_anon_1.__bindgen_anon_1.si_signo);
        println!("si_errno = {}", info.__bindgen_anon_1.__bindgen_anon_1.si_errno);
        println!("si_code  = {}", info.__bindgen_anon_1.__bindgen_anon_1.si_code);
        let rt = info.__bindgen_anon_1.__bindgen_anon_1._sifields._rt;
        println!("_rt._pid = {}", rt._pid);
        println!("_rt._uid = {}", rt._uid);
        println!("_rt._sigval = {}", rt._sigval.sival_int);

        let poison_count = bytes.iter().filter(|&&b| b == 0xAA).count();
        println!("poison (0xAA) byte count in materialized Siginfo = {poison_count} / {}", bytes.len());
        for (i, chunk) in bytes.chunks(16).enumerate() {
            print!("{:3}: ", i * 16);
            for b in chunk {
                print!("{b:02x} ");
            }
            println!();
        }
        if poison_count > 0 {
            println!("CONFIRMED: assume_init() materialized {poison_count} bytes of leftover stack poison as a 'valid' Siginfo.");
            std::process::exit(42);
        } else {
            println!("No poison bytes observed this run.");
            std::process::exit(0);
        }
    }
}
