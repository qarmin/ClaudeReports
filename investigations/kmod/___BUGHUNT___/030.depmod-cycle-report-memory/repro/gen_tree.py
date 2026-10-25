#!/usr/bin/env python3
"""gen_tree.py <root> <k>: <root>/lib/modules/1.0/kernel/m<i>.ko, minimal ELF64 relocatable modules.
Module i exports symbol s<i> (__ksymtab_strings) and imports s<j> for every j > i (__versions); the last
module imports s0, closing a cycle through a dense DAG. Dependencies are therefore derived by depmod
from symbols, exactly as for real modules."""
import os
import struct
import sys


def build(sections):
    """sections: list of (name, bytes). Returns an ELF64 ET_REL x86-64 image."""
    names = [n for n, _ in sections] + [".shstrtab"]
    shstr = b"\0"
    name_off = {}
    for n in names:
        name_off[n] = len(shstr)
        shstr += n.encode() + b"\0"
    body = b""
    placed = []
    off = 64
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
        typ = 3 if n == ".shstrtab" else 1
        shdrs += struct.pack("<IIQQQQIIQQ", name_off[n], typ, 0, 0, o, sz, 0, 0, 1, 0)
    return ehdr + body + shdrs


def versions(names):
    return b"".join(struct.pack("<Q", 0) + n.encode().ljust(56, b"\0") for n in names)


root, k = sys.argv[1], int(sys.argv[2])
d = os.path.join(root, "lib/modules/1.0/kernel")
os.makedirs(d)
for i in range(k):
    imports = [f"s{j}" for j in range(i + 1, k)] if i < k - 1 else ["s0"]
    strtab = b"\0"
    symtab = b"\0" * 24
    for n in imports:
        symtab += struct.pack("<IBBHQQ", len(strtab), 0x10, 0, 0, 0, 0)
        strtab += n.encode() + b"\0"
    img = build([("__ksymtab_strings", f"s{i}".encode() + b"\0"), ("__versions", versions(imports)),
                 (".strtab", strtab), (".symtab", symtab)])
    open(f"{d}/m{i}.ko", "wb").write(img)
