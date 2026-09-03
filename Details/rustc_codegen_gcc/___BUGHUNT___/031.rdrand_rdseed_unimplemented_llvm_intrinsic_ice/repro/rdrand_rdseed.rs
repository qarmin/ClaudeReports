// Compiler:
//   min-llvm-version: 0
//
// Run-time:
//   status: 0
//
// Ordinary safe-surface use of the `rdrand`/`rdseed` x86_64 hardware-RNG intrinsics, compiled
// with `-C target-feature=+rdrand,+rdseed` (the same feature gate `-C target-cpu=native` turns on
// on any CPU that supports these instructions). Before the fix, compiling this file with
// rustc_codegen_gcc panics with
// "not implemented: ***** unsupported LLVM intrinsic llvm.x86.rdrand.16" (or .32 / rdseed.16/32/64)
// inside `src/intrinsic/archs.rs`'s generated `map_arch_intrinsic` table - an ICE, not a normal
// diagnostic, on code a plain rustc/LLVM build compiles and runs without issue. Only
// `llvm.x86.rdrand.64` had an explicit special case in `src/intrinsic/llvm.rs`; the other five
// (`rdrand.16`, `rdrand.32`, `rdseed.16`, `rdseed.32`, `rdseed.64`) fell through to the
// auto-generated catch-all `unimplemented!()`.
use std::arch::x86_64::*;

fn main() {
    unsafe {
        let mut a: u16 = 0;
        let ok_a = _rdrand16_step(&mut a);
        let mut b: u32 = 0;
        let ok_b = _rdrand32_step(&mut b);
        let mut c: u64 = 0;
        let ok_c = _rdrand64_step(&mut c);
        let mut d: u16 = 0;
        let ok_d = _rdseed16_step(&mut d);
        let mut e: u32 = 0;
        let ok_e = _rdseed32_step(&mut e);
        let mut f: u64 = 0;
        let ok_f = _rdseed64_step(&mut f);

        // On real hardware that actually supports rdrand/rdseed, these report success (1) almost
        // always - the interesting part here is that compilation itself doesn't panic, and that
        // the reported success flags are non-negative sane values (0 or 1), not garbage from a
        // miscompiled call.
        assert!(ok_a == 0 || ok_a == 1);
        assert!(ok_b == 0 || ok_b == 1);
        assert!(ok_c == 0 || ok_c == 1);
        assert!(ok_d == 0 || ok_d == 1);
        assert!(ok_e == 0 || ok_e == 1);
        assert!(ok_f == 0 || ok_f == 1);

        println!("all rdrand/rdseed steps completed without an ICE");
    }
}
