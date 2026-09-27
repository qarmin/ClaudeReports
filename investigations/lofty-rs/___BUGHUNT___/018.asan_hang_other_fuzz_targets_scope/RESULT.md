# asan_hang_other_fuzz_targets_scope

`fuzz/fuzz_targets/*.rs`, specifically the file-format `*file_read_from` harnesses
(`mp4file_read_from.rs`, `flacfile_read_from.rs`, and by the same code path every other
`*file_read_from`/`filetype_from_buffer`/`probe_read` target) - flagged in `IDEAS.md` while
working candidate `009`, which isolated a default-ASAN (`cargo fuzz run --sanitizer address`)
cargo-fuzz hang to a tooling/sandbox interaction rather than a code bug in `Picture::from_reader`/
`from_flac_bytes_inner`/`from_ape_bytes`, but only checked two small `picture_from_*` targets. This
candidate checks whether the same hang reproduces on the project's larger, more heavily-used
file-format targets before relying on ASAN for a future candidate in this environment.

This is a tooling/environment finding, not a bug in `lofty`'s own source - no lofty source file was
suspected or touched. Recorded as a full candidate per the working instructions for this session.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `cargo +nightly fuzz build mp4file_read_from --sanitizer address` | fresh build | Builds cleanly in ~27s, no issue at build time. |
| `cargo +nightly fuzz run mp4file_read_from --sanitizer address -- -max_total_time=25/20`, empty `fuzz/corpus/mp4file_read_from/` | default cargo-fuzz ASAN build, watched via `ps aux` for CPU%/progress | Hung: process pegged at 99.9% CPU for 3+ minutes total observed (well past the requested 20-25s `max_total_time`), stuck printing exactly `NEW_FUNC[1/4]: ` with no further libFuzzer output, reproduced 2/2 separate runs (one ad hoc, one via `repro/run.sh`). |
| `cargo +nightly fuzz run flacfile_read_from --sanitizer address -- -max_total_time=20`, empty `fuzz/corpus/flacfile_read_from/` | same config | Hung identically: stuck at `NEW_FUNC[1/1]: ` (mid ASAN's separator line before what would be a symbolized function name), 99.9% CPU, 2/2 separate runs (one ad hoc, one via `repro/run.sh`). |
| `cargo +nightly fuzz run mp4file_read_from --sanitizer none -- -max_total_time=15/25`, same target | debug-assertions + overflow-checks on, no sanitizer | Ran cleanly every time: 4.9M-5.2M execs in 16-21s, 0 hangs, 0 crashes, across 3 separate runs. |
| `cargo +nightly fuzz run flacfile_read_from --sanitizer none -- -max_total_time=15` | same config | Ran cleanly: 4.7M execs in 16s, 0 hangs, 0 crashes. |
| Control: `cargo +nightly fuzz run mp4file_read_from --sanitizer address` with a **non-empty** `fuzz/corpus/mp4file_read_from/` (23-29 files left over from a prior `--sanitizer none` run) | same ASAN build, same target | Did **not** hang - completed cleanly in 21s (2.07M execs), isolating the hang to the empty-corpus case specifically, not to ASAN-on-this-target generally. |

## Status

**Confirmed.** The default-ASAN cargo-fuzz hang first observed in candidate `009` on
`picture_from_ape_bytes`/`picture_from_flac_bytes` also reproduces on the project's larger
file-format `*file_read_from` targets - checked directly on `mp4file_read_from` (2/2 runs hung
when starting from an empty corpus, 1/1 ran cleanly when starting from a pre-populated corpus) and
`flacfile_read_from` (2/2 runs hung from an empty corpus). Reproduction rate: 4/4 hangs across the
two targets tested, each reproduced twice, under an identical build (`cargo +nightly fuzz build
--sanitizer address`, this session's toolchain - see `README.md`), and 0/0 hangs across 5 separate
`--sanitizer none` control runs on the same two targets/build.

**Fully reproducible: Yes.** This is a single-participant finding (a real `cargo +nightly fuzz run`
invocation, the actual command a maintainer or a future session of this skill would run, against
the project's own real, already-existing fuzz target binaries) - there is no second participant
being faked; `CONFIRMATION.md` section 11's caveat about hand-constructed peer input doesn't apply
here. `repro/run.sh` runs the exact real command end-to-end and captures the exact real hang.

## Class

`tooling` (environment/sandbox interaction with `cargo-fuzz`'s default ASAN sanitizer build - not
a defect in `lofty`'s own source, and not something a source-code patch can fix).

## Root cause

Narrowed further than candidate `009` managed to: the hang is not simply "ASAN is slow/broken in
this sandbox" in general, and not tied to target size/complexity (`mp4file_read_from` and
`flacfile_read_from` are both substantially larger/more complex targets than the two
`picture_from_*` targets candidate `009` checked, and both hang identically). It is specifically
tied to libFuzzer's **first coverage-discovery (`NEW_FUNC`) event when the run starts from an empty
corpus**: every hung run's last line of output is `NEW_FUNC[N/M]: ` (once even followed by ASAN's
box-drawing separator line, suggesting it is mid-way through formatting/symbolizing that event) and
every hang happened on a target's very first execution against an empty `fuzz/corpus/<target>/`
directory (no corpus is committed to the repo - see `README.md`). The control run above shows the
exact same target, same binary, same ASAN build, completing cleanly in 21s once a non-empty corpus
already existed (so the "new function" coverage events had already been recorded by an earlier
`--sanitizer none` run and libFuzzer had nothing new to symbolize on this pass). This points at
ASAN/libFuzzer's coverage symbolizer (which typically shells out to `llvm-symbolizer` as a
subprocess, or uses in-process symbolization that still touches `/proc`/ptrace-adjacent facilities)
being the specific thing blocked or extremely slow under this sandbox's restrictions (candidate
`009`'s README note already established `gdb -p <pid>` can't attach here - `ptrace_scope`/no
`CAP_SYS_PTRACE` - which is consistent with a symbolizer subprocess/self-inspection call stalling
the same way). This is a plausible, well-evidenced mechanism, not a confirmed one at the
symbolizer-internals level - no in-process backtrace could be captured for the same reason
candidate `009` couldn't (no ptrace access in this sandbox).

## The fix

No fix attempted, and none is applicable in the "patch lofty's source" sense - this is a
sandbox/tooling interaction between cargo-fuzz's default ASAN build and this specific execution
environment, not a defect in the project. No `lofty` source file was ever suspected or modified for
this candidate (`git status` was clean of any source changes before and after, verified below).

The practical workaround (documented in `README.md`, not a source patch): use `--sanitizer none`
for this project's `cargo fuzz run` invocations in this environment by default, and/or seed
`fuzz/corpus/<target>/` with at least one file before ever using `--sanitizer address` here, since
the control run shows a pre-populated corpus avoids the hang entirely for the one target where this
was tried.

- Project test suite: not run for this candidate (no source files were modified; nothing to
  regress-test - `git status` was clean before and after, confirmed via `git -C
  /home/rafal/Projekty/Obce/lofty-rs status --porcelain=v1`, which showed only the untracked
  `___BUGHUNT___/` directory both times).

## What's not done / open questions

- Only 2 of the project's ~13 `*file_read_from`/`filetype_from_buffer`/`probe_read` targets were
  directly checked (`mp4file_read_from`, `flacfile_read_from`); given both hung identically and the
  root cause (first-`NEW_FUNC`-event-on-empty-corpus) is a property of libFuzzer/ASAN's own startup
  behavior rather than of any target-specific code, the remaining targets are expected to behave
  the same way, but this was not independently verified for each one.
- Whether seeding a real corpus before ever running `--sanitizer address` reliably avoids the hang
  on *every* target (not just the one control case tried here) wasn't swept exhaustively - the
  practical guidance in `README.md` is "prefer `--sanitizer none`" rather than relying on corpus
  seeding as the primary workaround, precisely because this is a rate of exactly one confirming
  control observation.
- The exact mechanism inside ASAN/libFuzzer's symbolizer that stalls under this sandbox was not
  identified at the source level (no `gdb`/`strace`-equivalent access was available, same
  limitation candidate `009` hit) - if a future session gets ptrace/strace access in this
  environment, tracing one of these hung processes directly would likely pin down the exact
  syscall/subprocess call it's blocked on.
