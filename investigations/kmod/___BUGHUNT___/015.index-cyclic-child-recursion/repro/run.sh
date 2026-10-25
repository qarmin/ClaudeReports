#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: real `modprobe -c` on a modules.alias.bin whose root node has a child pointing back at the root. Non-zero = crash/hang reproduced.
set -u
here=$(cd "$(dirname "$0")" && pwd)
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
root=$(mktemp -d); trap 'rm -rf "$root"' EXIT
d=$root/lib/modules/1.0; mkdir -p "$d" "$root/etc"
python3 - "$d" <<'P'
import struct, sys
empty = struct.pack(">III", 0xB007F457, 0x00020001, 0) + b"\0" * 16
for n in ("dep", "symbols", "builtin", "builtin.alias"):
    open(f"{sys.argv[1]}/modules.{n}.bin", "wb").write(empty)
P
python3 "$here/gen_index.py" cycle "$d/modules.alias.bin"
out=$("$here/memcap.sh" 2048 timeout -s KILL 20 "$bld/modprobe" -C "$root/etc" -d "$root" -S 1.0 -c 2>&1 | head -c 100000); rc=${PIPESTATUS[0]}
bad=0
grep -E "ERROR: AddressSanitizer|memcap" <<<"$out" | head -2 | cut -c1-200
echo "mm reader (modprobe -c) exit code: $rc"
grep -qE "AddressSanitizer|memcap: RSS" <<<"$out" && bad=1
[ "$rc" -eq 124 ] || [ "$rc" -eq 137 ] || [ "$rc" -eq 139 ] && bad=1
python3 "$here/gen_index.py" cycle-wild "$d/modules.alias.bin"
out=$("$here/memcap.sh" 2048 timeout -s KILL 20 "$bld/modinfo" -b "$root" -k 1.0 foo 2>&1 | head -c 100000); rc=${PIPESTATUS[0]}
grep -E "ERROR: AddressSanitizer|memcap" <<<"$out" | head -2 | cut -c1-200
echo "stdio reader (modinfo) exit code: $rc"
grep -qE "AddressSanitizer|memcap: RSS" <<<"$out" && bad=1
[ "$rc" -eq 124 ] || [ "$rc" -eq 137 ] || [ "$rc" -eq 139 ] && bad=1
exit $bad
