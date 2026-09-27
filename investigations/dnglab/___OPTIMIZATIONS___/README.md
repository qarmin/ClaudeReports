# What is under test, and how it is measured

Read this first if picking the work up cold - it explains the harness, `SUMMARY.md` lists the
candidates measured with it.

Scope of this whole workspace: the **`rawler`** crate (`rawler/`) only - the library used to
decode camera raw files and convert them to DNG/PNG/JPG. `bin/dnglab` (the CLI) and `embedftp` are
out of scope; don't spend candidates there even if something looks slow, unless it's purely a
thin wrapper calling straight into `rawler`.

## Workload

`rawler` already ships 4 criterion benches under `rawler/benches/`, all reused as-is (no new
harness needed for the first candidates):

| bench | what it stresses |
|---|---|
| `raw_decoder.rs` | `decoder.raw_image(...)` - reads the raw sensor data out of CR2/CR3/DNG files, several cameras/compression variants (Canon CR2/CR3/sRAW, several synthetic DNG compression variants) |
| `demosaic.rs` | `RawDevelop::new_with(&[ProcessingStep::Demosaic])` on an already-decoded `RawImage` - the demosaic/develop step, Fuji X-Trans (`X-T5`) and Bayer (Canon `EOS R5`) |
| `convert.rs` | `dng::convert::convert_raw_file` - full raw -> DNG pipeline (decode + repackage), Canon CR2 and CR3 |
| `perf.rs` | standalone, no raw file needed - `LjpegCompressor::encode` on synthetic 3000x2000 16-bit data; isolates the lossless-JPEG (LJPEG92) encoder used when writing compressed DNGs |

Note: `convert.rs` measures conversion **to DNG**, not to PNG/JPG. The PNG/JPG-facing path (what
the user actually cares about per this session's request) is `imgop::develop::RawDevelop` +
whatever turns the resulting buffer into a `image::DynamicImage`/PNG/JPEG (see
`src/imgop/develop.rs`, `src/analyze.rs` for how the `dnglab` CLI's own `--preview`/thumbnail-style
export drives it) - `demosaic.rs` covers the develop half already; if a candidate needs the
PNG/JPEG encode step specifically (not currently benched), that's new-harness work, see
"Instrumentation" below for how to add a mode rather than a whole new bench file.

## Measurement harness

Criterion (`cargo bench`), `harness = false` in `Cargo.toml`, standard `BenchmarkGroup` /
`bench_with_input` per sample. Each bench downloads its sample files on first run via
`rawdb_ensure_file` (see "Environment" below) and caches them - after the first run, benches are
fully offline.

Run a single bench:

```
RAWDB_CACHE=../target/rawdb_cache cargo bench -p rawler --features rawdb --bench raw_decoder
```

(`target/rawdb_cache` - gitignored, under the project's own `target/`, not `/tmp` - COMMANDS.md's
"/tmp is not durable" applies directly here: this cache needs to survive the whole session across
many candidates, and `/tmp` has been observed wiped mid-session in practice.)

Criterion's own baseline compare is the primary A/B mechanism for this project (preferred over
`tools/paired.sh`'s external "one bench -> one ms number" shape, which doesn't fit criterion's
own statistical treatment well):

```
# before the patch
cargo bench -p rawler --features rawdb --bench raw_decoder -- --save-baseline before
# after applying patch.diff
cargo bench -p rawler --features rawdb --bench raw_decoder -- --save-baseline after
critcmp before after   # or: cargo bench ... -- --baseline before  (prints regressed/improved inline)
```

Criterion reports its own significance verdict per benchmark ("Performance has
regressed/improved", with confidence bounds) - that satisfies MEASUREMENT.md's noise-floor
discipline without needing `tools/paired.sh` for these benches. `tools/matrix.sh`/`paired.sh` are
still there if a candidate needs a raw wall-clock comparison outside criterion (e.g. comparing the
`dnglab` CLI's end-to-end conversion time, or a bench that isn't criterion-based).

`--test` (as opposed to no flag) runs every benchmark function exactly once with no statistics -
useful only to smoke-test that a bench still builds/downloads/runs, never to draw a performance
conclusion from.

Correctness check for candidates: for the decode/demosaic/convert paths, output is a byte buffer
(`RawImage.data` / DNG bytes) - hash it (or compare structurally) before/after the patch across
every sample the bench touches, not just that criterion's numbers look sane. `rawler` also has a
large existing integration test suite reading real raw files (`rawler/tests/`, driven by
`RAWDB_CACHE` too) - run the relevant subset (e.g. `cargo test -p rawler --features rawdb
decoders::`) as an additional correctness signal when a candidate touches decoder code.

## Environment

- `RAWDB_CACHE=<dir>` **must** be set (the crate panics otherwise) - it's the on-disk cache
  directory for downloaded sample raw files. Use `<project_root>/target/rawdb_cache` (gitignored,
  durable for the session - NOT `/tmp`, see COMMANDS.md). First run downloads only the files the
  benches actually touch (~200MB so far for `raw_decoder`, not the full ~100 GiB dataset) from
  `https://rawdb.dnglab.org` - confirmed working with no `RAWDB_API_KEY` needed for the samples the
  bundled benches use. Already populated at `target/rawdb_cache` as of this session for
  `raw_decoder`'s samples - reuse it, don't re-download.
- Benches (and the `rawdb` devtools module generally) are gated behind the `rawdb` Cargo feature -
  `cargo build`/`bench` **without** `--features rawdb` fails to compile `raw_decoder`/`convert`/
  `demosaic` (E0432, module compiled out). `perf.rs` doesn't need the feature (no raw file I/O).
- Bench build (`cargo bench` profile, `opt-level = 3`, `lto = false` per workspace `Cargo.toml`)
  takes ~30s from a warm `target/` on this machine (24 cores). A full candidate cycle (build +
  bench + revert) is therefore cheap - closer to the "fast/cheap project" end of SKILL.md's budget
  table than the "needs a harness built from scratch" end.
- No `perf_event_paranoid` check done yet this session - fall back to criterion's own timing (see
  above) or in-code phase timers if `perf` isn't usable when a candidate needs instruction-level
  profiling.

## Ground rules used in this workspace

1. No claim without a command that reproduces it - every candidate folder has the numbers and the
   exact command (including the `RAWDB_CACHE`/`--features rawdb` flags above - easy to forget and
   silently get a compile error instead of a benchmark).
2. Every candidate that touches real source gets a real `patch.diff`, applied and built for real,
   plus the project's own tests where they cover the code.
3. Null results are written down with the same care as wins - a null result with no file is
   indistinguishable from "never checked".
4. If a later candidate changes the baseline enough that an earlier candidate's numbers no longer
   hold, don't silently rewrite that candidate's `RESULT.md` - add a short `RECHECK_<reason>.md`
   or `CORRECTION.md` in its folder explaining what changed and why, and update its `SUMMARY.md`
   row to point at it. History stays readable that way.
