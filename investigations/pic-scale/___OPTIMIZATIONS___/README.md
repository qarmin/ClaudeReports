# What is under test, and how it is measured

Read this first if picking the work up cold - it explains the harness, `SUMMARY.md` lists the
candidates measured with it.

## Workload

`app`'s criterion benches under `app/benches/{resize_rgb,resize_rgba,resize_cbcr,resize_plane}/main.rs`
each resize `../assets/nasa-4928x3279-rgba.png` to 1/4 size with various filters/pixel layouts,
comparing pic-scale against `fast_image_resize` (and, for some cases, Apple Accelerate). `harness
= false`, so each is its own criterion binary - run individually with `cargo bench -p app --bench
<name>` or `cargo bench -p app --bench <name> -- "<case name>"` to filter to one case.

| mode/scene | what it stresses |
|---|---|
| `resize_rgba` "Pic scale RGBA with alpha: Lanczos 3" | RGBA u8 horizontal+vertical convolution, `PreferQuality`, alpha premultiply, single-threaded |
| `resize_rgba` "... without alpha ..." | same, no alpha channel handling |
| `resize_rgb`/`resize_plane`/`resize_cbcr` | analogous for RGB, single-channel, and CbCr layouts |

All cases in `resize_rgba/main.rs` explicitly set `ThreadingPolicy::Single` - there is currently
no multi-threaded variant of this harness (see "Ground rules" below for what "multi-core
measurement" means here as a result).

## Measurement harness

Criterion (`harness = false`, criterion 0.8.2) prints one `time:   [low mean high]` line per
`bench_function` after warmup+sampling; that's the number `tools/paired.sh` parses (see below).

## Instrumentation added for this workspace

- **`app/Cargo.toml` needs `avx`/`sse`/`avx512` added to the `pic-scale` path-dependency's
  `features` list to exercise any x86 SIMD tier above the scalar fallback** - the default list is
  `["colorspaces", "rdm", "nightly_i8mm", "nightly_f16"]`, `default-features = false`, so a plain
  `cargo bench -p app` does **not** exercise SSE/AVX2/AVX-512 at all. This is done by each
  candidate's own `justfile` (`apply` recipe patches it via `sed`) rather than committed to the
  workspace, since it's a measurement-only change reverted at the end of every session along with
  the rest of the working tree.
- **`tools/paired.sh`'s `run()` was adapted** for this project's criterion output format. The
  generic template assumed `"<number> <unit>"` as two whitespace-separated fields; criterion
  actually prints `[51.829 ms 53.574 ms 55.652 ms]` where the opening bracket sticks to the first
  number, so the naive `$3` field is the literal string `ms`, not a number - this parses as `0`
  silently (no error, just wrong data) if not fixed. Fixed by `gsub`-stripping `[`/`]` first, then
  reading the point estimate from `$4` and its unit from `$5`. Also added a `PIN=<core>` env var
  to `run()` so the same script covers both the pinned-single-core and unpinned-multi-core cases
  from the candidate loop without a second copy of the script.
- **`bit_exact_check/`** (candidate-local, not a shared tool): a tiny standalone crate that
  resizes the same nasa asset with `Lanczos3`/`PreferQuality`/alpha and hashes the output bytes,
  used to confirm a patch is bit-exact against baseline without needing to touch the criterion
  bench's internals. Copied into each candidate folder that needs it rather than shared, since
  it's small and each candidate may need slightly different scaler settings to reach its own code
  path.
- **`app/benches/resize_cbcr/main.rs` had no `u8`/CN=2 case at all** (only `"Pic scale
  CbCr10/16: Lanczos 3"`, both `u16`) - candidate 004 needed a new `"Pic scale CbCr8: Lanczos 3"`
  `bench_function` (via `img.to_luma_alpha8()` as a stand-in 2-channel u8 source) to reach
  `src/factory/cbcr8.rs`'s u8 path at all. Same as the `avx`/`sse` feature flip below, this is
  measurement-only: it lives as `bench_addition.diff` in that candidate's own folder, reapplied by
  its `justfile`, not committed to the main tree.

## Environment

- CPU: AMD Ryzen 9 7900 (Zen4), 24 logical cores. Has `avx512f/bw/dq/cd/vl/vnni` (confirmed via
  `/proc/cpuinfo` and `std::arch::is_x86_feature_detected!`) but **not** `avxvnni` - i.e. it's a
  "real" AVX-512 machine, not one of the newer narrow-VEX-AVX-VNNI-only machines (Alder Lake+/
  Sierra Forest). This matters for any candidate touching `src/avx512/` or the several
  `is_x86_feature_detected!("avxvnni")` gates in `src/factory/*_u16.rs` /
  `src/factory/rgba_u8.rs` - they never fire on this box as shipped.
- `perf_event_paranoid` not checked yet this session; `taskset -c 3` used for single-core pinning
  (core 3 arbitrary, not otherwise reserved).
- Toolchain: pinned nightly via `rust-toolchain.toml` (see `projects/pic-scale.md`).
- Criterion runs on this machine took noticeably longer per invocation than the "~33s incremental
  build" note in `projects/pic-scale.md` might suggest for the *build* - each `--bench <name> --
  "<case>"` run itself (warmup + sampling) took ~10-20s wall-clock, so a `paired.sh` run with
  `RUNS=9` (18 invocations) takes several minutes; plan Bash-tool timeouts accordingly (it will
  auto-background past 120s - that's fine, just wait for the completion notification rather than
  re-issuing the command, which was observed to spawn a second concurrent instance contending for
  the same pinned core if done by mistake).

## Ground rules used in this workspace

1. No claim without a command that reproduces it - every candidate folder has the numbers and the
   exact command.
2. Every candidate that touches real source gets a real `patch.diff`, applied and built for real.
3. Null results are written down with the same care as wins - a null result with no file is
   indistinguishable from "never checked".
4. "Multi-core" measurement in this workspace means the same single-threaded bench process run
   *unpinned* (realistic wall-clock, whatever core the scheduler picks) vs. `taskset`-pinned for
   ranking - not a multi-threaded workload, since the existing harness has none. A candidate that
   specifically needs real multi-threaded scaling data would need to extend the harness (set
   `ThreadingPolicy::Adaptive`/`Fixed(n)` and enable the `threading` feature) - not done yet.
5. If a later candidate changes the baseline enough that an earlier candidate's numbers no longer
   hold, don't silently rewrite that candidate's `RESULT.md` - add a short `CORRECTION.md` in its
   folder and update its `SUMMARY.md` row to point at it.
