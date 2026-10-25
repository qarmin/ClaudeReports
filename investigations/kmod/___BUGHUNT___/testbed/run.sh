#!/usr/bin/env bash
# Entry point used by `just testbed` / `just testbed-long`. Rebuilds what is stale, then runs.
#   run.sh [--long] [--seed N] [--only comp1,comp2] [--refresh]
#   --refresh re-exports the main project's HEAD into _src/ and rebuilds everything.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
args=()
refresh=0
for a in "$@"; do
	if [ "$a" = --refresh ]; then refresh=1; else args+=("$a"); fi
done
if [ "$refresh" = 1 ] || [ ! -f "$here/_src/meson.build" ]; then
	"$here/setup.sh"
fi
ulimit -c 0
exec python3 "$here/run.py" "${args[@]}"
