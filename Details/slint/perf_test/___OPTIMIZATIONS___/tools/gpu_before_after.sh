#!/usr/bin/env bash
# Before/after on a real display for the text-layout work (candidate 03), plus the four scenes
# that act as regression sentinels.
#
# Builds a second binary from the *pristine* `internal/core/textlayout` and runs both, so the
# comparison is against this tree minus candidate 03 rather than against numbers recorded in some
# earlier session. That matters: allocation counts are deterministic and hardware independent, so
# any difference in that column is a real code difference and worth explaining.
#
# The working tree is restored by an EXIT trap even if a build fails or the run is interrupted.
#
# Appends to GPU_03.md. Env: FRAMES (default 10), SCENES (default: the list below).
set -uo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
opt_dir="$(dirname "$here")"
harness="$(dirname "$opt_dir")"
repo="$(dirname "$harness")"
report="$opt_dir/GPU_03.md"
bin="$repo/target/rdebug/perf_test"
base_bin="${TMPDIR:-/tmp}/perf_test_pristine_textlayout"

if [ -z "${DISPLAY:-}" ] && [ -z "${WAYLAND_DISPLAY:-}" ]; then
    echo "No display. This script needs one; use 'just run' in 03_text_document_shaping for the"
    echo "headless numbers instead."
    exit 1
fi

FRAMES=${FRAMES:-10}
# Runs per binary. Two is the minimum that shows a noise floor at all; the frame-time and
# startup columns on this harness need more than that before a few-percent difference means
# anything (the allocation column is exactly reproducible within a run, so two is plenty for it).
RUNS=${RUNS:-2}

# scene label|arguments
default_scenes=(
    "text 100000 no-wrap|--mode text --count 100000"
    "text 100000 word-wrap|--mode text --count 100000 --word-wrap"
    "text 10000 no-wrap|--mode text --count 10000"
    "rects 3000|--mode rects --count 3000"
    "rects 3000 --gradient|--mode rects --count 3000 --gradient"
    "list 100000|--mode list --count 100000"
    "scroll 100000|--mode scroll --count 100000"
)
IFS=$'\n' read -rd '' -a scenes <<< "${SCENES:-$(printf '%s\n' "${default_scenes[@]}")}"

cd "$repo" || exit 1

backup="$(mktemp -d)"
cp -a "$repo/internal/core/textlayout" "$backup/" || exit 1
cp -a "$harness/src" "$backup/src" || exit 1
restore() {
    rm -rf "$repo/internal/core/textlayout" "$harness/src"
    cp -a "$backup/textlayout" "$repo/internal/core/"
    cp -a "$backup/src" "$harness/src"
    # `cp -a` preserves mtimes, which puts them *behind* the pristine build's fingerprint: cargo
    # then reports "Finished" without rebuilding and leaves the pristine binary in place, so every
    # measurement after this point would silently be of the wrong tree. Cost one hour of confusion
    # the first time; touch the sources so the next build is real.
    find "$repo/internal/core/textlayout" -name '*.rs' -exec touch {} +
    find "$harness/src" -name '*.rs' -exec touch {} +
    rm -rf "$backup"
}
trap restore EXIT

echo "building the pristine-textlayout binary..."
git -C "$repo" checkout -- internal/core/textlayout || exit 1
find "$repo/internal/core/textlayout" -name '*.rs' -exec touch {} +
# The pristine tree has no paragraph counter, so the harness has to be stubbed down to what it
# offers. Anything else added to `i-slint-core` that `perf_test` calls needs a rule here, or the
# pristine build fails - which is loud, at least. The `cache.` forms have to go first: turning the
# bare call into `0` would otherwise leave `cache.0`.
sed -i -e 's/cache\.paragraph_shape_count()/0/g' \
       -e 's/cache\.reset_paragraph_shape_count();//g' \
       -e 's/^use i_slint_core::textlayout::sharedparley::paragraph_shape_count;$//' \
       -e 's/paragraph_shape_count()/0/g' \
       "$harness"/src/*.rs
(cd "$harness" && cargo build --profile rdebug --bin perf_test -q) || exit 1
cp "$bin" "$base_bin" || exit 1

restore
trap - EXIT
echo "building the session binary..."
(cd "$harness" && cargo build --profile rdebug --bin perf_test -q) || exit 1

# show+layout | later frames | allocations/frame | resident | window
measure() {
    "$1" $2 --frames "$FRAMES" 2>/dev/null | awk '
        /^window /         { w = $2" "$3" "$4" "$5" "$6 }
        /^show \+ layout/  { startup = $4 }
        /^resident memory/ { rss = $3" "$4 }
        /^later frames/    { gsub("[(,]", "", $7); time = $3; allocs = $7; shapes = $9 }
        END { printf "%s|%s|%s|%s|%s|%s\n", startup, time, allocs, shapes, rss, w }'
}

{
    echo
    echo "## Run $(date '+%Y-%m-%d %H:%M'), FRAMES=$FRAMES"
    echo
    echo "\`pristine\` is this tree with \`internal/core/textlayout\` reverted; everything else"
    echo "(candidates 06/08/09/10/11 and the local femtovg) is identical in both binaries."
    echo
    echo "Each binary is run $RUNS times per scene. Repeated rows of the *same* binary are the"
    echo "control: whatever they differ by is this machine's noise floor, and a pristine-vs-session"
    echo "difference smaller than that means nothing. Raise it with \`RUNS=4 just gpu-03\` for the"
    echo "time columns, which need it."
    echo
    echo "| scene | run | show + layout | later frames | allocations/frame | shapes/frame | resident |"
    echo "| --- | --- | --- | --- | --- | --- | --- |"
} >> "$report"

window_seen=""
for entry in "${scenes[@]}"; do
    label="${entry%%|*}"
    args="${entry#*|}"
    for which in "pristine|$base_bin" "session|$bin"; do
      for attempt in $(seq 1 "$RUNS"); do
        which_bin="${which#*|}"
        run="${which%%|*} $attempt"
        # shellcheck disable=SC2086
        IFS='|' read -r startup time allocs shapes rss window <<< "$(measure "$which_bin" "$args")"
        if [ -n "$window_seen" ] && [ -n "$window" ] && [ "$window" != "$window_seen" ]; then
            echo "WARNING: $label/$run ran at [$window], others at [$window_seen] - not comparable" >&2
        fi
        [ -z "$window_seen" ] && window_seen="$window"
        printf "| \`%s\` | %s | %s | %s | %s | %s | %s |\n" \
            "$label" "$run" "${startup:-n/a}" "${time:-n/a}" "${allocs:-n/a}" "${shapes:-n/a}" \
            "${rss:-n/a}" >> "$report"
        tail -1 "$report"
      done
    done
done

{
    echo
    echo "Read the repeated rows of the same binary before anything else."
    echo
    echo "**Never compare against a previous run of this file.** Allocation counts are exactly"
    echo "reproducible *within* one invocation and are not across invocations - the same session"
    echo "binary has produced 29 972 and 36 171 for \`rects 3000\` on different days. Only the"
    echo "pristine-vs-session rows of one table attribute anything."
    echo
    echo "\`show + layout\` covers everything up to the first rendered frame, which is where a"
    echo "scene that shapes a whole document spends its time; the per-frame timings start after"
    echo "it and miss all of it."
    echo
    echo "All runs measured at: $window_seen"
} >> "$report"

echo
echo "appended to: $report"
