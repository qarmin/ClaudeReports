#!/usr/bin/env bash
# run.sh <kmod_src_dir> [<build_dir, unused>]: scripts/stable-backport.sh --help. Exit 1 = bug reproduced (--help exits non-zero and prints the usage text on stderr only).
set -u
src=$(realpath "${1:?kmod source dir}")
out=$(bash "$src/scripts/stable-backport.sh" --help 2>/dev/null); rc=$?
err=$(bash "$src/scripts/stable-backport.sh" --help 2>&1 >/dev/null)
echo "### --help: exit status $rc, usage on stdout: $([ -n "$out" ] && echo yes || echo no), usage on stderr: $([ -n "$err" ] && echo yes || echo no)"
[ "$rc" != 0 ] && exit 1
exit 0
