#!/usr/bin/env python3
"""Corrupt one DW_RLE_* opcode byte in a real, gcc-compiled DWARF5 object's
.debug_rnglists section, to produce an object that triggers the missing
`default:` case in mold's read_rnglist_range() (src/gdb-index.cc).

Usage: make_evil.py real.o evil.o [opcode_index] [bad_byte]

opcode_index selects which decoded DW_RLE_* entry (0-based, in file order,
as printed by find_rnglists.py) gets its opcode byte overwritten.
bad_byte (default 0x08, the first value above the highest known DW_RLE_*
opcode 0x07) is the corrupted byte written in its place.
"""
import struct, sys
from find_rnglists import parse_sections, decode_rnglists

def main():
    src = sys.argv[1] if len(sys.argv) > 1 else "real.o"
    dst = sys.argv[2] if len(sys.argv) > 2 else "evil.o"
    idx = int(sys.argv[3]) if len(sys.argv) > 3 else 0
    bad = int(sys.argv[4], 0) if len(sys.argv) > 4 else 0x08

    data = bytearray(open(src, "rb").read())
    secs = parse_sections(data)
    rl = [s for s in secs if s["strname"] == ".debug_rnglists"][0]
    section_bytes = data[rl["offset"]:rl["offset"] + rl["size"]]
    entries = decode_rnglists(section_bytes, rl["offset"])

    opcode_pos, opcode, name = entries[idx]
    file_off = rl["offset"] + opcode_pos
    print(f"Corrupting entry #{idx}: file_off=0x{file_off:x} "
          f"opcode 0x{opcode:02x} ({name}) -> 0x{bad:02x}")
    assert data[file_off] == opcode
    data[file_off] = bad

    with open(dst, "wb") as f:
        f.write(data)
    print(f"Wrote {dst}")

if __name__ == "__main__":
    main()
