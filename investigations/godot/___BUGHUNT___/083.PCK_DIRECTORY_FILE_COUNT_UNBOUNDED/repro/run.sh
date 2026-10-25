#!/bin/bash
# usage: run.sh <godot-asan-editor-binary>   (exit code 124 = still running after the 20 s limit, i.e. hung)
ulimit -c 0
cd "$(dirname "$0")"
ASAN_OPTIONS=detect_leaks=0 XDG_DATA_HOME="${XDG_DATA_HOME:-/tmp/xdg}" timeout 20 "$1" --headless --path project --script repro.gd 2>&1
echo "exit code: ${PIPESTATUS[0]}"
