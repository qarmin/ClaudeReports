#!/usr/bin/env python3
"""gen_index.py <depth> <out_file> [wild]: a modules.*.bin (format v2.1) whose nodes share children.

Level i has one node with children 'a' and 'b' that both point to the single node of level i+1, the last level holds one value (with "wild" the root has the prefix "*", so that an alias lookup walks the whole DAG): no cycle, depth <depth>, size about 10 bytes per level, but 2**depth distinct paths.
"""
import struct
import sys

depth = int(sys.argv[1])
wild = len(sys.argv) > 3 and sys.argv[3] == "wild"
F_PREFIX, F_VALUES, F_CHILDS = 0x80000000, 0x40000000, 0x20000000
ROOT = 12
body = b"*\0" if wild else b""
first = ROOT + len(body)
node_size = 2 + 8
for i in range(depth):
    nxt = first + (i + 1) * node_size
    nxt |= F_CHILDS if i + 1 < depth else F_VALUES
    body += bytes([ord("a"), ord("b")]) + struct.pack(">II", nxt, nxt)
body += struct.pack(">I", 1) + struct.pack(">I", 0) + b"x\0"
root = ROOT | (F_PREFIX if wild else 0) | (F_CHILDS if depth else F_VALUES)
open(sys.argv[2], "wb").write(struct.pack(">III", 0xB007F457, 0x00020001, root) + body)
