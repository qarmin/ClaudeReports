#!/usr/bin/env python3
"""Generate a big-endian SPARC64 ET_REL object with N undefined function
symbols, each referenced by an R_SPARC_WPLT30 relocation, so that linking it
with `mold -shared` forces mold to emit N PLT entries.

sparc_num_small_plt == (0x100000 - 128) / 32 == 32764, so N > 32764 exercises
mold's "large PLT" layout (src/arch-sparc64.cc, src/mold.h to_plt_offset).

Usage: gen_sparc64_plt.py <nsyms> <out.o>
"""

import struct
import sys

EM_SPARC64 = 43
ET_REL = 1
SHT_PROGBITS, SHT_SYMTAB, SHT_STRTAB, SHT_RELA = 1, 2, 3, 4
SHF_ALLOC, SHF_EXECINSTR = 0x2, 0x4
STB_GLOBAL, STT_FUNC = 1, 2
R_SPARC_WPLT30 = 18

BE = ">"


def align(n, a):
    return (n + a - 1) & ~(a - 1)


def build(nsyms):
    # .text: one `call` instruction per symbol (disp30 filled in by the reloc).
    text = b"".join(struct.pack(BE + "I", 0x40000000) for _ in range(nsyms))

    # .rela.text: R_SPARC_WPLT30 against symbol i+1 at offset i*4.
    # ElfRel<SPARC64> == Elf64_Rela: r_offset, r_info(sym<<32|type), r_addend.
    rela = b"".join(
        struct.pack(BE + "QIIq", i * 4, i + 1, R_SPARC_WPLT30, 0)
        for i in range(nsyms)
    )

    # .strtab / .symtab: null symbol + nsyms SHN_UNDEF STT_FUNC globals.
    strtab = bytearray(b"\0")
    syms = [struct.pack(BE + "IBBHQQ", 0, 0, 0, 0, 0, 0)]
    st_info = (STB_GLOBAL << 4) | STT_FUNC
    for i in range(nsyms):
        name_off = len(strtab)
        strtab += b"plt_target_%06d\0" % i
        syms.append(struct.pack(BE + "IBBHQQ", name_off, st_info, 0, 0, 0, 0))
    strtab = bytes(strtab)
    symtab = b"".join(syms)

    shstr = bytearray(b"\0")
    names = {}
    for s in (".text", ".rela.text", ".symtab", ".strtab", ".shstrtab"):
        names[s] = len(shstr)
        shstr += s.encode() + b"\0"
    shstr = bytes(shstr)

    off = 64
    off = align(off, 8)
    text_off = off
    off = align(off + len(text), 8)
    rela_off = off
    off = align(off + len(rela), 8)
    symtab_off = off
    off = align(off + len(symtab), 8)
    strtab_off = off
    off = align(off + len(strtab), 8)
    shstr_off = off
    off = align(off + len(shstr), 8)
    shoff = off

    def shdr(name, typ, flags, offset, size, link, info, addralign, entsize):
        return struct.pack(BE + "IIQQQQIIQQ", name, typ, flags, 0, offset,
                           size, link, info, addralign, entsize)

    shdrs = [
        shdr(0, 0, 0, 0, 0, 0, 0, 0, 0),
        shdr(names[".text"], SHT_PROGBITS, SHF_ALLOC | SHF_EXECINSTR,
             text_off, len(text), 0, 0, 4, 0),
        shdr(names[".rela.text"], SHT_RELA, 0, rela_off, len(rela),
             3, 1, 8, 24),
        shdr(names[".symtab"], SHT_SYMTAB, 0, symtab_off, len(symtab),
             4, 1, 8, 24),
        shdr(names[".strtab"], SHT_STRTAB, 0, strtab_off, len(strtab),
             0, 0, 1, 0),
        shdr(names[".shstrtab"], SHT_STRTAB, 0, shstr_off, len(shstr),
             0, 0, 1, 0),
    ]

    ehdr = (b"\x7fELF" + bytes([2, 2, 1, 0]) + bytes(8) +
            struct.pack(BE + "HHIQQQIHHHHHH", ET_REL, EM_SPARC64, 1, 0, 0,
                        shoff, 0, 64, 0, 0, 64, len(shdrs), 5))

    buf = bytearray(ehdr)

    def put(offset, data):
        if len(buf) < offset:
            buf.extend(bytes(offset - len(buf)))
        buf[offset:offset + len(data)] = data

    put(text_off, text)
    put(rela_off, rela)
    put(symtab_off, symtab)
    put(strtab_off, strtab)
    put(shstr_off, shstr)
    put(shoff, b"".join(shdrs))
    return bytes(buf)


def main():
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    nsyms = int(sys.argv[1])
    with open(sys.argv[2], "wb") as f:
        f.write(build(nsyms))
    print(f"wrote {sys.argv[2]}: {nsyms} undefined syms "
          f"({max(0, nsyms - 32764)} large PLT entries)")


if __name__ == "__main__":
    main()
