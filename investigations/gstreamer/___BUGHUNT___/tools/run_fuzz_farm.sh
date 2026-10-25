#!/usr/bin/env bash
# Launch several parallel, long-running tools/mini_fuzz.py instances against the ASAN build,
# grinding in the background while other work happens. Pure collection - no analysis, no
# candidate write-up; that's a separate, later step (see tools/triage_fuzz_farm.sh).
#
# Usage: tools/run_fuzz_farm.sh [hours]   (default: 6 - just needs to outlast the session; harmless
# to leave running longer, kill with tools/stop_fuzz_farm.sh when done)
set -o pipefail
cd "$(git rev-parse --show-toplevel)"

HOURS="${1:-6}"
SECONDS_BUDGET=$((HOURS * 3600))
BUILDDIR="build-asan-fuzz"
OUTROOT="___BUGHUNT___/fuzz_farm"
mkdir -p "$OUTROOT/logs"

source "$BUILDDIR/env.sh"
export ASAN_OPTIONS="abort_on_error=1:halt_on_error=1:exitcode=99:detect_leaks=0"
export UBSAN_OPTIONS="print_stacktrace=1"

# Warm the plugin registry cache once, serially, before spawning anything concurrent - a cold
# cache forks a slow gst-plugin-scanner helper (~2.4s) and doing that from N parallel processes at
# once is exactly the scenario that (harmlessly, but pointlessly) stresses the registry-cache
# write path. See ___BUGHUNT___/README.md's "warm the registry" note.
echo "warming plugin registry cache..."
"./$BUILDDIR/ci/fuzzing/typefind" ci/fuzzing/seeds/gst-value/caps-simple > /dev/null 2>&1
echo "warm-up done."

PIDFILE="$OUTROOT/pids.txt"
: > "$PIDFILE"

launch() {
    local target="$1" corpus="$2" tag="$3" seed="$4" timeout="${5:-8}"
    local crashdir="$OUTROOT/crashes_${target}_${tag}"
    local log="$OUTROOT/logs/${target}_${tag}.log"
    nohup python3 ___BUGHUNT___/tools/mini_fuzz.py \
        "./$BUILDDIR/ci/fuzzing/$target" "$corpus" "$crashdir" \
        --seconds "$SECONDS_BUDGET" --timeout "$timeout" --seed "$seed" \
        > "$log" 2>&1 &
    echo "$! $target $tag" >> "$PIDFILE"
    echo "launched $target ($tag, seed=$seed, timeout=${timeout}s) -> pid $!, crashes -> $crashdir"
}

# typefind: cheapest per-exec, broadest reach (every registered typefinder's sniff()) - most
# parallel instances, driven by the full synthetic media corpus.
for i in 1 2 3 4 5 6; do
    launch typefind "___BUGHUNT___/__corpus" "media$i" "$i"
done

# gst-discoverer: deepest target (builds a real decodebin pipeline) but slower per-exec - fewer
# instances so it doesn't starve everything else of CPU. --timeout 30 (not the default 8): a
# malformed FLV that hits decodebin's "can't find group for element" dead end takes ~10.2s to fail
# cleanly (confirmed 2026-09-15, see ___BUGHUNT___/020.gst_discoverer_timeout_false_positive/) -
# 8s was too tight and produced false "crash" reports for perfectly normal, bounded, non-hanging
# behavior. 30s leaves real margin above that observed worst case.
for i in 1 2 3 4; do
    launch gst-discoverer "___BUGHUNT___/__corpus" "media$i" "$((i + 100))" 30
done

# gst-subparse: small dedicated subtitle corpus.
if [ -d "___BUGHUNT___/__corpus_subs" ] && [ "$(ls -A ___BUGHUNT___/__corpus_subs)" ]; then
    launch gst-subparse "___BUGHUNT___/__corpus_subs" "subs1" 201
    launch gst-subparse "___BUGHUNT___/__corpus_subs" "subs2" 202
fi

# gst-value / gst-tag: cheap, use the project's own existing hand-written seeds.
launch gst-value "ci/fuzzing/seeds/gst-value" "seeds1" 301
launch gst-tag "ci/fuzzing/seeds/gst-value" "seeds1" 302

echo
echo "$(wc -l < "$PIDFILE") fuzzer instance(s) running for up to ${HOURS}h."
echo "Check progress:  tail -f $OUTROOT/logs/*.log"
echo "Stop them all:   bash ___BUGHUNT___/tools/stop_fuzz_farm.sh"
