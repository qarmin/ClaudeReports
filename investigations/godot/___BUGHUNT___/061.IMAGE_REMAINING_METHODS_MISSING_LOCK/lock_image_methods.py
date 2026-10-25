import re
import sys

methods = """_get_data
_set_data
adjust_bcs
blend_rect
blend_rect_mask
blit_rect
blit_rect_mask
bump_map_to_normal_map
clear_mipmaps
compress
compress_from_channels
compute_image_metrics
convert
copy_from
crop
decompress
detect_alpha
detect_used_channels
fill
fill_rect
fix_alpha_edges
flip_x
flip_y
generate_mipmaps
get_data_size
get_format
get_height
get_mipmap_count
get_mipmap_offset
get_pixelv
get_region
get_size
get_used_rect
get_width
has_mipmaps
is_compressed
is_empty
is_invisible
linear_to_srgb
load
load_bmp_from_buffer
load_dds_from_buffer
load_exr_from_buffer
load_jpg_from_buffer
load_ktx_from_buffer
load_png_from_buffer
load_svg_from_buffer
load_svg_from_string
load_tga_from_buffer
load_webp_from_buffer
normal_map_to_xy
premultiply_alpha
resize_to_po2
rgbe_to_srgb
rotate_180
rotate_90
save_dds
save_dds_to_buffer
save_exr
save_exr_to_buffer
save_jpg
save_jpg_to_buffer
save_png
save_png_to_buffer
save_webp
save_webp_to_buffer
set_pixelv
shrink_x2
srgb_to_linear""".split("\n")

# Skip: resize, set_data, get_pixel, set_pixel (already locked by candidate 036)
# Skip: get_data (returns const ref, "caller outlives the lock" - can't fix this way)

path = "core/io/image.cpp"
with open(path) as f:
    content = f.read()

patched = set()
not_found = []

for m in methods:
    # Match: start of line, anything except '(' or newline, then Image::method(, then
    # anything except '{' or newline up to the opening brace, then newline.
    pattern = re.compile(
        r'(^[^\n(]*\bImage::' + re.escape(m) + r'\([^{\n]*\{\n)', re.MULTILINE
    )
    matches = list(pattern.finditer(content))
    if not matches:
        not_found.append(m)
        continue
    if len(matches) > 1:
        print(f"WARNING: multiple matches for {m}, using first", file=sys.stderr)
    match = matches[0]
    insert_pos = match.end()
    content = content[:insert_pos] + "\t_THREAD_SAFE_METHOD_\n\n" + content[insert_pos:]
    patched.add(m)

with open(path, "w") as f:
    f.write(content)

print(f"Patched {len(patched)}/{len(methods)} methods")
if not_found:
    print("NOT FOUND:", not_found)
