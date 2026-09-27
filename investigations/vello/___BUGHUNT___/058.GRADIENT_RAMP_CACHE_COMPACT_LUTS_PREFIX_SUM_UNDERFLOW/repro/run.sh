#!/usr/bin/env bash
# Runs the randomized stress harness against a given checkout of the repo.
#
# `GradientRampCache`/`compact_luts` are `pub(crate)`/private to `vello_hybrid`, so there is no
# external public API surface to drive them from an out-of-crate repro binary. The harness is
# therefore injected as `#[test]`s directly into `gradient_cache.rs`'s own `mod tests` (via
# `harness.patch`, applied by the caller before this script runs) and executed with
# `cargo test`, which is the closest thing to a "standalone runnable repro" this module's own
# visibility allows - see RESULT.md for why this stays a debug-assertions unit-test harness
# rather than an external fuzz binary.
#
# Usage: run.sh <path-to-repo-checkout>
set -euo pipefail
repo_dir="$1"

cd "$repo_dir"
cargo test -p vello_hybrid --lib gradient_cache -- --nocapture
