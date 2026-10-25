#!/bin/bash
# usage: run.sh <godot-asan-editor-binary> [width height]
ulimit -c 0
cd "$(dirname "$0")"
ASAN_OPTIONS=detect_leaks=0 XDG_DATA_HOME="${XDG_DATA_HOME:-/tmp/xdg}" "$1" --headless --path project --script repro.gd -- "${2:--1}" "${3:-2}" 2>&1
echo "exit code: ${PIPESTATUS[0]}"
