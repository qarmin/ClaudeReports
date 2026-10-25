#!/usr/bin/env bash
# usage: ab.sh <binA> <binB> <criterion-filter> [rounds=5] [core=4]
# Interleaved A/B runs of a criterion bench binary; prints per-round midpoint and mean ratio B/A (in the unit criterion prints).
set -euo pipefail
A="$1"; B="$2"; F="$3"; N="${4:-5}"; C="${5:-4}"
run() { taskset -c "$C" "$1" --bench "$F" --warm-up-time 1 --measurement-time 3 --noplot 2>&1 | awk '/time:/{for(k=1;k<=NF;k++)if($k=="time:")break; u=$(k+4); v=$(k+3); if(u=="ns")v/=1000; else if(u=="ms")v*=1000; else if(u=="s")v*=1e6; print v}'; }
for ((i=1;i<=N;i++)); do
  paste -d' ' <(run "$A") <(run "$B") | awk -v r=$i '{printf "round %d:", r; for(i=1;i<=NF;i+=2){printf " [A %.3f B %.3f ratio %.4f]", $i, $(i+1), $(i+1)/$i}; printf "\n"}'
done
