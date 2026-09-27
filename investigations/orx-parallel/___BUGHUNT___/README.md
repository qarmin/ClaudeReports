# What is under test, and how findings are confirmed

Read this first if picking the work up cold - it explains the tooling, `SUMMARY.md` lists the
candidates investigated with it.

`orx-parallel` is a native Rust parallel-iterator crate (edition 2024) with an optional `wasm`
target (wasm32 + a JS worker-pool helper) and an `experimental` feature gating some newer
combinators (`par_experimental_sort`, `collect_arb_over_bag`). Most confirmable bug candidates are
plain native `x86_64-unknown-linux-gnu` code; a handful (see `IDEAS.md`'s last few backlog rows)
only reproduce under a real wasm32+atomics browser/worker environment and are lower-confidence for
this session as a result.

## Toolchain availability in this project

| tool | works here? | notes |
|---|---|---|
| stable rustc/cargo | yes | 1.98.0, default toolchain |
| nightly rustc/cargo | yes | `nightly` (plain) and a dated `nightly-2026-08-21` toolchain both installed; use `cargo +nightly` |
| miri | yes | `rustup run nightly cargo miri --version` -> `miri 0.1.0`. Use `cargo +nightly miri run`/`test` |
| ASAN/UBSAN | expected to work | nightly is present; use `RUSTFLAGS='-Zsanitizer=address'` (or `undefined`) with `cargo +nightly ... --target x86_64-unknown-linux-gnu` |
| cargo-fuzz | yes | `~/.cargo/bin/cargo-fuzz` installed; no `fuzz/` directory exists yet in this repo, so a one-off harness needs its own throwaway crate (or `cargo fuzz init` in a scratch copy) rather than reusing an existing one |
| wasm32-unknown-unknown target | installed | `rustup target list --installed` includes it, so `cargo build --features wasm --target wasm32-unknown-unknown` is at least buildable; no real browser/Node worker harness has been set up in this workspace yet, so anything requiring *running* wasm workers (not just compiling for wasm32) is currently a compile-only check |

## Existing project harnesses worth reusing

None found - no `fuzz/` directory, no differential-testing setup, no property-test harness in
`tests/`. `tests/` holds ordinary `#[test]` integration tests exercising the public API
(`into_par.rs`, `kind_transform_par*.rs`, `parallelizable*.rs`, `trait_bounds.rs`, `using/`, etc.) -
useful as the "before/after" regression suite step 7 of the candidate loop requires, not as a
fuzzing/differential base to build on.

## Instrumentation/tooling set up for this workspace

Nothing beyond stock toolchains was needed to get any of miri/ASAN/UBSAN/cargo-fuzz/wasm32
working - everything in the table above worked out of the box in this environment.

## Ground rules used in this workspace

1. No claim without a command that reproduces it - every `Confirmed` candidate has a real,
   standalone `repro/` and the exact command that runs it.
2. Every candidate with a fix gets a real `patch.diff`, applied and built for real, plus the
   project's own tests where they cover the code.
3. `Not reproducible`/`Inconclusive` results are written down with the same care as confirmed
   bugs - a result with no file is indistinguishable from "never checked".
4. If a later candidate changes an earlier one's confirmation status, don't silently rewrite that
   candidate's `RESULT.md` - add a `CORRECTION.md` in its folder and update its `SUMMARY.md` row
   to point at it.
5. Candidates whose only real trigger path needs a browser/wasm-worker environment we haven't set
   up here (see the last few `IDEAS.md` rows) are expected to top out at a compile-time-only
   confirmation, or `Inconclusive`/`Not reproducible` if even that isn't possible - say so
   explicitly in `RESULT.md` rather than stretching a partial signal into `Confirmed`.
