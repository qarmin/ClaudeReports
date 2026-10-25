#!/usr/bin/env bash
set -uo pipefail
cd /home/rafal/Projekty/Obce/godot
export XDG_DATA_HOME="$PWD/tmp/attr/_xdg_data" XDG_CONFIG_HOME="$PWD/tmp/attr/_xdg_config"
N="${1:-30}"
for i in $(seq 1 $N); do
  tmp/bin_memstat --headless --editor --path ___OPTIMIZATIONS___/harness/empty_project --quit-after 1 > tmp/_bm_out.log 2> tmp/_bm_err.log &
  PID=$!
  PEAK=""
  while kill -0 $PID 2>/dev/null; do
    V=$(grep VmHWM /proc/$PID/status 2>/dev/null | awk '{print $2}' || true)
    [ -n "$V" ] && PEAK="$V"
    sleep 0.02
  done
  wait $PID; RC=$?
  MS=$(grep -h "^MEMSTAT" tmp/_bm_err.log | tail -1)
  echo "run $i peak_kB=$PEAK exit=$RC $MS"
done
