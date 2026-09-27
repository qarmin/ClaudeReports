import struct, sys

def u16(v): return struct.pack(">H", v)
def u32(v): return struct.pack(">I", v)
def u8(v): return struct.pack("B", v)

def build_j2c(xsiz, ysiz, num_decomp_levels, cb_exp, num_layers, num_components=1):
    out = b"\xff\x4f"  # SOC

    siz_content = b""
    siz_content += u16(0)
    siz_content += u32(xsiz)
    siz_content += u32(ysiz)
    siz_content += u32(0)
    siz_content += u32(0)
    siz_content += u32(xsiz)
    siz_content += u32(ysiz)
    siz_content += u32(0)
    siz_content += u32(0)
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
    qcd_content += u16(0x0000)
    qcd = b"\xff\x5c" + u16(len(qcd_content) + 2) + qcd_content

    header = siz + cod + qcd

    # Real packet-header body: LRCP order, layer outer, resolution inner
    # (0..num_resolution_levels), 1 component, 1 precinct/resolution.
    # Resolution 0: zero-length packet (byte 0x00).
    # Resolution >=1: non-zero-length packet whose header bits keep every
    # sub-band's inclusion tag-tree root climbing by exactly one ("0" bit)
    # per layer, so every code-block in every sub-band is cheaply resolved
    # as "not included" without ever needing to encode per-code-block data.
    num_resolution_levels = num_decomp_levels + 1
    body = bytearray()
    for _layer in range(num_layers):
        body.append(0x00)  # resolution 0 packet (LL only): zero-length
        for _res in range(1, num_resolution_levels):
            body.append(0x80)  # non-zero-length, 3 sub-band roots each get one "0" bit
    body = bytes(body)

    sot_content = b""
    sot_content += u16(0)
    psot = 12 + 2 + len(body)
    sot_content += u32(psot)
    sot_content += u8(0)
    sot_content += u8(1)
    sot = b"\xff\x90" + u16(len(sot_content) + 2) + sot_content

    sod = b"\xff\x93"

    out += header + sot + sod + body
    return out

if __name__ == "__main__":
    xsiz = int(sys.argv[1])
    ysiz = int(sys.argv[2])
    num_decomp = int(sys.argv[3])
    cb_exp = int(sys.argv[4])
    num_layers = int(sys.argv[5])
    outpath = sys.argv[6]
    data = build_j2c(xsiz, ysiz, num_decomp, cb_exp, num_layers)
    with open(outpath, "wb") as f:
        f.write(data)
    print(f"wrote {outpath}: {len(data)} bytes")
