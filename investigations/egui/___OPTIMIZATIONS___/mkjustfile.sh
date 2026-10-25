#!/usr/bin/env bash
# usage: mkjustfile.sh <candidate-dir> <criterion-bench-filter> <package> <bench-name>
set -euo pipefail
d="$1"; filter="$2"; pkg="$3"; bench="$4"
cat > "$d/justfile" <<EOF
# Self-contained: clones egui at base_commit into ./clone, benches baseline then patched with criterion.
repo_url := "https://github.com/emilk/egui.git"
base_commit := "86099aca1276fe786921992c8d5a68914ee79bec"

test: clone bench-baseline apply bench-patched

clone:
    rm -rf clone
    git clone {{repo_url}} clone
    cd clone && git checkout {{base_commit}}

apply:
    cd clone && git apply "{{justfile_directory()}}/patch.diff"

bench-baseline:
    @echo "=== baseline (before patch) ==="
    cd clone && cargo bench -p $pkg --bench $bench -- --save-baseline base '$filter'

bench-patched:
    @echo "=== patched (after patch), compared against baseline ==="
    cd clone && cargo bench -p $pkg --bench $bench -- --baseline base '$filter'

clean:
    rm -rf clone
EOF
