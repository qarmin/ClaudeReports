#!/usr/bin/env python3
"""gen_modules_dir.py <root>: <root>/etc/foo.conf with an install command using $CMDLINE_OPTS, plus empty modules.*.bin so modprobe -d <root> loads."""
import os
import struct
import sys

root = sys.argv[1]
d = os.path.join(root, "lib/modules/1.0")
os.makedirs(d)
os.makedirs(os.path.join(root, "etc"))
with open(os.path.join(root, "etc/foo.conf"), "w") as f:
    f.write("install foo /bin/true $CMDLINE_OPTS\n")
empty = struct.pack(">III", 0xB007F457, 0x00020001, 0) + b"\0" * 16
for name in ("dep", "alias", "symbols", "builtin", "builtin.alias"):
    open(f"{d}/modules.{name}.bin", "wb").write(empty)
