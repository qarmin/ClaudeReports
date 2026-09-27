#!/usr/bin/env bash
# Differential check: build `core` (via -Zbuild-std) for a given target with plain rustc/LLVM and
# with cg_clif, and report whether each side succeeded. Used for candidate 016 (128bit atomics
# hard-error blocking every target whose `target_has_atomic="128"` is set).
#
# usage: buildstd_core_check.sh <target-triple> [more targets...]
#   CLIF_ROOT=<worktree>   which cg_clif checkout's dist/ to use (default: the main tree)
#   TOOLCHAIN=<name>       rustup toolchain for the LLVM baseline (default: read rust-toolchain.toml)
set -uo pipefail

ROOT=/home/rafal/Projekty/Obce/rustc_codegen_cranelift
CLIF_ROOT=${CLIF_ROOT:-$ROOT}
TOOLCHAIN=${TOOLCHAIN:-$(grep -oP 'channel = "\K[^"]+' "$ROOT/rust-toolchain.toml")}
WORK=${WORK:-$ROOT/tmp/buildstd_core_check}
LOGDIR=$WORK/logs

mkdir -p "$WORK/src" "$LOGDIR"
cat > "$WORK/Cargo.toml" <<'TOML'
[package]
name = "buildstd_core_probe"
version = "0.0.0"
edition = "2021"

[workspace]

[profile.dev]
opt-level = 0
TOML
cat > "$WORK/src/lib.rs" <<'RS'
#![no_std]
pub fn add(a: u32, b: u32) -> u32 { a + b }
RS

for target in "$@"; do
    for side in llvm clif; do
        td="$WORK/target_$side"
        rm -rf "$td"
        log="$LOGDIR/${target}.${side}.log"
        if [ "$side" = llvm ]; then
            ( cd "$WORK" && CARGO_TARGET_DIR="$td" rustup run "$TOOLCHAIN" \
                cargo build -Zbuild-std=core --target "$target" ) > "$log" 2>&1
        else
            ( cd "$WORK" && CARGO_TARGET_DIR="$td" "$CLIF_ROOT/dist/cargo-clif" \
                build -Zbuild-std=core --target "$target" ) > "$log" 2>&1
        fi
        rc=$?
        if [ $rc -eq 0 ]; then
            printf '%-32s %-5s OK\n' "$target" "$side"
        else
            printf '%-32s %-5s FAIL(%d) %s\n' "$target" "$side" "$rc" \
                "$(grep -m3 '^error' "$log" | head -c 300 | tr '\n' '|')"
        fi
    done
done
