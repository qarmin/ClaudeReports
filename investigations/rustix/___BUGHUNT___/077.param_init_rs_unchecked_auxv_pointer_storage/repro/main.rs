// Digging harness for candidate 077 - NOT a Confirmed, real end-to-end repro.
//
// This calls rustix's real, public, doc(hidden) API
// (`rustix::param::init()` + `rustix::param::linux_execfn()`, built with the
// `use-explicitly-provided-auxv` feature, i.e. `src/backend/linux_raw/param/init.rs`)
// exactly the way a real caller would. The gap, per CONFIRMATION.md section 11: the
// envp/auxv array handed to `init()` below is hand-built by this harness, not
// produced by a real kernel `execve()` + a real explicit-auxv runtime-startup
// caller (e.g. the `origin` crate). Extensive probing (see RESULT.md) found no
// way to make a real, unmodified Linux kernel hand a genuinely invalid
// AT_EXECFN/AT_SYSINFO_EHDR to a correctly-written caller, so this harness
// exists to demonstrate the code-level mechanism only: given a bad pointer
// value in the aux vector, `init.rs` stores it with zero validation, and
// `linux_execfn()` dereferences it unconditionally.
//
// Run with `cargo run` from this directory (after `../clone` exists, see ../justfile).

fn main() {
    // A deliberately invalid, non-null, dangling pointer - standing in for
    // whatever a bad/corrupted AT_EXECFN auxv entry would contain.
    let bad_execfn: *mut u8 = 1 as *mut u8;

    const AT_EXECFN: usize = 31;
    const AT_NULL: usize = 0;

    // Memory layout `init()` expects: envp entries, NULL terminator, then the
    // auxv array (Elf_auxv_t { a_type: usize, a_val: *mut c_void }, two
    // words per entry) immediately following in memory.
    let mut buf: Vec<usize> = vec![
        0, // envp[0] = NULL (empty envp)
        AT_EXECFN,
        bad_execfn as usize,
        AT_NULL,
        0,
    ];

    unsafe {
        // SAFETY (of the call, not of the overall harness): this is the
        // real public API, called the way a real caller would - the
        // "faked second participant" is the envp/auxv content above, not
        // the call itself.
        rustix::param::init(buf.as_mut_ptr().cast());
    }

    // init.rs stores AT_EXECFN into a bare `AtomicPtr` with no null check, no
    // alignment check, no validity check of any kind. linux_execfn() then
    // unconditionally does `CStr::from_ptr(execfn)` on whatever was stored.
    println!("about to call linux_execfn() with a corrupted AT_EXECFN pointer...");
    let s = rustix::param::linux_execfn();
    println!("UNREACHABLE if the bug is real: execfn len = {}", s.to_bytes().len());
}
