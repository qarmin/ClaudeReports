#!/usr/bin/env python3
"""Generated LLVM-vs-cg_clif sweep over every `value_lane_dyn` caller reachable on x86-64.

`CValue::value_lane_dyn` (src/value_and_place.rs) turns a runtime lane index into a raw
`imul_imm_u` + `offset_value` with no bounds/mask handling, so each caller has to mask its own
index. This script hammers the callers that can actually be executed on an x86-64 host:

  * `sym::simd_extract_dyn`  / `sym::simd_insert_dyn` (src/intrinsics/simd.rs, via write_lane_dyn)
  * `llvm.x86.ssse3.pshuf.b.128` and `llvm.x86.avx2.pshuf.b` (src/intrinsics/llvm_x86.rs)
  * `llvm.x86.avx2.permd` (src/intrinsics/llvm_x86.rs) - only with in-range selectors, since
    out-of-range selectors there are the already-confirmed candidate 001 miscompilation and would
    swamp every run with the same failure.

The two aarch64 callers (`llvm.aarch64.neon.tbl1.v{8,16}i8`) cannot be exercised here: there is no
aarch64 libc/linker/assembler in this sandbox (see 007.value_lane_dyn_caller_audit/RESULT.md).

Usage:
    python3 ___BUGHUNT___/tools/lane_dyn_sweep.py [--count 30] [--seed 0] [--clif-root DIR]
                                                  [--opt] [--native] [--keep]

Exit status 0 = every generated program agreed with LLVM, 1 = at least one disagreed (the
offending .rs file is kept and its path printed).
"""

import argparse
import os
import random
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DIFFCHECK = ROOT / "___BUGHUNT___" / "tools" / "diffcheck.sh"

# (rust type, printf-able, python value generator)
LANE_TYPES = [
    ("u8", lambda r: r.randrange(0, 256)),
    ("i8", lambda r: r.randrange(-128, 128)),
    ("u16", lambda r: r.randrange(0, 1 << 16)),
    ("i16", lambda r: r.randrange(-(1 << 15), 1 << 15)),
    ("u32", lambda r: r.randrange(0, 1 << 32)),
    ("i32", lambda r: r.randrange(-(1 << 31), 1 << 31)),
    ("u64", lambda r: r.randrange(0, 1 << 64)),
    ("i64", lambda r: r.randrange(-(1 << 63), 1 << 63)),
    ("u128", lambda r: r.randrange(0, 1 << 128)),
    ("i128", lambda r: r.randrange(-(1 << 127), 1 << 127)),
]
# Deliberately includes non-power-of-two lane counts: `value_lane_dyn` cannot be fixed by a plain
# bitmask for those, so they are the interesting boundary for any masking change.
LANE_COUNTS = [1, 2, 3, 4, 5, 7, 8, 12, 16, 31, 32, 64]


def gen_generic(r):
    """simd_extract_dyn / simd_insert_dyn over a random #[repr(simd)] vector."""
    ty, mk = r.choice(LANE_TYPES)
    n = r.choice(LANE_COUNTS)
    elems = [mk(r) for _ in range(n)]
    marker = mk(r)
    # index order deliberately hits 0 and n-1 first, then random in-bounds picks
    idxs = [0, n - 1] + [r.randrange(0, n) for _ in range(min(2 * n, 24))]
    return f"""#![feature(repr_simd, core_intrinsics)]
#![allow(internal_features)]
use std::intrinsics::simd::{{simd_extract_dyn, simd_insert_dyn}};

#[repr(simd)]
#[derive(Copy, Clone)]
struct V([{ty}; {n}]);

#[inline(never)]
fn bb(x: u32) -> u32 {{ std::hint::black_box(x) }}

fn main() {{
    let v = V([{", ".join(str(e) for e in elems)}]);
    let mut h: u128 = 0xcbf29ce484222325;
    for i in [{", ".join(str(i) for i in idxs)}] {{
        let e: {ty} = unsafe {{ simd_extract_dyn(v, bb(i as u32)) }};
        h = h.wrapping_mul(0x100000001b3).wrapping_add(e as u128);
        let w: V = unsafe {{ simd_insert_dyn(v, bb(i as u32), {marker}) }};
        for j in 0..{n}u32 {{
            let f: {ty} = unsafe {{ simd_extract_dyn(w, bb(j)) }};
            h = h.wrapping_mul(0x100000001b3).wrapping_add(f as u128);
        }}
    }}
    println!("{ty}x{n} {{:032x}}", h);
}}
"""


def gen_pshufb(r):
    """_mm_shuffle_epi8 / _mm256_shuffle_epi8 with fully random selector bytes."""
    a16 = [r.randrange(-128, 128) for _ in range(16)]
    a32 = [r.randrange(-128, 128) for _ in range(32)]
    rounds = r.randrange(200, 600)
    seed = r.randrange(1, 1 << 31)
    return f"""use std::arch::x86_64::*;

fn main() {{
    let a16: [i8; 16] = [{", ".join(str(x) for x in a16)}];
    let a32: [i8; 32] = [{", ".join(str(x) for x in a32)}];
    let mut s: u32 = {seed};
    let mut next = move || {{ s = s.wrapping_mul(1664525).wrapping_add(1013904223); (s >> 13) as i8 }};
    let mut h: u64 = 0xcbf29ce484222325;
    for _ in 0..{rounds} {{
        let b16: [i8; 16] = core::array::from_fn(|_| next());
        let b32: [i8; 32] = core::array::from_fn(|_| next());
        unsafe {{
            let r = _mm_shuffle_epi8(
                _mm_loadu_si128(a16.as_ptr() as *const __m128i),
                _mm_loadu_si128(b16.as_ptr() as *const __m128i),
            );
            let mut o = [0i8; 16];
            _mm_storeu_si128(o.as_mut_ptr() as *mut __m128i, r);
            for x in o {{ h = h.wrapping_mul(0x100000001b3) ^ (x as u8 as u64); }}

            let r2 = _mm256_shuffle_epi8(
                _mm256_loadu_si256(a32.as_ptr() as *const __m256i),
                _mm256_loadu_si256(b32.as_ptr() as *const __m256i),
            );
            let mut o2 = [0i8; 32];
            _mm256_storeu_si256(o2.as_mut_ptr() as *mut __m256i, r2);
            for x in o2 {{ h = h.wrapping_mul(0x100000001b3) ^ (x as u8 as u64); }}
        }}
    }}
    println!("pshufb {{:016x}}", h);
}}
"""


def gen_permd(r):
    """_mm256_permutevar8x32_epi32 with in-range selectors only (see module docstring)."""
    a = [r.randrange(-(1 << 31), 1 << 31) for _ in range(8)]
    rounds = r.randrange(200, 600)
    seed = r.randrange(1, 1 << 31)
    return f"""use std::arch::x86_64::*;

fn main() {{
    let a: [i32; 8] = [{", ".join(str(x) for x in a)}];
    let mut s: u32 = {seed};
    let mut next = move || {{ s = s.wrapping_mul(1664525).wrapping_add(1013904223); ((s >> 13) & 7) as i32 }};
    let mut h: u64 = 0xcbf29ce484222325;
    for _ in 0..{rounds} {{
        let idx: [i32; 8] = core::array::from_fn(|_| next());
        unsafe {{
            let r = _mm256_permutevar8x32_epi32(
                _mm256_loadu_si256(a.as_ptr() as *const __m256i),
                _mm256_loadu_si256(idx.as_ptr() as *const __m256i),
            );
            let mut o = [0i32; 8];
            _mm256_storeu_si256(o.as_mut_ptr() as *mut __m256i, r);
            for x in o {{ h = h.wrapping_mul(0x100000001b3) ^ (x as u32 as u64); }}
        }}
    }}
    println!("permd {{:016x}}", h);
}}
"""


GENERATORS = [("generic", gen_generic), ("pshufb", gen_pshufb), ("permd", gen_permd)]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--count", type=int, default=30)
    ap.add_argument("--seed", type=int, default=0)
    ap.add_argument("--clif-root", default=None)
    ap.add_argument("--opt", action="store_true")
    ap.add_argument("--native", action="store_true")
    ap.add_argument("--keep", action="store_true", help="keep every generated .rs, not just failures")
    args = ap.parse_args()

    r = random.Random(args.seed)
    env = dict(os.environ)
    if args.clif_root:
        env["CLIF_ROOT"] = args.clif_root
    if args.opt:
        env["OPT"] = "1"
    if args.native:
        env["RUSTFLAGS_EXTRA"] = "-Ctarget-cpu=native"

    outdir = Path(tempfile.mkdtemp(prefix="lane_dyn_sweep.", dir=ROOT / "tmp"))
    failures = []
    for i in range(args.count):
        name, gen = GENERATORS[i % len(GENERATORS)]
        src = outdir / f"{i:03d}_{name}.rs"
        src.write_text(gen(r))
        proc = subprocess.run(
            ["bash", str(DIFFCHECK), str(src)], env=env, capture_output=True, text=True
        )
        status = "ok" if proc.returncode == 0 else "MISMATCH"
        print(f"[{i:03d}] {name:8s} {status}", flush=True)
        if proc.returncode != 0:
            failures.append(src)
            (src.with_suffix(".log")).write_text(proc.stdout + proc.stderr)
        elif not args.keep:
            src.unlink()

    print(f"\n{args.count - len(failures)}/{args.count} agreed with LLVM")
    for f in failures:
        print(f"  mismatch: {f} (output in {f.with_suffix('.log')})")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
