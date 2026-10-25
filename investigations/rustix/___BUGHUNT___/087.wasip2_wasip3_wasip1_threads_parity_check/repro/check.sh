#!/usr/bin/env bash
# Standalone reproduction for candidate 087.
#
# Follow-up to candidate 081, which found rustix's own #[cfg(test)] Stat-layout
# self-check (src/fs/constants.rs) does not compile for wasm32-wasip1, and
# explicitly left unchecked whether the sibling wasi targets - wasip2, wasip1-threads,
# and wasip3 - hit the identical bug or something additionally different.
#
# This bug lives entirely inside rustix's own #[cfg(test)] self-check code, which is
# unreachable through any public API - there is no "caller" to simulate. The real,
# end-to-end reproduction is simply: compile rustix's real, unmodified lib tests for
# each real wasi target, using the project's own real vendored `libc` dependency (no
# mocked/hand-authored struct definitions anywhere in this script).
#
# wasm32-wasip1/wasip2/wasip1-threads have a prebuilt rust-std (installed here via
# plain `rustup target add`, stable toolchain). wasm32-wasip3 has no prebuilt
# rust-std at all yet, so it is instead cross-interpreted^H^H^H^H^H^H^H^H^H^H^H^H
# cross-*compiled* via `cargo +nightly -Zbuild-std` (requires the nightly toolchain's
# own `rust-src` component, installed here too) - this is a real compile against the
# real target, not an emulation/interpretation shortcut.
#
# Usage: check.sh <path-to-a-rustix-checkout>
set -u

RUSTIX_DIR="${1:?usage: check.sh <path-to-rustix-checkout>}"
cd "$RUSTIX_DIR" || exit 1

STAT_FIELD_RE="no field \`st_(a|m|c)time(_nsec)?\` on type \`libc::stat\`|struct \`libc::stat\` does not have a field named \`st_(a|m|c)time(_nsec)?\`"

rustup target add wasm32-wasip1 wasm32-wasip2 wasm32-wasip1-threads >/dev/null 2>&1
rustup component add rust-src --toolchain nightly >/dev/null 2>&1

declare -A RESULTS

check_stable_target() {
    local target="$1"
    echo "=== $target (stable, prebuilt rust-std) ==="
    local out
    out=$(cargo test --no-run --target "$target" --features "std,fs" --lib 2>&1)
    local status=$?
    echo "$out" | grep '^error'
    echo "exit status: $status"
    if echo "$out" | grep -qE "$STAT_FIELD_RE"; then
        echo "=== $target: st_atime/st_mtime/st_ctime field-name bug IS present ==="
        RESULTS[$target]="present"
    else
        echo "=== $target: st_atime/st_mtime/st_ctime field-name bug NOT present (fixed) ==="
        RESULTS[$target]="absent"
    fi
    echo
}

check_wasip3() {
    local target="wasm32-wasip3"
    echo "=== $target (nightly, -Zbuild-std, no prebuilt rust-std exists) ==="
    local out
    out=$(cargo +nightly test --no-run --target "$target" \
        -Zbuild-std=std,core,alloc,panic_abort --features "std,fs" --lib 2>&1)
    local status=$?
    echo "$out" | grep '^error'
    echo "exit status: $status"
    if echo "$out" | grep -qE "$STAT_FIELD_RE"; then
        echo "=== $target: st_atime/st_mtime/st_ctime field-name bug IS present ==="
        RESULTS[$target]="present"
    else
        echo "=== $target: st_atime/st_mtime/st_ctime field-name bug NOT present (fixed) ==="
        RESULTS[$target]="absent"
    fi
    echo
}

check_stable_target wasm32-wasip1
check_stable_target wasm32-wasip2
check_stable_target wasm32-wasip1-threads
check_wasip3

echo "=== summary ==="
for t in wasm32-wasip1 wasm32-wasip2 wasm32-wasip1-threads wasm32-wasip3; do
    echo "$t: ${RESULTS[$t]:-unknown}"
done

# Always exit 0: this script's job is to report which state was observed on each
# target, not to fail `just`'s own control flow (both "before" and "after" are
# expected, correct outcomes for their respective step).
exit 0
