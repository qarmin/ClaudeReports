# What is under test, and how findings are confirmed

Read this first if picking the work up cold - it explains the tooling, `SUMMARY.md` lists the
candidates investigated with it.

## Toolchain availability in this project

| tool | works here? | notes |
|---|---|---|
| `cargo test --doc` | yes | standard rustdoc doctest runner, no special setup needed |
| miri / ASAN / UBSAN | not yet exercised | no candidate in this workspace has needed them so far (see `SUMMARY.md`) |

## Existing project harnesses worth reusing

None noted yet.

## Instrumentation/tooling set up for this workspace

Nothing beyond stock `cargo`/`rustdoc` was needed for the candidates recorded so far.

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
