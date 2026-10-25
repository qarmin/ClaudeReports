#!/usr/bin/env bash
# mkoutput.sh <candidate_dir> <before_build> <after_build> [runs]: write output.txt with N-run exit codes + one sample output, before and after.
c=$1; b=$2; a=$3; n=${4:-10}
{
echo "### BEFORE (HEAD, unpatched build), $n runs: exit codes"
for i in $(seq "$n"); do bash "$c/repro/run.sh" "${SRC_BEFORE:-x}" "$b" >/dev/null 2>&1; echo -n "$? "; done; echo
echo "### one run"; bash "$c/repro/run.sh" "${SRC_BEFORE:-x}" "$b" 2>&1 | head -${5:-14}
echo; echo "### AFTER (patch.diff applied), $n runs: exit codes"
for i in $(seq "$n"); do bash "$c/repro/run.sh" "${SRC_AFTER:-x}" "$a" >/dev/null 2>&1; echo -n "$? "; done; echo
echo "### one run"; bash "$c/repro/run.sh" "${SRC_AFTER:-x}" "$a" 2>&1 | head -${5:-14}
} > "$c/output.txt" 2>&1
cat "$c/output.txt" | cut -c1-150
