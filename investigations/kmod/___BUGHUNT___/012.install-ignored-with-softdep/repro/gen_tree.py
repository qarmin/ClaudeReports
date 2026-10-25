#!/usr/bin/env python3
"""gen_tree.py <root>: <root>/lib/modules/1.0/kernel/{foo,bar}.ko (minimal ELF64 modules, no dependencies between them)
and <root>/etc/m.conf with `softdep foo pre: bar` plus `install foo /bin/echo INSTALL-CMD-RAN`."""
import os
import struct
import sys


def build(sections):
    names = [n for n, _ in sections] + [".shstrtab"]
    shstr = b"\0"
    name_off = {}
    for n in names:
        name_off[n] = len(shstr)
        shstr += n.encode() + b"\0"
    body, placed, off = b"", [], 64
    for n, data in sections + [(".shstrtab", shstr)]:
        pad = (-off) % 8
        body += b"\0" * pad
        off += pad
        placed.append((n, off, len(data)))
        body += data
        off += len(data)
    pad = (-off) % 8
    body += b"\0" * pad
    off_sh = off + pad
    nsec = len(placed) + 1
    ehdr = b"\x7fELF" + bytes([2, 1, 1, 0]) + b"\0" * 8
    ehdr += struct.pack("<HHIQQQIHHHHHH", 1, 62, 1, 0, 0, off_sh, 0, 64, 0, 0, 64, nsec, nsec - 1)
    shdrs = b"\0" * 64
    for n, o, sz in placed:
        shdrs += struct.pack("<IIQQQQIIQQ", name_off[n], 3 if n == ".shstrtab" else 1, 0, 0, o, sz, 0, 0, 1, 0)
    return ehdr + body + shdrs


root = sys.argv[1]
d = os.path.join(root, "lib/modules/1.0/kernel")
os.makedirs(d)
os.makedirs(os.path.join(root, "etc"))
for n in ("foo", "bar"):
    open(f"{d}/{n}.ko", "wb").write(build([("__ksymtab_strings", f"sym_{n}".encode() + b"\0")]))
with open(os.path.join(root, "etc/m.conf"), "w") as f:
    f.write("softdep foo pre: bar\ninstall foo /bin/echo INSTALL-CMD-RAN\n")
