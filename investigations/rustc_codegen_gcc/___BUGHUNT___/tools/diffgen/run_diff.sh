#!/usr/bin/env bash
# Compile every generated case/*.rs with BOTH the plain (LLVM) backend and rustc_codegen_gcc,
# run both binaries, and report any divergence: a compile-time crash in the GCC backend on
# otherwise-valid code, a runtime crash (SIGSEGV/SIGABRT/etc) the LLVM build doesn't have, or a
# stdout/exit-code mismatch between the two (a real, reproduced behavioral difference - not a
# guess). Every finding here is directly minimal (one construct + one parameter per case file)
# and directly promotable to an IDEAS.md entry / a candidate's own repro/.
#
# Third verification track: Miri (see MIRI=0 below to disable). Miri interprets MIR directly - it
# never runs through either backend, so it's an independent third oracle, AND it's the one thing
# here that can tell "this case's own input is Undefined Behavior" apart from "the GCC backend
# computed the wrong answer for well-defined code" - a LLVM-vs-GCC mismatch on a case Miri flags
# as UB is not evidence of a codegen_gcc bug (anything goes under UB), so those get a distinct
# tag instead of being reported as a plain DIFF. Cheap in practice for cases this small (~0.3s
# each via a prebuilt Miri sysroot - run `cargo miri setup` once beforehand if that's missing).
# Every case runs in its own fresh temp Cargo project (never a single shared one) so this stays
# safe under MAX_PARALLEL - two concurrent cases never share a src/main.rs or target dir.
#
# Usage: run_diff.sh [opt-level] [max-parallel]
#   opt-level: rustc -C opt-level value, default 3 (release-like; issue #705 was release-only)
#   max-parallel: how many case files to compile/run concurrently, default 4 (kept modest so this
#                 doesn't starve a concurrently-running bug-hunt Workflow of CPU - see README.md)
#   MIRI=0 env var: skip the Miri track entirely (e.g. if `cargo miri setup` was never run here)
set -uo pipefail
ulimit -c 0   # a crashing case dumps a core file per run otherwise - never wanted here

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$HERE/../../.." && pwd)"
SYSROOT="$PROJECT_ROOT/build/build_sysroot/sysroot"
BACKEND="$PROJECT_ROOT/target/release/librustc_codegen_gcc.so"
OUT="$HERE/results"
BIN_DIR="$OUT/bin"
mkdir -p "$BIN_DIR"

OPT="${1:-3}"
MAX_PARALLEL="${2:-4}"
MIRI="${MIRI:-1}"

[ -f "$BACKEND" ] || { echo "GCC backend not found at $BACKEND - build it first (./y.sh build --release)"; exit 1; }
[ -d "$SYSROOT" ] || { echo "sysroot not found at $SYSROOT - build it first (./y.sh build --sysroot --release)"; exit 1; }

MIRI_TEMPLATE="$HERE/.miri_template"
if [ "$MIRI" = "1" ]; then
    if ! command -v cargo-miri >/dev/null 2>&1; then
        echo "MIRI=1 but cargo-miri isn't installed (rustup component add miri --toolchain \$(cat $PROJECT_ROOT/rust-toolchain | grep channel | cut -d'\"' -f2); cargo miri setup) - continuing with MIRI=0" >&2
        MIRI=0
    else
        mkdir -p "$MIRI_TEMPLATE/src"
        printf '[package]\nname = "diffgen_miri_case"\nversion = "0.1.0"\nedition = "2021"\n' > "$MIRI_TEMPLATE/Cargo.toml"
        cp "$PROJECT_ROOT/rust-toolchain" "$MIRI_TEMPLATE/rust-toolchain"
    fi
fi

REPORT="$OUT/report_O${OPT}_$(date +%Y%m%d_%H%M%S).txt"
: > "$REPORT"

# Run one case under Miri in a fresh, isolated temp project. Writes its verdict into $2 (a
# caller-provided, already-created directory) as plain files - NOT returned via a variable, since
# `$(run_miri ...)` would fork a subshell and any variable assignment inside it would be invisible
# to the caller (a real bug this had until it was caught by an empty-`miri_rc` smoke test - keep
# it this way, don't "simplify" back to command-substitution + global variables).
#   $2/tag        : "" (empty file) = ran clean, "UB" = real Undefined Behavior in the case's own
#                   code, "UNSUPPORTED" = Miri can't run this construct at all, "TIMEOUT"
#   $2/stdout, $2/rc : only written when tag is empty - Miri's own stdout/exit code
#   $2/ub_detail     : only written when tag is "UB" - a short excerpt of Miri's UB diagnostic
run_miri() {
    local f="$1" resdir="$2"
    local d; d="$(mktemp -d)"
    cp "$MIRI_TEMPLATE/Cargo.toml" "$MIRI_TEMPLATE/rust-toolchain" "$d/"
    mkdir -p "$d/src"
    cp "$f" "$d/src/main.rs"
    local stderr_file; stderr_file="$(mktemp)"
    local out rc
    # Miri defaults to overflow-checks=on regardless of profile (unlike a normal compiled binary,
    # where it follows debug-assertions' own opt-level-tied default) - without matching $OPT here,
    # every overflow-triggering case looks like a "MIRI DISAGREES" false positive at opt-level > 0.
    out="$(cd "$d" && RUSTFLAGS="-C opt-level=$OPT" timeout 30 cargo miri run 2>"$stderr_file")"
    rc=$?
    local err; err="$(cat "$stderr_file")"
    rm -rf "$d" "$stderr_file"
    if [ "$rc" = 124 ]; then
        echo "TIMEOUT" > "$resdir/tag"; return
    fi
    if echo "$err" | grep -q "error: Undefined Behavior"; then
        echo "UB" > "$resdir/tag"
        echo "$err" | grep -A4 "error: Undefined Behavior" | head -6 > "$resdir/ub_detail"
        return
    fi
    if echo "$err" | grep -qE "unsupported operation|unsupported Rust|unimplemented"; then
        echo "UNSUPPORTED" > "$resdir/tag"; return
    fi
    : > "$resdir/tag"
    printf '%s' "$out" > "$resdir/stdout"
    echo "$rc" > "$resdir/rc"
}

run_one() {
    local f="$1"
    local name; name="$(echo "$f" | tr '/' '_' | sed 's/\.rs$//')"
    local llvm_bin="$BIN_DIR/${name}_llvm"
    local gcc_bin="$BIN_DIR/${name}_gcc"
    local llvm_log; llvm_log="$(mktemp)"
    local gcc_log; gcc_log="$(mktemp)"

    if ! timeout 30 rustc -C "opt-level=$OPT" --edition 2021 "$f" -o "$llvm_bin" >"$llvm_log" 2>&1; then
        echo "COMPILE_FAIL_LLVM $f opt=$OPT (unexpected - plain rustc rejected a case that should always compile; harness bug, not a codegen_gcc finding)" >> "$REPORT"
        tail -5 "$llvm_log" | sed 's/^/    /' >> "$REPORT"
        rm -f "$llvm_log" "$gcc_log"
        return
    fi

    if ! timeout 60 rustc -C "opt-level=$OPT" --edition 2021 --sysroot "$SYSROOT" -Zcodegen-backend="$BACKEND" "$f" -o "$gcc_bin" >"$gcc_log" 2>&1; then
        echo "COMPILE_CRASH_GCC $f opt=$OPT (GCC backend failed/crashed compiling code plain rustc accepted)" >> "$REPORT"
        tail -8 "$gcc_log" | sed 's/^/    /' >> "$REPORT"
        rm -f "$llvm_log" "$gcc_log" "$llvm_bin"
        return
    fi

    local llvm_out gcc_out llvm_rc gcc_rc
    llvm_out="$(timeout 10 "$llvm_bin" 2>&1)"; llvm_rc=$?
    gcc_out="$(timeout 10 "$gcc_bin" 2>&1)"; gcc_rc=$?

    # Third oracle: Miri, run once per case regardless of $OPT (it interprets MIR directly, no
    # meaningful opt-level distinction). Only ever used to (a) mark a LLVM-vs-GCC divergence as
    # "not necessarily a codegen_gcc bug" when Miri says the case's OWN code is UB, and (b) flag
    # the rare case where Miri disagrees with LLVM+GCC even though they agree with each other.
    local miri_tag="" miri_note="" miri_stdout="" miri_rc="" miri_ub_detail=""
    if [ "$MIRI" = "1" ]; then
        local miri_resdir; miri_resdir="$(mktemp -d)"
        run_miri "$f" "$miri_resdir"
        miri_tag="$(cat "$miri_resdir/tag" 2>/dev/null)"
        if [ "$miri_tag" = "UB" ]; then
            miri_ub_detail="$(cat "$miri_resdir/ub_detail" 2>/dev/null)"
        elif [ "$miri_tag" = "" ]; then
            miri_stdout="$(cat "$miri_resdir/stdout" 2>/dev/null)"
            miri_rc="$(cat "$miri_resdir/rc" 2>/dev/null)"
            if [ "$miri_stdout" != "$llvm_out" ] || [ "$miri_rc" != "$llvm_rc" ]; then
                miri_note=" [MIRI DISAGREES WITH LLVM TOO]"
            fi
        fi
        rm -rf "$miri_resdir"
    fi

    if [ "$gcc_rc" -ge 128 ] && [ "$llvm_rc" -lt 128 ]; then
        if [ "$miri_tag" = "UB" ]; then
            echo "CRASH_BUT_MIRI_UB $f opt=$OPT (gcc exit=$gcc_rc signal=$((gcc_rc - 128)), llvm exit=$llvm_rc - Miri says this case's OWN code is Undefined Behavior, so this is not necessarily a codegen_gcc bug - fix the case, don't file it as a candidate as-is)" >> "$REPORT"
            echo "$miri_ub_detail" | sed 's/^/    MIRI-UB: /' >> "$REPORT"
        else
            echo "RUNTIME_CRASH_GCC $f opt=$OPT (gcc exit=$gcc_rc signal=$((gcc_rc - 128)), llvm exit=$llvm_rc)${miri_note}" >> "$REPORT"
        fi
    elif [ "$llvm_out" != "$gcc_out" ] || [ "$llvm_rc" != "$gcc_rc" ]; then
        if [ "$miri_tag" = "UB" ]; then
            echo "DIFF_BUT_MIRI_UB $f opt=$OPT (llvm_rc=$llvm_rc gcc_rc=$gcc_rc - Miri says this case's OWN code is Undefined Behavior, so this mismatch is not necessarily a codegen_gcc bug - fix the case, don't file it as a candidate as-is)" >> "$REPORT"
            echo "$miri_ub_detail" | sed 's/^/    MIRI-UB: /' >> "$REPORT"
        else
            echo "DIFF $f opt=$OPT (llvm_rc=$llvm_rc gcc_rc=$gcc_rc)${miri_note}" >> "$REPORT"
        fi
        diff <(echo "$llvm_out") <(echo "$gcc_out") | head -12 | sed 's/^/    /' >> "$REPORT"
    elif [ "$MIRI" = "1" ] && [ "$miri_tag" = "" ] && { [ "$miri_stdout" != "$llvm_out" ] || [ "$miri_rc" != "$llvm_rc" ]; }; then
        # LLVM and GCC agree with each other but Miri disagrees with both - rare, worth surfacing
        # on its own even with no LLVM-vs-GCC divergence (could be a harness/test-case bug, or an
        # LLVM optimization exploiting UB that Miri catches and GCC happens to reproduce too).
        echo "MIRI_DISAGREES_WITH_BOTH $f opt=$OPT (llvm_rc=$llvm_rc gcc_rc=$gcc_rc miri_rc=$miri_rc)" >> "$REPORT"
        diff <(echo "$llvm_out") <(echo "$miri_stdout") | head -12 | sed 's/^/    MIRI-DIFF: /' >> "$REPORT"
    fi
    rm -f "$llvm_log" "$gcc_log" "$llvm_bin" "$gcc_bin"
}

jobs_running=0
for f in $(find "$HERE/cases" -name '*.rs' | sort); do
    run_one "$f" &
    jobs_running=$((jobs_running + 1))
    if [ "$jobs_running" -ge "$MAX_PARALLEL" ]; then
        wait -n
        jobs_running=$((jobs_running - 1))
    fi
done
wait

echo "=== done, opt-level=$OPT ==="
if [ -s "$REPORT" ]; then
    echo "findings written to $REPORT:"
    grep -E "^(COMPILE_CRASH_GCC|RUNTIME_CRASH_GCC|DIFF|DIFF_BUT_MIRI_UB|CRASH_BUT_MIRI_UB|COMPILE_FAIL_LLVM|MIRI_DISAGREES_WITH_BOTH)" "$REPORT" | sed 's/^/  /'
else
    echo "no divergence found at opt-level=$OPT (report is empty: $REPORT)"
fi
