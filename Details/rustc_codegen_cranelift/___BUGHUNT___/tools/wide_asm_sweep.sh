#!/usr/bin/env bash
# Sweep every >128-bit inline-asm register-class operand shape one case per file, so cg_clif's
# abort-on-first-error does not hide the rest. Each case is compiled+run with plain rustc and with
# dist/rustc-clif via diffcheck.sh; prints OK / DIFFER per case.
#
# Usage: [CLIF_ROOT=<worktree>] bash wide_asm_sweep.sh
set -u
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../.." && pwd)"
WORK="$ROOT/tmp/wide_asm_sweep"
mkdir -p "$WORK"

emit() { # $1 = name, $2 = target_feature, rest = body on stdin
    local name="$1" feat="$2"
    { echo 'use std::arch::x86_64::*;'
      echo '#[repr(align(64))] struct A64<T>(T);'
      echo "#[target_feature(enable = \"$feat\")]"
      echo "unsafe fn case() {"
      cat
      echo "}"
      echo 'fn main() { unsafe { case() } }'
    } > "$WORK/$name.rs"
}

emit stream_pd avx <<'E'
    let mut m = A64([0.0f64; 4]);
    _mm256_stream_pd(m.0.as_mut_ptr(), _mm256_set1_pd(1.5));
    // NB: no _mm_sfence() - x86 guarantees same-thread self-consistency for the read below, and
    // llvm.x86.sse.sfence is a separate unimplemented intrinsic in cg_clif that aborts at run time.
    println!("{:?}", m.0);
E
emit stream_ps avx <<'E'
    let mut m = A64([0.0f32; 8]);
    _mm256_stream_ps(m.0.as_mut_ptr(), _mm256_set1_ps(2.5));
    // NB: no _mm_sfence() - x86 guarantees same-thread self-consistency for the read below, and
    // llvm.x86.sse.sfence is a separate unimplemented intrinsic in cg_clif that aborts at run time.
    println!("{:?}", m.0);
E
emit stream_si256 avx <<'E'
    let mut m = A64([0i64; 4]);
    _mm256_stream_si256(m.0.as_mut_ptr().cast(), _mm256_set1_epi64x(-3));
    // NB: no _mm_sfence() - x86 guarantees same-thread self-consistency for the read below, and
    // llvm.x86.sse.sfence is a separate unimplemented intrinsic in cg_clif that aborts at run time.
    println!("{:?}", m.0);
E
emit stream_load_si256 avx2 <<'E'
    let m = A64([7i64, 8, 9, 10]);
    let v = _mm256_stream_load_si256(m.0.as_ptr().cast());
    let mut o = A64([0i64; 4]);
    _mm256_storeu_si256(o.0.as_mut_ptr().cast(), v);
    println!("{:?}", o.0);
E
emit stream512_pd avx512f <<'E'
    let mut m = A64([0.0f64; 8]);
    _mm512_stream_pd(m.0.as_mut_ptr(), _mm512_set1_pd(4.5));
    // NB: no _mm_sfence() - x86 guarantees same-thread self-consistency for the read below, and
    // llvm.x86.sse.sfence is a separate unimplemented intrinsic in cg_clif that aborts at run time.
    println!("{:?}", m.0);
E
emit stream512_ps avx512f <<'E'
    let mut m = A64([0.0f32; 16]);
    _mm512_stream_ps(m.0.as_mut_ptr(), _mm512_set1_ps(5.5));
    // NB: no _mm_sfence() - x86 guarantees same-thread self-consistency for the read below, and
    // llvm.x86.sse.sfence is a separate unimplemented intrinsic in cg_clif that aborts at run time.
    println!("{:?}", m.0);
E
emit stream512_si512 avx512f <<'E'
    let mut m = A64([0i64; 8]);
    _mm512_stream_si512(m.0.as_mut_ptr().cast(), _mm512_set1_epi64(-6));
    // NB: no _mm_sfence() - x86 guarantees same-thread self-consistency for the read below, and
    // llvm.x86.sse.sfence is a separate unimplemented intrinsic in cg_clif that aborts at run time.
    println!("{:?}", m.0);
E
emit stream512_load avx512f <<'E'
    let m = A64([1i64, 2, 3, 4, 5, 6, 7, 8]);
    let v = _mm512_stream_load_si512(m.0.as_ptr().cast());
    let mut o = A64([0i64; 8]);
    _mm512_storeu_si512(o.0.as_mut_ptr().cast(), v);
    println!("{:?}", o.0);
E
emit raw_ymm_in_out avx <<'E'
    let a = _mm256_set1_epi32(11);
    let r: __m256i;
    std::arch::asm!("vmovdqa {0}, {1}", out(ymm_reg) r, in(ymm_reg) a, options(nostack, nomem, pure));
    let mut o = [0i32; 8];
    _mm256_storeu_si256(o.as_mut_ptr().cast(), r);
    println!("{:?}", o);
E
emit raw_ymm_inout avx <<'E'
    let mut a = _mm256_set1_epi32(13);
    std::arch::asm!("vpaddd {0}, {0}, {0}", inout(ymm_reg) a, options(nostack, nomem, pure));
    let mut o = [0i32; 8];
    _mm256_storeu_si256(o.as_mut_ptr().cast(), a);
    println!("{:?}", o);
E
emit raw_ymm_f64x4 avx <<'E'
    let a = _mm256_set1_pd(1.25);
    let r: __m256d;
    std::arch::asm!("vmovapd {0}, {1}", out(ymm_reg) r, in(ymm_reg) a, options(nostack, nomem, pure));
    let mut o = [0.0f64; 4];
    _mm256_storeu_pd(o.as_mut_ptr(), r);
    println!("{:?}", o);
E
emit raw_zmm_in_out avx512f <<'E'
    let a = _mm512_set1_epi32(17);
    let r: __m512i;
    std::arch::asm!("vmovdqa64 {0}, {1}", out(zmm_reg) r, in(zmm_reg) a, options(nostack, nomem, pure));
    let mut o = [0i32; 16];
    _mm512_storeu_si512(o.as_mut_ptr().cast(), r);
    println!("{:?}", o);
E
emit raw_zmm_inout avx512f <<'E'
    let mut a = _mm512_set1_epi64(19);
    std::arch::asm!("vpaddq {0}, {0}, {0}", inout(zmm_reg) a, options(nostack, nomem, pure));
    let mut o = [0i64; 8];
    _mm512_storeu_si512(o.as_mut_ptr().cast(), a);
    println!("{:?}", o);
E
# scalar-in-a-wide-register-class: legal per rustc, must keep working
emit raw_ymm_scalar_f64 avx <<'E'
    let a = 3.5f64;
    let r: f64;
    std::arch::asm!("vmovapd {0}, {1}", out(ymm_reg) r, in(ymm_reg) a, options(nostack, nomem, pure));
    println!("{:?}", r);
E
# 128-bit operand: the already-working control case
emit raw_xmm_control sse2 <<'E'
    let mut a = _mm_set1_epi32(23);
    std::arch::asm!("paddd {0}, {0}", inout(xmm_reg) a, options(nostack, nomem, pure));
    let mut o = [0i32; 4];
    _mm_storeu_si128(o.as_mut_ptr().cast(), a);
    println!("{:?}", o);
E

fail=0
for f in "$WORK"/*.rs; do
    name="$(basename "$f" .rs)"
    out=$(RUSTFLAGS_EXTRA='-Ctarget-cpu=native' bash "$HERE/diffcheck.sh" "$f" 2>&1)
    if grep -q "^RESULT: identical" <<<"$out"; then
        printf '  OK      %-22s %s\n' "$name" "$(grep -m1 -A1 '=== LLVM: exit 0 ===' <<<"$out" | tail -1)"
    else
        reason=$(grep -m1 -oP 'Error while defining [^ ]+: \K.*' <<<"$out")
        [ -z "$reason" ] && reason=$(grep -m1 "^RESULT:" <<<"$out")
        printf '  DIFFER  %-22s %s\n' "$name" "$reason"
        fail=1
    fi
done
exit $fail
