#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: real `modprobe -c` (index dump) and `modinfo` (alias lookup with wildcards) on a modules.alias.bin of about 400 bytes whose nodes share children (2**depth paths). Non-zero = the walk does not finish within the time limit.
set -u
here=$(cd "$(dirname "$0")" && pwd)
bld=$(realpath "${2:?build dir}")
depth=${DEPTH:-40}
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
python3 "$here/gen_index.py" "$depth" "$d/modules.alias.bin"
echo "### modules.alias.bin: $(stat -c %s "$d/modules.alias.bin") bytes, depth $depth"
bad=0
start=$(date +%s)
"$here/memcap.sh" 2048 timeout -s KILL 20 "$bld/modprobe" -C "$root/etc" -d "$root" -S 1.0 -c >/dev/null 2>&1; rc=$?
echo "### modprobe -c: exit code $rc after $(( $(date +%s) - start )) s"
{ [ "$rc" -eq 124 ] || [ "$rc" -eq 137 ]; } && bad=1
python3 "$here/gen_index.py" "$depth" "$d/modules.alias.bin" wild
for t in "mmapped reader: modprobe -n foo|$bld/modprobe -C $root/etc -d $root -S 1.0 -n foo" "stdio reader: modinfo foo|$bld/modinfo -b $root -k 1.0 foo"; do
	start=$(date +%s)
	"$here/memcap.sh" 2048 timeout -s KILL 20 ${t#*|} >/dev/null 2>&1; rc=$?
	echo "### wildcard lookup, ${t%%|*}: exit code $rc after $(( $(date +%s) - start )) s"
	{ [ "$rc" -eq 124 ] || [ "$rc" -eq 137 ]; } && bad=1
done
exit $bad
