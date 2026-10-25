#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: run depmod once normally, then again with a file-size limit (ulimit -f, SIGXFSZ ignored) so that writing modules.dep fails with EFBIG. Exit 1 = bug reproduced (the previous, complete modules.dep was replaced by the truncated file).
set -u
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
root=$(mktemp -d); trap 'rm -rf "$root"' EXIT
k=$root/lib/modules/1.0/kernel; d=$root/lib/modules/1.0
mkdir -p "$k"
touch "$d"/modules.{order,builtin,builtin.modinfo}
PYTHONPATH="$(dirname "$0")" python3 -c "
import sys; from synth import module
for i in range(60):
    open('%s/mod_with_a_long_name_%02d.ko' % (sys.argv[1], i), 'wb').write(module(['s%d' % i], ['s%d' % (i - 1)] if i else []))" "$k"
"$bld/depmod" -b "$root" 1.0 || { echo "initial depmod failed"; exit 2; }
good=$(stat -c %s "$d/modules.dep")
echo "### first run: modules.dep $good bytes"
(trap '' XFSZ; ulimit -f 2; "$bld/depmod" -b "$root" 1.0 2>&1 | sed 's|^|    |'; echo "    depmod exit status ${PIPESTATUS[0]}")
now=$(stat -c %s "$d/modules.dep")
echo "### second run (writes limited to 2 KiB): modules.dep is now $now bytes ($([ "$now" -lt "$good" ] && echo "TRUNCATED, old good file lost" || echo "old file kept"))"
[ "$now" -lt "$good" ] && exit 1
exit 0
