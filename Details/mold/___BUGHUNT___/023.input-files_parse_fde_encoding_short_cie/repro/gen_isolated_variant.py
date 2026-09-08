#!/usr/bin/env python3
"""Variant that isolates candidate 023's bug from candidate 008's separate, already-known
unbounded-substr(pos) bug (both live in this pristine checkout). Uses size=3 (not size=1) so the
version byte at data[8] lands on a middle byte (not the MSB) of the "next record"'s reinterpreted
size field, keeping that fake size small (256) instead of accidentally huge (16777216), so the
outer parse_ehframe() loop's own substr() doesn't throw. Lets us observe candidate 023's own
remove_prefix()/operator[] issue in total isolation, in both a Debug (_GLIBCXX_ASSERTIONS) and a
Release build.
"""
import struct
import sys


def build() -> bytes:
    n = 300
    b = bytearray(n)
    b[0] = 3     # size = 3 (declares a 7-byte record: 4-byte size + 3-byte payload)
    b[8] = 1     # version byte (read by parse_fde_encoding's data[8]) = 1
    b[15] = 1    # version byte of the "next" record (offset 7), also = 1, so it too passes cleanly
    eh_frame_data = bytes(b)

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
        1, 62, 1, 0, 0, off_shdrs, 0, 64, 0, 0, 64, 3, 2,
    )
    assert len(ehdr) == 64

    def shdr(name, type_, flags, offset, size):
        return struct.pack("<IIQQQQIIQQ", name, type_, flags, 0, offset, size, 0, 0, 1, 0)

    sh_null = shdr(0, 0, 0, 0, 0)
    sh_ehframe = shdr(name_eh_frame, 1, 2, off_ehframe, len(eh_frame_data))
    sh_shstrtab = shdr(name_shstrtab, 3, 0, off_shstrtab, len(shstrtab))

    return ehdr + eh_frame_data + shstrtab + b"\x00" * pad + sh_null + sh_ehframe + sh_shstrtab


with open(sys.argv[1], "wb") as f:
    f.write(build())
