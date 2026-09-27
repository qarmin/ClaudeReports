import re

objs = {}

# Object 8: Form XObject used as the soft mask's group (G). Its content stream sets ExtGState
# /R1 (object 6) again and fills - R1's SMask points right back at this same object 8, creating
# a self-referential soft mask.
content8 = b"q\n1 0 0 1 0 0 cm\n/R1 gs\n1 g\n0 0 100 100 re f\nQ\n"
objs[8] = (
    b"<<\n"
    b"  /BBox [ 0 0 612 792 ]\n"
    b"  /FormType 1\n"
    b"  /Matrix [ 1 0 0 1 0 0 ]\n"
    b"  /Subtype /Form\n"
    b"  /Type /XObject\n"
    b"  /Group << /S /Transparency /CS /DeviceGray >>\n"
    b"  /Resources << /ExtGState << /R1 6 0 R >> >>\n"
    b"  /Length " + str(len(content8)).encode() + b"\n"
    b">>\nstream\n" + content8 + b"\nendstream"
)

objs[6] = (
    b"<<\n"
    b"  /SMask << /G 8 0 R /S /Luminosity /BC [ 1 ] >>\n"
    b"  /Type /ExtGState\n"
    b">>"
)

content4 = b"q\n1 0 0 1 0 0 cm\n/R1 gs\n1 0 0 rg\n0 0 200 200 re f\nQ\n"
objs[4] = (
    b"<<\n  /Length " + str(len(content4)).encode() + b"\n>>\nstream\n" + content4 + b"\nendstream"
)

objs[3] = (
    b"<<\n"
    b"  /Parent 2 0 R\n"
    b"  /Resources << /ExtGState << /R1 6 0 R >> >>\n"
    b"  /Type /Page\n"
    b"  /MediaBox [ 0 0 200 200 ]\n"
    b"  /Contents 4 0 R\n"
    b">>"
)

objs[2] = b"<<\n  /Count 1\n  /Kids [ 3 0 R ]\n  /Type /Pages\n>>"

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

with open("self_softmask.pdf", "wb") as f:
    f.write(out)

print("wrote", len(out), "bytes")
