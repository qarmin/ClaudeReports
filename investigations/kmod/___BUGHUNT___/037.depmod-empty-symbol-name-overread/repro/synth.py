"""Minimal synthetic ELF64 x86-64 kernel modules (exports via __ksymtab_strings, imports via __versions + .symtab)."""
import struct


def elf(sections):
    names = [n for n, _ in sections] + [".shstrtab"]
    shstr, name_off = b"\0", {}
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


def module(exports, imports, modinfo=()):
    versions = b"".join(struct.pack("<Q", 0) + n.encode().ljust(56, b"\0") for n in imports)
    strtab, symtab = b"\0", b"\0" * 24
    for n in imports:
        symtab += struct.pack("<IBBHQQ", len(strtab), 0x10, 0, 0, 0, 0)
        strtab += n.encode() + b"\0"
    secs = [("__ksymtab_strings", b"".join(e.encode() + b"\0" for e in exports))]
    if modinfo:
        secs.append((".modinfo", b"".join(m.encode() + b"\0" for m in modinfo)))
    secs += [("__versions", versions), (".strtab", strtab), (".symtab", symtab)]
    return elf(secs)
