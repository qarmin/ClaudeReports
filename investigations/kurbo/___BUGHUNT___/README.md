# What is under test, and how findings are confirmed

Read this first if picking the work up cold - it explains the tooling, `SUMMARY.md` lists the
candidates investigated with it.

## Project shape (2026-09-01)

kurbo is a pure-safe-Rust 2D curves/geometry library (`rust.unsafe_code = "forbid"` at the
workspace level, confirmed zero `unsafe` blocks in `kurbo/src`). This rules out miri/ASAN/UBSAN as
primary tools - there is no UB to find in the classic sense. The bug classes that actually exist
here are: panics (index out of bounds, `unwrap`/`expect` on `None`/`Err`, integer overflow in a
debug build, division by zero), and **logic bugs** (wrong geometric/numerical results - bad
intersection points, wrong arc length, wrong nearest-point, NaN/Infinity propagating silently
through a computation that should have rejected or handled it, wrong root count from
`polycool`'s polynomial solvers). The workspace has two crates: `kurbo` (curves, `Bezier`, `Arc`,
`Rect`, offsetting, intersection, etc.) and `polycool` (polynomial root-finding, used internally by
kurbo for cubic/quartic solves).

## Toolchain availability in this project

| tool | works here? | notes |
|---|---|---|
| miri | not useful | no `unsafe` anywhere in `kurbo/src` (grep confirmed 0 hits); miri adds nothing over a debug build here. Don't spend a candidate's tool slot on it unless a specific candidate turns out to touch `unsafe` in a dependency boundary. |
| ASAN/UBSAN | not useful | same reason as miri - no unsafe/FFI surface to instrument. |
| debug-assertions build | primary tool | `cargo build`/`cargo test` (no explicit `[profile.dev]` override in the root `Cargo.toml`, so debug-assertions and overflow-checks are on by default) - this is what turns a silent wraparound into a panic, use it for every candidate. |
| cargo-fuzz | available, no existing fuzz/ dir | `cargo-fuzz` binary is installed (`which cargo-fuzz` succeeded). No `fuzz/` directory exists yet in this repo - a candidate needing fuzzing builds a short one-off harness per `templates/fuzz_harness_template.rs`, or a standalone `arbtest`-based property check (see below), whichever is a better match for the input shape. |
| `arbitrary` + `arbtest` | already a project dependency | both `kurbo` and `polycool` already depend on `arbitrary`/`arbtest` in `[dev-dependencies]` - `polycool` even exposes an `arbitrary` cargo feature. This means property-style fuzzing (`arbtest::builder().run(|u| ...)`) is idiomatic here and cheap to bolt onto any candidate without cargo-fuzz's build overhead - prefer it for a quick boundary/invariant check, reach for `cargo-fuzz` only when a persistent corpus or crash-minimization workflow is actually needed. |

## Existing project harnesses worth reusing

No dedicated `tests/` or `fuzz/` directories - tests live inline as `#[cfg(test)] mod tests` in
each source file (`cargo test --workspace --all-features` runs everything). `polycool`'s own
existing arbtest-based tests (search `polycool/src` for `arbtest::builder()`) are a good reference
for how this project already writes property checks against its root-finders - a new candidate's
one-off harness should follow the same style rather than inventing a different one.

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
