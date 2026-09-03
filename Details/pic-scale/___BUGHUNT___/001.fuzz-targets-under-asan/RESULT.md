# Run existing cargo-fuzz targets under ASAN

The project already defines 16 cargo-fuzz targets (`resize_rgba`, `resize_rgb`, `resize_cbcr8/16`,
`resize_cbcr_f16/f32`, `resize_plane*`, `resize_rgba_f16/f32/u16`, `resize_rgb_f16/f32/u16`,
`colorspaces`) with no corpus/crash dir present, so it was unclear whether any had been run for
real recently. Running each for a few minutes under ASAN (cargo-fuzz's default) was the intended
highest-leverage first move before writing any new harness.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `cargo +nightly fuzz build` (all 16 targets, default = ASAN) | release, `--features avx,sse,nightly_avx512` (+`colorspaces` for that one target) | All 16 build cleanly. |
| `cargo +nightly fuzz run <target>` (default ASAN), all 16, `-max_total_time=150` each, run in parallel | empty corpus, `-rss_limit_mb=1536` | Every one of the 16 hung: stuck at ~100% CPU right after `INITED`, 0 corpus files ever written, still running minutes past the requested `-max_total_time` until force-killed. |
| Same, single target (`resize_rgba`) run alone, no parallelism, host confirmed idle (>99% system-wide idle via `/proc/stat` delta) | `-max_total_time=5` | Hung on ~5/5 separate trials; `/proc/<pid>/wchan` empty and state `R` the whole time (pure userspace spin, not blocked on a syscall/lock). |
| Same, with `ASAN_OPTIONS=detect_leaks=0` (LeakSanitizer is known to hang under ptrace-restricted sandboxes) | `-max_total_time=5` | Succeeded on the very first trial (`Done 8131 runs in 6s`), then hung on the next 10/10 trials with the same binary, no rebuild in between - not a reliable fix, so LSan is not the sole cause. |
| Trivial `-Zsanitizer=address` "hello world" Rust binary (no libFuzzer, no pic-scale) | plain `rustc` build | 3/3 instant, correct runs - rules out "any ASAN binary hangs in this sandbox". |
| pic-scale's `resize_rgba()` resize logic called directly (verbatim copy of the fuzz harness body: padded stride, double-resample), grid of 50,176 cases (all src/dst size and flag combinations up to 2500x2500 -> 512x512), **no ASAN, no libFuzzer** | standalone binary, `path = "../../../"` dependency | All 50,176 cases completed in single-digit milliseconds each; ran to completion in well under a minute. No hang anywhere in the parameter space. |
| Same 50,176-case grid, same logic, built directly with `RUSTFLAGS="-Zsanitizer=address"` against the library, **bypassing cargo-fuzz/libFuzzer entirely** | standalone binary | All 50,176 cases completed within a 60s window, each case ~10-35ms (~3x slower than without ASAN, consistent with normal allocation overhead - not a hang). |
| `cargo +nightly fuzz build/run --sanitizer none` (keeps libFuzzer's coverage instrumentation and cargo-fuzz's default `-Cdebug-assertions`, drops AddressSanitizer), all 16 targets, `-max_total_time=150` each, in parallel | empty corpus, `-rss_limit_mb=2048` | All 16 ran cleanly to completion (`Done` + clean exit 0), totaling 6,752,027 executions across all targets. Zero crashes, panics, timeouts, or sanitizer warnings. |

## Status

**Not reproducible** - no bug in pic-scale's own code was found. 6,752,027 fuzz executions across
all 16 existing targets (~150 seconds each, guided/coverage-instrumented, debug-assertions on)
completed with zero crashes, panics, or overflow aborts. This is a single short burst per target,
not an exhaustive campaign - see "What's not done" below.

Separately, and this is the substantive finding of this candidate: **cargo-fuzz's default
AddressSanitizer build of every one of the 16 existing targets hangs almost immediately in this
sandboxed environment** (pinned at ~100% CPU, zero fuzzing progress, `-max_total_time` not
honored, has to be force-killed) - reproduced on every target, and on ~15/16 repeated trials of a
single target run in isolation with no other host load. This is an environment/tooling limitation,
not a pic-scale bug: the exact same resize code, called directly (bypassing cargo-fuzz/libFuzzer),
runs correctly and quickly both with and without `-Zsanitizer=address` across the same extreme
input space the fuzz harness explores (see "What was tried" table). The hang is specific to the
combination of {cargo-fuzz's libFuzzer driver} + {AddressSanitizer} in this sandbox.

## Class

N/A (Not reproducible - no pic-scale bug confirmed).

## Root cause

For the "no pic-scale bug found" result: not applicable - nothing to root-cause, this is a clean
negative from the coverage actually achieved (see Status).

For the ASAN-hang tooling finding (kept here since it explains why the fuzzing coverage above used
`--sanitizer none` instead of the originally intended ASAN default): the hang lives in the
libFuzzer/ASAN runtime interaction, not in pic-scale's source. Concretely ruled out:
- Not pic-scale's resize algorithms (direct-call grid of 50,176 boundary cases completes in
  milliseconds, with or without ASAN).
- Not "ASAN doesn't work in this sandbox at all" (a trivial ASAN hello-world binary runs fine).
- Not purely LeakSanitizer (a commonly-cited cause of ASAN hangs under ptrace-restricted sandboxes,
  since this container has `yama/ptrace_scope=1` and gdb could not attach to the actual spinning
  worker process for the same ptrace-ancestor reason) - `ASAN_OPTIONS=detect_leaks=0` fixed it on
  only 1 of 11 trials, not reliably, so it's at most a partial contributor.
- Not host contention/noisy-neighbor CPU steal (verified system-wide idle >99% via `/proc/stat`
  deltas during trials that still hung).

The remaining, unconfirmed suspect is libFuzzer's own ASAN-instrumented run loop (its RSS-limit
watchdog and/or its per-execution fork/monitor pairing observed in `ps` as a `Sl`-state launcher
plus an `R`-state worker child) doing something that doesn't complete in this container. This
wasn't root-caused further - `strace`/`perf`/`ltrace` are not installed and not installable
(no root in this sandbox), and gdb cannot attach to the actual spinning process because it's a
second-generation fork that libFuzzer's default `detach-on-fork on` releases untraced before gdb
(attached only to the immediate child) ever sees it.

## The fix

No fix applicable - not a pic-scale source bug. The practical workaround for fuzzing this project
in this sandbox is `cargo fuzz run --sanitizer none`, which retains libFuzzer's coverage-guided
fuzzing and cargo-fuzz's default debug-assertions (catching integer overflow/panics) while
avoiding the hang; it just loses ASAN's own memory-safety instrumentation. This is recorded in
`___BUGHUNT___/README.md`'s tooling notes for future sessions.

- Project test suite: not run - no source change was made in this candidate (nothing to verify a
  fix against).

## What's not done / open questions

- The ASAN hang itself was not root-caused all the way down (see above) - a future session with
  `strace`/`perf` available, or on a different host/sandbox, could pin down whether it's
  libFuzzer's RSS watchdog, its fork-per-execution monitoring, or something specific to this
  container's seccomp/ptrace restrictions. Added to `IDEAS.md`.
- The `--sanitizer none` campaign was a single ~150s burst per target, not a sustained campaign -
  it does not rule out rarer logic/memory bugs a longer run (via `fuzz-bug-repro`, run separately)
  might find. It also does not give ASAN's own memory-safety coverage (out-of-bounds, use-after-free)
  since ASAN itself doesn't run here - only what debug-assertions + coverage-guided fuzzing catch
  (panics, overflow, obviously-wrong branches) was actually exercised.
- NEON/SVE2/WASM32 code paths were not exercised at all (this host is x86_64) - unchanged from
  `README.md`'s existing caveat, not something this candidate added.
- Only the `Bilinear` resampling function was exercised heavily by name in the manual grid probe
  (matching what the `resize_rgba` harness itself hardcodes); the actual fuzz targets pick their
  own sampler internally via `Arbitrary` in some cases - not independently re-verified as covering
  every `ResamplingFunction` variant.
