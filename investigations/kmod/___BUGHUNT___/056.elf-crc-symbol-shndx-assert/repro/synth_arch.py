"""Synthetic kernel modules for any ELF class/endianness (x86, x86_64, ppc, ppc64, s390, ...), with real CRCs.

Exports are described by `__crc_<name>` symbols (SHN_ABS, st_value = crc) plus `__ksymtab_strings`; imports by `__versions`
(unsigned long crc + name padded to 64 bytes) and undefined `.symtab` entries - the layout kbuild produces.
"""
import struct


def build(sections, bits=64, be=False, machine=62):
    e = ">" if be else "<"
    names = [n for n, *_ in sections] + [".shstrtab"]
    shstr, name_off = b"\0", {}
    for n in names:
        name_off[n] = len(shstr)
        shstr += n.encode() + b"\0"
    ehsz, shsz = (64, 64) if bits == 64 else (52, 40)
    al = 8 if bits == 64 else 4
    body, placed, off = b"", [], ehsz
    allsecs = [(n, d, t) for n, d, t in sections] + [(".shstrtab", shstr, 3)]
    for n, data, typ in allsecs:
        pad = (-off) % al
        body += b"\0" * pad
        off += pad
        placed.append((n, off, len(data), typ))
        body += data
        off += len(data)
    pad = (-off) % al
    body += b"\0" * pad
    off_sh = off + pad
    nsec = len(placed) + 1
    ident = b"\x7fELF" + bytes([2 if bits == 64 else 1, 2 if be else 1, 1, 0]) + b"\0" * 8
    if bits == 64:
        eh = ident + struct.pack(e + "HHIQQQIHHHHHH", 1, machine, 1, 0, 0, off_sh, 0, 64, 0, 0, 64, nsec, nsec - 1)
        sh = b"\0" * 64
        for n, o, sz, typ in placed:
            sh += struct.pack(e + "IIQQQQIIQQ", name_off[n], typ, 0, 0, o, sz, 0, 0, 1, 0)
    else:
        eh = ident + struct.pack(e + "HHIIIIIHHHHHH", 1, machine, 1, 0, 0, off_sh, 0, 52, 0, 0, 40, nsec, nsec - 1)
        sh = b"\0" * 40
        for n, o, sz, typ in placed:
            sh += struct.pack(e + "IIIIIIIIII", name_off[n], typ, 0, 0, o, sz, 0, 0, 1, 0)
    return eh + body + sh


def module(exports=(), imports=(), modinfo=(), bits=64, be=False, machine=62):
    """exports: [(name, crc)], imports: [(name, crc)]"""
    e = ">" if be else "<"
    crcfmt = "Q" if bits == 64 else "I"
    namelen = 64 - (8 if bits == 64 else 4)
    versions = b"".join(struct.pack(e + crcfmt, crc) + n.encode().ljust(namelen, b"\0") for n, crc in imports)
    strtab, syms = b"\0", []
    for n, crc in exports:
        syms.append((len(strtab), crc, 0x10, 0xFFF1))  # global, SHN_ABS
        strtab += b"__crc_" + n.encode() + b"\0"
    for n, crc in imports:
        syms.append((len(strtab), 0, 0x10, 0))  # global, undefined
        strtab += n.encode() + b"\0"
    if bits == 64:
        symtab = b"\0" * 24 + b"".join(struct.pack(e + "IBBHQQ", no, info, 0, sh, v, 0) for no, v, info, sh in syms)
    else:
        symtab = b"\0" * 16 + b"".join(struct.pack(e + "IIIBBH", no, v, 0, info, 0, sh) for no, v, info, sh in syms)
    secs = [("__ksymtab_strings", b"".join(n.encode() + b"\0" for n, _ in exports), 1)]
    if modinfo:
        secs.append((".modinfo", b"".join(m.encode() + b"\0" for m in modinfo), 1))
    secs += [("__versions", versions, 1), (".strtab", strtab, 3), (".symtab", symtab, 2)]
    return build(secs, bits, be, machine)
