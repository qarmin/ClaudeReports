#!/usr/bin/env python3
"""Write crafted kmod index files (modules.*.bin, format v2.1).

gen_index.py <case> <out_file>
cases:
  stdio-last255   root: prefix "*", children first=last=255 (child offset 0). The stdio reader
                  accepts last == 255 and its uint8_t loop never terminates.
  cycle           root: children first=last='a', child offset points back to root.
  valcount        root: values flag, value_count = 0xffffffff, one real value, then EOF.
  prefix-noterm   root: prefix flag, prefix bytes run to EOF without a NUL.
  valid           a tiny well-formed index: "foo" -> "bar".
"""
import struct
import sys

MAGIC = 0xB007F457
VERSION = 0x00020001
F_PREFIX, F_VALUES, F_CHILDS = 0x80000000, 0x40000000, 0x20000000
ROOT = 12


def hdr(root):
    return struct.pack(">III", MAGIC, VERSION, root)


def build(case):
    if case == "stdio-last255":
        body = b"*\0" + bytes([255, 255]) + struct.pack(">I", 0)
        return hdr(ROOT | F_PREFIX | F_CHILDS) + body
    if case == "cycle":
        root = ROOT | F_CHILDS
        body = bytes([ord("a"), ord("a")]) + struct.pack(">I", root)
        return hdr(root) + body
    if case == "cycle-wild":
        root = ROOT | F_PREFIX | F_CHILDS
        body = b"*\0" + bytes([ord("a"), ord("a")]) + struct.pack(">I", root)
        return hdr(root) + body
    if case == "valcount":
        body = struct.pack(">I", 0xFFFFFFFF) + struct.pack(">I", 0) + b"x\0"
        return hdr(ROOT | F_VALUES) + body
    if case == "prefix-noterm":
        data = hdr(ROOT | F_PREFIX)
        return data + b"A" * (4096 - len(data))
    if case == "valid":
        leaf = 12
        body = b"foo\0" + struct.pack(">I", 1) + struct.pack(">I", 0) + b"bar\0"
        return hdr(leaf | F_PREFIX | F_VALUES) + body
    raise SystemExit(f"unknown case {case}")


if __name__ == "__main__":
    if len(sys.argv) != 3:
        raise SystemExit(__doc__)
    with open(sys.argv[2], "wb") as f:
        f.write(build(sys.argv[1]))
