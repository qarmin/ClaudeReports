#!/usr/bin/env python3
"""Give a (statically linked, non-PIE, x86_64) ELF executable a
non-8-byte-aligned `e_phoff`, while keeping it fully loadable by the real
Linux kernel.

How: append one extra PT_LOAD program-header TABLE entry describing a small
new read-only segment whose own file content *is* the (now 10-entry) program
header table itself, placed at a file offset that is not a multiple of 8.
Since the kernel's `load_addr` bias (p_vaddr - p_offset of the first PT_LOAD
segment) is always page-aligned (a multiple of 4096, hence of 8), this makes
the kernel-reported `AT_PHDR = load_addr + e_phoff` land exactly on this new
segment's own (real, mapped, valid) address - misaligned, but not pointing at
unmapped memory, so nothing in the C runtime's own early (pre-`main`) use of
AT_PHDR (e.g. musl's PT_TLS scan) crashes before the target binary's own
`main` gets to run and call rustix's real public API.

Usage: corrupt_phdr.py <src-elf> <dst-elf> [misalign-residue (1-7, default 1)]
"""
import struct
import sys
import os


def corrupt(src: str, dst: str, residue: int = 1) -> int:
    assert 1 <= residue <= 7
    data = bytearray(open(src, "rb").read())

    e_phoff = struct.unpack_from("<Q", data, 0x20)[0]
    e_phentsize, e_phnum = struct.unpack_from("<HH", data, 0x36)

    entries = [
        list(struct.unpack_from("<IIQQQQQQ", data, e_phoff + i * e_phentsize))
        for i in range(e_phnum)
    ]

    load_addr = entries[0][3] - entries[0][2]  # first segment's p_vaddr - p_offset

    t = len(data)
    while t % 8 != residue:
        t += 1
    pad = t - len(data)

    new_vaddr = load_addr + t
    assert new_vaddr % 8 == residue

    n = len(entries) + 1
    table_size = e_phentsize * n
    # PT_LOAD, PF_R, describing exactly the table bytes we're about to append.
    entries.append([1, 4, t, new_vaddr, new_vaddr, table_size, table_size, 0x1000])

    table_bytes = b"".join(struct.pack("<IIQQQQQQ", *e) for e in entries)
    assert len(table_bytes) == table_size

    new_data = bytes(data) + (b"\x00" * pad) + table_bytes
    new_data = bytearray(new_data)
    struct.pack_into("<Q", new_data, 0x20, t)  # e_phoff
    struct.pack_into("<H", new_data, 0x38, n)  # e_phnum

    open(dst, "wb").write(new_data)
    os.chmod(dst, 0o755)
    return new_vaddr


if __name__ == "__main__":
    src, dst = sys.argv[1], sys.argv[2]
    residue = int(sys.argv[3]) if len(sys.argv) > 3 else 1
    addr = corrupt(src, dst, residue)
    print(f"corrupted {dst}: AT_PHDR will report {hex(addr)} (mod 8 = {addr % 8})")
