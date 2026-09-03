import os, sys
outdir = sys.argv[1]
os.makedirs(outdir, exist_ok=True)
HDR = """#![feature(f16, f128, core_intrinsics, repr_simd)]
#![allow(internal_features)]
use std::intrinsics::simd::{simd_cast, simd_as};
"""
variants = []
for ty, bitsty, vals in [("f16","u16",["1.5","-2.25","3.0","0.5","0.0","-0.0","f16::INFINITY","f16::NAN","f16::MIN","f16::MAX","6.103515625e-5","1.0"]),
                         ("f128","u128",["1.5","-2.25","3.0","0.5","0.0","-0.0","f128::INFINITY","f128::NAN","f128::MIN","f128::MAX","1e-4000","1.0"])]:
    for n in [1,2,4,8,16]:
        for intr in ["simd_cast","simd_as"]:
            name = f"simd_{intr}_{ty}_id_n{n}"
            v = ", ".join(vals[:n] if n <= len(vals) else (vals*((n//len(vals))+1))[:n])
            src = HDR + f"""
#[repr(simd)]
#[derive(Copy, Clone)]
struct V([{ty}; {n}]);

#[inline(never)]
fn go(a: V) -> V {{ unsafe {{ {intr}(a) }} }}

fn main() {{
    let a = V([{v}]);
    let b = go(a);
    let out: [{ty}; {n}] = unsafe {{ std::mem::transmute(b) }};
    println!("{{:?}}", out.map(|x| x.to_bits()));
}}
"""
            variants.append((name, src))
    # scalar identity cast
    name = f"scalar_cast_{ty}_id"
    src = HDR + f"""
#[inline(never)]
fn go(a: {ty}) -> {ty} {{ a as {ty} }}
fn main() {{
    for v in [{", ".join(vals)}] {{ println!("{{}}", go(v).to_bits()); }}
}}
"""
    variants.append((name, src))
# cross f16<->f128
for a_ty, b_ty, n in [("f16","f128",4),("f128","f16",4),("f16","f32",4),("f32","f16",4),("f128","f64",4),("f64","f128",4)]:
    for intr in ["simd_cast","simd_as"]:
        name = f"simd_{intr}_{a_ty}_to_{b_ty}_n{n}"
        src = HDR + f"""
#[repr(simd)]
#[derive(Copy, Clone)]
struct A([{a_ty}; {n}]);
#[repr(simd)]
#[derive(Copy, Clone)]
struct B([{b_ty}; {n}]);

#[inline(never)]
fn go(a: A) -> B {{ unsafe {{ {intr}(a) }} }}

fn main() {{
    let a = A([1.5, -2.25, 3.0, 0.5]);
    let b = go(a);
    let out: [{b_ty}; {n}] = unsafe {{ std::mem::transmute(b) }};
    println!("{{:?}}", out.map(|x| x.to_bits()));
}}
"""
        variants.append((name, src))
for name, src in variants:
    open(os.path.join(outdir, name + ".rs"), "w").write(src)
print(len(variants), "variants written to", outdir)
