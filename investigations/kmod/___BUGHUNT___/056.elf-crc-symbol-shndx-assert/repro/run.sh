#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: a module whose `__crc_foo` .symtab entry has st_shndx = 0x7777 (beyond the section table); real `modprobe --show-exports` and `depmod`. Exit 1 = bug reproduced (assertion failure / abort in elf_get_section_header_offset()).
set -u
here=$(cd "$(dirname "$0")" && pwd)
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
work=$(mktemp -d); trap 'rm -rf "$work"' EXIT
k=$work/root/lib/modules/1.0/kernel; mkdir -p "$k"; touch "$work"/root/lib/modules/1.0/modules.{order,builtin,builtin.modinfo}
PYTHONPATH="$here" python3 -c "
import struct, sys
from synth_arch import module
m = bytearray(module([('foo', 0x1234)], [], ['license=GPL']))
i = m.find(struct.pack('<Q', 0x1234))      # st_value of the __crc_foo symbol
struct.pack_into('<H', m, i - 2, 0x7777)   # st_shndx
open(sys.argv[1], 'wb').write(m)" "$k/m.ko"
bad=0
"$bld/modprobe" --show-exports "$k/m.ko" >/dev/null 2>"$work/e1"; r1=$?
echo "### modprobe --show-exports: exit $r1 $(head -c 150 "$work/e1")"
"$bld/depmod" -b "$work/root" 1.0 >/dev/null 2>"$work/e2"; r2=$?
echo "### depmod: exit $r2 $(grep -v 'could not open' "$work/e2" | head -c 150)"
{ [ "$r1" -ge 128 ] || [ "$r2" -ge 128 ] || grep -q Assertion "$work/e1" "$work/e2"; } && bad=1
exit $bad
