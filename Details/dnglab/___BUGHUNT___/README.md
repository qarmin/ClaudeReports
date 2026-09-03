# What is under test, and how findings are confirmed

Read this first if picking the work up cold - it explains the tooling, `SUMMARY.md` lists the
candidates investigated with it.

`dnglab` is a raw-photo-to-DNG converter; the actual parsing/decoding surface lives in the
`rawler` crate (`rawler/src/`) - decoders for CR2/CR3/NEF/RAF/RW2/X3F/DNG/etc under
`rawler/src/decoders/`, shared container/format parsers under `rawler/src/formats/`
(TIFF/BMFF/CIFF/JFIF), and post-decode image processing (demosaic, color, rotation) under
`rawler/src/imgop/`. `bin/dnglab` is a thin CLI wrapper around `rawler`'s public API. This is
exactly the shape of codebase this skill targets: parsers/decoders over untrusted file input,
plus `unsafe`/raw-pointer code in the image-processing hot paths.

## Toolchain availability in this project

| tool | works here? | notes |
|---|---|---|
| `cargo +nightly fuzz build/run` (libFuzzer) | **Yes**, out of the box | `rawler/fuzz/` already exists with 8 targets (see below) and a working `Cargo.toml`. Default `cargo fuzz build`/`run` flags give ASAN + debug-assertions + overflow-checks automatically - no extra setup needed. |
| ASAN (`-Z sanitizer=address` on a plain `cargo +nightly build/run`, outside `cargo fuzz`) | **Yes** | Used for candidate 001's release-mode confirmation. First run needs `-Zbuild-std` (rebuilds `std` with ASAN) which takes several minutes; subsequent runs are fast. Command: `RUSTFLAGS="-Z sanitizer=address" cargo +nightly run --release --target x86_64-unknown-linux-gnu`. |
| `miri` | Not tried this session | `cargo-miri` is installed (`which cargo-miri` succeeds) but no candidate this session needed it - all confirmed findings reproduced via plain debug-assertions panics or ASAN. Worth trying first for any future `unsafe`/raw-pointer-only (no panic, no ASAN-visible symptom) suspect. |
| `cargo clippy --all-targets -- -W clippy::pedantic` | Works, but **very low signal-to-noise** (4200+ warnings, almost all style/precision-loss) on this codebase - not useful as a discovery tool here. Plain `cargo clippy` (no pedantic) not tried. |

## Existing project harnesses worth reusing

`rawler/fuzz/fuzz_targets/` (already in the repo, not added this session):

| target | exercises |
|---|---|
| `fuzz_get_decoder` | Format sniffing / decoder selection (`rawler::get_decoder`) - the very first step of opening any file. Cheapest, broadest-reach target; found candidate 003 (X3F) here. |
| `fuzz_decode_metadata` | Metadata-only decode path. |
| `fuzz_decode_full` | Full decode pipeline. |
| `fuzz_decode_unwrapped` | `#[doc(hidden)]` fuzzing-only entry point that bypasses format detection to hit decompressor internals directly - **not a genuine reachable path for any real caller**, see `IDEAS.md`'s note on its found hang. |
| `fuzz_bmff_parse` | `formats::bmff` (CR3/ISO-BMFF container parsing) directly. |
| `fuzz_tiff_parse` | `formats::tiff` directly. |
| `fuzz_jfif_parse` | `formats::jfif` (embedded JPEG/EXIF) directly. |
| `fuzz_ciff_parse` | `formats::ciff` (older Canon CRW container) directly. |

None had an existing corpus or crash directory before this session (`rawler/fuzz/corpus`,
`rawler/fuzz/artifacts` are gitignored and didn't exist on disk) - every finding below came from
cold, unseeded fuzzing. No sample raw files exist in this checkout either
(`rawler/data/testdata/` only has metadata digests, not the binary raw files themselves - they're
fetched separately via the `rawdb` feature/`devtools::rawdb`, not present in this sandbox), so
there was no way to seed a corpus with real camera files this session.

## Instrumentation/tooling set up for this workspace

Nothing needed adding - `cargo fuzz` and ASAN both worked with the versions already installed
(`cargo-fuzz` 0.12.0-ish, a recent nightly toolchain). One practical gotcha hit this session:
running multiple `cargo fuzz run` invocations for *different* targets concurrently from the same
`rawler/fuzz/` working directory makes them collide on cargo-fuzz's own `fuzz-0.log`/`fuzz-1.log`
worker-output filenames (relative to CWD, not per-target) - the per-target `-artifact_prefix`
still worked correctly (real crash inputs landed in the right `artifacts/<target>/` subdirs), but
the human-readable stderr logs got interleaved/overwritten across targets. Not worth fixing
properly for a single session; a future multi-target parallel run should either give each target
its own working directory copy or accept that only the artifact files (not the logs) are trustworthy
per-target.

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
