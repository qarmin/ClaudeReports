#!/usr/bin/env bash
# Run any command under a hard RAM cap so a runaway repro/build/test can never push the machine into swap/OOM-freeze.
#
#   memcap.sh <limit_mb> <command...>        e.g. memcap.sh 4096 ./repro
#
# stdin is forwarded to the command (a plain `&` would redirect it from /dev/null).
# Env: MEMCAP_POLL (seconds, default 0.05), MEMCAP_MIN_AVAIL_MB (system-wide floor, default 6144). Exit 137 and a message on stderr when the cap was hit.
#
# Why a polling watchdog: `ulimit -v`/`prlimit --as` cap virtual size, which ASAN/TSAN/MSAN reserve in the TB range, so
# they break sanitizer builds. This sums RSS of the whole process tree instead, which works for every tool.
set -uo pipefail

if [ "$#" -lt 2 ]; then
    sed -n '2,5p' "$0" >&2
    exit 2
fi

limit_mb="$1"; shift
poll="${MEMCAP_POLL:-0.05}"
min_avail_mb="${MEMCAP_MIN_AVAIL_MB:-6144}"
ulimit -c 0

# Make sanitizer/fuzzer allocators fail cleanly instead of growing until the watchdog fires.
export ASAN_OPTIONS="${ASAN_OPTIONS:-}:hard_rss_limit_mb=${limit_mb}:allocator_may_return_null=1"
export LSAN_OPTIONS="${LSAN_OPTIONS:-}"

setsid bash -c 'echo 1000 > /proc/self/oom_score_adj 2>/dev/null; exec "$@"' _ "$@" <&0 &
pid=$!

tree_rss_kb() {
    local pgid="$1"
    ps -o rss= -g "$pgid" 2>/dev/null | awk '{s+=$1} END {print s+0}'
}

avail_mb() {
    awk '/^MemAvailable:/ {print int($2 / 1024)}' /proc/meminfo
}

# The system-wide floor also covers many capped commands running at once (thread pools, parallel builds), which the per-tree cap cannot see.
while kill -0 "$pid" 2>/dev/null; do
    if [ "$(avail_mb)" -lt "$min_avail_mb" ]; then
        echo "memcap: system MemAvailable below ${min_avail_mb} MB, killing: $*" >&2
        kill -KILL -- "-$pid" 2>/dev/null
        wait "$pid" 2>/dev/null
        exit 137
    fi
    if [ "$(tree_rss_kb "$pid")" -gt $((limit_mb * 1024)) ]; then
        echo "memcap: RSS exceeded ${limit_mb} MB, killing: $*" >&2
        kill -KILL -- "-$pid" 2>/dev/null
        wait "$pid" 2>/dev/null
        exit 137
    fi
    sleep "$poll"
done
wait "$pid"
