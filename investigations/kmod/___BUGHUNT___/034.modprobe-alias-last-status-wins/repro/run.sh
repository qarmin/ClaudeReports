#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: `modprobe foo` where the alias foo resolves to two modules m0 and m1; the install command of one of them fails. Exit 1 = bug reproduced (modprobe's exit status depends on the order of the two modules, not on whether an insertion failed).
set -u
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
root=$(mktemp -d); trap 'rm -rf "$root"' EXIT
mkdir -p "$root/lib/modules/1.0/kernel" "$root/conf"
touch "$root"/lib/modules/1.0/modules.{order,builtin,builtin.modinfo}
PYTHONPATH="$(dirname "$0")" python3 -c "
import sys; from synth import module
for n in ('m0', 'm1'):
    open(sys.argv[1] + '/lib/modules/1.0/kernel/%s.ko' % n, 'wb').write(module(['s' + n], []))" "$root"
"$bld/depmod" -b "$root" 1.0 2>/dev/null
rcs=()
for bad in m0 m1; do
	good=m1; [ $bad = m1 ] && good=m0
	printf 'alias foo m0\nalias foo m1\ninstall %s exit 3\ninstall %s true\n' $bad $good > "$root/conf/a.conf"
	"$bld/modprobe" -d "$root" -S 1.0 -C "$root/conf" foo >/dev/null 2>&1; rcs+=($?)
	echo "### install command of $bad fails (the other succeeds): modprobe exit status ${rcs[-1]}"
done
[ "${rcs[0]}" != 0 ] && [ "${rcs[1]}" != 0 ] && exit 0 || exit 1
