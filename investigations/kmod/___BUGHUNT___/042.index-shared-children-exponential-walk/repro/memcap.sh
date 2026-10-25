#!/usr/bin/env bash
# Run any command under a hard RAM cap so a runaway repro/build/test can never push the machine into swap/OOM-freeze.
#
#   memcap.sh <limit_mb> <command...>        e.g. memcap.sh 4096 ./repro
#
# Env: MEMCAP_POLL (seconds, default 0.2). Exit 137 and a message on stderr when the cap was hit.
#
# Why a polling watchdog: `ulimit -v`/`prlimit --as` cap virtual size, which ASAN/TSAN/MSAN reserve in the TB range, so
# they break sanitizer builds. This sums RSS of the whole process tree instead, which works for every tool.
set -uo pipefail

if [ "$#" -lt 2 ]; then
    sed -n '2,5p' "$0" >&2
    exit 2
fi

limit_mb="$1"; shift
poll="${MEMCAP_POLL:-0.2}"
ulimit -c 0

# Make sanitizer/fuzzer allocators fail cleanly instead of growing until the watchdog fires.
export ASAN_OPTIONS="${ASAN_OPTIONS:-}:hard_rss_limit_mb=${limit_mb}:allocator_may_return_null=1"
export LSAN_OPTIONS="${LSAN_OPTIONS:-}"

setsid "$@" &
pid=$!

tree_rss_kb() {
    local pgid="$1"
    ps -o rss= -g "$pgid" 2>/dev/null | awk '{s+=$1} END {print s+0}'
}

while kill -0 "$pid" 2>/dev/null; do
    if [ "$(tree_rss_kb "$pid")" -gt $((limit_mb * 1024)) ]; then
        echo "memcap: RSS exceeded ${limit_mb} MB, killing: $*" >&2
        kill -KILL -- "-$pid" 2>/dev/null
        wait "$pid" 2>/dev/null
        exit 137
    fi
    sleep "$poll"
done
wait "$pid"
