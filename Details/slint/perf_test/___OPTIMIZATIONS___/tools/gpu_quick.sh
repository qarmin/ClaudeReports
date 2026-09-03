#!/usr/bin/env bash
# ~1 minute on a real display. No rebuilds of anything but perf_test, no pristine binary, no
# mimalloc: those questions are already answered (GPU_RESULTS.md). This one only checks that the
# current tree still performs, and prints the size of the remaining structural win.
#
# Appends to GPU_QUICK.md with a timestamp so runs accumulate.
set -uo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
opt_dir="$(dirname "$here")"
harness="$(dirname "$opt_dir")"
report="$opt_dir/GPU_QUICK.md"
bin="$(dirname "$harness")/target/rdebug/perf_test"

if [ -z "${DISPLAY:-}" ] && [ -z "${WAYLAND_DISPLAY:-}" ]; then
    echo "No display. Use: cd $harness && just bench-all   (software renderer, no display needed)"
    exit 1
fi

FRAMES=${FRAMES:-10}
cd "$harness" && cargo build --profile rdebug --bin perf_test -q || exit 1

run() { # mode count extra...
    local mode=$1 count=$2; shift 2
    "$bin" --mode "$mode" --count "$count" "$@" --frames "$FRAMES" 2>/dev/null |
        awk '
            /^window /         { w = $2" "$3" "$4" "$5" "$6 }
            /^first frame/     { first = $3 }
            /^resident memory/ { rss = $3" "$4 }
            /^later frames/    { gsub("[(,]","",$7); time = $3; allocs = $7; shapes = $9 }
            END { printf "%s|%s|%s|%s|%s|%s\n", time, allocs, shapes, first, rss, w }'
}

{
    echo
    echo "## Run $(date '+%Y-%m-%d %H:%M')"
    echo
    echo "| scene | frame time | allocations/frame | shapes/frame | first frame | resident | window |"
    echo "| --- | --- | --- | --- | --- | --- | --- |"
} >> "$report"

# The `text` scenes are what candidate 03 changed; their headline numbers are the *first* frame
# (which carries the layout pass that shapes the document) and what stays resident, not the
# steady-state frame time. The rest are regression sentinels.
for scene in "text 100000" "text 100000 --word-wrap" "rects 3000" "rects 3000 --gradient" \
             "list 100000" "scroll 100000" "scroll 100000 --row-style bare"; do
    # shellcheck disable=SC2086
    out=$(run $scene)
    IFS='|' read -r time allocs shapes first rss window <<< "$out"
    printf "| \`%s\` | %s | %s | %s | %s | %s | %s |\n" "$scene" "${time:-n/a}" \
        "${allocs:-n/a}" "${shapes:-n/a}" "${first:-n/a}" "${rss:-n/a}" "${window:-n/a}" >> "$report"
    tail -1 "$report"
done

{
    echo
    echo "\`scroll\` vs \`scroll --row-style bare\` is the prize still on the table: pruning the"
    echo "children of culled items (candidate 12). Everything else in this table is the current"
    echo "state of the tree - compare against the previous run above."
    echo
    echo "For the two \`text\` rows read *first frame* and *resident*: that is where candidate 03"
    echo "acts (\`no-wrap\` shapes the document lazily, \`word-wrap\` deliberately does not). To"
    echo "attribute a change to it rather than to the machine, run \`tools/gpu_before_after.sh\`,"
    echo "which builds a pristine-textlayout binary and runs both."
} >> "$report"

echo
echo "appended to: $report"
