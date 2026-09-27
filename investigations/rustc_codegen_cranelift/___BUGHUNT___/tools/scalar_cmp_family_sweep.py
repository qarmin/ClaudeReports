#!/usr/bin/env python3
"""Sweep the whole *public, stable* SSE/SSE2 scalar float-compare family
(`_mm_cmpXX_ss` / `_mm_cmpXX_sd`, plus the `_mm_comiXX_*`/`_mm_ucomiXX_*`
relatives as controls) one intrinsic per program, diffing cg_clif against LLVM.

One program per intrinsic is mandatory: an unimplemented llvm.x86.* intrinsic
lowers to `codegen_panic_nounwind`, so the first one executed aborts the whole
process and everything after it would be invisible.

usage: [CLIF_ROOT=<worktree>] python3 scalar_cmp_family_sweep.py [outdir]
"""
import os, subprocess, sys, pathlib

ROOT = pathlib.Path(__file__).resolve().parents[2]
DIFFCHECK = pathlib.Path(os.environ.get("DIFFCHECK", ROOT / "___BUGHUNT___" / "tools" / "diffcheck.sh"))

PREDS = ["eq", "lt", "le", "gt", "ge", "neq", "nlt", "nle", "ngt", "nge", "ord", "unord"]
# (name, kind) - kind selects the body template
FAMILY = []
for p in PREDS:
    FAMILY.append((f"_mm_cmp{p}_ss", "ss"))
    FAMILY.append((f"_mm_cmp{p}_sd", "sd"))
for p in ["eq", "lt", "le", "gt", "ge", "neq"]:
    FAMILY.append((f"_mm_comi{p}_ss", "comi_ss"))
    FAMILY.append((f"_mm_comi{p}_sd", "comi_sd"))
    FAMILY.append((f"_mm_ucomi{p}_ss", "comi_ss"))
    FAMILY.append((f"_mm_ucomi{p}_sd", "comi_sd"))

CASES_F32 = [
    ("[1.0, 2.0, 3.0, 4.0]", "[1.0, 5.0, 0.0, 4.0]"),
    ("[2.0, 2.0, 3.0, 4.0]", "[1.0, 5.0, 0.0, 4.0]"),
    ("[f32::NAN, 1.0, f32::NAN, 0.0]", "[1.0, f32::NAN, f32::NAN, 0.0]"),
    ("[1.0, 1.0, 1.0, 1.0]", "[f32::NAN, 2.0, 3.0, 4.0]"),
    ("[-0.0, 0.0, f32::INFINITY, f32::NEG_INFINITY]", "[0.0, -0.0, f32::INFINITY, f32::INFINITY]"),
    ("[f32::NEG_INFINITY, 0.0, 0.0, 0.0]", "[f32::INFINITY, 0.0, 0.0, 0.0]"),
]
CASES_F64 = [
    ("[1.0, 2.0]", "[1.0, 5.0]"),
    ("[2.0, 2.0]", "[1.0, 5.0]"),
    ("[f64::NAN, 1.0]", "[1.0, f64::NAN]"),
    ("[1.0, 1.0]", "[f64::NAN, 2.0]"),
    ("[-0.0, f64::INFINITY]", "[0.0, f64::INFINITY]"),
    ("[f64::NEG_INFINITY, 0.0]", "[f64::INFINITY, 0.0]"),
]

TEMPL = {
    "ss": ("f32", 4, "_mm_set_ps", CASES_F32, "_mm_storeu_ps"),
    "sd": ("f64", 2, "_mm_set_pd", CASES_F64, "_mm_storeu_pd"),
    "comi_ss": ("f32", 4, "_mm_set_ps", CASES_F32, None),
    "comi_sd": ("f64", 2, "_mm_set_pd", CASES_F64, None),
}


def gen(name, kind):
    st, n, setfn, cases, storefn = TEMPL[kind]
    body = []
    for a, b in cases:
        aa = ", ".join(f"a[{i}]" for i in reversed(range(n)))
        bb = ", ".join(f"b[{i}]" for i in reversed(range(n)))
        if storefn is None:
            body.append(f"""        {{
            let a: [{st}; {n}] = {a};
            let b: [{st}; {n}] = {b};
            let r = {name}({setfn}({aa}), {setfn}({bb}));
            println!("{{:?}} {{:?}} -> {{:?}}", a, b, r);
        }}""")
        else:
            body.append(f"""        {{
            let a: [{st}; {n}] = {a};
            let b: [{st}; {n}] = {b};
            let r = {name}({setfn}({aa}), {setfn}({bb}));
            let mut o = [0 as {st}; {n}];
            {storefn}(o.as_mut_ptr(), r);
            println!("{{:?}} {{:?}} -> {{:?}}", a, b, o.map(|v| v.to_bits()));
        }}""")
    return "use std::arch::x86_64::*;\nfn main() {\n    unsafe {\n" + "\n".join(body) + "\n    }\n}\n"


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    out = pathlib.Path(args[0]) if args else ROOT / "tmp" / "scalar_cmp_family"
    out.mkdir(parents=True, exist_ok=True)
    tally = {"identical": 0, "abort_or_wrong": 0, "nobuild": 0}
    for name, kind in FAMILY:
        src = out / f"{name}.rs"
        src.write_text(gen(name, kind))
        p = subprocess.run(["bash", str(DIFFCHECK), str(src)], capture_output=True, text=True, env=os.environ)
        txt = p.stdout + p.stderr
        (out / f"{name}.log").write_text(txt)
        if "=== CLIF: COMPILE FAILED ===" in txt:
            print(f"NOBUILD {name}")
            tally["nobuild"] += 1
        elif "RESULT: identical" in txt:
            print(f"ok      {name}")
            tally["identical"] += 1
        else:
            reason = "abort" if "is not yet supported" in txt else "WRONG VALUE"
            print(f"DIFFER  {name:20s} ({reason})")
            tally["abort_or_wrong"] += 1
    print("----", " ".join(f"{k}={v}" for k, v in tally.items()), f"(logs in {out})")


main()
