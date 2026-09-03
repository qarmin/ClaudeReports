# What is under test, and how findings are confirmed

Read this first if picking the work up cold - it explains the tooling, `SUMMARY.md` lists the
candidates investigated with it.

## Toolchain availability in this project

The library crate (`pic-scale-safe`, `src/`) has `#![forbid(unsafe_code)]` (see `src/lib.rs:30`) -
there is no `unsafe` anywhere in the shipping library. This changes tool priority vs. a typical
Rust project: miri's main value (catching UB from raw pointers/unsafe) mostly doesn't apply here.
It's still occasionally useful as a second opinion on arithmetic, but the primary tools for this
project are a debug-assertions build with deliberately constructed boundary values (for
panic/overflow candidates) and differential testing against hand-derived reference values or a
simpler code path (for logic/math-formula candidates). The `app/` crate (a demo/benchmark
comparison binary, not shipping library code) is out of scope for this hunt, same exclusion the
prior `code-audit-structured` pass used.

| tool | works here? | notes |
|---|---|---|
| `cargo test` (debug, checked arithmetic) | yes | fast, catches overflow/panics directly; use this first for every candidate |
| `cargo +nightly miri test` | yes (sysroot installed this session via `cargo +nightly miri setup`) | low expected value given `forbid(unsafe_code)` - only reach for it if a candidate's mechanism is genuinely unclear from a normal debug build |
| ASAN/UBSAN (`-Zsanitizer=address`/`undefined`, nightly) | not yet tried this session | likely low value for the same reason as miri; try only if a candidate needs it |
| `cargo fuzz` (0.12.0 installed) | yes | good fit for panic-hunting across the public `resize_*`/`premultiply_*`/`trc_*` API with arbitrary width/height/pixel-data combinations |
| `cargo clippy --all-targets -- -W clippy::pedantic` | yes | standard idiomatic first pass |

## Existing project harnesses worth reusing

None. No `fuzz/` directory, no property tests, no existing differential-testing setup. The library
crate itself has zero `#[test]`s (`cargo test -p pic-scale-safe` runs 0 tests); the only existing
test is a single slow (~70s) integration-style test in `app/src/image_wrapper.rs` that resizes a
real image and doesn't assert numeric correctness beyond "it runs".

## Instrumentation/tooling set up for this workspace

| what | why | caveat |
|---|---|---|
| `cargo +nightly miri setup` | make `cargo +nightly miri test`/`run` usable if a candidate needs it | took ~15s, no issues; the `nightly-x86_64-unknown-linux-gnu` toolchain (rustc 1.100.0-nightly, 2026-08-21) was already installed |

## Ground rules used in this workspace

1. No claim without a command that reproduces it - every `Confirmed` candidate has a real,
   standalone `repro/` and the exact command that runs it.
2. Every candidate with a fix gets a real `patch.diff`, applied and built for real, plus the
   project's own tests where they cover the code (currently none for the library crate itself -
   state "N/A, no tests in this package" honestly rather than skipping the check).
3. `Not reproducible`/`Inconclusive` results are written down with the same care as confirmed
   bugs - a result with no file is indistinguishable from "never checked".
4. If a later candidate changes an earlier one's confirmation status, don't silently rewrite that
   candidate's `RESULT.md` - add a `CORRECTION.md` in its folder and update its `SUMMARY.md` row
   to point at it.

## Note on this session's starting backlog

This bug-hunt session started immediately after a `code-audit-structured` pass on the same commit
(`83c4801428c436678d7c6b6bea74af800924a469`) that produced `___FINDINGS___/findings.toml` (27
static findings, no runtime verification). `IDEAS.md`'s initial backlog was seeded from that
report's most concrete, runtime-checkable leads (panics, overflow, wrong-formula logic errors) -
this hunt's job is to actually reproduce, root-cause, and (where bounded) fix them, which the prior
pass did not do. Style-only findings from that report (`duplicate_code`, `copy_paste` naming/message
issues, `architecture`) were left out of the backlog since they have nothing for this skill's
runtime-verification tools to confirm.
