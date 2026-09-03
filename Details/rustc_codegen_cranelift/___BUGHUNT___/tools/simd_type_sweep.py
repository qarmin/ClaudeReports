#!/usr/bin/env python3
"""Generate one single-file Rust program per (generic SIMD intrinsic, lane type, lane count)
and classify what cg_clif does with it.

Purpose: find Cranelift lowering gaps that reach `src/base.rs`'s
`Err(ModuleError::Compilation(..)) => bug!(..)` funnel, i.e. an ICE ("the compiler unexpectedly
panicked") on valid Rust instead of a diagnostic.

    python3 simd_type_sweep.py gen  <outdir>
    python3 simd_type_sweep.py run  <outdir> [--clif <root>] [-j N]

`run` compiles each file with dist/rustc-clif only (cheap) and buckets the result:
  FUNNEL  - "Compilation(Unsupported" / "Error while defining"  <- the interesting bucket
  ICE     - some other "internal compiler error"
  REJECT  - a normal compile error (rustc frontend rejected the program: not our problem)
  OK      - compiled
Only the FUNNEL/ICE cases are then worth re-checking against LLVM with diffcheck.sh.
"""

import os
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor

INT_TYPES = ["i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64", "i128", "u128"]
FLOAT_TYPES = ["f16", "f32", "f64"]
TYPES = INT_TYPES + FLOAT_TYPES
COUNTS = [1, 2, 3, 4, 8, 16, 32]

BITS = {"i8": 8, "u8": 8, "i16": 16, "u16": 16, "i32": 32, "u32": 32, "i64": 64, "u64": 64,
        "i128": 128, "u128": 128, "f16": 16, "f32": 32, "f64": 64}


def signed_of(ty):
    return {8: "i8", 16: "i16", 32: "i32", 64: "i64", 128: "i128"}[BITS[ty]]


def mask_int(count):
    if count <= 8:
        return "u8"
    if count <= 16:
        return "u16"
    if count <= 32:
        return "u32"
    return "u64"


HEADER = """#![feature(repr_simd, core_intrinsics, f16, f128)]
#![allow(internal_features, dead_code, unused_mut, unused_variables)]
use std::intrinsics::simd::*;

#[repr(simd)]
#[derive(Copy, Clone)]
struct V([{ty}; {n}]);
#[repr(simd)]
#[derive(Copy, Clone)]
struct M([{sty}; {n}]);

#[inline(never)]
fn show<T: Copy>(v: T) {{
    let n = std::mem::size_of::<T>();
    let p = &v as *const T as *const u8;
    let mut acc: u64 = 0;
    for i in 0..n {{
        acc = acc.wrapping_mul(31).wrapping_add(unsafe {{ *p.add(i) }} as u64);
    }}
    println!("{{acc}}");
}}

fn main() {{
    let s = std::env::args().count() as u64; // 1, but opaque to the optimizer
    let mut a = [0 as {ty}; {n}];
    let mut b = [0 as {ty}; {n}];
    let mut m = [0 as {sty}; {n}];
    for i in 0..{n} {{
        a[i] = {mk_a};
        b[i] = {mk_b};
        m[i] = if i % 2 == 0 {{ !0 }} else {{ 0 }};
    }}
    let a = V(a);
    let b = V(b);
    let m = M(m);
    unsafe {{
{body}
    }}
}}
"""


def make_source(intr, ty, n):
    sty = signed_of(ty)
    is_float = ty in FLOAT_TYPES
    bits = BITS[ty]
    if is_float:
        mk_a = f"((i as u64 + s) as f64) as {ty}"
        mk_b = f"((i as u64 + s + 1) as f64) as {ty}"
    else:
        mk_a = f"(i as u64).wrapping_add(s) as {ty}"
        mk_b = f"((i as u64).wrapping_add(s) % {bits} + 1) as {ty}"

    body = None
    unary_float_libm = ["simd_fsin", "simd_fcos", "simd_fexp", "simd_fexp2", "simd_flog",
                        "simd_flog10", "simd_flog2", "simd_round", "simd_round_ties_even"]
    unary_float_native = ["simd_fabs", "simd_fsqrt", "simd_ceil", "simd_floor", "simd_trunc"]
    unary_int = ["simd_bswap", "simd_bitreverse", "simd_ctlz", "simd_ctpop", "simd_cttz"]
    binop_any = ["simd_add", "simd_sub", "simd_mul", "simd_div"]
    binop_int_only = ["simd_shl", "simd_shr", "simd_and", "simd_or", "simd_xor",
                      "simd_saturating_add", "simd_saturating_sub"]
    cmp = ["simd_eq", "simd_ne", "simd_lt", "simd_le", "simd_gt", "simd_ge"]

    if intr in unary_float_libm + unary_float_native:
        if not is_float:
            return None
        body = f"        show::<V>({intr}(a));"
    elif intr == "simd_neg":
        body = f"        show::<V>({intr}(a));"
        if ty.startswith("u"):
            return None
    elif intr in unary_int:
        if is_float:
            return None
        body = f"        show::<V>({intr}(a));"
    elif intr in binop_any:
        body = f"        show::<V>({intr}(a, b));"
    elif intr == "simd_rem":
        body = f"        show::<V>({intr}(a, b));"
    elif intr in binop_int_only:
        if is_float:
            return None
        body = f"        show::<V>({intr}(a, b));"
    elif intr in cmp:
        body = f"        show::<M>({intr}(a, b));"
    elif intr in ("simd_minimum_number_nsz", "simd_maximum_number_nsz"):
        if not is_float:
            return None
        body = f"        show::<V>({intr}(a, b));"
    elif intr in ("simd_fma", "simd_relaxed_fma"):
        if not is_float:
            return None
        body = f"        show::<V>({intr}(a, b, a));"
    elif intr == "simd_select":
        body = "        show::<V>(simd_select(m, a, b));"
    elif intr == "simd_select_bitmask":
        body = f"        show::<V>(simd_select_bitmask({mask_int(n)}::MAX, a, b));"
    elif intr == "simd_bitmask":
        body = f"        show::<{mask_int(n)}>(simd_bitmask(a));"
    elif intr == "simd_as_f64":
        body = f"""        #[repr(simd)] #[derive(Copy, Clone)] struct T([f64; {n}]);
        show::<T>(simd_as::<V, T>(a));"""
    elif intr == "simd_as_i32":
        body = f"""        #[repr(simd)] #[derive(Copy, Clone)] struct T([i32; {n}]);
        show::<T>(simd_as::<V, T>(a));"""
    elif intr == "simd_cast_f16":
        body = f"""        #[repr(simd)] #[derive(Copy, Clone)] struct T([f16; {n}]);
        show::<T>(simd_cast::<V, T>(a));"""
    elif intr == "simd_cast_f128":
        body = f"""        #[repr(simd)] #[derive(Copy, Clone)] struct T([f128; {n}]);
        show::<T>(simd_cast::<V, T>(a));"""
    elif intr == "simd_extract":
        body = f"        show::<{ty}>(simd_extract(a, 0));"
    elif intr == "simd_extract_dyn":
        body = f"        show::<{ty}>(simd_extract_dyn(a, (s - 1) as u32));"
    elif intr == "simd_insert":
        body = f"        show::<V>(simd_insert(a, 0, b_lane(&b)));"
    elif intr == "simd_insert_dyn":
        body = f"        show::<V>(simd_insert_dyn(a, (s - 1) as u32, b_lane(&b)));"
    elif intr in ("simd_reduce_add_ordered", "simd_reduce_mul_ordered"):
        body = f"        show::<{ty}>({intr}(a, a_lane(&a)));"
    elif intr in ("simd_reduce_add_unordered", "simd_reduce_mul_unordered",
                  "simd_reduce_min", "simd_reduce_max"):
        body = f"        show::<{ty}>({intr}(a));"
    elif intr in ("simd_reduce_and", "simd_reduce_or", "simd_reduce_xor"):
        if is_float:
            return None
        body = f"        show::<{ty}>({intr}(a));"
    elif intr in ("simd_reduce_all", "simd_reduce_any"):
        body = f"        show::<bool>({intr}(m));"
    elif intr == "simd_gather":
        body = f"""        let mut ptrs = [std::ptr::null::<{ty}>(); {n}];
        for i in 0..{n} {{ ptrs[i] = &raw const a.0[i]; }}
        #[repr(simd)] #[derive(Copy, Clone)] struct P([*const {ty}; {n}]);
        show::<V>(simd_gather(b, P(ptrs), m));"""
    elif intr == "simd_masked_load":
        body = f"""        let mut ptrs = [std::ptr::null::<{ty}>(); {n}];
        show::<V>(simd_masked_load(m, &raw const a.0[0], b));"""
    else:
        return None

    helpers = ""
    if "a_lane(" in body or "b_lane(" in body:
        helpers = f"""
#[inline(never)]
fn a_lane(v: &V) -> {ty} {{ v.0[0] }}
#[inline(never)]
fn b_lane(v: &V) -> {ty} {{ v.0[0] }}
"""

    src = HEADER.format(ty=ty, n=n, sty=sty, mk_a=mk_a, mk_b=mk_b, body=body)
    return src.replace("fn main() {", helpers + "\nfn main() {", 1)


ALL_INTRINSICS = [
    "simd_neg", "simd_bswap", "simd_bitreverse", "simd_ctlz", "simd_ctpop", "simd_cttz",
    "simd_fabs", "simd_fsqrt", "simd_ceil", "simd_floor", "simd_trunc",
    "simd_fsin", "simd_fcos", "simd_fexp", "simd_fexp2", "simd_flog", "simd_flog10", "simd_flog2",
    "simd_round", "simd_round_ties_even",
    "simd_add", "simd_sub", "simd_mul", "simd_div", "simd_rem",
    "simd_shl", "simd_shr", "simd_and", "simd_or", "simd_xor",
    "simd_saturating_add", "simd_saturating_sub",
    "simd_eq", "simd_ne", "simd_lt", "simd_le", "simd_gt", "simd_ge",
    "simd_minimum_number_nsz", "simd_maximum_number_nsz",
    "simd_fma", "simd_relaxed_fma",
    "simd_select", "simd_select_bitmask", "simd_bitmask",
    "simd_as_f64", "simd_as_i32", "simd_cast_f16", "simd_cast_f128",
    "simd_extract", "simd_extract_dyn", "simd_insert", "simd_insert_dyn",
    "simd_reduce_add_ordered", "simd_reduce_mul_ordered",
    "simd_reduce_add_unordered", "simd_reduce_mul_unordered",
    "simd_reduce_min", "simd_reduce_max",
    "simd_reduce_and", "simd_reduce_or", "simd_reduce_xor",
    "simd_reduce_all", "simd_reduce_any",
    "simd_gather", "simd_masked_load",
]


def gen(outdir):
    os.makedirs(outdir, exist_ok=True)
    count = 0
    for intr in ALL_INTRINSICS:
        for ty in TYPES:
            for n in COUNTS:
                src = make_source(intr, ty, n)
                if src is None:
                    continue
                with open(os.path.join(outdir, f"{intr}__{ty}__{n}.rs"), "w") as f:
                    f.write(src)
                count += 1
    print(f"generated {count} cases in {outdir}")


def classify(text):
    if "Compilation(Unsupported" in text or "Error while defining" in text:
        return "FUNNEL"
    if "internal compiler error" in text or "panicked at" in text:
        return "ICE"
    if "error" in text:
        return "REJECT"
    return "OK"


def run(outdir, clif_root, jobs):
    rustc = os.path.join(clif_root, "dist", "rustc-clif")
    files = sorted(f for f in os.listdir(outdir) if f.endswith(".rs"))
    binroot = os.path.join(outdir, "bin")
    os.makedirs(binroot, exist_ok=True)

    def one(f):
        src = os.path.join(outdir, f)
        out = os.path.join(binroot, f[:-3] + ".o")
        p = subprocess.run([rustc, "-Copt-level=0", "--emit=obj", "-o", out, src],
                           capture_output=True, text=True, timeout=300)
        return f, classify(p.stdout + p.stderr), (p.stdout + p.stderr)

    buckets = {"FUNNEL": [], "ICE": [], "REJECT": [], "OK": []}
    with ThreadPoolExecutor(max_workers=jobs) as ex:
        for f, kind, text in ex.map(one, files):
            buckets[kind].append((f, text))

    for kind in ("FUNNEL", "ICE"):
        print(f"===== {kind}: {len(buckets[kind])}")
        for f, text in buckets[kind]:
            first = ""
            for line in text.splitlines():
                if "Unsupported" in line or "internal compiler error" in line or "panicked at" in line:
                    first = line.strip()[:220]
                    break
            print(f"  {f}: {first}")
    print(f"===== REJECT: {len(buckets['REJECT'])}   OK: {len(buckets['OK'])}")
    with open(os.path.join(outdir, "sweep_report.txt"), "w") as fh:
        for kind in buckets:
            for f, text in buckets[kind]:
                fh.write(f"### {kind} {f}\n{text}\n")


if __name__ == "__main__":
    mode = sys.argv[1]
    outdir = sys.argv[2]
    if mode == "gen":
        gen(outdir)
    else:
        root = "/home/rafal/Projekty/Obce/rustc_codegen_cranelift"
        jobs = 24
        args = sys.argv[3:]
        if "--clif" in args:
            root = args[args.index("--clif") + 1]
        if "-j" in args:
            jobs = int(args[args.index("-j") + 1])
        run(outdir, root, jobs)
