import os, sys

# Generates the scalar int<->float / float<->float cast matrix used by candidate 013.
#   python3 scalar_cast_matrix_gen.py <outdir>
outdir = sys.argv[1]
os.makedirs(outdir, exist_ok=True)
HDR="#![feature(f16, f128)]\n"
ints=["i8","u8","i16","u16","i32","u32","i64","u64","i128","u128"]
flts=["f16","f32","f64","f128"]
n=0
for i in ints:
    for f in flts:
        src=HDR+f"""
#[inline(never)] fn a(x: {i}) -> {f} {{ x as {f} }}
#[inline(never)] fn b(x: {f}) -> {i} {{ x as {i} }}
fn main() {{
    for v in [0 as {i}, 1 as {i}, {i}::MIN, {i}::MAX, 7 as {i}, ({i}::MAX/3)] {{
        let y = a(v); println!("{{}} {{}}", y.to_bits(), b(y));
    }}
    for v in [0.0{f}, -0.0{f}, 1.5{f}, -1.5{f}, {f}::NAN, {f}::INFINITY, {f}::NEG_INFINITY, {f}::MAX, {f}::MIN, 1e30{f}] {{
        println!("{{}}", b(v));
    }}
}}
"""
        open(f"{outdir}/int_{i}_{f}.rs","w").write(src); n+=1
for f1 in flts:
    for f2 in flts:
        src=HDR+f"""
#[inline(never)] fn a(x: {f1}) -> {f2} {{ x as {f2} }}
fn main() {{
    for v in [0.0{f1}, -0.0{f1}, 1.5{f1}, -1.5{f1}, {f1}::NAN, {f1}::INFINITY, {f1}::NEG_INFINITY, {f1}::MAX, {f1}::MIN, {f1}::MIN_POSITIVE] {{
        println!("{{}}", a(v).to_bits());
    }}
}}
"""
        open(f"{outdir}/flt_{f1}_{f2}.rs","w").write(src); n+=1
print(n)
