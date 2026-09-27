import sys

jpg_path = sys.argv[1] if len(sys.argv) > 1 else "big.jpg"
out_path = sys.argv[2] if len(sys.argv) > 2 else "dct_downscale.pdf"
img_w = int(sys.argv[3]) if len(sys.argv) > 3 else 3000
img_h = int(sys.argv[4]) if len(sys.argv) > 4 else 2000
draw_w = int(sys.argv[5]) if len(sys.argv) > 5 else 300
draw_h = int(sys.argv[6]) if len(sys.argv) > 6 else 200

jpg_data = open(jpg_path, "rb").read()

objs = {}

content5 = f"q\n{draw_w} 0 0 {draw_h} 0 0 cm\n/Im0 Do\nQ\n".encode()
objs[5] = (
    b"<<\n  /Length " + str(len(content5)).encode() + b"\n>>\nstream\n" + content5 + b"\nendstream"
)

# Image XObject: DCTDecode filtered, real baseline JPEG, embedded far larger
# than its on-page draw size (img_w x img_h drawn at draw_w x draw_h) so the
# renderer's target_dimension hint implies a >=2x reduction on both axes.
objs[6] = (
    b"<<\n"
    b"  /Type /XObject\n"
    b"  /Subtype /Image\n"
    b"  /Width " + str(img_w).encode() + b"\n"
    b"  /Height " + str(img_h).encode() + b"\n"
    b"  /ColorSpace /DeviceRGB\n"
    b"  /BitsPerComponent 8\n"
    b"  /Filter /DCTDecode\n"
    b"  /Length " + str(len(jpg_data)).encode() + b"\n"
    b">>\nstream\n" + jpg_data + b"\nendstream"
)

objs[4] = (
    b"<<\n"
    b"  /Parent 2 0 R\n"
    b"  /Resources << /XObject << /Im0 6 0 R >> >>\n"
    b"  /Type /Page\n"
    b"  /MediaBox [ 0 0 " + str(draw_w).encode() + b" " + str(draw_h).encode() + b" ]\n"
    b"  /Contents 5 0 R\n"
    b">>"
)

objs[2] = b"<<\n  /Count 1\n  /Kids [ 4 0 R ]\n  /Type /Pages\n>>"

objs[1] = b"<<\n  /Type /Catalog\n  /Pages 2 0 R\n>>"

out = bytearray()
out += b"%PDF-1.7\n%\xc2\xb5\xc2\xb6\n\n"

offsets = {}
max_obj = max(objs.keys())
for n in range(1, max_obj + 1):
    if n not in objs:
        continue
    offsets[n] = len(out)
    out += f"{n} 0 obj\n".encode()
    out += objs[n]
    out += b"\nendobj\n\n"

xref_offset = len(out)
out += f"xref\n0 {max_obj+1}\n".encode()
out += b"0000000000 65535 f \n"
for n in range(1, max_obj + 1):
    if n in offsets:
        out += f"{offsets[n]:010d} 00000 n \n".encode()
    else:
        out += b"0000000000 65535 f \n"

out += b"trailer\n"
out += f"<< /Size {max_obj+1} /Root 1 0 R >>\n".encode()
out += b"startxref\n"
out += f"{xref_offset}\n".encode()
out += b"%%EOF\n"

with open(out_path, "wb") as f:
    f.write(out)

print("wrote", out_path, len(out), "bytes")
