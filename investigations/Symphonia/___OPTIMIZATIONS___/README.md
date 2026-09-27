# What is under test, and how it is measured

Read this first if picking the work up cold - it explains the harness, `SUMMARY.md` lists the
candidates measured with it.

## Workload

Test signal: 60s stereo 44.1kHz, sine (220Hz) + pink noise mix, generated deterministically with
`ffmpeg` (no downloads needed, fully reproducible from `ffmpeg -f lavfi ...`) then transcoded to
each container/codec under test. Files live in `___OPTIMIZATIONS___/testdata/`:

| file | what it stresses |
|---|---|
| `master.flac` | FLAC decode (bundle-flac) |
| `master.mp3` | MP3 decode (bundle-mp3, requires `symphonia/mp3` feature) |
| `master.ogg` | Ogg/Vorbis decode |
| `master.wav` | WAV/PCM decode (RIFF format reader) |
| `master.m4a` | ALAC decode (codec-alac, requires `symphonia/alac,symphonia/isomp4` features) |
| `master.aac.m4a` | AAC decode (codec-aac, requires `symphonia/aac,symphonia/isomp4` features) - added session 2, not yet used by any candidate |

Regenerate with:
```bash
cd ___OPTIMIZATIONS___/testdata
ffmpeg -y -f lavfi -i "sine=frequency=220:duration=60:sample_rate=44100[a];anoisesrc=duration=60:color=pink:sample_rate=44100[b];[a][b]amix=inputs=2:duration=first,aformat=channel_layouts=stereo" -c:a pcm_s16le master.wav -loglevel error
ffmpeg -y -i master.wav -c:a flac -compression_level 5 master.flac -loglevel error
ffmpeg -y -i master.wav -c:a libmp3lame -b:a 192k master.mp3 -loglevel error
ffmpeg -y -i master.wav -c:a libvorbis -q:a 5 master.ogg -loglevel error
ffmpeg -y -i master.wav -c:a alac master.m4a -loglevel error
ffmpeg -y -i master.wav -c:a aac -b:a 192k master.aac.m4a -loglevel error
```

## Measurement harness

`___OPTIMIZATIONS___/harness/bench_decode/bench_decode.rs` (written this session - not present
upstream, and NOT tracked in the main project's own `symphonia/examples/` directory, so the main
project's working tree stays clean - see "the one rule that matters most" in SKILL.md). Decodes
the whole file N times, hashing every decoded f32 sample (via `DefaultHasher`) each iteration and
aborting with a mismatch error if any iteration's hash differs from the first (or from an explicit
expected hash passed as arg 3) - this is the correctness check for every candidate in this
workspace, not just a separate step.

To use it against the live working tree during a session, copy it in temporarily:

```bash
cp ___OPTIMIZATIONS___/harness/bench_decode/bench_decode.rs symphonia/examples/bench_decode.rs
cargo build --release --example bench_decode -p symphonia --features symphonia/mp3
taskset -c 2 ./target/release/examples/bench_decode <path> <iterations> [expected_hash_hex]
# then remove it again: rm symphonia/examples/bench_decode.rs
```

Each candidate's own `justfile` copies it into its isolated `clone/` instead (see "Bench harnesses"
shape 3 in SKILL.md) - no manual copy needed when running `just test` in a candidate folder.

Prints `samples_per_decode`, `hash` (16 hex digits), and `min/p50/mean/p95/max` wall-clock time
per full-file decode. Use `taskset -c 2` for low-noise single-core ranking; also run unpinned for
realistic multi-core-context numbers when relevant (this workload is inherently single-threaded
per file, so unpinned numbers mostly show system-noise sensitivity here, not multi-core scaling).

Baseline current commit (`ee35874`), `taskset -c 2`, 20 iterations, `black_box`-guarded:

| format | min | p50 | mean |
|---|---|---|---|
| flac | ~21.0ms | ~21.1ms | ~21.2ms |
| mp3  | ~38.7ms | ~38.8ms | ~38.8ms |
| ogg  | ~28.0ms | ~28.0ms | ~28.1ms |
| wav  | ~13.3ms | ~13.3ms | ~13.4ms |

## Instrumentation added for this workspace

`bench_decode.rs` was written from scratch this session (no existing project benchmark reaches the
decode hot path directly - `symphonia-check` compares against reference decoders via
subprocess+ffmpeg, `symphonia-play` is an interactive player, neither suits repeated in-process
timing). It reuses the exact decode loop from `symphonia/examples/basic-interleaved.rs` almost
verbatim, adding timing + a running hash for correctness. Kept under
`___OPTIMIZATIONS___/harness/bench_decode/` (not in the main project's own `examples/` directory)
so the main project's working tree has nothing new to clean up between sessions - each candidate's
own justfile copies it into its isolated `clone/` at run time (see "Bench harnesses" shape 3 in
SKILL.md), and using it during a live session means copying it into `symphonia/examples/`
temporarily and removing it again afterward (see "Measurement harness" above).

## Environment

`perf_event_paranoid = -1` (root/full perf access), `perf`, `valgrind` availability not checked
(perf preferred). 24 logical cores. `ffmpeg`/`hyperfine` available for signal generation.

## Ground rules used in this workspace

1. No claim without a command that reproduces it - every candidate folder has the numbers and the
   exact command.
2. Every candidate that touches real source gets a real `patch.diff`, applied and built for real,
   plus the project's own tests where they cover the code.
3. Null results are written down with the same care as wins - a null result with no file is
   indistinguishable from "never checked".
4. If a later candidate changes the baseline enough that an earlier candidate's numbers no longer
   hold, don't silently rewrite that candidate's `RESULT.md` - add a short `RECHECK_<reason>.md`
   or `CORRECTION.md` in its folder explaining what changed and why, and update its `SUMMARY.md`
   row to point at it. History stays readable that way.
