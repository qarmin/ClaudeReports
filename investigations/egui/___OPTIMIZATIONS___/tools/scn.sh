#!/usr/bin/env bash
# usage: scn.sh <binary> [filter] - prints "name midpoint_us" for each scene bench of the scenes harness
B="$1"; F="${2:-scene_}"
taskset -c 4 "$B" --bench "$F" --warm-up-time 1 --measurement-time 3 --noplot 2>&1 | awk '/^scene_/{n=$1} /time:/{for(k=1;k<=NF;k++)if($k=="time:")break; v=$(k+3);u=$(k+4); if(u=="ms")v*=1000; if(u=="ns")v/=1000; if($1 ~ /^scene_/)n=$1; printf "%-38s %.1f\n", n, v}'
