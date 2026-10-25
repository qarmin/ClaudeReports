#!/usr/bin/env python3
"""mkseeds.py <out_dir> <bin files...>: prefix each real index with a few lookup keys."""
import os
import sys

KEYS = [b"", b"foo", b"mod_simple", b"pci:v00008086d00001234sv*", b"symbol:printk", b"abc*"]
out = sys.argv[1]
os.makedirs(out, exist_ok=True)
seen = set()
for path in sys.argv[2:]:
    data = open(path, "rb").read()
    if len(data) > 64 * 1024 or data in seen:
        continue
    seen.add(data)
    for i, k in enumerate(KEYS):
        name = f"{len(seen):04d}_{i}"
        with open(os.path.join(out, name), "wb") as f:
            f.write(bytes([len(k)]) + k + data)
print(f"{len(seen)} unique index files")
