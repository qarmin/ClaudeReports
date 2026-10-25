#!/usr/bin/env bash
# run_sim.sh <build_dir> <fake_state_spec> <modprobe args...>: build a/b/c/d tree with depmod and a fake kernel state, run modprobe under sysfake.
# spec: "mod:refcnt:holder1,holder2;..." e.g. "a:1:b;b:1:c;c:0:"
set -u
bld=$(realpath "$1"); spec=$2; shift 2
here=$(cd "$(dirname "$0")" && pwd)
work=$(mktemp -d)
cc -O1 -shared -fPIC -w "$here/sysfake.c" -ldl -o "$work/sysfake.so" || exit 2
T=$work/tree; F=$work/fake; k=$T/lib/modules/1.0/kernel; mkdir -p "$k" "$F/proc" "$work/conf"
touch "$T"/lib/modules/1.0/modules.{order,builtin,builtin.modinfo}
[ -n "${CONF:-}" ] && printf '%b' "$CONF" > "$work/conf/a.conf"
PYTHONPATH="$here/../lib" python3 - "$k" "${DEPS-b:a;c:b}" <<'P'
import sys
from synth import module
k, deps = sys.argv[1], dict(x.split(':') for x in sys.argv[2].split(';') if x)
names = sorted(set(list(deps) + list(deps.values()) + list('abcde')))
for n in names:
    imp = ['s' + d for d in deps.get(n, '').split(',') if d]
    open('%s/%s.ko' % (k, n), 'wb').write(module(['s' + n], imp))
P
LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") "$bld/depmod" -b "$T" 1.0 2>/dev/null
[ -n "${CMDLINE:-}" ] && printf '%s\n' "$CMDLINE" > "$F/proc/cmdline"
IFS=';' read -ra mods <<< "$spec"
for m in "${mods[@]}"; do
	IFS=: read -r n rc holders <<< "$m"
	mkdir -p "$F/sys/module/$n/holders"; echo live > "$F/sys/module/$n/initstate"; echo "$rc" > "$F/sys/module/$n/refcnt"
	for h in ${holders//,/ }; do touch "$F/sys/module/$n/holders/$h"; done
done
env LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0:verify_asan_link_order=0 \
	SYSFAKE="$F" LD_PRELOAD="$work/sysfake.so" "$bld/${TOOL:-modprobe}" ${TOOL:+} $( [ -z "${TOOL:-}" ] && echo "-d $T -S 1.0 -C $work/conf" ) "$@"
rc=$?
echo "### rc=$rc log: $(grep -v '^$' "$F/log" 2>/dev/null | tr '\n' ';')"
echo "### loaded: $(ls "$F/sys/module" | tr '\n' ' ')"
rm -rf "$work"
