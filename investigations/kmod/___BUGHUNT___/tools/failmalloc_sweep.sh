#!/usr/bin/env bash
# failmalloc_sweep.sh <label> <cmd...>: run cmd once to count allocations, then once per allocation with that one failing; report signal exits.
set -u
ulimit -c 0
here=$(cd "$(dirname "$0")" && pwd); label=$1; shift
total=$(FAILMALLOC_COUNT=1 LD_PRELOAD=$here/failmalloc.so "$@" 2>&1 >/dev/null | grep -a -o 'FAILMALLOC_TOTAL=[0-9]*' | tail -1 | cut -d= -f2)
echo "$label: $total allocations"
step=$(( ${total:-0} / ${MAX_RUNS:-1500} + 1 ))
for n in $(seq 1 "$step" "${total:-0}"); do
	timeout -s KILL 20 env FAILMALLOC_N=$n LD_PRELOAD=$here/failmalloc.so "$@" >/dev/null 2>&1
	rc=$?
	if [ "$rc" -ge 124 ]; then echo "  $label alloc#$n -> exit $rc"; fi
done
