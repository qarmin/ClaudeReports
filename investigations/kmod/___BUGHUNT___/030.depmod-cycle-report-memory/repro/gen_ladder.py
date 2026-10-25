#!/usr/bin/env python3
"""gen_ladder.py <root> <levels>: a 2-module cycle (c0 <-> c1) with a diamond ladder hanging off c1 (levels x two modules, each level depends on both modules of the next), so the number of simple paths from c0 is 2^levels while only two modules are cyclic."""
import os
import runpy
import struct
import sys

src = open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "gen_tree.py")).read().split("root, k = sys.argv")[0]
ns = {}
exec(compile(src, "gen_tree_lib", "exec"), ns)
build, versions = ns["build"], ns["versions"]

root, levels = sys.argv[1], int(sys.argv[2])
d = os.path.join(root, "lib/modules/1.0/kernel")
os.makedirs(d)
mods = {"c0": ["c1"], "c1": ["c0", "x0a", "x0b"]}
for lv in range(levels):
    nxt = [f"x{lv + 1}a", f"x{lv + 1}b"] if lv + 1 < levels else []
    mods[f"x{lv}a"] = nxt
    mods[f"x{lv}b"] = nxt
for name, deps in mods.items():
    imports = [f"s_{n}" for n in deps]
    strtab, symtab = b"\0", b"\0" * 24
    for n in imports:
        symtab += struct.pack("<IBBHQQ", len(strtab), 0x10, 0, 0, 0, 0)
        strtab += n.encode() + b"\0"
    img = build([("__ksymtab_strings", f"s_{name}".encode() + b"\0"), ("__versions", versions(imports)),
                 (".strtab", strtab), (".symtab", symtab)])
    open(f"{d}/{name}.ko", "wb").write(img)
