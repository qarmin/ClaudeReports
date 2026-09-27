# What is under test, and how findings are confirmed

Read this first if picking the work up cold - it explains the tooling, `SUMMARY.md` lists the
candidates investigated with it.

## Toolchain availability in this project

Checked at session start (2026-08-25):

| tool | works here? | notes |
|---|---|---|
| `cargo +nightly miri` | yes (`miri-x86_64-unknown-linux-gnu` component installed on `nightly-x86_64-unknown-linux-gnu`) | Use for anything touching `unsafe`/raw slices/transmutes. Will refuse real syscalls/FFI - that's a tooling gap, not a finding. |
| ASAN/UBSAN (`-Zsanitizer=address`/`undefined`) | yes (`x86_64-unknown-linux-gnu` target installed, several nightly toolchains present) | Use `cargo +nightly ...` with `RUSTFLAGS` per `COMMANDS.md`. **Caveat (candidates 009, 018, 019, root-caused in 024):** the default cargo-fuzz `--sanitizer address` build hangs indefinitely (100% CPU, no further libFuzzer progress) on 18 of this project's 19 fuzz targets (only `filetype_from_buffer` never hangs) whenever ASAN's libFuzzer instance hits a code path it hasn't recorded coverage for yet - confirmed directly on 5 targets across candidates 009/018 and swept across all 19 in candidate 019. Every hang's last output line is `NEW_FUNC[n/m]: `. **This is not simply "empty corpus"**: candidate `019` showed a single-file corpus seed (even a large, real, valid file) still triggers the hang on 17/18 of the affected targets, because libFuzzer's own minimization passes over dozens of derived inputs almost always hit at least one never-before-covered function. **Root cause (candidate `024`):** this sandbox sets `RLIMIT_NOFILE` (soft and hard) to `2147483584` (~2.1 billion) for every process. ASAN's `SymbolizerProcess` forks to launch `llvm-symbolizer` on the first symbolization request (i.e. the first `NEW_FUNC` event); in the forked child, `compiler-rt`'s `StartSubprocess()` closes every fd number from 3 up to `RLIMIT_NOFILE` before `exec`ing, which with this limit never finishes in practice, so `exec` is never reached and the parent blocks forever reading the (never-written-to) symbolizer pipe. Confirmed via `gdb` spawning the target directly (`ptrace_scope` blocks *attaching* to an unrelated running process in this sandbox, but not tracing a process gdb forks itself, including following its own internal `fork()`s with `set follow-fork-mode child`). **Better workaround than `--sanitizer none`: lower `RLIMIT_NOFILE` before the run**, e.g. `ulimit -Sn 65536`, then `cargo +nightly fuzz run <target> --sanitizer address ...` - verified to eliminate the hang entirely (including on `mp4file_read_from`, which hangs 6/6 with any corpus-seeding workaround) while keeping real ASAN instrumentation active, unlike `--sanitizer none`. Fall back to `--sanitizer none` (still debug-assertions + overflow-checks on, 300K+ exec/s) only if lowering `RLIMIT_NOFILE` isn't an option in whatever shell is running the command. **Corpus-saturation threshold (candidate `028`):** a large-enough/diverse-enough pre-existing corpus also avoids the hang (no fresh coverage event -> symbolizer never spawns), but the size needed is target- and composition-specific (not just file count) and not cheaply predictable in advance (~45-50% of an already-diverse pool sufficed for `mp4file_read_from`, ~70-90%+ for `speexfile_read_from`) - prefer the `ulimit` workaround over trying to pre-seed a "big enough" corpus. **Tooling pitfall:** `timeout <n> cargo +nightly fuzz run ...` only kills the `cargo` wrapper at the deadline, not the actual libFuzzer target binary `cargo fuzz` spawns as a child - a hung target keeps running as an orphan (visible via `ps aux`) even after `timeout` returns. Follow up any timed-out `cargo fuzz run` with `pkill -9 -f "release/<target> .*<corpus-dir>"` to actually stop it, and classify hang/no-hang by the log's own last line (`Done N runs in Ms` = clean, stuck at `NEW_FUNC[...]:` = hung), not by `timeout`'s exit code alone. |
| `cargo fuzz` (cargo-fuzz 0.12.0) | yes, and the project already has a `fuzz/` directory (see below) | Prefer extending/reusing an existing fuzz target over hand-rolling a new harness from scratch. Default sanitizer is ASAN - see the caveat in the row above. |
| Debug-assertions build | yes, default `cargo build`/`cargo test` | Catches integer overflow/underflow for free - cheap first pass on any arithmetic-on-untrusted-size suspect. |

Disk: 156G free at session start on the filesystem backing this checkout - fine for normal use,
but sanitizer/miri builds plus repeated fresh `cargo fuzz build`s add up; `cargo clean` on stale
`target/`s if it gets tight (see `SKILL.md` "Watching disk space during a long session").

## Existing project harnesses worth reusing

The project ships a real `fuzz/` directory (`cargo-fuzz` based) at the repo root with one target
per file-type reader, already exercising the untrusted-input entry points this skill's own
heuristics care most about:

`filetype_from_buffer`, `probe_read`, `aacfile_read_from`, `aifffile_read_from`,
`apefile_read_from`, `flacfile_read_from`, `mp4file_read_from`, `mpcfile_read_from`,
`mpegfile_read_from`, `opusfile_read_from`, `speexfile_read_from`, `vorbisfile_read_from`,
`wavfile_read_from`, `wavpackfile_read_from`, `picture_from_ape_bytes`,
`picture_from_flac_bytes`, `picture_from_reader`, `pictureinformation_from_jpeg`,
`pictureinformation_from_png`.

Prefer running/extending one of these (`cargo +nightly fuzz run <target> -- -max_total_time=120`)
over writing a new standalone `fuzz_harness_template.rs` from scratch, when the candidate's
suspect code path is already reachable from one of them. There is no persistent corpus checked
into the repo (no `fuzz/corpus/` committed) - a short burst starts from an empty/seed corpus each
time unless one is built up locally during the session.

Also: `lofty/tests/fuzz` holds prior *crash artifacts* saved as regression test fixtures (files
that used to crash the library and now have an explicit test asserting they no longer do) - worth
a read before starting discovery, since it shows the classes of bugs this project has already been
bitten by (helps calibrate where a new one is more or less likely to still be lurking) but is not
itself something to re-investigate as a candidate on its own (it's a passing regression test, not
an open lead) unless a suspect specifically points back at a gap in one of those tests' coverage.

## Instrumentation/tooling set up for this workspace

Nothing needed setting up from scratch - miri, ASAN/UBSAN, and cargo-fuzz all work out of the box
in this environment with the toolchains already installed.

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
