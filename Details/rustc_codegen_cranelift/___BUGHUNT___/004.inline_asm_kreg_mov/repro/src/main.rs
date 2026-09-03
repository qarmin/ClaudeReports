//! cg_clif emits `mov k1, [rbx+0x18]` when it has to spill/reload an AVX-512 mask register
//! (`kreg`) around an `asm!` block. `mov` cannot address mask registers, so the generated
//! `global_asm!` wrapper is rejected by the assembler and the whole crate fails to build.
//!
//! The reproduction is therefore the *build* under cg_clif: plain rustc/LLVM compiles this file
//! and prints correct values, cg_clif refuses it with
//! `error: invalid operand for instruction` / `error: Failed to assemble ...`.
//!
//! Running it additionally checks the values, which needs an AVX-512 CPU; without one the program
//! prints SKIP and exits 0 - the build result above is still the reproduction.

use std::arch::asm;
use std::arch::x86_64::*;

// ---- direct `asm!` with a kreg operand: input, output, inout, and two at once ----------------

#[target_feature(enable = "avx512f")]
unsafe fn kreg_in(v: u16) -> u32 {
    let o: u32;
    unsafe { asm!("kmovw {o:e}, k1", o = out(reg) o, in("k1") v) };
    o
}

#[target_feature(enable = "avx512f")]
unsafe fn kreg_out(v: u32) -> u16 {
    let o: u16;
    unsafe { asm!("kmovw k7, {v:e}", v = in(reg) v, out("k7") o) };
    o
}

#[target_feature(enable = "avx512f")]
unsafe fn kreg_inout(v: u16) -> u16 {
    let mut m = v;
    unsafe { asm!("knotw k3, k3", inout("k3") m) };
    m
}

#[target_feature(enable = "avx512f")]
unsafe fn kreg_two(a: u16, b: u16) -> u16 {
    let o: u16;
    unsafe { asm!("kandw k4, k4, k5", inout("k4") a => o, in("k5") b) };
    o
}

// With avx512bw a kreg operand may be 64 bits wide, which needs `kmovq` rather than `kmovw`.

#[target_feature(enable = "avx512bw")]
unsafe fn kreg_in_u64(v: u64) -> u64 {
    let o: u64;
    unsafe { asm!("kmovq {o}, k2", o = out(reg) o, in("k2") v) };
    o
}

#[target_feature(enable = "avx512bw")]
unsafe fn kreg_inout_u64(v: u64) -> u64 {
    let mut m = v;
    unsafe { asm!("knotq k6, k6", inout("k6") m) };
    m
}

// ---- the real-world trigger: six stdarch intrinsics are implemented with `asm!` on a kreg ----

#[target_feature(enable = "avx512f")]
unsafe fn stdarch_masked_scalar() -> (f64, f64, f32, f32, f64, f32) {
    let sd = [4.0f64];
    let ss = [5.0f32];
    let a = _mm_mask_load_sd(_mm_set_sd(1.0), 0b1, sd.as_ptr());
    let b = _mm_maskz_load_sd(0b0, sd.as_ptr());
    let c = _mm_mask_load_ss(_mm_set_ss(2.0), 0b1, ss.as_ptr());
    let d = _mm_maskz_load_ss(0b0, ss.as_ptr());
    let mut od = [9.0f64];
    let mut os = [9.0f32];
    _mm_mask_store_sd(od.as_mut_ptr(), 0b1, _mm_set_sd(7.5));
    _mm_mask_store_ss(os.as_mut_ptr(), 0b1, _mm_set_ss(8.5));
    (_mm_cvtsd_f64(a), _mm_cvtsd_f64(b), _mm_cvtss_f32(c), _mm_cvtss_f32(d), od[0], os[0])
}

fn main() {
    if !std::arch::is_x86_feature_detected!("avx512f")
        || !std::arch::is_x86_feature_detected!("avx512bw")
    {
        println!("SKIP: this CPU lacks avx512f/avx512bw - the build result above is the repro");
        return;
    }

    let mut failures = 0u32;
    let mut check = |name: &str, got: String, want: &str| {
        if got == want {
            println!("ok    {name} = {got}");
        } else {
            println!("WRONG {name}\n        got  {got}\n        want {want}");
            failures += 1;
        }
    };

    unsafe {
        check("kreg_in(0x5a5a)", format!("{:#x}", kreg_in(0x5a5a)), "0x5a5a");
        check("kreg_in(0xffff)", format!("{:#x}", kreg_in(0xffff)), "0xffff");
        check("kreg_out(0x00ff)", format!("{:#x}", kreg_out(0x00ff)), "0xff");
        check("kreg_inout(0x000b)", format!("{:#x}", kreg_inout(0x000b)), "0xfff4");
        check("kreg_two(0x5a5a,0x0f0f)", format!("{:#x}", kreg_two(0x5a5a, 0x0f0f)), "0xa0a");
        check("kreg_in_u64(MAX)", format!("{:#x}", kreg_in_u64(u64::MAX)), "0xffffffffffffffff");
        check(
            "kreg_in_u64(0x0123456789abcdef)",
            format!("{:#x}", kreg_in_u64(0x0123_4567_89ab_cdef)),
            "0x123456789abcdef",
        );
        check("kreg_inout_u64(1<<63)", format!("{:#x}", kreg_inout_u64(1 << 63)), "0x7fffffffffffffff");
        check(
            "stdarch masked scalar load/store",
            format!("{:?}", stdarch_masked_scalar()),
            "(4.0, 0.0, 5.0, 0.0, 7.5, 8.5)",
        );
    }

    println!("\n{failures} check(s) produced a wrong value");
    if failures > 0 {
        std::process::exit(1);
    }
}
