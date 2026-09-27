# What is under test, and how it is measured

Read this first if picking the work up cold - it explains the harness, `SUMMARY.md` lists the
candidates measured with it.

## Workload

`___OPTIMIZATIONS___/harness/bench_pipeline` is a standalone Rust crate (its own `[workspace]`
table, path-dependencies on `../../../{core,object,encoding,pixeldata,transfer-syntax-registry,
dictionary-std,json,ul}`) with a single `bench` binary exercising real dicom-rs library calls
end-to-end - not micro-benchmarks of internals, except where noted. `dicom-ul` (default features
off, no tokio) was added in candidate 020 for the network/DIMSE transmission modes below. Usage:
`bench <mode> <args...>`.

| mode | what it stresses |
|---|---|
| `parse <file> <iters>` | `dicom_object::from_reader` (full parse: file meta + dataset) |
| `write-only <file> <iters>` | parses once outside the loop, then repeats `obj.write_all()` on the same held object - isolates write-without-reparse |
| `write-only-peak-mem <file>` | one-shot `obj.write_all()`, then reports peak RSS (`VmHWM`) - same technique as `write-dataset-peak-mem`/`transcode-to-native-peak-mem`, used for candidates 001/008/016's peak-RSS follow-up |
| `write-dataset-only <file> <iters>` | like `write-only` but calls `InMemDicomObject::write_dataset_with_ts` directly (dataset only, no preamble/meta group) instead of `write_all` - the actual entry point candidate 019's borrowed write path changed; `write-only`/`write_all` does NOT exercise that fix, see 019's RESULT.md |
| `write-dataset-peak-mem <file>` | one `write_dataset_with_ts` call, then reports this process's own peak RSS from `/proc/self/status`'s `VmHWM` - added because `/usr/bin/time -v` isn't available in this environment (candidate 019) |
| `dump-dataset <file>` | one-shot `write_dataset_with_ts` output to stdout (dataset only, no preamble/meta) - like `dump` but for the entry point candidate 019 changed |
| `verify-borrowed-edge-cases` | builds a synthetic in-process object (charset_changed, two-level nested SQ, encapsulated pixel data with a non-empty offset table and a mix of empty/non-empty fragments) and prints one checksum per `write_dataset*` method variant - for candidate 019's differential correctness check, since no single real fixture combines all of these |
| `write-dataset-only-consuming` / `write-dataset-peak-mem-consuming` / `dump-dataset-consuming` | same shape as their non-`-consuming` counterparts above but call `write_dataset_with_ts2` (the `self`-consuming comparison method) instead - **requires `--features comparison-ts2`**, see "Instrumentation added" below; used to compare candidate 019's borrowed fix against the issue's own originally-proposed consuming API |
| `roundtrip <file> <iters>` | parse + write every iteration - the realistic "load, modify, save" pattern |
| `dump <file>` | one-shot `write_all` output to stdout, for byte-diffing two builds |
| `colorspace <file> <iters>` | `decode_pixel_data()` + `to_dynamic_image(0)` - decode + LUT/colorspace conversion together |
| `colorspace-fast <file> <iters>` | same as `colorspace` but a length-only checksum instead of a full hash - **only safe to use above ~300k pixels**, see the "colorspace-fast gotcha" note below |
| `colorspace-normalize <file> <iters>` | same as `colorspace` but forces `ConvertOptions::new().with_voi_lut(VoiLutOption::Normalize)` via `to_dynamic_image_with_options` - the only mode that reliably exercises `Lut::new_rescale_and_normalize`'s min/max-scanning fold regardless of whether the fixture has its own window center/width (added for candidate 018) |
| `dump-image <file>` | one-shot raw decoded-image bytes to stdout, for byte-diffing |
| `tovec <file> <iters>` | `decode_pixel_data().to_vec_frame::<f32>(0)` - isolates `convert_pixel_slice`'s LUT-mapped/plain-numeric conversion path |
| `decode-raw <file> <iters>` | `decode_pixel_data()` alone - isolates a codec adapter's own decode, no LUT/windowing/colorspace on top |
| `decode-frame-raw <file> <iters>` | `decode_pixel_data_frame(0)` alone - single-frame codec adapter decode only |
| `text-decode <iters>` / `text-encode <iters>` | isolated `SpecificCharacterSet::from_code("ISO_IR 100")` decode/encode microbenchmark on fixed Latin-1 content |
| `text-decode-cs <code> <iters>` / `text-encode-cs <code> <iters>` | same shape as `text-decode`/`text-encode` but for any `SpecificCharacterSet` code, with real multi-byte sample text (Japanese/Korean/Chinese) for the 5 remaining generic-`encoding`-crate-backed codecs - `ISO_IR 13` (WINDOWS_31J), `ISO_IR 87` (ISO_2022_JP), `ISO_IR 149` (WINDOWS_949), `GB18030`, `GBK` - added for candidate 049's profiling step |
| `json-encode <file> <iters>` | `dicom_json::to_string(&obj)` |
| `json-decode <file> <iters>` | `dicom_json::from_str` alone (encodes to JSON once outside the timed loop) - isolates the deserialize path; checksum is a hash of the last-decoded object re-serialized back to JSON, a full content round-trip check, not just an element count |
| `json-decode-fromvalue <file> <iters>` | `dicom_json::from_value` alone (builds a `serde_json::Value` once outside the timed loop, clones it per iteration outside the timer since `from_value` consumes its argument) - isolates the entry point that never has raw JSON text at all, for candidate 026's own follow-up (candidate 037) |
| `json-decode-peak-mem <file>` | single `dicom_json::from_str` call, then reports peak RSS (`VmHWM`) - same technique as `write-only-peak-mem`, for candidate 026's peak-RSS follow-up |
| `dictlookup <iters>` | `StandardDataDictionary::by_tag` throughput over a realistic tag mix (10,000 lookups/iter) |
| `info <file>` | one-shot: prints BitsAllocated/SamplesPerPixel/PhotometricInterpretation/NumberOfFrames/PlanarConfiguration/Rows/Columns - use to check a fixture's shape before trusting assumptions about it |
| `gen-ybr <src> <out> <row-mult>` | tiles a real `PlanarConfiguration::Standard` YBR/RGB source's pixel data vertically to reach an arbitrary pixel count |
| `gen-ybr-random <src> <out> <row-mult>` | same shape as `gen-ybr` but pixel data is a deterministic xorshift PRNG fill - needed to exercise rounding-boundary differences that tiled real content may never hit |
| `gen-planar-rgb <src> <out> <target-rows> [bits-allocated]` | synthetic `PlanarConfiguration::PixelFirst`, `PhotometricInterpretation=RGB` fixture (random plane content) - isolates `interleave()` alone, no colorspace conversion. Optional `bits-allocated` (candidate 034, default 8) - pass `16` for a `BitsAllocated=16`/`OW` fixture instead, to exercise `interleave::<u16>()` (use a 16-bit source like `pydicom/SC_rgb_16bit.dcm` so BitsStored/HighBit stay consistent) |
| `gen-mono <src> <out> <row-mult>` | tiles a `SamplesPerPixel=1` (monochrome) source vertically |
| `gen-at <out> <n-tags>` | builds a synthetic object with one AT-VR element (`ATTRIBUTE_IDENTIFIER_LIST`) containing `n-tags` distinct `Tag` values, Implicit VR LE (AT's 2-byte explicit-VR length field caps at ~16,000 tags) - for measuring `AsAttributeTags`'s JSON serialization (candidate 035), since no real `dicom-test-files` fixture exercises the AT VR |
| `gen-datetime-vrs <out> <n-items>` | builds a flat object with `n-items` groups of 5 single-valued elements (one DA, DS, DT, IS, TM each, distinct private-group tags) - for measuring `read_value_da`/`ds`/`dt`/`is`/`tm`'s single-value fast path (candidate 043), since no real fixture has enough of these VRs to time meaningfully |
| `gen-fragmented <src> <out> <n-fragments> <fragment-size-mb>` | replaces the source's PixelData with a large synthetic encapsulated `PixelFragmentSequence` (n-fragments x fragment-size-mb MB of raw, non-JPEG bytes - structural only, not decodable) - for candidates 001/008/016's peak-RSS follow-up, since `color3d_jpeg_baseline.dcm` (6.14MB) is too small for their memory effect to clear this process's own fixed footprint |
| `gen-seq <out> <n-items> [str-len]` | builds a synthetic object with one `SQ` element containing `n-items` nested objects (2 short `UI`-VR strings each) - for many-small-elements/text-heavy measurement, since no real `dicom-test-files` fixture is large enough for this (see below). Optional `str-len` (candidate 032) adds a single-valued `LO` element of exactly that many bytes to each item, for measuring a per-element-value-length-scaling effect that the short UI-length values alone can't separate from noise |
| `gen-flat <out> <n-elements>` | builds an object with `n-elements` top-level `UI`-VR string elements (private-group tags, no `SQ` nesting) - candidate 026's own `gen-seq`-generated fixtures compared directly, at the same total element count, to isolate an `SQ`-recursion effect from a flat-multi-element effect (candidate 036) |
| `pdatawriter-throughput <payload-mb> <iters>` | real loopback TCP client+server association (`dicom-ul`), times `send_pdata`/`write_all`/`finish` on an N MB payload, server reconstructs and checksums it - realistic (but noisy, see candidate 020) transmission-time measurement |
| `pdatawriter-inprocess <payload-mb> <iters>` | same write path but into an in-memory `Vec<u8>` sink instead of a real socket, isolating candidate 020's userspace-copy-avoidance mechanism from socket/kernel noise - **requires `--features pdatawriter-inprocess,dicom-ul/bench-internals`**, see "Instrumentation added" below |
| `async-pdatawriter-inprocess <payload-mb> <iters>` | async counterpart to `pdatawriter-inprocess` (`AsyncPDataWriter` instead of `PDataWriter`), isolating candidate 029's direct-dispatch mechanism the same way - **requires `--features async-pdatawriter-inprocess,dicom-ul/async,dicom-ul/bench-internals`**, see "Instrumentation added" below |
| `async-pdatawriter-peak-mem <payload-mb>` | one-shot peak-RSS probe (`VmHWM`) for candidate 029's fix, same technique as `write-dataset-peak-mem` (candidate 038) - same feature/shim requirements as `async-pdatawriter-inprocess` above |
| `transcode-to-native <file> <iters>` | `Transcode::transcode` from an encapsulated transfer syntax to a native/uncompressed one - exercises `transcode.rs`'s `decode_inline` (candidate 021's target) |
| `transcode-to-native-peak-mem <file>` | one-shot `transcode` + `write_all`, then reports peak RSS (`VmHWM`) - same technique as `write-dataset-peak-mem` (candidate 019), for candidate 021 |
| `overlay-decode <rows> <cols> <iters>` | builds a synthetic in-memory object with one standard-packed overlay plane and calls `PixelDecoder::decode_overlays()` - isolates `unpack_overlay_bits` (candidate 022's target) |

All `<file>` arguments accept either a literal filesystem path, or a `pydicom/<name>` /
`WG04/<name>` style name that gets resolved via the `dicom-test-files` crate (downloaded once,
cached under this harness's own `target/dicom_test_files/` - see the "target/ directory
constraint" note below). `resolve_path()` tries the literal path first and falls back to
`dicom_test_files::path()` if it doesn't exist, so any real dicom-test-files name works without
the `pydicom/` prefix (needed for e.g. `WG04/RLE/CT1_RLE`).

## Measurement harness

Each mode prints one line: `<mode> <file> (n=<N> min=<X>us p50=<X>us mean=<X>us p95=<X>us
max=<X>us/iter) checksum=<hex>`. `stats()` discards the first ~10% of samples as warmup, sorts the
rest, and reports min/p50/mean/p95/max. `checksum` is an FNV-1a hash (`fnv1a()`) of whatever the
mode's own "hash this to prove no dead-code-elimination and let two builds be compared" data is
(decoded bytes, encoded JSON text, etc.) - modes that need real bit-exactness verification (not
just "didn't crash") use `dump`/`dump-image` piped through `sha256sum` instead, since FNV-1a alone
isn't collision-resistant enough to trust for a correctness claim.

`tools/paired.sh` (copied from the skill's own `templates/tools/`, adapted `run()` to grep
`p50=<N>(us|ms)` from this harness's output and convert to milliseconds) implements the
alternating-A/B-runs paired comparison discipline - always use this over hand-rolled
before/after loops, see MEASUREMENT.md section 2b for why unpaired medians are unreliable on a box
with real background load (confirmed present here - `uptime` swung between ~2 and ~26 across this
session's own measurements).

## Instrumentation added for this workspace

**`cargo build --release --no-default-features`** now produces a binary with zero `rayon_core`
symbols linked (confirmed via `nm target/release/bench | grep -c rayon_core` = 0), for single-core
measurements of any rayon-threshold-guarded candidate. Added by candidate 023 after finding that
candidates 012/018's own prose described this exact build ("`--no-default-features --features
image,native` build, no rayon linked") but the harness `Cargo.toml` as it existed at the time
couldn't actually produce it - `dicom-pixeldata`'s dependency line had no `default-features =
false`, so its own `rayon` default feature stayed linked regardless of flags passed to the harness
build. Fixed by adding `default-features = false` to the `dicom-pixeldata` dependency line and a
new `bench_pipeline` feature `default = ["dicom-pixeldata/rayon"]`, so a plain `cargo build
--release` still behaves exactly as before (rayon on by default) but `--no-default-features` now
genuinely strips it. Use this for any future single-core rayon-guard measurement instead of
re-deriving a workaround.

**`--features diag-mimalloc`** swaps the harness's global allocator to `mimalloc` instead of the
system default (glibc malloc on this box). Added specifically to distinguish "this change is a
real regression" from "this change is only a regression under glibc's specific allocator behavior
for this access pattern" - used successfully twice this session (candidates 005 and 016, see their
`RESULT.md`'s own "Follow-up" sections) to reverse an initial "large, reproducible regression"
finding into "real win" / "no regression" once measured under a different allocator. Not the
harness's default configuration (baseline candidate measurements throughout this workspace use the
system allocator, for consistency) - only build with this feature when specifically investigating
whether an allocator-arena effect explains a surprising result.

**`--features diag-jemalloc`** - same diagnostic purpose as `diag-mimalloc` above, but swaps to
`tikv-jemallocator` instead. Added by candidate 041 to corroborate candidates 005/016's
allocator-specific-regression findings with a second, architecturally different allocator (both
regressions resolved the same way under jemalloc as they did under mimalloc). Mutually exclusive
with `diag-mimalloc` and `diag-alloccount` (all three define `#[global_allocator]`).

**`--features diag-alloccount`** swaps the global allocator to a thin wrapper around `System` that
counts every `alloc`/`dealloc` call via atomics, for measuring an allocation-*count*-reducing fix
(peak RSS/`VmHWM` can't see this dimension at all if each allocation is small - see candidate
016's own "peak-RSS follow-up" for a concrete case where this mattered). Pairs with the
`write-dataset-alloc-count <file> <iters>` mode, and (candidate 040) `write-only-alloc-count
<file> <iters>` - the `write_all`-based counterpart, used to root-cause candidate 001's own
Ir-count-vs-wall-clock gap by confirming its allocation-count delta matches the fixture's own
frame count exactly. Mutually exclusive with `diag-mimalloc` (both define `#[global_allocator]`) -
don't build with both features at once.

**`--features comparison-ts2`** enables the harness's `write-dataset-peak-mem-consuming`/
`write-dataset-only-consuming`/`dump-dataset-consuming` modes, which call
`InMemDicomObject::write_dataset_with_ts2` - a method that only exists once candidate 019's
`patch.diff` (its fenced "comparison-only code" block, see that candidate's RESULT.md) is applied.
Off by default so a plain baseline build of the harness (no patch applied) still compiles; only
needed when reproducing candidate 019's comparison between the borrowed and consuming write paths.

**`--features pdatawriter-inprocess,dicom-ul/bench-internals`** (both, always together) enables the
`pdatawriter-inprocess` mode, which needs `PDataWriter::new_for_bench` - a **benchmark-only** shim
(`ul/Cargo.toml` + `ul/src/association/pdata.rs`) that is applied and reverted as its own step by
candidate 020's `justfile` (`bench-shim.diff`), separate from `patch.diff` - it is not part of the
real fix and must not be treated as such if read out of context. This harness's own
`pdatawriter-inprocess` feature deliberately does **not** forward to `dicom-ul/bench-internals` in
`Cargo.toml` (that forward would be validated by Cargo at manifest-resolution time even for a
plain default build with the feature off, breaking it whenever the shim isn't applied - see
`Cargo.toml`'s own comment on this feature and MEASUREMENT.md's "stale optional-feature forward"
pitfall) - both features must be requested explicitly together on the command line. See candidate
020's RESULT.md "Instrumentation added for this candidate" for the full reasoning.

**`--features async-pdatawriter-inprocess,dicom-ul/async,dicom-ul/bench-internals`** (all three,
always together) enables the `async-pdatawriter-inprocess` mode, the async counterpart to
`pdatawriter-inprocess` above - same reasoning, same benchmark-only-shim caveat
(`AsyncPDataWriter::new_for_bench`, applied/reverted by candidate 029's own `bench-shim.diff`), and
deliberately kept as its own separate harness feature (not shared with `pdatawriter-inprocess`)
so this mode's own compilation never depends on candidate 020's sync-side shim also being applied
at the same time. `dicom-ul/async` (unlike `bench-internals`) is a real, permanent feature of
`dicom-ul` not tied to any unapplied patch - it's still passed explicitly rather than added to this
harness's own default features, since nothing else in the harness needs it. See candidate 029's
RESULT.md for the full reasoning.

The harness (`harness/bench_pipeline/`) was built from scratch this session (no `criterion`
dependency, no `benches/`, no `examples/` exist anywhere in the real dicom-rs workspace) and grown
incrementally across candidates rather than rewritten - see "Bench harnesses: grow a few, don't
multiply" in `SKILL.md`. The generator modes (`gen-*`) exist because real `dicom-test-files`
fixtures were, in every case tried, either too small (`rtplan.dcm`, 2.7KB - not enough sequence
content to profile) or shaped wrong for the specific question being asked (e.g. no real
`PlanarConfiguration::PixelFirst` + `YbrFull` fixture exists anywhere in the ~100-file `pydicom/`
set, needed to test `interleave()` combined with colorspace conversion).

## Environment

- 24 logical cores, rustc/cargo 1.98.0 (edition 2024). `valgrind` (callgrind + massif) is
  available; `perf`/`strace`/`ltrace` are not. `cargo +nightly miri` works
  (`MIRIFLAGS=-Zmiri-disable-isolation` needed - without it, any test printing a `snafu::Backtrace`
  hits `getcwd` under isolation and aborts the whole run, unrelated to whatever unsafe code is
  actually being checked).
- Disk: single large filesystem, checked periodically during the session, never became tight.
- **`dicom_test_files::get_data_path()` finds its download cache by walking up from the running
  binary's own path looking for an ancestor directory literally named `target`.** A mode that
  calls `dicom_test_files::path()` at runtime must be run from inside `.../target/release/` (or
  have the fixture pre-resolved/copied to a literal path first) - copying the built `bench` binary
  out to a candidate folder root and running it from there panics with "Cannot find target
  directory". Modes that only take a literal path argument (most `gen-*` outputs, `dump-image`,
  etc.) don't have this constraint. Every candidate's own `justfile` in this workspace copies
  pre-resolved `.dcm` fixtures into its own `testdata/` specifically to avoid needing network
  access or the `target/` constraint at `just test` time.
- **`colorspace-fast`'s length-only checksum can disagree in *direction* from `colorspace`'s real
  hash at small pixel counts** (confirmed once, candidate 003: patched measured ~10-13%
  slower via `colorspace-fast` but ~6-8.5% faster via `colorspace` at 10,000px, both reproducible) -
  plausibly a cross-core cache-line migration effect where only the real-hash pass re-reads the
  full converted buffer. Don't use `colorspace-fast` below ~300k pixels; re-verify against
  `colorspace` if the two ever disagree.
- **A callgrind `Ir` delta is not reliable evidence for wall-clock direction when a change alters
  allocation patterns**, confirmed multiple times this session: a case with *fewer* total `Ir` and
  *faster* execution under Valgrind's own instrumentation was reproducibly *slower* in real,
  uninstrumented execution (candidate 016) - and separately, callgrind's `Ir` count is not
  deterministic run-to-run for any `rayon`-enabled (multi-threaded) binary at all (candidate 030,
  ~23% swing between two runs of the identical binary), since Valgrind serializes
  multi-threaded execution onto one virtual CPU and how much of a work-stealing loop each thread
  executes before another grabs a job varies with that serialization's own scheduling luck. Treat
  `Ir` profiles as directional hints for *where* to look, never as a substitute for a real paired
  wall-clock measurement of the actual change.

## Ground rules used in this workspace

1. No claim without a command that reproduces it - every candidate folder has the numbers and the
   exact command.
2. Every candidate that touches real source gets a real `patch.diff`, applied and built for real,
   plus the project's own tests (`cargo test --workspace`) before and after - and Miri
   (`cargo +nightly miri test -p <crate> --lib`) for any candidate introducing `unsafe` code.
3. Null results are written down with the same care as wins - a null result with no file is
   indistinguishable from "never checked". This workspace has several (candidates 007, 015, 017).
4. Always measure via `write-only`, `parse`, *and* `roundtrip` before trusting a write-path
   change's direction - this session found three separate cases (candidates 001, 005, 016) where a
   change looked fine (or like a clean win) in one or two of these modes but showed a large,
   reproducible effect in a different combination. Two of the three (005, 016) were confirmed to be
   **glibc-allocator-specific artifacts, not fundamental** - rebuilding the harness with
   `--features diag-mimalloc` (swaps the global allocator to `mimalloc`, added this session) made
   both regressions disappear or turn into real wins. See
   `~/.claude/skills/perf-optimize/projects/dicom-rs.md`'s "recurring, unexplained phenomenon"
   section for the full detail and its resolution - always try the `diag-mimalloc` feature before
   concluding a write-path change is a genuine regression on this project.
5. If a later candidate changes the baseline enough that an earlier candidate's numbers no longer
   hold, don't silently rewrite that candidate's `RESULT.md` - add a short `RECHECK_<reason>.md`
   or `CORRECTION.md` in its folder explaining what changed and why, and update its `SUMMARY.md`
   row to point at it. History stays readable that way. (Candidate 005's `RESULT.md` was instead
   substantially rewritten in place, with its own "Revision history" section, since both rounds
   were within the same session and rewriting in place kept the full story in one readable place -
   see that file for the reasoning.)
