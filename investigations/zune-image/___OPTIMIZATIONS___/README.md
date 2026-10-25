# ___OPTIMIZATIONS___/ - zune-image

Autonomous perf-optimize workspace for [zune-image](https://github.com/etemesi254/zune-image),
created 2026-09-30, base_commit `533296caf9a6783691d42b2dfbc623029216cbaf` (branch `dev`).

## Harness

The project already ships a criterion benchmark crate at `benchmarks/` (workspace member,
`zune-benches`), with benches in `benchmarks/benches/`:

- `decode_jpeg.rs`, `decode_jpeg_colorspaces.rs` - zune-jpeg vs mozjpeg, various colorspace outputs
- `decode_png.rs` - zune-png vs the `png` crate and `spng`
- `decode_qoi.rs` - zune-qoi vs rapid-qoi
- `decode_inflate.rs` - zune-inflate vs flate2/libdeflater
- `decode_hdr.rs` - HDR decode path
- `imageprocs.rs` - image processing operations (crate `zune-imageprocs`)

Test images live under `test-images/<format>/benchmarks/` (e.g.
`test-images/jpeg/benchmarks/speed_bench*.jpg`, `test-images/png/benchmarks/speed_bench*.png`) -
already-curated speed-bench inputs covering subsampling variants, interlacing, palette, 16-bit,
etc. No new test images needed so far; reuse these.

Run a single bench group directly, e.g.:

```
cargo bench --bench decode_jpeg
cargo bench --bench decode_png
```

Criterion itself handles warmup/statistical comparison/noise floor - for this workspace's paired
before/after comparisons we still follow MEASUREMENT.md's discipline (repeat runs, `taskset` for
single-core ranking, also an unpinned multi-core run) on top of criterion's own numbers rather than
trusting a single criterion run in isolation.

`perf` is available and usable (`perf_event_paranoid = -1`). Machine has 24 cores.

## Instrumentation added for this workspace

None applied to the project's own tracked `benchmarks/` crate yet as of workspace creation
(2026-09-30) - the existing criterion harness above was sufficient to start.

**Candidate 008 (QOI encode)** found this project had no *encode*-side QOI bench (only
`decode_qoi.rs`). A criterion `encode_qoi.rs` bench modeled on `decode_qoi.rs` was written and
tried first, but its baseline-vs-patched comparison turned out to be confounded by build-heat/CPU
frequency drift between the two measurement sessions (see `008.QOI_ENCODE_VEC_BUFFERED/RESULT.md`
for the full story) - it was not kept. A **standalone, non-criterion, one-shot bench**
(`008.QOI_ENCODE_VEC_BUFFERED/bench/src/main.rs`, adapted from `templates/bench_template.rs`) used
via a genuine alternating `A,B,A,B,...` paired comparison (build both binaries once, then
alternate) is what actually produced a trustworthy number for that candidate, and is the
recommended pattern for any future QOI-encode (or similarly short-running, sub-15ms) candidate on
this project: a long criterion session on this machine (`powersave` governor) is vulnerable to
exactly this drift when a baseline and patched criterion run are each preceded by a differently-
sized rebuild. Prefer `tools/paired.sh`-style alternation over two sequential criterion sessions
for anything this short.

## Correctness checks

The project's own test suite: `cargo test --workspace`. Run before and after every candidate per
SKILL.md's "Correctness" section, in addition to whatever bit-exact/pixel check the specific bench
already performs (most zune-image benches compare decoded pixel buffers against a reference
decoder's output as part of correctness, but that's a dev-time sanity check inside the bench, not
a substitute for the real test suite).
