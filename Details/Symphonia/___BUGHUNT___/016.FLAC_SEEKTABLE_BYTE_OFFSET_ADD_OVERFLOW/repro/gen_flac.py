#!/usr/bin/env python3
"""Regenerates assets/seek_overflow_real.flac and assets/seek_overflow_midpoint.flac (both
already committed - this script documents/reproduces how they were built, it does not need to be
re-run to use the repro).

Both are a real, format-valid FLAC file (STREAMINFO + a crafted SEEKTABLE block spliced in right
after STREAMINFO) with a SEEKTABLE `byte_offset` field set to attacker-chosen values near
u64::MAX, which is what triggers the two overflow panics in FlacReader::do_seek. The base file
(everything other than the injected SEEKTABLE block) comes from a real encoder
(`ffmpeg -f lavfi -i "sine=frequency=440:duration=1" -ar 44100 -ac 1 <out>.flac`) so the
STREAMINFO/VorbisComment/Padding/frame data around the injected SEEKTABLE are all genuine,
independently-encoder-produced bytes - only the SEEKTABLE block itself is hand-crafted, which is
the field under test.

Usage: python3 gen_flac.py <source.flac>
  (produces a real source.flac via `ffmpeg -f lavfi -i "sine=frequency=440:duration=1" ...`
   if it doesn't already exist)
"""
import os
import struct
import subprocess
import sys


def ensure_source(path: str) -> None:
    if os.path.exists(path):
        return
    os.makedirs(os.path.dirname(path) or ".", exist_ok=True)
    subprocess.run(
        [
            "ffmpeg", "-y", "-f", "lavfi", "-i", "sine=frequency=440:duration=1",
            "-ar", "44100", "-ac", "1", path, "-hide_banner", "-loglevel", "error",
        ],
        check=True,
    )


def find_streaminfo_end(data: bytes) -> int:
    assert data[:4] == b"fLaC"
    hdr = data[4]
    block_len = int.from_bytes(data[5:8], "big")
    assert (hdr & 0x7F) == 0, "first metadata block must be STREAMINFO"
    return 4 + 4 + block_len


def splice_seektable(data: bytes, seekpoints: bytes) -> bytes:
    streaminfo_end = find_streaminfo_end(data)
    header = bytes([0x03]) + len(seekpoints).to_bytes(3, "big")  # type=SEEKTABLE(3), not last
    return data[:streaminfo_end] + header + seekpoints + data[streaminfo_end:]


def main() -> None:
    source = sys.argv[1] if len(sys.argv) > 1 else "assets/_source.flac"
    ensure_source(source)
    base = open(source, "rb").read()

    os.makedirs("assets", exist_ok=True)

    # One seek point, byte_offset = u64::MAX: triggers SeekSearchResult::Lower, panicking on the
    # `first_frame_offset + lower.byte_offset` addition itself (demuxer.rs:305).
    real = splice_seektable(base, struct.pack(">QQH", 0, 0xFFFFFFFFFFFFFFFF, 1))
    with open("assets/seek_overflow_real.flac", "wb") as f:
        f.write(real)
    print(f"wrote assets/seek_overflow_real.flac ({len(real)} bytes)")

    # Two seek points, byte_offset ~ 0xC0000000_00000000 each: triggers SeekSearchResult::Range;
    # each individual `first_frame_offset + byte_offset` addition stays in range, but their sum in
    # the midpoint expression `(start_byte_offset + end_byte_offset) / 2` (demuxer.rs:322) then
    # overflows - the literal clippy::manual_midpoint line.
    p0 = struct.pack(">QQH", 0, 0xC000000000000000, 1)
    p1 = struct.pack(">QQH", 1000, 0xC000000000004000, 1)
    midpoint = splice_seektable(base, p0 + p1)
    with open("assets/seek_overflow_midpoint.flac", "wb") as f:
        f.write(midpoint)
    print(f"wrote assets/seek_overflow_midpoint.flac ({len(midpoint)} bytes)")


if __name__ == "__main__":
    main()
