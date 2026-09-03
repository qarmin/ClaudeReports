import sys

# Same 8 base font dicts as candidate 020's gen_cache_defeating_pdf.py, but here every `Tf` call
# gets its OWN, never-reused resource name (F0, F1, F2, ...) pointing at one of the 8 underlying
# font objects (cycled just to keep them all valid Type1 fonts, not because it matters - what
# matters is the resource *name* is unique per call, since `resolved_font_cache`/`font_dict_cache`
# are both keyed by name, not by underlying font object). This guarantees every single `Tf` call
# is a genuine cache miss on both caches - the actual worst case, unlike candidate 018's
# `cache_defeating.pdf` (8 *reused* names, which candidate 020 showed hits 318/326 times).

FONTS = [
    "Helvetica", "Helvetica-Bold", "Times-Roman", "Courier",
    "Helvetica-Oblique", "Times-Bold", "Times-Italic", "Courier-Bold",
]

N_TF = int(sys.argv[2]) if len(sys.argv) > 2 else 326
out_path = sys.argv[1]

objects = []

def add_obj(body):
    objects.append(body)
    return len(objects)  # 1-indexed obj number

# Build content stream: every Tf call gets a distinct, never-repeated resource name.
lines = ["BT", "12 700 Td"]
y = 700
for i in range(N_TF):
    fname = f"F{i}"
    lines.append(f"/{fname} 10 Tf")
    lines.append(f"(line {i}) Tj")
    lines.append("0 -1 Td")
content = "\n".join(lines) + "\nET\n"
content_bytes = content.encode("latin1")

catalog_num = 1
pages_num = 2
page_num = 3
content_num = 4
font_base = 5

# N_TF distinct resource names, each pointing at one of only 8 real font objects (cycled) - keeps
# the underlying font object count small while every *name* is still unique.
font_res_entries = " ".join(f"/F{i} {font_base + (i % len(FONTS))} 0 R" for i in range(N_TF))

catalog = f"<< /Type /Catalog /Pages {pages_num} 0 R >>"
pages = f"<< /Type /Pages /Kids [{page_num} 0 R] /Count 1 >>"
page = (f"<< /Type /Page /Parent {pages_num} 0 R /MediaBox [0 0 612 792] "
        f"/Resources << /Font << {font_res_entries} >> >> /Contents {content_num} 0 R >>")
content_obj = f"<< /Length {len(content_bytes)} >>\nstream\n" + content.rstrip("\n") + "\nendstream"

font_objs = []
for i, fname in enumerate(FONTS):
    font_objs.append(f"<< /Type /Font /Subtype /Type1 /BaseFont /{fname} >>")

all_objs = [catalog, pages, page, content_obj] + font_objs

out = bytearray()
out += b"%PDF-1.4\n%\xe2\xe3\xcf\xd3\n"
offsets = [0]  # obj 0 placeholder
for idx, body in enumerate(all_objs, start=1):
    offsets.append(len(out))
    out += f"{idx} 0 obj\n".encode("latin1")
    out += body.encode("latin1")
    out += b"\nendobj\n"

xref_offset = len(out)
num_objs = len(all_objs) + 1
out += f"xref\n0 {num_objs}\n".encode("latin1")
out += b"0000000000 65535 f \n"
for off in offsets[1:]:
    out += f"{off:010d} 00000 n \n".encode("latin1")

out += b"trailer\n"
out += f"<< /Size {num_objs} /Root {catalog_num} 0 R >>\n".encode("latin1")
out += f"startxref\n{xref_offset}\n%%EOF".encode("latin1")

with open(out_path, "wb") as f:
    f.write(out)

print(f"Wrote {out_path}: {N_TF} Tf calls, {N_TF} distinct resource names (over {len(FONTS)} "
      f"underlying font objects), {len(out)} bytes")
