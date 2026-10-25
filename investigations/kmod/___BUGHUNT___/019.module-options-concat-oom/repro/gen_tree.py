#!/usr/bin/env python3
"""gen_tree.py <root>: depmod'd-less tree: kernel/foo.ko, kernel/bar.ko (minimal ELF modules) and etc/a.conf with softdep + options for foo.
The script runs depmod itself through the HERE env (see run.sh)."""
import os
import struct
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from synth import module  # noqa: E402

root = sys.argv[1]
kd = os.path.join(root, "lib/modules/1.0/kernel")
os.makedirs(kd)
os.makedirs(os.path.join(root, "etc"))
open(os.path.join(kd, "foo.ko"), "wb").write(module(["sf"], ["sb"], modinfo=["license=GPL"]))
open(os.path.join(kd, "bar.ko"), "wb").write(module(["sb"], [], modinfo=["license=GPL"]))
open(os.path.join(root, "etc/a.conf"), "w").write("softdep foo pre: bar\noptions foo x=1\n")
