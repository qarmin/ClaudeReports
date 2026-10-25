#!/usr/bin/env bash
# Run kmod's own meson test suite under ASAN/UBSAN.
#   run_tests.sh <build_dir>
# The test rootfs needs kernel headers to build dummy modules; the container has none, so a
# user-space extraction of linux-headers-7.0.0-38-generic lives in ___BUGHUNT___/__kheaders/.
set -uo pipefail
here=$(dirname "$(realpath "$0")")
export CC=clang
export KDIR=${KDIR:-$here/../__kheaders/root/usr/src/linux-headers-7.0.0-38-generic}
meson test -C "$1" --num-processes 1 2>&1 | grep -E "^ *[0-9]+/[0-9]+ |^Ok:|^Fail:"
