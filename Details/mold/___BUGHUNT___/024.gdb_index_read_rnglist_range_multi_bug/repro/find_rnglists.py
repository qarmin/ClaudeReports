#!/usr/bin/env python3
"""Minimal ELF64 section-header parser (no pyelftools available on this host)
to locate .debug_rnglists' file offset/size and decode its DW_RLE_* opcode
stream, so we know exactly which file byte to corrupt."""
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
        secs.append(dict(name=name, type=stype, flags=flags, addr=addr,
                          offset=offset, size=size, link=link, info=info))
    shstr_off = secs[e_shstrndx]["offset"]
    for s in secs:
        end = data.index(b"\0", shstr_off + s["name"])
        s["strname"] = data[shstr_off + s["name"]:end].decode()
    return secs

def read_uleb(data, pos):
    result = 0
    shift = 0
    while True:
        b = data[pos]; pos += 1
        result |= (b & 0x7f) << shift
        if not (b & 0x80):
            break
        shift += 7
    return result, pos

RLE_NAMES = {
    0x00: "DW_RLE_end_of_list", 0x01: "DW_RLE_base_addressx",
    0x02: "DW_RLE_startx_endx", 0x03: "DW_RLE_startx_length",
    0x04: "DW_RLE_offset_pair", 0x05: "DW_RLE_base_address",
    0x06: "DW_RLE_start_end", 0x07: "DW_RLE_start_length",
}

def decode_rnglists(section_bytes, file_off_of_section):
    # header: unit_length(4) version(2) addr_size(1) seg_sel(1) offset_entry_count(4)
    pos = 12
    print(f"rnglists header: unit_length=0x{struct.unpack_from('<I', section_bytes, 0)[0]:x} "
          f"version={struct.unpack_from('<H', section_bytes, 4)[0]} "
          f"addr_size={section_bytes[6]}")
    entries = []
    while pos < len(section_bytes):
        opcode_pos = pos
        opcode = section_bytes[pos]; pos += 1
        name = RLE_NAMES.get(opcode, f"UNKNOWN(0x{opcode:x})")
        entries.append((opcode_pos, opcode, name))
        print(f"  section_off=0x{opcode_pos:x} file_off=0x{file_off_of_section+opcode_pos:x} "
              f"opcode=0x{opcode:02x} ({name})")
        if opcode == 0x00:
            break
        elif opcode == 0x01:
            _, pos = read_uleb(section_bytes, pos)
        elif opcode == 0x02:
            _, pos = read_uleb(section_bytes, pos)
            _, pos = read_uleb(section_bytes, pos)
        elif opcode == 0x03:
            _, pos = read_uleb(section_bytes, pos)
            _, pos = read_uleb(section_bytes, pos)
        elif opcode == 0x04:
            _, pos = read_uleb(section_bytes, pos)
            _, pos = read_uleb(section_bytes, pos)
        elif opcode == 0x05:
            pos += 8
        elif opcode == 0x06:
            pos += 16
        elif opcode == 0x07:
            pos += 8
            _, pos = read_uleb(section_bytes, pos)
        else:
            print("  -- hit unknown opcode, stopping decode (this IS the bug) --")
            break
    return entries

if __name__ == "__main__":
    path = sys.argv[1] if len(sys.argv) > 1 else "real.o"
    data = bytearray(open(path, "rb").read())
    secs = parse_sections(data)
    rl = [s for s in secs if s["strname"] == ".debug_rnglists"][0]
    print(f".debug_rnglists: file_offset=0x{rl['offset']:x} size=0x{rl['size']:x}")
    section_bytes = data[rl["offset"]:rl["offset"] + rl["size"]]
    decode_rnglists(section_bytes, rl["offset"])
