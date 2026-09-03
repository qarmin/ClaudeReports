#!/usr/bin/env bash
# Differential unwinding/panic-codegen check for candidate 016.
#
# Builds five small Rust programs (in this folder's src/) once with this repository's own GCC
# backend (against $REPO_DIR, a rustc_codegen_gcc checkout with `./y.sh build --sysroot --release`
# already run once) and compares their real stdout/exit-code against a reference captured from the
# host's normal LLVM-backed rustc (baked into expected/*.txt and the exit-code checks below, since
# a machine running this may not have a matching LLVM rustc handy for a live comparison).
#
# Usage: run.sh <REPO_DIR>
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(cd "${1:?usage: run.sh <rustc_codegen_gcc checkout>}" && pwd)"

if [ ! -f "$REPO_DIR/y.sh" ] || [ ! -d "$REPO_DIR/src" ]; then
    echo "error: $REPO_DIR does not look like a rustc_codegen_gcc checkout" >&2
    exit 1
fi

cd "$REPO_DIR"
rm -rf /tmp/candidate016_target
CHANNEL=release ./y.sh cargo build --manifest-path "$SCRIPT_DIR/Cargo.toml" --bins \
    --target-dir /tmp/candidate016_target
BIN=/tmp/candidate016_target/debug

status=0
ulimit -c 0

check_stdout() {
    name="$1"
    echo "=== $name (stdout diff vs LLVM reference) ==="
    if diff "$SCRIPT_DIR/expected/$name.txt" <("$BIN/$name"); then
        echo "MATCH: $name"
    else
        echo "MISMATCH: $name"
        status=1
    fi
    echo
}

check_abort() {
    name="$1"
    echo "=== $name (expected: process aborts, exit code 134 = SIGABRT) ==="
    set +e
    "$BIN/$name" >/tmp/candidate016_$name.out 2>&1
    code=$?
    set -e
    tail -3 "/tmp/candidate016_$name.out"
    if [ "$code" -eq 134 ]; then
        echo "MATCH: $name exited 134 (SIGABRT), as the real LLVM-compiled reference does"
    else
        echo "MISMATCH: $name exited $code, expected 134"
        status=1
    fi
    echo
}

check_stdout unwind_drop
check_stdout multi_unwind
check_stdout thread_unwind
check_abort double_panic
check_abort extern_c_unwind

exit $status
