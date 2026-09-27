# What is under test, and how findings are confirmed

Read this first if picking the work up cold - it explains the tooling, `SUMMARY.md` lists the
candidates investigated with it.

## Toolchain availability in this project

Symphonia's decoders/demuxers (`symphonia-bundle-flac`, `symphonia-bundle-mp3`,
`symphonia-codec-*`, `symphonia-format-*`) are almost entirely safe Rust - a repo-wide grep for
`unsafe` blocks/fns/impls (excluding `unsafe_code` lint attributes) found real `unsafe` only in
`symphonia-play/src/resampler.rs` (a player example binary, not a library crate, wrapping the
`rubato` resampler's unchecked-access adapter trait). That means:

- **miri**: low value here - there's essentially no `unsafe` in the actual parsing/decoding code
  paths to catch UB in. Not installed on the active toolchain (`rustup component add miri` would
  be needed on a nightly toolchain - several are already installed, e.g.
  `nightly-x86_64-unknown-linux-gnu`). Skip by default; only worth adding if a candidate's own
  suspicion is specifically about `symphonia-play`'s resampler adapter.
- **ASAN/UBSAN**: same reasoning - low expected value on pure-safe-Rust decode paths. Debug
  assertions (Rust's own overflow checks in a non-release build) cover the "arithmetic on
  untrusted sizes" class for free and are cheaper to run than a sanitizer build. Not set up this
  session; revisit only if a candidate's suspicion is specifically about a C dependency or FFI
  (none found so far - this is a pure-Rust workspace).
- **cargo-fuzz**: works out of the box. `cd symphonia/fuzz && cargo +nightly fuzz build <target>`
  built and ran cleanly on the first try (verified with `decode_flac`, `-max_total_time=20`,
  release profile). This is the primary tool for this project - most bugs here are expected to be
  panics (index out of bounds, arithmetic overflow caught by fuzz build's debug-assertion-like
  overflow checks - libfuzzer-sys builds in release by default; add `-C debug-assertions` via
  `RUSTFLAGS` or `cargo fuzz build -O -d` for overflow checks alongside fuzzing throughput) or
  logic bugs (wrong decoded sample values, wrong seek position, incorrect metadata) on malformed
  or adversarial input.
  - `cargo +nightly fuzz run <target> -- -max_total_time=N` - `N` is a soft budget, actual wall
    time can run somewhat longer (observed ~30-40s wall for `-max_total_time=20` once startup/JIT
    overhead is included - budget accordingly, don't assume it cuts off exactly at N).
  - `cargo +nightly fuzz build <target>` alone is a fast, valid sanity/regression check even
    without running the fuzzer (compiles the harness against the current tree).
- **`cargo clippy` pedantic**: works normally (`cargo clippy --version` confirms 0.1.98). Use for
  discovery per the standard heuristic sweep.

## Existing project harnesses worth reusing

`symphonia/fuzz/fuzz_targets/` already has one libFuzzer target per decoder and per
demuxer/container format - **do not build a new standalone harness from `templates/fuzz_harness_template.rs`
for a codec/format that already has one here**, reuse the existing target instead (add a seed
input to `symphonia/fuzz/corpus/<target>/` if useful, don't duplicate the harness itself):

| target | covers |
|---|---|
| `decode_any` | any codec, dispatched generically |
| `decode_mpa` | MP3/MPEG audio (`symphonia-bundle-mp3`) |
| `decode_flac` | FLAC (`symphonia-bundle-flac`) |
| `decode_aac` | AAC (`symphonia-codec-aac`) |
| `decode_vorbis` | Vorbis (`symphonia-codec-vorbis`) |
| `decode_pcm` | PCM (`symphonia-codec-pcm`) |
| `decode_adpcm` | ADPCM (`symphonia-codec-adpcm`) |
| `decode_alac` | ALAC (`symphonia-codec-alac`) |
| `demux_mkv` | Matroska/WebM (`symphonia-format-mkv`) |
| `demux_isomp4` | MP4/M4A (`symphonia-format-isomp4`) |
| `demux_ogg` | Ogg (`symphonia-format-ogg`) |
| `demux_flac` | native FLAC container |
| `demux_wav` | WAV/RIFF (`symphonia-format-riff`) |
| `demux_aiff` | AIFF (`symphonia-format-riff`) |
| `demux_caf` | Core Audio Format (`symphonia-format-caf`) |

Not covered by an existing fuzz target: `symphonia-codec-wavpack` (WavPack), and metadata-only
paths in `symphonia-metadata` (ID3v2/APEv2/Vorbis comments etc. parsed standalone rather than via
a demuxer) - these are the ones a new standalone harness (from `templates/fuzz_harness_template.rs`)
actually makes sense for, if a candidate targets them.

**No seed corpus exists anywhere in this repo** (`find . -iname '*.flac' -o -iname '*.mp3' -o
-iname '*.ogg' -o -iname '*.wav' -o -iname '*.m4a' -o -iname '*.mkv' -o -iname '*.aac'` under the
project root, excluding `target/`, returned nothing). Every fuzz run starts from libFuzzer's own
empty-corpus bootstrap, which is slow to get past magic-byte/header checks for formats with
non-trivial framing (MP4, MKV especially). If a candidate needs faster coverage: hand-craft a
minimal valid file for the target format (a few dozen bytes satisfying just the required header
fields) and drop it in `symphonia/fuzz/corpus/<target>/` before running - much cheaper than
fuzzing from nothing for that format. Don't download external audio test files from the internet
for this purpose without checking with whoever is driving the session first.

## Instrumentation/tooling set up for this workspace

Nothing beyond what ships in the repo - `symphonia/fuzz` already builds and runs cleanly on the
active stable+nightly toolchains present on this machine. No sanitizer/miri setup was performed
(see above - low expected value for this codebase's safe-Rust decode paths).

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
