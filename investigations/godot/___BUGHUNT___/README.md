# What is under test, and how findings are confirmed

Read this first if picking the work up cold - it explains the tooling, `SUMMARY.md` lists the
candidates investigated with it.

## Toolchain availability in this project

| tool | works here? | notes |
|---|---|---|
| `perf` | Yes | `perf_event_paranoid=-1`, no privilege escalation needed |
| doctest unit tests | Yes, with a caveat | Build with `tests=yes`; must set `XDG_DATA_HOME` to a writable path or the whole `user://` tree fails to initialize (see `___OPTIMIZATIONS___/README.md`'s Environment section - same container, same facts, not duplicated here) |
| miri / ASAN / UBSAN | Not evaluated this session | Godot is C++/SCons, not Rust - miri doesn't apply; ASAN/UBSAN would need a dedicated `scons` build variant (`use_asan=yes` et al.), not attempted since candidate 001 reproduced without needing a sanitizer |
| Existing fuzz targets | None found | No `fuzz/`/oss-fuzz integration noticed in this repo during this session's (brief, incidental) look |

## Existing project harnesses worth reusing

None - `tests/` (doctest, `--headless --test`) is the only existing automated-check harness in
this repo. See `___OPTIMIZATIONS___/README.md` for benchmark-harness notes (a separate concern).

## Instrumentation/tooling set up for this workspace

Nothing added - candidate 001 was found and confirmed using only the project's own existing
`--headless --test` entry point plus a `chmod`'d scratch `$HOME` to simulate the trigger condition
(see `001.LOGGER_NULL_DEREF_ON_UNWRITABLE_USERDIR/repro/run.sh`).

## Ground rules used in this workspace

1. No claim without a command that reproduces it - every `Confirmed` candidate has a real,
   standalone `repro/` and the exact command that runs it.
2. Every candidate with a fix gets a real `patch.diff`, applied and built for real, plus the
   project's own `--headless --test` suite run before and after (compared against the known
   baseline in `___OPTIMIZATIONS___/README.md`, not a naive 100%-pass expectation).
3. `Not reproducible`/`Inconclusive` results are written down with the same care as confirmed
   bugs - a result with no file is indistinguishable from "never checked".
4. If a later candidate changes an earlier one's confirmation status, don't silently rewrite that
   candidate's `RESULT.md` - add a `CORRECTION.md` in its folder and update its `SUMMARY.md` row
   to point at it.

## How this workspace came to exist

Candidate 001 was found incidentally while setting up a `perf-optimize` session's correctness
baseline (establishing that `--headless --test` runs cleanly before measuring any performance
candidates) - not from a deliberate `bug-hunt` session. This workspace exists per that skill's own
"incidental findings" convention: performance work regularly turns up non-performance findings,
and they get recorded here rather than lost or force-fit into `___OPTIMIZATIONS___/`.
