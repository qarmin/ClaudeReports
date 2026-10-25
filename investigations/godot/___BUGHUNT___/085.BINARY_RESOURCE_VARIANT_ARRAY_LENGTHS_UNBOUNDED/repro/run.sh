#!/bin/bash
# usage: run.sh <godot-asan-editor-binary>   (allocator_may_return_null=1 makes the huge allocation fail like malloc does on a normal machine instead of ASan aborting)
ulimit -c 0
cd "$(dirname "$0")"
ASAN_OPTIONS=detect_leaks=0:allocator_may_return_null=1 XDG_DATA_HOME="${XDG_DATA_HOME:-/tmp/xdg}" timeout 60 "$1" --headless --path project --script repro.gd 2>&1
echo "exit code: ${PIPESTATUS[0]}"
