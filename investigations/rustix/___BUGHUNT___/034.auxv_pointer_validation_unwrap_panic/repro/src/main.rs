// Real public-API entry point: `rustix::runtime_448b8ad740e2a26f::exe_phdrs()`
// wraps `getauxval(AT_PHDR)`/`getauxval(AT_PHNUM)` (the module is deliberately
// mangled-named by the project itself, but still `pub mod`, i.e. real public
// API - see src/lib.rs's own `pub mod runtime_448b8ad740e2a26f;`).
//
// This program is built, then its own compiled ELF binary is corrupted by
// `corrupt_phdr.py` (run by this folder's justfile) to give it a
// non-8-byte-aligned `e_phoff` in its own ELF header. The REAL, unmodified
// Linux kernel's own ELF loader (binfmt_elf.c) places no alignment
// requirement on `e_phoff`, so it happily execve()s the corrupted binary and
// reports `AT_PHDR = load_bias + e_phoff` (itself therefore misaligned) via
// the real `PR_GET_AUXV` prctl / `/proc/self/auxv` - exactly the situation
// `src/backend/linux_raw/param/auxv.rs`'s own `check_raw_pointer::<Elf_Phdr>`
// alignment check is designed to catch.
fn main() {
    let (phdr, phent, phnum) = rustix::runtime_448b8ad740e2a26f::exe_phdrs();
    println!("phdr={phdr:p} phent={phent} phnum={phnum}");
}
