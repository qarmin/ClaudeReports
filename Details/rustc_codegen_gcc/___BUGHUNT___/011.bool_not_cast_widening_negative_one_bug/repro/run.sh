#!/usr/bin/env bash
# Runs the candidate-011 investigation against a checkout of rustc_codegen_gcc at $REPO_DIR
# (defaults to the current directory if it looks like the project root, otherwise pass it as $1).
#
# Requires the project's own build already done once (`./y.sh prepare && ./y.sh build --sysroot
# --release`) - see ___BUGHUNT___/README.md's toolchain table. This script builds the backend .so
# in both `dev` and `release` cargo profiles, then compiles+runs issue691.rs (the verbatim
# issue #691 repro) through each, and diffs the two outputs.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CARGO_MANIFEST="$SCRIPT_DIR/Cargo.toml"

REPO_DIR="$(cd "${1:-$(pwd)}" && pwd)"
cd "$REPO_DIR"

if [ ! -f "y.sh" ] || [ ! -d "src" ]; then
    echo "error: $REPO_DIR does not look like a rustc_codegen_gcc checkout" >&2
    exit 1
fi

echo "=== 1. rustc_codegen_gcc backend built in \`dev\` profile (\"GCC debug\"), issue691.rs compiled unoptimized (cargo dev profile) ==="
./y.sh build >/dev/null
rm -rf /tmp/issue691_dev
CHANNEL=debug ./y.sh cargo build --manifest-path "$CARGO_MANIFEST" --target-dir /tmp/issue691_dev
/tmp/issue691_dev/debug/issue691

echo
echo "=== 2. rustc_codegen_gcc backend built in \`release\` profile (\"GCC release\"), issue691.rs compiled optimized (cargo release profile) ==="
./y.sh build --release >/dev/null
rm -rf /tmp/issue691_release
CHANNEL=release ./y.sh cargo build --release --manifest-path "$CARGO_MANIFEST" --target-dir /tmp/issue691_release
/tmp/issue691_release/release/issue691

echo
echo "=== 3. Diff: dev-profile output vs. release-profile output ==="
if diff <(/tmp/issue691_dev/debug/issue691) <(/tmp/issue691_release/release/issue691); then
    echo "MATCH: dev and release builds print the same value for \`!arg as usize\` - issue #691 does not reproduce on this checkout."
else
    echo "MISMATCH: see diff above - this would reproduce issue #691."
fi
