#!/usr/bin/env python3
"""Sweep all 32 x86 float-compare predicate immediates (0x00..0x1f) through
_mm_cmp_ps / _mm_cmp_pd (and the _mm_cmp_ss / _mm_cmp_sd scalar forms) and diff
cg_clif against LLVM one immediate at a time.

One program per immediate is mandatory: an ICE in the backend aborts the whole
crate, so a single program containing all 32 would only ever reveal the first
failing immediate.

usage: [CLIF_ROOT=<worktree>] python3 cmp_imm_sweep.py [outdir] [--only ps,pd]
"""
import os, re, subprocess, sys, pathlib

ROOT = pathlib.Path(__file__).resolve().parents[2]
DIFFCHECK = pathlib.Path(os.environ.get("DIFFCHECK", ROOT / "___BUGHUNT___" / "tools" / "diffcheck.sh"))

# (intrinsic suffix, scalar type, lane count, set fn, store fn)
VARIANTS = {
    "ps": ("f32", 4, "_mm_set_ps", "_mm_storeu_ps"),
    "pd": ("f64", 2, "_mm_set_pd", "_mm_storeu_pd"),
    "ss": ("f32", 4, "_mm_set_ps", "_mm_storeu_ps"),
    "sd": ("f64", 2, "_mm_set_pd", "_mm_storeu_pd"),
}

CASES = {
    "f32": [
        ("[1.0, 2.0, 3.0, 4.0]", "[1.0, 5.0, 0.0, 4.0]"),
        ("[f32::NAN, 1.0, f32::NAN, 0.0]", "[1.0, f32::NAN, f32::NAN, 0.0]"),
        ("[-0.0, 0.0, f32::INFINITY, f32::NEG_INFINITY]", "[0.0, -0.0, f32::INFINITY, f32::INFINITY]"),
    ],
    "f64": [
        ("[1.0, 2.0]", "[1.0, 5.0]"),
        ("[f64::NAN, 1.0]", "[1.0, f64::NAN]"),
        ("[-0.0, f64::INFINITY]", "[0.0, f64::INFINITY]"),
    ],
}


def gen(suffix, imm):
    st, n, setfn, storefn = VARIANTS[suffix]
    body = []
    for a, b in CASES[st]:
        args_a = ", ".join(f"a[{i}]" for i in reversed(range(n)))
        args_b = ", ".join(f"b[{i}]" for i in reversed(range(n)))
        body.append(f"""        {{
            let a: [{st}; {n}] = {a};
            let b: [{st}; {n}] = {b};
            let r = _mm_cmp_{suffix}::<{imm:#04x}>({setfn}({args_a}), {setfn}({args_b}));
            let mut o = [0 as {st}; {n}];
            {storefn}(o.as_mut_ptr(), r);
            println!("{{:?}} {{:?}} -> {{:?}}", a, b, o.map(|v| v.to_bits()));
        }}""")
    return "use std::arch::x86_64::*;\nfn main() {\n    unsafe {\n" + "\n".join(body) + "\n    }\n}\n"


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    only = None
    for a in sys.argv[1:]:
        if a.startswith("--only"):
            only = a.split("=", 1)[1].split(",")
    out = pathlib.Path(args[0]) if args else ROOT / "tmp" / "cmp_sweep"
    out.mkdir(parents=True, exist_ok=True)
    tally = {"identical": 0, "ice": 0, "nobuild": 0, "wrong": 0}
    for suffix in (only or ["ps", "pd", "ss", "sd"]):
        for imm in range(32):
            src = out / f"cmp_{suffix}_{imm:#04x}.rs"
            src.write_text(gen(suffix, imm))
            log = out / f"cmp_{suffix}_{imm:#04x}.log"
            p = subprocess.run(["bash", str(DIFFCHECK), str(src)],
                               capture_output=True, text=True, env=os.environ)
            txt = p.stdout + p.stderr
            log.write_text(txt)
            name = f"_mm_cmp_{suffix}::<{imm:#04x}>"
            if "=== CLIF: COMPILE FAILED ===" in txt:
                m = re.search(r"panicked at (\S+):\s*\n(.*)", txt)
                if m:
                    print(f"ICE     {name:26s} {m.group(1)} {m.group(2).strip()}")
                    tally["ice"] += 1
                else:
                    print(f"NOBUILD {name:26s}")
                    tally["nobuild"] += 1
            elif "RESULT: identical" in txt:
                tally["identical"] += 1
            else:
                d = "\n".join(l for l in txt.splitlines() if l.startswith(("<", ">")))
                print(f"WRONG   {name:26s}\n{d}")
                tally["wrong"] += 1
    print("----", " ".join(f"{k}={v}" for k, v in tally.items()), f"(logs in {out})")


main()
