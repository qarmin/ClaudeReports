#!/usr/bin/env python3
"""Sweep the 24 public `_mm_comiXX_ss/_sd` / `_mm_ucomiXX_ss/_sd` scalar
compare-and-set-flags intrinsics against LLVM, one program per intrinsic.

They lower to llvm.x86.sse.{,u}comi{eq,lt,le,gt,ge,neq}.ss and
llvm.x86.sse2.{,u}comi{...}.sd and return a plain `i32` 0/1 rather than a
vector mask, so a wrong lowering shows up directly as a wrong printed number.

One program per intrinsic is mandatory while the intrinsics are unimplemented:
the `_ =>` arm of llvm_x86.rs emits `codegen_panic_nounwind`, so the first
executed call aborts the whole process and every later case is invisible.

usage: [CLIF_ROOT=<worktree>] python3 comi_ucomi_sweep.py [outdir]
"""

import os
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
DIFFCHECK = pathlib.Path(
    os.environ.get("DIFFCHECK", ROOT / "___BUGHUNT___" / "tools" / "diffcheck.sh")
)

PREDS = ["eq", "lt", "le", "gt", "ge", "neq"]
FAMILY = []
for p in PREDS:
    FAMILY.append((f"_mm_comi{p}_ss", "ss"))
    FAMILY.append((f"_mm_ucomi{p}_ss", "ss"))
    FAMILY.append((f"_mm_comi{p}_sd", "sd"))
    FAMILY.append((f"_mm_ucomi{p}_sd", "sd"))

# Boundary values for lane 0; the upper lanes are deliberately made *different*
# between a and b so that a lowering which accidentally looks at more than lane
# 0 (or at the wrong lane) produces a visibly wrong answer.
LANE0_F32 = [
    ("1.0", "1.0"),
    ("1.0", "2.0"),
    ("2.0", "1.0"),
    ("f32::NAN", "1.0"),
    ("1.0", "f32::NAN"),
    ("f32::NAN", "f32::NAN"),
    ("-0.0", "0.0"),
    ("0.0", "-0.0"),
    ("f32::INFINITY", "f32::INFINITY"),
    ("f32::NEG_INFINITY", "f32::INFINITY"),
    ("f32::INFINITY", "f32::MAX"),
    ("f32::MIN_POSITIVE", "0.0"),
    ("-1.0", "-2.0"),
    ("f32::from_bits(0x0000_0001)", "0.0"),
    ("f32::from_bits(0x7f80_0001)", "1.0"),  # signalling NaN
]
LANE0_F64 = [
    ("1.0", "1.0"),
    ("1.0", "2.0"),
    ("2.0", "1.0"),
    ("f64::NAN", "1.0"),
    ("1.0", "f64::NAN"),
    ("f64::NAN", "f64::NAN"),
    ("-0.0", "0.0"),
    ("0.0", "-0.0"),
    ("f64::INFINITY", "f64::INFINITY"),
    ("f64::NEG_INFINITY", "f64::INFINITY"),
    ("f64::INFINITY", "f64::MAX"),
    ("f64::MIN_POSITIVE", "0.0"),
    ("-1.0", "-2.0"),
    ("f64::from_bits(0x0000_0000_0000_0001)", "0.0"),
    ("f64::from_bits(0x7ff0_0000_0000_0001)", "1.0"),  # signalling NaN
]

TEMPL = {
    # kind: (scalar ty, lane count, set fn, cases, upper lanes of a, upper lanes of b)
    "ss": ("f32", 4, "_mm_set_ps", LANE0_F32, ["7.0", "8.0", "9.0"], ["-7.0", "-8.0", "-9.0"]),
    "sd": ("f64", 2, "_mm_set_pd", LANE0_F64, ["7.0"], ["-7.0"]),
}


def gen(name, kind):
    st, n, setfn, cases, up_a, up_b = TEMPL[kind]
    body = []
    for a0, b0 in cases:
        # _mm_set_p{s,d} takes the highest lane first.
        aa = ", ".join(list(reversed(up_a)) + [f"({a0}) as {st}"])
        bb = ", ".join(list(reversed(up_b)) + [f"({b0}) as {st}"])
        body.append(
            f"""        {{
            let r = {name}({setfn}({aa}), {setfn}({bb}));
            println!("{name}({a0}, {b0}) = {{r}}");
        }}"""
        )
    return (
        "use std::arch::x86_64::*;\nfn main() {\n    unsafe {\n"
        + "\n".join(body)
        + "\n    }\n}\n"
    )


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    out = pathlib.Path(args[0]) if args else ROOT / "tmp" / "comi_ucomi"
    out.mkdir(parents=True, exist_ok=True)
    tally = {"identical": 0, "abort": 0, "wrong": 0, "nobuild": 0}
    for name, kind in FAMILY:
        src = out / f"{name}.rs"
        src.write_text(gen(name, kind))
        p = subprocess.run(
            ["bash", str(DIFFCHECK), str(src)], capture_output=True, text=True, env=os.environ
        )
        txt = p.stdout + p.stderr
        (out / f"{name}.log").write_text(txt)
        if "=== CLIF: COMPILE FAILED ===" in txt:
            print(f"NOBUILD {name}")
            tally["nobuild"] += 1
        elif "RESULT: identical" in txt:
            print(f"ok      {name}")
            tally["identical"] += 1
        elif "is not yet supported" in txt:
            print(f"DIFFER  {name:20s} (abort: unimplemented intrinsic)")
            tally["abort"] += 1
        else:
            print(f"DIFFER  {name:20s} (WRONG VALUE)")
            tally["wrong"] += 1
    print("----", " ".join(f"{k}={v}" for k, v in tally.items()), f"(logs in {out})")
    return 1 if tally["identical"] != len(FAMILY) else 0


sys.exit(main())
