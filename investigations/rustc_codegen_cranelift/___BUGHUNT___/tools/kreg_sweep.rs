// Single-file sweep over cg_clif's AVX-512 mask-register (`kreg`) inline-asm save/restore path
// (candidate 004). Run it through the differential loop, which is the whole point:
//
//     bash ___BUGHUNT___/tools/diffcheck.sh ___BUGHUNT___/tools/kreg_sweep.rs
//     CLIF_ROOT=<worktree> bash ___BUGHUNT___/tools/diffcheck.sh ___BUGHUNT___/tools/kreg_sweep.rs
//
// Unpatched cg_clif fails to compile it (`mov k1, [rbx+0x0]` is not a valid instruction);
// patched, its output is byte-identical to LLVM's. Needs an AVX-512 CPU to *run*, but the compile
// failure - which is the bug - happens on any x86-64 host.
// Sweep across the kreg inline-asm bug class: every operand kind, several k registers,
// both the avx512f (16-bit, kmovw) and avx512bw (64-bit, kmovq) widths, plus the real
// stdarch intrinsics that are implemented with `asm!` on a kreg.
use std::arch::asm;
use std::arch::x86_64::*;

#[target_feature(enable = "avx512f")]
unsafe fn f_in_k1(v: u16) -> u32 {
    let o: u32;
    unsafe { asm!("kmovw {o:e}, k1", o = out(reg) o, in("k1") v) };
    o
}

#[target_feature(enable = "avx512f")]
unsafe fn f_out_k7(v: u32) -> u16 {
    let o: u16;
    unsafe { asm!("kmovw k7, {v:e}", v = in(reg) v, out("k7") o) };
    o
}

#[target_feature(enable = "avx512f")]
unsafe fn f_inout_k3(v: u16) -> u16 {
    let mut m = v;
    unsafe { asm!("knotw k3, k3", inout("k3") m) };
    m
}

// Two kregs live at once - two independent save/restore slots.
#[target_feature(enable = "avx512f")]
unsafe fn f_two_kregs(a: u16, b: u16) -> u16 {
    let o: u16;
    unsafe { asm!("kandw k4, k4, k5", inout("k4") a => o, in("k5") b) };
    o
}

// NOTE: k0 (register class `kreg0`) supports no operand types at all in rustc, so it can only
// ever appear as a clobber - not reachable through the save/restore path being tested here.

// avx512bw widens kreg operands to 64 bits, which needs `kmovq` rather than `kmovw`.
#[target_feature(enable = "avx512bw")]
unsafe fn f_in_k2_u64(v: u64) -> u64 {
    let o: u64;
    unsafe { asm!("kmovq {o}, k2", o = out(reg) o, in("k2") v) };
    o
}

#[target_feature(enable = "avx512bw")]
unsafe fn f_inout_k6_u64(v: u64) -> u64 {
    let mut m = v;
    unsafe { asm!("knotq k6, k6", inout("k6") m) };
    m
}

// The real-world triggers: stdarch implements these six with `asm!` on a kreg.
#[target_feature(enable = "avx512f")]
unsafe fn f_stdarch() -> (f64, f64, f32, f32, f64, f32) {
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
    (
        _mm_cvtsd_f64(a),
        _mm_cvtsd_f64(b),
        _mm_cvtss_f32(c),
        _mm_cvtss_f32(d),
        od[0],
        os[0],
    )
}

fn main() {
    unsafe {
        for v in [0u16, 1, 0b1011, 0x00ff, 0xff00, 0xffff, 0x8000, 0x5a5a] {
            println!("in_k1({v:#06x})    = {:#x}", f_in_k1(v));
            println!("out_k7({v:#06x})   = {:#x}", f_out_k7(v as u32));
            println!("inout_k3({v:#06x}) = {:#x}", f_inout_k3(v));
            println!("two_kregs({v:#06x}) = {:#x}", f_two_kregs(v, 0x0f0f));
        }
        for v in [0u64, 1, u64::MAX, 0x0123_4567_89ab_cdef, 1 << 63, 0xffff_ffff] {
            println!("in_k2_u64({v:#018x})    = {:#x}", f_in_k2_u64(v));
            println!("inout_k6_u64({v:#018x}) = {:#x}", f_inout_k6_u64(v));
        }
        println!("stdarch = {:?}", f_stdarch());
    }
}
