#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: `modprobe -n -v foo` where foo has a softdep and an install command. Non-zero = install command ignored (insmod used instead).
set -u
here=$(cd "$(dirname "$0")" && pwd)
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
root=$(mktemp -d); trap 'rm -rf "$root"' EXIT
python3 "$here/gen_tree.py" "$root"
"$bld/depmod" -b "$root" 1.0 >/dev/null 2>&1
out=$("$here/memcap.sh" 2048 "$bld/modprobe" -n -v -C "$root/etc" -d "$root" -S 1.0 foo 2>&1)
sed "s|$root|<root>|g" <<<"$out"
grep -q "^install /bin/echo INSTALL-CMD-RAN" <<<"$out"
