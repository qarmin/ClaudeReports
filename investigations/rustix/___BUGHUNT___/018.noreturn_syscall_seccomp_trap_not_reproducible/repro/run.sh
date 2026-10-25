#!/usr/bin/env bash
# Real repro for GitHub issue #1433 - see this candidate's RESULT.md for the
# full explanation. Run from inside a rustix checkout at base_commit
# (287214b889865d8e1406a0ee71cc409b6f6191c8) - see ../justfile, which applies
# test_addition.diff then invokes this script.
#
# EXPECTED (safe) OUTCOME: `cargo test` reports the test as failed because
# the process was killed by signal 4 (SIGILL) - this is SUCCESS, it means
# rustix's `ud2` trap instruction fired and stopped execution before any UB
# could occur. The seccomp filter installed by the test makes the real
# SYS_exit_group syscall instruction return (as if a permissive seccomp-bpf
# filter had intercepted it) instead of actually terminating the process -
# exactly the scenario issue #1433 describes.
#
# The BUG (if it existed) would look like: the test reports
# "bughunt_018_noreturn_seccomp_repro ... panicked at ... BUG (issue #1433):
# exit_group() returned to safe Rust code" - i.e. an ordinary Rust panic
# instead of the process dying via a hardware trap.
set -uo pipefail
cargo test --lib --features "std,all-apis" bughunt_018_noreturn_seccomp_repro -- --test-threads=1 --nocapture
echo "cargo test exit code: $?"
