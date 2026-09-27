import sys

jp2_path = sys.argv[1] if len(sys.argv) > 1 else "grad4g2.jp2"
out_path = sys.argv[2] if len(sys.argv) > 2 else "jpx_scale_gray4.pdf"

jp2_data = open(jp2_path, "rb").read()

objs = {}

content5 = b"q\n200 0 0 200 0 0 cm\n/Im0 Do\nQ\n"
objs[5] = (
    b"<<\n  /Length " + str(len(content5)).encode() + b"\n>>\nstream\n" + content5 + b"\nendstream"
)

# Image XObject: JPXDecode filtered, 16x16 gray, intrinsic (JPX-header) bit depth 4.
# No /BitsPerComponent and no /ColorSpace keys - both must be inferred from the JPX
# codestream itself (allowed per spec for JPXDecode-filtered images), which is what
# drives hayro-syntax::filter::jpx::decode()'s `bpc != 8` branch into scale()/round_f32.
objs[6] = (
    b"<<\n"
    b"  /Type /XObject\n"
    b"  /Subtype /Image\n"
    b"  /Width 16\n"
    b"  /Height 16\n"
    b"  /Filter /JPXDecode\n"
    b"  /Length " + str(len(jp2_data)).encode() + b"\n"
    b">>\nstream\n" + jp2_data + b"\nendstream"
)

objs[4] = (
    b"<<\n"
    b"  /Parent 2 0 R\n"
    b"  /Resources << /XObject << /Im0 6 0 R >> >>\n"
    b"  /Type /Page\n"
    b"  /MediaBox [ 0 0 200 200 ]\n"
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
