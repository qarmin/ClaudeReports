#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: depmod on a 9-module graph with one dependency cycle (a -> b,h; b -> e,c; d -> b; e -> a,e; f -> b; h -> h,g). kmod_list_remove_data() returns the node after the removed one as the new list head, so depmod's cycle report starts at a rotated position. Exit 1 = the report starts at e (rotated), 0 = starts at b (list order kept).
set -u
bld=$(realpath "${2:?build dir}")
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
root=$(mktemp -d); trap 'rm -rf "$root"' EXIT
k=$root/lib/modules/1.0/kernel; mkdir -p "$k"
touch "$root"/lib/modules/1.0/modules.{order,builtin,builtin.modinfo}
PYTHONPATH="$(dirname "$0")" python3 -c "
import sys; from synth import module
deps = {'a': ['b', 'h'], 'b': ['e', 'c'], 'c': [], 'd': ['b'], 'e': ['a'], 'f': ['b'], 'g': [], 'h': ['g'], 'i': []}
for n, d in deps.items():
    open('%s/%s.ko' % (sys.argv[1], n), 'wb').write(module(['s' + n], ['s' + t for t in d]))" "$k"
line=$("$bld/depmod" -b "$root" 1.0 2>&1 | grep "Cycle detected")
echo "### $line"
case "$line" in *"Cycle detected: e ->"*) exit 1;; esac
exit 0
