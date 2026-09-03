#!/usr/bin/env bash
# aarch64 LLVM-vs-cg_clif differential runner for a single-file `#![no_std] #![no_main]` program.
#
# Written for candidate 008 (llvm.aarch64.neon.tbl1). The x86_64 diffcheck.sh cannot test the
# ~1240 lines of src/intrinsics/llvm_aarch64.rs at all, and this machine has no aarch64 rust-std
# and no aarch64 C toolchain, so this script builds everything from what *is* available:
#
#   * `core` is compiled straight from rust-src, once per backend, and cached next to this script
#     (in $WORKDIR, default ./aarch64_work);
#   * the test program must be `#![no_std] #![no_main]`, define its own `_start`, and print with a
#     raw `svc #0` write(2) - see repro/tbl1_sweep.rs in the 008 candidate folder for a template
#     (it also has to define memcpy/memset/memcmp/bcmp/memmove itself);
#   * rust-lld links it as a static PIE and qemu-aarch64 runs it.
#
# PREREQUISITE for the cg_clif side: unpatched cg_clif cannot compile `core` for aarch64 at all -
# `atomic_load::<u128>`/`atomic_store::<u128>` are monomorphised for aarch64 (target_has_atomic
# = "128") and src/intrinsics/mod.rs turns those into a `span_fatal`. The project's own sysroot
# build dodges this with patches/0027-stdlib-128bit-atomic-operations.patch, which does not apply
# to a hand-built core. Apply 008.aarch64_neon_tbl1_oob_load/repro/128bit_atomics_measurement.diff
# to the checkout named by CLIF_ROOT (it flips those two `span_fatal`s to the existing
# codegen_panic_nounwind path) and ./y.sh build. That is a measurement aid, not a fix.
#
# Usage: bash aarch64_diffcheck.sh <prog.rs>
#   CLIF_ROOT=<dir>   cg_clif checkout whose dist/rustc-clif is used (default: repo root)
#   WORKDIR=<dir>     where cached core rlibs and binaries go (default: ./aarch64_work)
#   TOOLCHAIN=<dir>   rustup toolchain dir (must match rust-toolchain.toml and have rust-src)
# Exit 0 = identical output, 1 = they disagree / something failed to build or run.
set -uo pipefail

PROG="${1:?usage: aarch64_diffcheck.sh <prog.rs>}"
PROG="$(cd "$(dirname "$PROG")" && pwd)/$(basename "$PROG")"
NAME="$(basename "$PROG" .rs)"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CLIF_ROOT="${CLIF_ROOT:-$REPO_ROOT}"
WORKDIR="${WORKDIR:-$PWD/aarch64_work}"
TOOLCHAIN="${TOOLCHAIN:-$HOME/.rustup/toolchains/nightly-2026-08-24-x86_64-unknown-linux-gnu}"

LLD="$TOOLCHAIN/lib/rustlib/x86_64-unknown-linux-gnu/bin/rust-lld"
CORE_SRC="$TOOLCHAIN/lib/rustlib/src/rust/library/core/src/lib.rs"
RUSTC_CLIF="$CLIF_ROOT/dist/rustc-clif"
TARGET=aarch64-unknown-linux-gnu
CLIF_TAG="$(echo "$CLIF_ROOT" | tr -c 'a-zA-Z0-9' '_')"

for f in "$LLD" "$CORE_SRC" "$RUSTC_CLIF"; do
    [ -e "$f" ] || { echo "missing: $f" >&2; exit 1; }
done
command -v qemu-aarch64 >/dev/null || { echo "missing qemu-aarch64" >&2; exit 1; }

mkdir -p "$WORKDIR/llvm" "$WORKDIR/clif$CLIF_TAG"
cd "$WORKDIR" || exit 1

COMMON=(--target "$TARGET" --edition 2024 -Copt-level=0 -Awarnings)

# rustc insists a `compiler_builtins` crate exists. The real one isn't buildable here, and the
# handful of 128-bit helpers cg_clif's core references are never called by these programs, hence
# --unresolved-symbols=ignore-all below.
if [ ! -f libcompiler_builtins.rlib ]; then
    cat > cb_stub.rs <<'EOF'
#![no_std]
#![no_core]
#![feature(no_core, compiler_builtins)]
#![compiler_builtins]
EOF
    RUSTC_BOOTSTRAP=1 rustc "${COMMON[@]}" --crate-type lib \
        --crate-name compiler_builtins --out-dir . cb_stub.rs || exit 1
fi

if [ ! -f llvm/libcore.rlib ]; then
    echo "[build] core (LLVM)"
    RUSTC_BOOTSTRAP=1 rustc "${COMMON[@]}" --crate-type lib --crate-name core \
        --out-dir llvm "$CORE_SRC" || exit 1
fi
if [ ! -f "clif$CLIF_TAG/libcore.rlib" ]; then
    echo "[build] core (cg_clif @ $CLIF_ROOT)"
    RUSTC_BOOTSTRAP=1 "$RUSTC_CLIF" "${COMMON[@]}" --crate-type lib --crate-name core \
        --out-dir "clif$CLIF_TAG" "$CORE_SRC" || exit 1
fi

LINK=(-Cpanic=abort -Zunstable-options -Clinker-flavor=gnu-lld "-Clinker=$LLD"
      -Clink-arg=-static -Clink-arg=-nostdlib -Clink-arg=--unresolved-symbols=ignore-all
      --extern compiler_builtins=libcompiler_builtins.rlib)

RUSTC_BOOTSTRAP=1 rustc "${COMMON[@]}" "${LINK[@]}" \
    --extern core=llvm/libcore.rlib -o "${NAME}_llvm" "$PROG" || exit 1
RUSTC_BOOTSTRAP=1 "$RUSTC_CLIF" "${COMMON[@]}" "${LINK[@]}" \
    --extern "core=clif$CLIF_TAG/libcore.rlib" -o "${NAME}_clif" "$PROG" || exit 1

qemu-aarch64 "./${NAME}_llvm" > "${NAME}_llvm.out" 2>&1; echo "llvm exit=$?"
qemu-aarch64 "./${NAME}_clif" > "${NAME}_clif.out" 2>&1; echo "clif exit=$?"

if diff -u "${NAME}_llvm.out" "${NAME}_clif.out" > "${NAME}.diff"; then
    echo "IDENTICAL ($(wc -l < "${NAME}_llvm.out") lines)"
    exit 0
else
    echo "DIFFER:"; head -40 "${NAME}.diff"; exit 1
fi
