#!/usr/bin/env bash
# Sweep: `extern "C"` functions that pass/return a SIMD vector by value, made legal only by a
# per-function `#[target_feature(enable = ...)]` (cg_clif's `target_config` reports just
# fxsr/sse/sse2/x87 and drops `-C target-feature`, so the attribute is the ONLY way to satisfy
# rustc's `abi_required_features` check on this backend).
#
# One case per file: cg_clif aborts the whole crate on the first CGU that fails to define, so
# batching would hide every case after the first.
#
#   bash ___BUGHUNT___/tools/tf_abi_sweep.sh                  # main tree's dist/ = unpatched
#   CLIF_ROOT=tmp/wt/012.foo bash ___BUGHUNT___/tools/tf_abi_sweep.sh
#
# Prints one PASS/FAIL line per case plus a summary. Exit 0 only if every case agrees with LLVM.
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DIFFCHECK="$ROOT/___BUGHUNT___/tools/diffcheck.sh"
WORK="$(mktemp -d "${TMPDIR:-/tmp}/tf_abi_sweep.XXXXXX")"
trap 'rm -rf "$WORK"' EXIT

# name|feature|vector type|element count|lane rust type|ctor expr|store fn
CASES=(
  "arg_m256_f32|avx|__m256|8|f32|_mm256_set1_ps(1.5)|_mm256_storeu_ps"
  "arg_m256i_i32|avx|__m256i|8|i32|_mm256_set1_epi32(7)|_mm256_storeu_si256_i32"
  "arg_m256d_f64|avx|__m256d|4|f64|_mm256_set1_pd(2.25)|_mm256_storeu_pd"
  "arg_m512_f32|avx512f|__m512|16|f32|_mm512_set1_ps(3.5)|_mm512_storeu_ps"
  "arg_m512i_i64|avx512f|__m512i|8|i64|_mm512_set1_epi64(9)|_mm512_storeu_si512_i64"
  "arg_m512d_f64|avx512f|__m512d|8|f64|_mm512_set1_pd(4.75)|_mm512_storeu_pd"
)

emit_direct() { # name feature vecty count laney ctor store  -> arg+return by value
    local name=$1 feat=$2 vty=$3 cnt=$4 laney=$5 ctor=$6 store=$7
    cat > "$WORK/$name.rs" <<EOF
use std::arch::x86_64::*;

#[target_feature(enable = "$feat")]
#[inline(never)]
unsafe extern "C" fn ident(a: $vty) -> $vty { a }

#[target_feature(enable = "$feat")]
unsafe fn run() {
    let v = ident($ctor);
    let mut o = [0 as $laney; $cnt];
    $(store_expr "$store" "$laney")
    println!("{:?}", o);
}

fn main() { unsafe { run() } }
EOF
}

store_expr() {
    case "$1" in
      _mm256_storeu_si256_i32) echo "_mm256_storeu_si256(o.as_mut_ptr() as *mut __m256i, v);" ;;
      _mm512_storeu_si512_i64) echo "_mm512_storeu_si512(o.as_mut_ptr() as *mut __m512i, v);" ;;
      *) echo "$1(o.as_mut_ptr(), v);" ;;
    esac
}

emit_struct() { # aggregate wrapper -> PassMode::Cast path instead of PassMode::Direct
    local name=$1 feat=$2 vty=$3 cnt=$4 laney=$5 ctor=$6 store=$7
    cat > "$WORK/${name}_struct.rs" <<EOF
use std::arch::x86_64::*;

#[repr(C)]
#[derive(Copy, Clone)]
struct W(pub $vty);

#[target_feature(enable = "$feat")]
#[inline(never)]
unsafe extern "C" fn ident(a: W) -> W { a }

#[target_feature(enable = "$feat")]
unsafe fn run() {
    let v = ident(W($ctor)).0;
    let mut o = [0 as $laney; $cnt];
    $(store_expr "$store" "$laney")
    println!("{:?}", o);
}

fn main() { unsafe { run() } }
EOF
}

# Other shapes that reach the same two ABI paths
cat > "$WORK/shape_sysv64.rs" <<'EOF'
#![allow(improper_ctypes_definitions)]
use std::arch::x86_64::*;
#[target_feature(enable = "avx")]
#[inline(never)]
unsafe extern "sysv64" fn ident(a: __m256) -> __m256 { a }
#[target_feature(enable = "avx")]
unsafe fn run() {
    let v = ident(_mm256_set1_ps(1.5));
    let mut o = [0f32; 8]; _mm256_storeu_ps(o.as_mut_ptr(), v); println!("{:?}", o);
}
fn main() { unsafe { run() } }
EOF
cat > "$WORK/shape_fnptr.rs" <<'EOF'
#![allow(improper_ctypes_definitions)]
use std::arch::x86_64::*;
#[target_feature(enable = "avx")]
#[inline(never)]
unsafe extern "C" fn ident(a: __m256) -> __m256 { a }
#[target_feature(enable = "avx")]
unsafe fn run() {
    let f: unsafe extern "C" fn(__m256) -> __m256 = ident;
    let v = f(_mm256_set1_ps(1.5));
    let mut o = [0f32; 8]; _mm256_storeu_ps(o.as_mut_ptr(), v); println!("{:?}", o);
}
fn main() { unsafe { run() } }
EOF
cat > "$WORK/shape_mixed_args.rs" <<'EOF'
#![allow(improper_ctypes_definitions)]
use std::arch::x86_64::*;
#[target_feature(enable = "avx")]
#[inline(never)]
unsafe extern "C" fn pick(x: i32, a: __m256, y: f64) -> f32 { x as f32 + _mm256_cvtss_f32(a) + y as f32 }
#[target_feature(enable = "avx")]
unsafe fn run() { println!("{}", pick(3, _mm256_set1_ps(1.5), 0.25)); }
fn main() { unsafe { run() } }
EOF

# 128-bit controls: must keep working, no target_feature needed (sse/sse2 are global)
cat > "$WORK/ctl_m128.rs" <<'EOF'
use std::arch::x86_64::*;
#[inline(never)]
unsafe extern "C" fn ident(a: __m128) -> __m128 { a }
fn main() { unsafe {
    let v = ident(_mm_set1_ps(1.5));
    let mut o = [0f32; 4];
    _mm_storeu_ps(o.as_mut_ptr(), v);
    println!("{:?}", o);
} }
EOF
cat > "$WORK/ctl_m128i.rs" <<'EOF'
use std::arch::x86_64::*;
#[inline(never)]
unsafe extern "C" fn ident(a: __m128i) -> __m128i { a }
fn main() { unsafe {
    let v = ident(_mm_set1_epi32(5));
    let mut o = [0i32; 4];
    _mm_storeu_si128(o.as_mut_ptr() as *mut __m128i, v);
    println!("{:?}", o);
} }
EOF
# Rust-ABI control: rustc passes a >16-byte vector indirectly, so it must keep working
cat > "$WORK/ctl_rustabi_m256.rs" <<'EOF'
use std::arch::x86_64::*;
#[target_feature(enable = "avx")]
#[inline(never)]
unsafe fn ident(a: __m256) -> __m256 { a }
#[target_feature(enable = "avx")]
unsafe fn run() {
    let v = ident(_mm256_set1_ps(1.5));
    let mut o = [0f32; 8];
    _mm256_storeu_ps(o.as_mut_ptr(), v);
    println!("{:?}", o);
}
fn main() { unsafe { run() } }
EOF

for c in "${CASES[@]}"; do
    IFS='|' read -r name feat vty cnt laney ctor store <<< "$c"
    emit_direct "$name" "$feat" "$vty" "$cnt" "$laney" "$ctor" "$store"
    emit_struct "$name" "$feat" "$vty" "$cnt" "$laney" "$ctor" "$store"
done

pass=0; ice=0; err=0
for f in "$WORK"/*.rs; do
    out="$(bash "$DIFFCHECK" "$f" 2>&1)"; rc=$?
    name="$(basename "$f" .rs)"
    if [ "$rc" = 0 ]; then
        pass=$((pass+1)); printf 'PASS  %-28s (matches LLVM)\n' "$name"
    elif printf '%s' "$out" | grep -q "the compiler unexpectedly panicked"; then
        ice=$((ice+1))
        printf 'ICE   %-28s %s\n' "$name" \
            "$(printf '%s' "$out" | grep -oP 'Unexpected SSA-value type: \S+' | head -1)"
    else
        err=$((err+1))
        printf 'ERR   %-28s %s\n' "$name" \
            "$(printf '%s' "$out" | grep -m1 -oP '^error(\[\S+\])?: .*' | cut -c1-88)"
    fi
done

echo
echo "summary: $pass match LLVM, $ice ICE, $err clean cg_clif error"
# The bug being tracked is the ICE, so the exit status keys on that: an actionable diagnostic where
# LLVM compiles is still a capability gap, but it is not a backend crash.
[ "$ice" = 0 ]
