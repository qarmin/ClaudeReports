#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: module m carries its own softdep (softdep=pre: a in .modinfo, exported to modules.softdep by depmod); the admin adds `softdep m pre: b` in a modprobe.d file named aa.conf or zz.conf. Exit 1 = bug reproduced (which softdep wins depends on the admin file's name).
set -u
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
declare -A res
for name in aa.conf zz.conf; do
	root=$(mktemp -d); d=$root/lib/modules/1.0; k=$d/kernel; mkdir -p "$k" "$root/conf"
	touch "$d"/modules.{order,builtin,builtin.modinfo}
	PYTHONPATH="$(dirname "$0")" python3 -c "
import sys; from synth import module
k = sys.argv[1]
open(k + '/m.ko', 'wb').write(module(['sm'], [], ['softdep=pre: a']))
for n in 'ab':
    open('%s/%s.ko' % (k, n), 'wb').write(module(['s' + n], []))" "$k"
	"$bld/depmod" -b "$root" 1.0 2>/dev/null
	echo 'softdep m pre: b' > "$root/conf/$name"
	res[$name]=$("$bld/modprobe" -d "$root" -S 1.0 -C "$root/conf" -n -v m 2>&1 | sed "s|$k/||;s|insmod ||" | tr -s ' \n' ' ')
	rm -rf "$root"
	echo "### admin file $name -> inserts: ${res[$name]}"
done
[ "${res[aa.conf]}" = "${res[zz.conf]}" ] || exit 1
exit 0
