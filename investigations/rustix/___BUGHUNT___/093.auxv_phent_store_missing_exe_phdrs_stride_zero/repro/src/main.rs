// Real end-to-end repro for candidate 093: `rustix::runtime::exe_phdrs()` always
// reports `phent == 0`, even though the kernel really did hand this exact
// process a nonzero `AT_PHENT` in its auxv at exec time.
//
// The "second participant" here is the Linux kernel itself at process start:
// it is the one that actually produces the auxv this process receives, not
// anything hand-authored by this repro. We independently re-derive the real
// `AT_PHENT` value by reading this same process's own `/proc/self/auxv` (the
// kernel's own record of what it handed us), and compare it against what
// rustix's public API reports for the exact same running process.

use std::fs;
use std::mem::size_of;

const AT_PHENT: usize = 4;
const AT_NULL: usize = 0;

/// Independently read the real auxv the kernel gave this process, without
/// going through rustix at all, to find the ground-truth `AT_PHENT`.
fn real_at_phent_from_proc_self_auxv() -> usize {
    let bytes = fs::read("/proc/self/auxv").expect("read /proc/self/auxv");
    let word = size_of::<usize>();
    assert_eq!(bytes.len() % (2 * word), 0, "auxv file has unexpected size");

    let mut i = 0;
    while i + 2 * word <= bytes.len() {
        let a_type = usize::from_ne_bytes(bytes[i..i + word].try_into().unwrap());
        let a_val = usize::from_ne_bytes(bytes[i + word..i + 2 * word].try_into().unwrap());
        if a_type == AT_NULL {
            break;
        }
        if a_type == AT_PHENT {
            return a_val;
        }
        i += 2 * word;
    }
    panic!("AT_PHENT not found in /proc/self/auxv");
}

fn main() {
    let real_phent = real_at_phent_from_proc_self_auxv();

    // The actual public API under test, exercised exactly as any real caller
    // (e.g. a from-scratch TLS-setup routine walking program headers) would
    // use it.
    let (phdr, phent, phnum) = rustix::runtime_448b8ad740e2a26f::exe_phdrs();

    println!("kernel-reported AT_PHENT (ground truth) = {real_phent}");
    println!("rustix exe_phdrs() -> phdr={phdr:p} phent={phent} phnum={phnum}");

    assert_ne!(
        real_phent, 0,
        "test is only meaningful if the kernel really did report a nonzero AT_PHENT"
    );
    assert!(!phdr.is_null(), "exe_phdrs() unexpectedly returned a null phdr");
    assert_ne!(phnum, 0, "exe_phdrs() unexpectedly returned phnum == 0");

    if phent == 0 {
        println!(
            "BUG CONFIRMED: exe_phdrs() reports phent=0 while the kernel really \
             reported AT_PHENT={real_phent} for this exact process - a real \
             program-header walker using this phent as the stride would read \
             every header at offset 0 instead of advancing by {real_phent} bytes."
        );
        std::process::exit(1);
    } else {
        println!("exe_phdrs() reported phent correctly ({phent}) - bug not present.");
    }
}
