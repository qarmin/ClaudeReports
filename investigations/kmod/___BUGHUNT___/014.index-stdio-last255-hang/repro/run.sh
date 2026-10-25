#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: real `modinfo` on a modules.alias.bin whose root node has children range 255..255 (stdio reader). Non-zero = hang reproduced.
set -u
here=$(cd "$(dirname "$0")" && pwd)
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
root=$(mktemp -d); trap 'rm -rf "$root"' EXIT
d=$root/lib/modules/1.0; mkdir -p "$d"
python3 - "$d" <<'P'
import struct, sys
empty = struct.pack(">III", 0xB007F457, 0x00020001, 0) + b"\0" * 16
for n in ("dep", "symbols", "builtin", "builtin.alias"):
    open(f"{sys.argv[1]}/modules.{n}.bin", "wb").write(empty)
P
python3 "$here/gen_index.py" stdio-last255 "$d/modules.alias.bin"
"$here/memcap.sh" 1500 timeout -s KILL 10 "$bld/modinfo" -b "$root" -k 1.0 foo 2>&1 | head -3
rc=${PIPESTATUS[0]}
echo "modinfo exit code: $rc (124/137 = killed after 10 s)"
[ "$rc" -ne 124 ] && [ "$rc" -ne 137 ]
