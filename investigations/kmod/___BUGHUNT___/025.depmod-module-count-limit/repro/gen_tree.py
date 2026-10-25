#!/usr/bin/env python3
"""gen_tree.py <root> <count>: <count> minimal ELF modules (no dependencies) under <root>/lib/modules/1.0/kernel/."""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from synth import module  # noqa: E402

root, count = sys.argv[1], int(sys.argv[2])
d = os.path.join(root, "lib/modules/1.0/kernel")
os.makedirs(d)
img = module([], [])
for i in range(count):
    with open(os.path.join(d, f"m{i}.ko"), "wb") as f:
        f.write(img)
