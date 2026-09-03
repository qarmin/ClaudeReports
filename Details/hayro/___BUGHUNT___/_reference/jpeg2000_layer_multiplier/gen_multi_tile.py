import struct, sys

def u16(v): return struct.pack(">H", v)
def u32(v): return struct.pack(">I", v)
def u8(v): return struct.pack("B", v)

def build_j2c_multi_tile(xsiz, ysiz, xtsiz, ytsiz, num_decomp_levels, cb_exp, num_layers, num_components=1):
    out = b"\xff\x4f"  # SOC

    siz_content = b""
    siz_content += u16(0)          # Rsiz
    siz_content += u32(xsiz)       # Xsiz
    siz_content += u32(ysiz)       # Ysiz
    siz_content += u32(0)          # XOsiz
    siz_content += u32(0)          # YOsiz
    siz_content += u32(xtsiz)      # XTsiz
    siz_content += u32(ytsiz)      # YTsiz
    siz_content += u32(0)          # XTOsiz
    siz_content += u32(0)          # YTOsiz
    siz_content += u16(num_components)
    for _ in range(num_components):
        siz_content += u8(7)
        siz_content += u8(1)
        siz_content += u8(1)
    siz = b"\xff\x51" + u16(len(siz_content) + 2) + siz_content

    cod_content = b""
    cod_content += u8(0x00)  # Scod: no precincts/SOP/EPH
    cod_content += u8(0x00)  # LRCP
    cod_content += u16(num_layers)
    cod_content += u8(0x00)  # MCT off
    cod_content += u8(num_decomp_levels)
    cod_content += u8(cb_exp)
    cod_content += u8(cb_exp)
    cod_content += u8(0x00)
    cod_content += u8(0x01)  # reversible 5/3
    cod = b"\xff\x52" + u16(len(cod_content) + 2) + cod_content

    qcd_content = b""
    qcd_content += u8(0x41)
    # exponent=8 in the top 5 bits so ScalarDerived's `e_0 - n_ll` never underflows
    # (InvalidExponents) regardless of num_decomp_levels, letting decode succeed fully.
    qcd_content += u16(8 << 11)
    qcd = b"\xff\x5c" + u16(len(qcd_content) + 2) + qcd_content

    header = siz + cod + qcd

    num_resolution_levels = num_decomp_levels + 1
    body = bytearray()
    for _layer in range(num_layers):
        body.append(0x00)  # resolution 0 packet (LL only): zero-length
        for _res in range(1, num_resolution_levels):
            body.append(0x80)  # non-zero-length, cheap "not included" packet
    body = bytes(body)

    num_x_tiles = -(-xsiz // xtsiz)
    num_y_tiles = -(-ysiz // ytsiz)
    num_tiles = num_x_tiles * num_y_tiles

    out += header

    for tile_idx in range(num_tiles):
        sot_content = b""
        sot_content += u16(tile_idx)
        psot = 12 + 2 + len(body)
        sot_content += u32(psot)
        sot_content += u8(0)
        sot_content += u8(1)
        sot = b"\xff\x90" + u16(len(sot_content) + 2) + sot_content
        sod = b"\xff\x93"
        out += sot + sod + body

    return out, num_tiles

if __name__ == "__main__":
    xsiz = int(sys.argv[1])
    ysiz = int(sys.argv[2])
    xtsiz = int(sys.argv[3])
    ytsiz = int(sys.argv[4])
    num_decomp = int(sys.argv[5])
    cb_exp = int(sys.argv[6])
    num_layers = int(sys.argv[7])
    outpath = sys.argv[8]
    data, num_tiles = build_j2c_multi_tile(xsiz, ysiz, xtsiz, ytsiz, num_decomp, cb_exp, num_layers)
    with open(outpath, "wb") as f:
        f.write(data)
    print(f"wrote {outpath}: {len(data)} bytes, {num_tiles} tiles")
