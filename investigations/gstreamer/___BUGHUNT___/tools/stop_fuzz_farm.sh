#!/usr/bin/env bash
# Stop every mini_fuzz.py instance launched by run_fuzz_farm.sh.
set -uo pipefail
cd "$(git rev-parse --show-toplevel)"
PIDFILE="___BUGHUNT___/fuzz_farm/pids.txt"
[ -f "$PIDFILE" ] || { echo "no $PIDFILE - nothing to stop"; exit 0; }
while read -r pid target tag; do
    if kill -0 "$pid" 2>/dev/null; then
        kill "$pid" 2>/dev/null && echo "stopped $target ($tag), pid $pid"
    fi
done < "$PIDFILE"
