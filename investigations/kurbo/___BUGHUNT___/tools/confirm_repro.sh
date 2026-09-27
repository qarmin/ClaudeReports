#!/usr/bin/env bash
# Run a repro binary N times and report how many times it actually reproduced the bug.
#
#   confirm_repro.sh <repro binary> [-- <args...>]
#
# One occurrence is not a confirmation (see CONFIRMATION.md section 1) - this script exists so
# that discipline is a single command instead of something to remember to do by hand every time.
#
# ADAPT: this assumes the repro follows repro_template.rs's exit-code convention (0 = did not
# reproduce, non-zero = reproduced). If the repro instead signals via a crash/abort (SIGABRT/
# SIGSEGV from a sanitizer), that already shows up as a non-zero exit code from bash's point of
# view, so no change needed - just make sure stderr is worth capturing for the write-up.
#
# Env: RUNS (default 10).
set -uo pipefail

if [ "$#" -lt 1 ]; then
    sed -n '2,8p' "$0" >&2
    exit 2
fi

bin="$1"; shift
[ "${1:-}" = "--" ] && shift
RUNS=${RUNS:-10}

[ -x "$bin" ] || { echo "not executable: $bin" >&2; exit 1; }

hits=0
for i in $(seq 1 "$RUNS"); do
    if "$bin" "$@" >/tmp/confirm_repro_out.$$ 2>&1; then
        status="did not reproduce"
    else
        status="REPRODUCED"
        hits=$((hits + 1))
    fi
    echo "run $i/$RUNS: $status"
done
rm -f /tmp/confirm_repro_out.$$

echo
echo "reproduced $hits/$RUNS runs"
if [ "$hits" -eq 0 ]; then
    echo "verdict: does not reproduce under this exact invocation - see CONFIRMATION.md section 3" \
         "before concluding the bug isn't real (layout/seed-dependent bugs can need pinning)."
elif [ "$hits" -eq "$RUNS" ]; then
    echo "verdict: fully deterministic reproduction ($hits/$RUNS) - safe to write Confirmed."
else
    echo "verdict: partial/flaky reproduction ($hits/$RUNS) - still Confirmed, but state this exact" \
         "ratio in RESULT.md rather than rounding up to deterministic (CONFIRMATION.md section 3)."
fi
