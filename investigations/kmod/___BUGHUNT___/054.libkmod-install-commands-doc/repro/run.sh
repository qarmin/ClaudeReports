#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: libkmod API doc check: kmod_module_get_install_commands()/get_remove_commands() "return a string with all install/remove commands separated by semicolons". Exit 1 = bug reproduced (only the first command is returned).
set -u
here=$(cd "$(dirname "$0")" && pwd)
src=$(realpath "${1:?kmod src dir}"); bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)"):$bld ASAN_OPTIONS=detect_leaks=0
work=$(mktemp -d); trap 'rm -rf "$work"' EXIT
mkdir -p "$work/conf" "$work/moddir"
printf 'install m echo one\ninstall m echo two\nremove m echo rone\nremove m echo rtwo\n' > "$work/conf/a.conf"
cc -I"$src/libkmod" "$here/repro.c" -L"$bld" -lkmod -Wl,-rpath,"$bld" -o "$work/repro" || exit 2
rt=$(ldd "$bld/libkmod.so" | awk '/asan/ {print $3}')
LD_PRELOAD=$rt "$work/repro" "$work/moddir" "$work/conf"
