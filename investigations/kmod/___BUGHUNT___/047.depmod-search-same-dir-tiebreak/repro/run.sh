#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: two modules with the same name in two subdirectories of the same search directory (updates/a/foo.ko, updates/b/foo.ko), depmod with `search updates built-in` and with `search built-in updates`. Exit 1 = bug reproduced (the two configurations pick different files, although the two candidates have the same priority under both).
set -u
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
pick() {
	local conf=$1 root; root=$(mktemp -d)
	mkdir -p "$root/c"; printf '%s\n' "$conf" > "$root/c/a.conf"
	touch "$root"/lib/modules/1.0/modules.{order,builtin,builtin.modinfo} 2>/dev/null || { mkdir -p "$root/lib/modules/1.0"; touch "$root"/lib/modules/1.0/modules.{order,builtin,builtin.modinfo}; }
	for d in updates/a updates/b; do
		mkdir -p "$root/lib/modules/1.0/$d"
		PYTHONPATH="$(dirname "$0")" python3 -c "
import sys; from synth import module
open(sys.argv[1], 'wb').write(module(['s'], []))" "$root/lib/modules/1.0/$d/foo.ko"
	done
	"$bld/depmod" -b "$root" -C "$root/c" 1.0 2>/dev/null
	cut -d: -f1 "$root/lib/modules/1.0/modules.dep"
	rm -rf "$root"
}
a=$(pick 'search updates built-in'); b=$(pick 'search built-in updates')
echo "### search updates built-in -> $a"
echo "### search built-in updates -> $b"
[ "$a" = "$b" ] || exit 1
exit 0
