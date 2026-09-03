#!/usr/bin/env bash
# Sweep of cg_clif's `build_isa` target-spec-cpu path (src/lib.rs, the `None` arm of the
# `sess.opts.cg.target_cpu` match - i.e. NO `-Ctarget-cpu` on the command line).
#
# For every x86 CPU name LLVM accepts (plus a few obviously bogus ones), it writes a custom
# x86_64 bare-metal target spec whose "cpu" field is that name, compiles a tiny `#![no_core]`
# crate with plain rustc first as a go/no-go filter, then with cg_clif, and classifies cg_clif's
# behaviour as CLIF_OK / ERROR (clean diagnostic) / ICE.
#
#   CLIF_ROOT=<a built rustc_codegen_cranelift checkout> bash sweep_target_spec_cpu.sh > sweep.txt
#
# The point of going through a custom target spec rather than `-Ctarget-cpu` is that the two are
# different code paths in build_isa: `-Ctarget-cpu=<bad>` already produces a clean fatal
# diagnostic, while the target-spec value is `unwrap()`ed.
set -uo pipefail
ulimit -c 0
# Stop rustc from dropping a rustc-ice-*.txt in the cwd for every ICE the sweep provokes.
export RUSTC_ICE=0

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLIF_ROOT="${CLIF_ROOT:-$HERE/../clone}"
RUSTC_CLIF="$CLIF_ROOT/dist/rustc-clif"
[ -x "$RUSTC_CLIF" ] || { echo "MISSING: $RUSTC_CLIF (run ./y.sh build in $CLIF_ROOT)" >&2; exit 2; }

O=$(mktemp -d)
trap 'rm -rf "$O"' EXIT

cat >"$O/nocore.rs" <<'EOF'
#![feature(no_core, lang_items)]
#![no_core]
#![no_main]
#![allow(internal_features)]

#[lang = "pointee_sized"]
pub trait PointeeSized {}
#[lang = "meta_sized"]
pub trait MetaSized: PointeeSized {}
#[lang = "sized"]
pub trait Sized: MetaSized {}
#[lang = "copy"]
pub trait Copy {}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
EOF

# `<omitted>` means the spec has no "cpu" key at all, which is how every custom target spec that
# doesn't bother to set one looks - rustc then defaults it to "generic".
cpus=$( { echo "<omitted>"
          rustc --print target-cpus --target x86_64-unknown-linux-gnu 2>/dev/null \
              | tail -n +2 | awk '{print $1}' | grep -v '^native$'
          printf 'no-such-cpu-12345\n\n' ; } )

while IFS= read -r cpu; do
    if [ "$cpu" = "<omitted>" ]; then
        cpu_field=""
    else
        cpu_field="\"cpu\": \"$cpu\","
    fi
    cat >"$O/t.json" <<EOF
{
  "arch": "x86_64",
  $cpu_field
  "crt-objects-fallback": "false",
  "data-layout": "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128",
  "linker": "rust-lld",
  "linker-flavor": "gnu-lld",
  "llvm-target": "x86_64-unknown-none-elf",
  "max-atomic-width": 64,
  "metadata": { "description": "sweep", "host_tools": false, "std": false, "tier": 3 },
  "panic-strategy": "abort",
  "relro-level": "full",
  "target-pointer-width": 64
}
EOF
    rustc -Zunstable-options --edition 2021 --crate-type lib --target "$O/t.json" \
        --emit=obj -o "$O/l.o" "$O/nocore.rs" >/dev/null 2>&1 \
        || { echo "${cpu:-<empty>}|LLVM_FAIL|"; continue; }

    out=$("$RUSTC_CLIF" -Zunstable-options --edition 2021 --crate-type lib --target "$O/t.json" \
        --emit=obj -o "$O/c.o" "$O/nocore.rs" 2>&1)
    rc=$?
    if [ $rc -eq 0 ]; then
        echo "${cpu:-<empty>}|CLIF_OK|"
    elif grep -q "the compiler unexpectedly panicked" <<<"$out"; then
        echo "${cpu:-<empty>}|ICE|$(grep -m1 -A1 'panicked at' <<<"$out" | tr '\n' ' ' | head -c 140)"
    else
        echo "${cpu:-<empty>}|ERROR|$(grep -m1 '^error' <<<"$out" | head -c 140)"
    fi
done <<<"$cpus"
