#!/usr/bin/env python3
"""Generate the AIFF test files for candidate 028.

Every file is a structurally valid AIFF: FORM/AIFF container, a mandatory 18-byte COMM chunk with
an 80-bit IEEE-754 extended sample rate, and an SSND chunk with real (silent) sample data. The
only thing "adversarial" about them is the COMM chunk's num_channels field, which AIFF stores as
an unsigned 16-bit integer and Symphonia's own map_aiff_channel_count() deliberately accepts for
any value >= 4 (mapping it to Channels::Discrete(count)).
"""

import struct
import sys
import os

# 80-bit IEEE-754 extended precision encoding of 44100.0
RATE_44100 = bytes([0x40, 0x0E, 0xAC, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])


def chunk(tag: bytes, body: bytes) -> bytes:
    assert len(tag) == 4
    out = tag + struct.pack(">I", len(body)) + body
    # AIFF chunks are padded to an even length.
    if len(body) & 1:
        out += b"\x00"
    return out


def build_aiff(num_channels: int, sample_size: int, num_sample_frames: int = 4) -> bytes:
    comm = struct.pack(">HIH", num_channels, num_sample_frames, sample_size) + RATE_44100
    assert len(comm) == 18

    # SSND: offset + block_size + the sample data itself.
    ssnd_data = b"\x00" * 16
    ssnd = struct.pack(">II", 0, 0) + ssnd_data

    body = b"AIFF" + chunk(b"COMM", comm) + chunk(b"SSND", ssnd)
    return b"FORM" + struct.pack(">I", len(body)) + body


def main() -> None:
    out_dir = sys.argv[1] if len(sys.argv) > 1 else os.path.join(os.path.dirname(__file__), "assets")
    os.makedirs(out_dir, exist_ok=True)

    files = {
        # 32-bit samples -> 4 bytes/sample. 4 * 16384 == 65536, one past u16::MAX.
        "aiff_chan16384_size32.aiff": (16384, 32),
        # 24-bit samples -> 3 bytes/sample. 3 * 21846 == 65538, also past u16::MAX.
        "aiff_chan21846_size24.aiff": (21846, 24),
        # The extreme: 65535 channels, the largest value the 16-bit field can hold.
        "aiff_chan65535_size32.aiff": (65535, 32),
        # Control: a perfectly ordinary stereo 16-bit file, must keep working.
        "aiff_chan2_size16.aiff": (2, 16),
        # Control: many channels, but 8-bit samples -> 1 * 30000 fits in u16 fine.
        "aiff_chan30000_size8.aiff": (30000, 8),
    }

    for name, (chans, size) in files.items():
        path = os.path.join(out_dir, name)
        with open(path, "wb") as f:
            f.write(build_aiff(chans, size))
        print(f"wrote {path} ({chans} channels, {size}-bit)")


if __name__ == "__main__":
    main()
