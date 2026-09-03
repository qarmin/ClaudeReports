// Self-checking repro for `llvm.x86.avx2.permd` (`_mm256_permutevar8x32_epi32`) in
// rustc_codegen_cranelift.
//
// Intel's definition of VPERMD ymm is:
//     FOR j := 0 to 7
//         id := idx[j*32+2 : j*32]      // only the LOW 3 BITS of the index lane
//         dst[j*32+31 : j*32] := a[id*32+31 : id*32]
//
// so any index value is legal and behaves as `idx & 7`. cg_clif uses the raw 32-bit index lane as
// a dynamic lane offset, so an index >= 8 reads past the end of the vector's stack slot.
//
// The reference below is that pseudocode written out in plain scalar Rust; it is what LLVM also
// produces. Run under both toolchains and diff, or just read the exit code: the process exits 1
// as soon as any lane disagrees with the reference.

use std::arch::x86_64::*;

fn reference(a: &[u32; 8], idx: &[u32; 8]) -> [u32; 8] {
    let mut out = [0u32; 8];
    for j in 0..8 {
        out[j] = a[(idx[j] & 7) as usize];
    }
    out
}

#[inline(never)]
fn permutevar(a: &[u32; 8], idx: &[u32; 8]) -> [u32; 8] {
    unsafe {
        let av = _mm256_loadu_si256(a.as_ptr() as *const __m256i);
        let iv = _mm256_loadu_si256(idx.as_ptr() as *const __m256i);
        let r = _mm256_permutevar8x32_epi32(av, iv);
        let mut out = [0u32; 8];
        _mm256_storeu_si256(out.as_mut_ptr() as *mut __m256i, r);
        out
    }
}

fn lcg(s: &mut u64) -> u32 {
    *s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    (*s >> 33) as u32
}

fn main() {
    let a: [u32; 8] = [0xAA00, 0xAA01, 0xAA02, 0xAA03, 0xAA04, 0xAA05, 0xAA06, 0xAA07];
    let mut bad = 0usize;
    let mut checked = 0usize;

    let mut check = |label: &str, idx: [u32; 8]| {
        let got = permutevar(&a, &idx);
        let want = reference(&a, &idx);
        checked += 1;
        if got == want {
            println!("ok   {label} idx={idx:?} -> {got:?}");
        } else {
            bad += 1;
            println!("WRONG {label} idx={idx:?} -> got {got:?} want {want:?}");
        }
    };

    // The headline case: all eight index lanes are the in-range indices plus 8.
    check("all+8", [8, 9, 10, 11, 12, 13, 14, 15]);

    // In-range indices, for contrast - these are already correct today.
    check("in-range", [0, 1, 2, 3, 4, 5, 6, 7]);
    check("reversed", [7, 6, 5, 4, 3, 2, 1, 0]);

    // A single out-of-range lane, at each lane position, for a spread of index values.
    for v in [
        8u32, 9, 15, 16, 31, 127, 128, 255, 256, 65535, 0x7fff_ffff, 0x8000_0000, 0x8000_0007,
        0xffff_fff8, 0xffff_ffff,
    ] {
        for lane in 0..8 {
            let mut idx = [0u32, 1, 2, 3, 4, 5, 6, 7];
            idx[lane] = v;
            check(&format!("v={v:#x} lane={lane}"), idx);
        }
    }

    // Random index vectors over the whole u32 range.
    let mut s = 0x1234_5678_9abc_def0u64;
    for i in 0..200 {
        let idx = [
            lcg(&mut s),
            lcg(&mut s),
            lcg(&mut s),
            lcg(&mut s),
            lcg(&mut s),
            lcg(&mut s),
            lcg(&mut s),
            lcg(&mut s),
        ];
        check(&format!("rnd{i}"), idx);
    }

    println!("\n{checked} cases checked, {bad} wrong");
    if bad != 0 {
        println!("FAIL: _mm256_permutevar8x32_epi32 does not mask the index to 3 bits");
        std::process::exit(1);
    }
    println!("PASS: all lanes match the Intel-documented idx & 7 semantics");
}
