import struct, sys

def u16(v): return struct.pack(">H", v)
def u32(v): return struct.pack(">I", v)
def u8(v): return struct.pack("B", v)

def build_j2c(xsiz, ysiz, num_decomp_levels, cb_w_exp, cb_h_exp, num_layers, num_components=1):
    # SOC
    out = b"\xff\x4f"

    # SIZ
    siz_content = b""
    siz_content += u16(0)             # Rsiz
    siz_content += u32(xsiz)          # Xsiz
    siz_content += u32(ysiz)          # Ysiz
    siz_content += u32(0)             # XOsiz
    siz_content += u32(0)             # YOsiz
    siz_content += u32(xsiz)          # XTsiz (single tile)
    siz_content += u32(ysiz)          # YTsiz
    siz_content += u32(0)             # XTOsiz
    siz_content += u32(0)             # YTOsiz
    siz_content += u16(num_components)  # Csiz
    for _ in range(num_components):
        siz_content += u8(7)  # Ssiz: 8-bit unsigned (precision-1=7)
        siz_content += u8(1)  # XRsiz
        siz_content += u8(1)  # YRsiz
    siz = b"\xff\x51" + u16(len(siz_content) + 2) + siz_content

    # COD
    cod_content = b""
    cod_content += u8(0x00)          # Scod: no precincts, no SOP, no EPH
    cod_content += u8(0x00)          # progression order: LRCP
    cod_content += u16(num_layers)   # num layers
    cod_content += u8(0x00)          # MCT off
    cod_content += u8(num_decomp_levels)  # num decomposition levels
    cod_content += u8(cb_w_exp)      # code-block width exponent (actual = +2)
    cod_content += u8(cb_h_exp)      # code-block height exponent (actual = +2)
    cod_content += u8(0x00)          # code-block style
    cod_content += u8(0x01)          # transformation: reversible 5/3
    cod = b"\xff\x52" + u16(len(cod_content) + 2) + cod_content

    # QCD (ScalarDerived)
    qcd_content = b""
    qcd_content += u8(0x41)  # style=ScalarDerived(1), guard_bits=2
    qcd_content += u16(0x0000)  # exponent=0, mantissa=0
    qcd = b"\xff\x5c" + u16(len(qcd_content) + 2) + qcd_content

    header = siz + cod + qcd

    body = b""  # empty tile-part body

    sot_content = b""
    sot_content += u16(0)  # Isot
    psot = 12 + 2 + len(body)  # 12 (SOT segment) + SOD(2) + body
    sot_content += u32(psot)
    sot_content += u8(0)  # TPsot
    sot_content += u8(1)  # TNsot
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
    data = build_j2c(xsiz, ysiz, num_decomp, cb_exp, cb_exp, num_layers)
    with open(outpath, "wb") as f:
        f.write(data)
    print(f"wrote {outpath}: {len(data)} bytes")
