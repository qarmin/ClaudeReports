"""Render a single glyph from the COLRv1 test font via both vello_cpu (real
end-to-end path, through vello_harness/) and resvg (an independent,
ttf-parser/usvg-based COLR implementation used as the differential reference
renderer), crop to the union bounding box, and save a side-by-side comparison
PNG.

Prerequisites (see ../justfile):
  - `cargo build --release` already run inside vello_harness/
  - `resvg` CLI on PATH (independent Rust COLR/SVG renderer, NOT part of vello)
  - python3 with fontTools, Pillow, numpy installed

Usage: compare.py <glyph_id> <out_prefix>
"""
import subprocess
import sys
from pathlib import Path

from fontTools.ttLib import TTFont
from PIL import Image
import numpy as np

HERE = Path(__file__).resolve().parent
FONT = str(HERE / "../clone/examples/assets/colr_test_glyphs/test_glyphs-glyf_colr_1.ttf")
HARNESS = str(HERE / "vello_harness/target/release/colr_diff_harness")
SIZE = 2000
CX = 700
CY = 1300

_font_cache = None


def get_font():
    global _font_cache
    if _font_cache is None:
        _font_cache = TTFont(FONT)
    return _font_cache


def cmap_for(gid):
    f = get_font()
    cmap = f.getBestCmap()
    gorder = f.getGlyphOrder()
    name = gorder[gid]
    for cp, n in cmap.items():
        if n == name:
            return cp, name
    return None, name


def render_vello(gid, out_path):
    subprocess.run([HARNESS, str(gid), out_path], check=True)


def render_resvg(gid, out_path):
    cp, name = cmap_for(gid)
    if cp is None:
        raise RuntimeError(f"no cmap entry for glyph {gid} ({name})")
    svg_path = f"/tmp/_colr_diff_{gid}.svg"
    svg = f'''<svg xmlns="http://www.w3.org/2000/svg" width="{SIZE}" height="{SIZE}" viewBox="0 0 {SIZE} {SIZE}">
<text x="{CX}" y="{CY}" font-family="COLRv1 Static Test Glyphs" font-size="1000">&#{cp};</text>
</svg>'''
    Path(svg_path).write_text(svg)
    subprocess.run([
        "resvg",
        "--use-font-file", FONT,
        "--skip-system-fonts",
        "-w", str(SIZE), "-h", str(SIZE),
        svg_path, out_path,
    ], check=True, capture_output=True)
    return name


def union_bbox(*imgs):
    x0 = y0 = 10**9
    x1 = y1 = -1
    for im in imgs:
        arr = np.array(im)
        alpha = arr[:, :, 3]
        ys, xs = np.nonzero(alpha > 2)
        if len(xs) == 0:
            continue
        x0 = min(x0, xs.min())
        x1 = max(x1, xs.max())
        y0 = min(y0, ys.min())
        y1 = max(y1, ys.max())
    if x1 < 0:
        return (0, 0, SIZE, SIZE)
    pad = 20
    return (max(0, x0 - pad), max(0, y0 - pad), min(SIZE, x1 + pad), min(SIZE, y1 + pad))


def main():
    gid = int(sys.argv[1])
    out_prefix = sys.argv[2]

    vello_path = f"{out_prefix}_vello.png"
    resvg_path = f"{out_prefix}_resvg.png"

    render_vello(gid, vello_path)
    name = render_resvg(gid, resvg_path)

    im_v = Image.open(vello_path).convert("RGBA")
    im_r = Image.open(resvg_path).convert("RGBA")

    bbox = union_bbox(im_v, im_r)
    im_v_c = im_v.crop(bbox)
    im_r_c = im_r.crop(bbox)

    def flatten(im):
        bg = Image.new("RGBA", im.size, (255, 255, 255, 255))
        bg.alpha_composite(im)
        return bg.convert("RGB")

    fv = flatten(im_v_c)
    fr = flatten(im_r_c)

    w, h = fv.size
    combo = Image.new("RGB", (w * 2 + 10, h), (200, 200, 200))
    combo.paste(fv, (0, 0))
    combo.paste(fr, (w + 10, 0))
    combo_path = f"{out_prefix}_combo.png"
    combo.save(combo_path)
    print(f"glyph {gid} ({name}): vello|resvg -> {combo_path}")


if __name__ == "__main__":
    main()
