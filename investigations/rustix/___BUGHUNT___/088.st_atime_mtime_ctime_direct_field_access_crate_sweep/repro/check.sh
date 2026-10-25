#!/usr/bin/env bash
# Standalone reproduction for candidate 088.
#
# This is the same "flat st_atime/st_mtime/st_ctime field name assumed, wasi's
# vendored `libc::stat` nests them inside timespec sub-structs instead" bug
# class candidate 081 confirmed in rustix's own `#[cfg(test)]` unit-test
# self-check (src/fs/constants.rs) - this candidate found it recurs
# independently in two of rustix's *integration* test files
# (tests/fs/negative_timestamp.rs, tests/fs/y2038.rs), which read
# `stat.st_mtime`/`stat.st_atime` directly and were never given the
# `#[cfg(not(target_os = "wasi"))]` guard their sibling test files
# (tests/fs/futimens.rs, tests/fs/utimensat.rs) already carry.
#
# The real, end-to-end reproduction is simply: compile rustix's real,
# unmodified test suite (not just --lib) for the real wasm32-wasip1 target,
# using the project's own real vendored `libc` dependency (no mocked/
# hand-authored struct definitions anywhere in this script) and a real
# installed `wasm32-wasip1` Rust target.
#
# Usage: check.sh <path-to-a-rustix-checkout>
set -u

RUSTIX_DIR="${1:?usage: check.sh <path-to-rustix-checkout>}"

rustup target add wasm32-wasip1 >/dev/null 2>&1

cd "$RUSTIX_DIR" || exit 1

OUT=$(cargo test --no-run --target wasm32-wasip1 --features "std,fs" 2>&1)
STATUS=$?

echo "$OUT"
echo
echo "=== exit status: $STATUS ==="

# A hit only counts if the field-name error is actually attributed (on the
# immediately following "-->" line) to one of this candidate's two files -
# this keeps the check specific to *this* candidate's bug instead of also
# tripping on the separate, already-recorded candidate 081 instance in
# src/fs/constants.rs (still present in a pristine clone, since this
# workspace never leaves a fix applied to the real tree) or the other,
# unrelated pre-existing wasi gaps (readlinkat.rs/dup.rs/file.rs/ugid/dup2).
if echo "$OUT" | grep -A1 'no field `st_\(a\|m\|c\)time\(_nsec\)\?` on type `libc::stat`\|struct `libc::stat` does not have a field named `st_\(a\|m\|c\)time\(_nsec\)\?`' \
    | grep -q 'tests/fs/negative_timestamp\.rs\|tests/fs/y2038\.rs'; then
    echo "=== REPRO: tests/fs/negative_timestamp.rs or tests/fs/y2038.rs fails to compile for wasm32-wasip1 on the st_atime/st_mtime/st_ctime field names ==="
else
    echo "=== the st_atime/st_mtime/st_ctime bug in tests/fs/negative_timestamp.rs or tests/fs/y2038.rs is NOT present (fixed) ==="
fi
# Always exit 0: this script's job is to report which state was observed,
# not to fail `just`'s own control flow (both "before" and "after" are
# expected, correct outcomes for their respective step).
exit 0
