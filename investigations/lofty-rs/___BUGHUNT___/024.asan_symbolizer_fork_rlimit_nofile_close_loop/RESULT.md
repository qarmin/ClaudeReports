# asan_symbolizer_fork_rlimit_nofile_close_loop

A backlog item from `IDEAS.md` (noticed while working candidate `018`, added 2026-08-26): candidates
`009`/`018`/`019` characterized the default-ASAN `cargo fuzz run` hang behaviorally (always stuck at
a fresh libFuzzer `NEW_FUNC` coverage-symbolization event) but had no `strace`/`gdb`-equivalent
access to pin down the exact blocked syscall/subprocess call. This candidate re-checked whether that
access exists and, having found a workable path in, root-caused the hang down to the exact call.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `strace` | checked `which strace` | Not installed - confirms candidates 018/019's note. |
| `gdb -p <pid>` (attach to an already-running sibling process, e.g. a backgrounded `sleep`) | default sandbox config | **Fails**: "Could not attach to process... ptrace: Inappropriate ioctl for device." `/proc/sys/kernel/yama/ptrace_scope` is `1` (restricted - only a process's own direct children are traceable), and the hung fuzz target isn't a direct child of the investigating shell. This matches what candidates 018/019 assumed was a hard "no ptrace access" wall. |
| `gdb --args <bin>` / `gdb -batch -ex run ...<bin>` (gdb spawns the target **itself**, as its own direct child, instead of attaching to an unrelated running process) | plain C test program (`break foo; run; bt`) | **Works** - a real breakpoint, a real stop, a real backtrace. `ptrace_scope=1` only blocks attaching to processes gdb didn't itself fork; it does not block ptrace entirely. This is the access candidates 018/019 said the workaround wasn't worth pursuing without. |
| `gdb -batch -ex run` directly on the ASAN-built `picture_from_flac_bytes`/`mp4file_read_from` fuzz binaries, with `set follow-fork-mode child` / `set detach-on-fork off` so gdb also traces the process ASAN forks internally, interrupted with `SIGINT` mid-hang | `--sanitizer address` build, empty corpus, `-max_total_time=15/40` | **Direct hit.** 3 independent captures (2 on `picture_from_flac_bytes`, 1 on `mp4file_read_from`) all show the exact same call chain blocked in the forked-but-not-yet-`exec`'d symbolizer child: `__sanitizer::internal_close` <- `StartSubprocess` <- `SymbolizerProcess::StartSymbolizerSubprocess` <- `SymbolizerProcess::SendCommand` <- `LLVMSymbolizer::SymbolizePC` <- ... <- libFuzzer's `NEW_FUNC` coverage-print path. The parent thread is correspondingly blocked in `internal_read`/`ReadFromFile`/`SymbolizerProcess::ReadFromSymbolizer`, waiting on a pipe the child never writes to because it never reaches `execve`. |

That third approach immediately explained *why*: `ulimit -Hn`/`/proc/self/limits` show this
sandbox's `RLIMIT_NOFILE` (soft **and** hard) is `2147483584` (~2.1 billion). Captured register
state (`rdi`, the fd argument to the `close()` syscall `internal_close` wraps) was in the same
2-billion-ish range and visibly climbing between captures - i.e. the forked child is in a loop
calling `close()` on every fd number up to `RLIMIT_NOFILE`, and with the limit this high that loop
does not finish in any practical amount of time (confirmed separately: a plain background run of
the forked child was still spinning at 99% CPU after 100+ seconds with no progress, vs. it having
appeared ~98% through the range after only 8 seconds in an earlier capture - the last stretch is
disproportionately slow in this sandbox, not linear).

Tested the fix this implies directly: lowering `RLIMIT_NOFILE` (`ulimit -Sn 65536`) **before**
invoking the exact same `cargo fuzz run --sanitizer address` command made the hang disappear
entirely - `mp4file_read_from` (the target candidate `019` showed hangs 6/6 times even with a
pre-seeded corpus) ran to completion cleanly (~1.9M real executions in 16s, exit 0) in every one of
3 independent runs (2 direct + 1 via the candidate's own `repro/`+`justfile` against a genuinely
fresh clone), with full ASAN instrumentation still active - not disabled the way `--sanitizer none`
disables it.

## Status

**Confirmed.** Reproduction rate: the exact same gdb backtrace (`internal_close` <-
`StartSubprocess` <- `SymbolizerProcess::StartSymbolizerSubprocess`) was captured 3/3 independent
times (2 runs on `picture_from_flac_bytes`, 1 on `mp4file_read_from`), all release/ASAN builds, no
fixed seed. The `RLIMIT_NOFILE`-lowering fix was verified 3/3 times (2 direct runs against this
checkout + 1 run of the candidate's own `repro/run.sh` against a fresh `git clone` at
`5c6565e36d35b9ea7de049daa67acc0be0fd3b01`, via `just test`), against `mp4file_read_from`
specifically (the most stubborn target from candidate `019`'s sweep). The negative control (same
target, same empty corpus, real sandbox `ulimit`, no fix) hung (exit 124 after a 30s kill,
stuck at `NEW_FUNC[1/4]`) in every run, matching candidates 009/018/019's signature exactly.

**Fully reproducible: Yes.** Single-participant finding: `repro/run.sh` runs the real
`cargo +nightly fuzz run --sanitizer address` command a maintainer would actually run, against the
project's own real, already-existing `fuzz/fuzz_targets/mp4file_read_from.rs` binary, with no
hand-constructed input standing in for anything - `CONFIRMATION.md` section 11's caveat about a
faked second participant doesn't apply (there is no second participant here at all; the "other
side" is this sandbox's own kernel/rlimit configuration, which the repro queries and prints
directly via `ulimit`).

## Class

`tooling` (environment/sandbox interaction with `cargo-fuzz`'s default ASAN sanitizer runtime, same
class as candidates 018/019 - not a defect in `lofty`'s own source).

## Root cause

ASAN's runtime (`compiler-rt`'s `sanitizer_common`) lazily spawns `llvm-symbolizer` as a
long-lived subprocess the first time it needs to symbolize an address - which for a `cargo fuzz`
run under libFuzzer is exactly the first `NEW_FUNC` coverage event, matching what candidates
009/018/019 already observed as the hang's trigger point. `SymbolizerProcess::StartSymbolizerSubprocess()`
calls `StartSubprocess()`, which `fork()`s and, in the child, closes every inherited file descriptor
number from 3 up to the process's `RLIMIT_NOFILE` **soft limit** (via a plain loop of `close()`
calls, `__sanitizer::internal_close`) before calling `execve("llvm-symbolizer", ...)` - a standard
defensive measure to avoid leaking fds into the subprocess. This sandbox environment sets
`RLIMIT_NOFILE` (both soft and hard) to `2147483584` (~2.1 billion) by default for every process,
so that loop must issue on the order of two billion individual `close()` syscalls before it can
reach `execve()`. In this sandbox that never completes in any practical amount of time (confirmed:
still spinning after 100+ seconds of continuous CPU use with no forward progress observed). Because
`execve()` is never reached, the symbolizer subprocess never writes anything back on the
communication pipe, so the parent thread's `SymbolizerProcess::ReadFromSymbolizer()` (a blocking
`read()` on that pipe, `__sanitizer::internal_read`) blocks forever - which is the actual "hang" a
maintainer observes as `cargo fuzz run --sanitizer address` making no further libFuzzer progress
after a `NEW_FUNC[...]:` line.

This refines, rather than replaces, candidate `019`'s "any run where a fresh, previously-unrecorded
coverage event occurs" characterization: it explains *why* a fresh coverage event specifically is
the trigger (that's when `SymbolizePC` is first called, which is when the symbolizer subprocess is
first spawned) and pins the exact blocked call, closing the open question both `018` and `019` left.

## The fix

No `lofty` source change - this is entirely a sandbox/environment + ASAN-runtime interaction, not a
defect in `lofty`'s own code (no `lofty` source file was suspected or touched; `git status
--porcelain=v1` in the main project showed only the untracked `___BUGHUNT___/` directory, confirmed
both before and after this session).

There is, however, a genuine, verified, and *strictly better* workaround than the project's existing
blanket `--sanitizer none` advice: **lower `RLIMIT_NOFILE` before invoking `cargo fuzz run
--sanitizer address`** (e.g. `ulimit -Sn 65536`), rather than disabling ASAN entirely. This keeps
real ASAN instrumentation active while avoiding the hang, verified against `mp4file_read_from` (the
target candidate `019` found reliably hangs 6/6 times even with a pre-seeded corpus - the corpus-based
workaround does not help here, but this does) and `picture_from_flac_bytes`, 5 total clean runs (2+1+2
across the two targets) with zero hangs once the limit is lowered, versus the baseline hanging every
single time it was tried at the sandbox's real (huge) limit.

`___BUGHUNT___/README.md`'s ASAN caveat row has been updated in this session to record this root
cause and the better workaround, superseding the "use `--sanitizer none`, always" framing with
"prefer `ulimit -Sn <reasonable value>` before an ASAN run; fall back to `--sanitizer none` only if
that's not an option". No change was needed to the main project's own top-level `README.md` - it
never documented ASAN/sanitizer usage in the first place (the `--sanitizer none` guidance the
original backlog item referred to lives only in `___BUGHUNT___/README.md`, not `lofty-rs/README.md`).

- Project test suite: not run for this candidate (no `lofty` source files were modified; nothing to
  regress-test).

## What's not done / open questions

- The exact lower bound for `RLIMIT_NOFILE` that avoids the hang wasn't bisected - `65536` was
  chosen as a generously large-but-sane value and worked every time it was tried; a much lower value
  (e.g. `4096`, a typical default) would very likely also work but wasn't specifically tested.
- Whether this sandbox's `RLIMIT_NOFILE=2147483584` default is itself something worth reporting
  upstream (to whatever tool/config sets up this sandbox) versus treating it as a fixed environment
  property to work around locally wasn't decided - out of scope for a `lofty`-focused bug hunt.
- This closes the `strace`/`gdb`-equivalent-access open question from candidates 018/019's own
  `IDEAS.md` entries; the entry has been removed from the backlog per `SKILL.md`.
