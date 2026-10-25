// Standalone reproduction for rustix GitHub issue #1518 ("syscalls.rs panics on sparc with
// rustix 1.0.8"), still present in rustix HEAD 287214b8 (v1.1.5) as of 2026-10-01.
//
// WHY THIS ISN'T A CROSS-COMPILED-AND-QEMU-RUN BINARY (see RESULT.md for the full account):
// this environment has no working sparc/sparc64 cross-linker or libc (confirmed by an actual
// failed `cargo build --target sparc64-unknown-linux-gnu` link attempt - rust-lld refuses to
// link ELF64 sparc64 object files at all, and no `gcc-sparc64-linux-gnu`-style cross toolchain
// package is available via apt in this container), so no real rustix binary can actually be
// executed for a sparc/sparc64 Linux target here, even under qemu-sparc64 (which IS installed).
//
// What this repro demonstrates instead: the real, verbatim array-length mismatch that rustix's
// own source computes at compile time for the `sparc`/`sparc64`-`unknown-linux-gnu` targets, by
// reproducing rustix's own two lines of logic unmodified, with the real sizes plugged in - not
// reimplemented, not guessed:
//
//   - `libc` crate 0.2.189, `src/unix/linux_like/linux/arch/sparc/mod.rs` (shared by both
//     `sparc-unknown-linux-gnu` and `sparc64-unknown-linux-gnu`): the kernel ioctl-facing
//     `termios2` struct's `c_cc` field is a fixed-size `[cc_t; 19]` - this size comes from the
//     Linux kernel's own `arch/sparc/include/uapi/asm/termbits.h`, it is not a value libc chose.
//   - `libc` crate 0.2.189, `src/unix/linux_like/linux/gnu/b32/mod.rs` (sparc branch, line 316)
//     and `src/unix/linux_like/linux/gnu/b64/sparc64/mod.rs` (line 383): `pub const NCCS: usize
//     = 17` for BOTH 32- and 64-bit sparc glibc - this is the size rustix's own
//     `SpecialCodes([c::cc_t; c::NCCS as usize])` (src/termios/types.rs:1124) uses, i.e. 17 on
//     sparc, not 19.
//
// rustix's own `src/backend/libc/termios/syscalls.rs` (as of HEAD 287214b8):
//
//   tcgetattr (line ~94-95):
//       let nccs = termios2.c_cc.len();
//       result.special_codes.0[..nccs].copy_from_slice(&termios2.c_cc);
//
//   tcsetattr (line ~242-245, the same bug in the opposite copy direction):
//       let nccs = termios2.c_cc.len();
//       termios2.c_cc.copy_from_slice(&termios.special_codes.0[..nccs]);
//
// On every architecture this crate ships prebuilt rust-std for except sparc/sparc64, `NCCS >=
// termios2.c_cc.len()` (e.g. x86_64/aarch64/riscv64: c_cc.len() == 19, NCCS == 32), so this never
// panics there - which is exactly why this bug survived rustix's own CI and this project's own
// earlier bug-hunt sessions, all run on x86_64. On sparc/sparc64, `nccs` (19) exceeds
// `special_codes.0`'s real length (17), so `result.special_codes.0[..19]` panics with EXACTLY
// the message the original issue reporter saw: "range end index 19 out of range for slice of
// length 17".
//
// Usage: `repro before` reproduces the panic (pre-fix rustix logic). `repro after` runs the
// patched logic (this session's fix) against the same real sparc sizes and prints success.

const SPARC_TERMIOS2_C_CC_LEN: usize = 19; // libc 0.2.189 arch/sparc/mod.rs: c_cc: [cc_t; 19]
const SPARC_NCCS: usize = 17; // libc 0.2.189 b32/mod.rs (sparc) & b64/sparc64/mod.rs: NCCS = 17

/// The exact pre-fix logic from `tcgetattr`, src/backend/libc/termios/syscalls.rs:94-95,
/// with the real sparc sizes substituted for what would otherwise be generic, per-target
/// `libc`-crate-derived array lengths.
fn tcgetattr_copy_before(termios2_c_cc: [u8; SPARC_TERMIOS2_C_CC_LEN]) -> [u8; SPARC_NCCS] {
    let mut special_codes = [0u8; SPARC_NCCS];

    // Verbatim from rustix (src/backend/libc/termios/syscalls.rs:94-95):
    let nccs = termios2_c_cc.len();
    special_codes[..nccs].copy_from_slice(&termios2_c_cc);

    special_codes
}

/// This session's fix: clamp `nccs` to the shorter of the two arrays before slicing either
/// side, so a kernel `c_cc` longer than rustix's own `NCCS`-sized `SpecialCodes` array (sparc)
/// never over-indexes - the extra kernel-reported control codes beyond index `NCCS` are simply
/// not representable in rustix's own `SpecialCodes` type on this architecture and are dropped,
/// exactly as they already are on every architecture where `NCCS` is the smaller side.
fn tcgetattr_copy_after(termios2_c_cc: [u8; SPARC_TERMIOS2_C_CC_LEN]) -> [u8; SPARC_NCCS] {
    let mut special_codes = [0u8; SPARC_NCCS];

    let nccs = termios2_c_cc.len().min(special_codes.len());
    special_codes[..nccs].copy_from_slice(&termios2_c_cc[..nccs]);

    special_codes
}

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();

    // A plausible real kernel-reported `c_cc` (TCGETS2 ioctl result) - the actual byte values
    // don't matter to this bug (it is a pure length mismatch), only the array's real length.
    let termios2_c_cc = [0u8; SPARC_TERMIOS2_C_CC_LEN];

    match mode.as_str() {
        "before" => {
            println!("running PRE-FIX tcgetattr special_codes copy (real sparc sizes: termios2.c_cc.len()={}, SpecialCodes len={})", SPARC_TERMIOS2_C_CC_LEN, SPARC_NCCS);
            let result = tcgetattr_copy_before(termios2_c_cc);
            println!("BUG NOT REPRODUCED (unexpected): got {:?}", result);
        }
        "after" => {
            println!("running PATCHED tcgetattr special_codes copy (real sparc sizes: termios2.c_cc.len()={}, SpecialCodes len={})", SPARC_TERMIOS2_C_CC_LEN, SPARC_NCCS);
            let result = tcgetattr_copy_after(termios2_c_cc);
            println!("OK: patched copy completed without panicking, special_codes = {:?}", result);
        }
        _ => {
            eprintln!("usage: repro <before|after>");
            std::process::exit(2);
        }
    }
}
