# What is under test, and how it is measured

Read this first if picking the work up cold - it explains the harness, `SUMMARY.md` lists the
candidates measured with it.

## Workload

This is `image-rs/image` (crate `image`, currently on the pre-1.0 branch, `Cargo.toml` version
0.25.9). It's a general-purpose image processing crate: format codecs under `src/codecs/`
(avif, bmp, gif, hdr, ico, jpeg, openexr, png, pnm, qoi, tga, tiff, webp, farbfeld) plus
in-memory pixel operations under `src/imageops/`. The crate already ships criterion benches in
`benches/`, each isolating a different subsystem:

| bench | what it stresses |
|---|---|
| `decode.rs` | format decoders (per-codec decode throughput) |
| `encode.rs` | format encoders |
| `imageops.rs` | resize/crop/rotate/overlay and other pixel ops |
| `blur.rs` | `fast_blur` and `blur_advanced` (Gaussian) at several sigmas |
| `convert.rs` | pixel format/colortype conversions |
| `copy_from.rs` | `copy_from`/`copy_within`-style buffer copies |
| `bgra_to_rgba.rs` | BGRA<->RGBA channel-swap paths, old vs. new implementations side by side |

No new harness was built for this session - these existing criterion benches are reused as-is.

## Measurement harness

Run via `cargo bench --bench <name>` from the repo root (each is registered in `Cargo.toml`'s
`[[bench]]` list with `harness = false`, using `criterion`). Criterion itself provides
warmup, iteration batching, and percentile/outlier reporting, which covers most of
MEASUREMENT.md's baseline requirements out of the box. To pin single-core for ranking, wrap with
`taskset -c N cargo bench --bench <name> -- <filter>`; always also run unpinned (multi-core,
realistic wall-clock) per SKILL.md point 6.

`cargo bench --bench <name> -- --list` shows the individual benchmark IDs inside a file so a run
can be filtered to just the one relevant to a candidate instead of the whole file.

## Instrumentation added for this workspace

Candidate 001 (`affine.rs` row/pixel-slice fast path) added one new criterion case,
`rotate180_in_place`, to `benches/imageops.rs` - no prior bench exercised
`imageops::rotate180_in_place` directly (only the allocating `rotate180()` wrapper, which calls
`rotate180_in`, a different function). `flip_vertical_in` was already reachable via the existing
`flip_vertical` case, reused as-is. This bench addition is bundled into candidate 001's
`patch.diff` (see that candidate's `RESULT.md`) rather than kept separately, since it's needed to
reach the patched code at all, not a standalone change of its own. Uses `b.iter_batched(..,
BatchSize::LargeInput)` to keep the per-iteration `ImageBuffer` clone (needed to reset the image
before each in-place mutation) out of the timed portion - the file's other in-place benches
(`invert`, `dither`) clone inside `b.iter` instead, which was tried first here too and produced
much noisier, less trustworthy numbers (the clone's allocation cost dominated and swamped the
signal from the change under test).

Candidate 002 (pic-scale-safe rayon feature forwarding) added a second new criterion group,
`bench_resize_dynamic` (`dynamic_resize` benchmark_group), to the same `benches/imageops.rs` - the
pre-existing `bench_resize` calls the standalone `imageops::resize` free function, which is a
completely separate, older implementation that never delegates to pic-scale-safe at all (confirmed
by grep and by a bit-exact hash comparison), so it could not show any difference from that
candidate's patch. `bench_resize_dynamic` instead exercises `DynamicImage::resize_exact`, the entry
point that actually goes through `src/imageops/resize.rs` -> pic-scale-safe, on a synthetic
4032x3024 RGB8 source (matching the size used in this workspace's earlier callgrind profiling) sized
down to 400x300, for `Lanczos3` and `Triangle`. Uses `iter_batched(.., BatchSize::LargeInput)` to
clone the source per iteration, same convention candidate 001 established for in-place mutation
benchmarks. Bundled into candidate 002's own `patch.diff` (see that candidate's `RESULT.md`) since
it's needed to reach the patched code path at all, not a standalone change of its own.

Candidate 004 (`flip_horizontal_in`/`flip_horizontal_in_place` row/pixel-slice fast path) added a
third new criterion case, `flip_horizontal_in_place`, to the same `benches/imageops.rs` - same
`b.iter_batched(.., BatchSize::LargeInput)` convention as 001/002, since no prior bench exercised
`flip_horizontal_in_place` directly (only the allocating `flip_horizontal()` wrapper, which calls
`flip_horizontal_in`, a different function, already covered by the pre-existing `flip_horizontal`
case). Bundled into candidate 004's own `patch.diff`.

Candidate 006 (`imageops::thumbnail`'s `thumbnail_sample_block` row/pixel-slice fast path) added a
fourth new criterion function, `bench_thumbnail`, to the same `benches/imageops.rs` - no prior
bench in this repo exercised `imageops::thumbnail` at all. Uses the same 4032x3024 synthetic
`RgbImage` size as this workspace's earlier callgrind profiling of JPEG decode/`Lanczos3` resize,
downscaled to 200x150 (the representative "large photo -> small icon/preview" scenario
`IDEAS.md`'s original entry for this candidate suggested). No `iter_batched`/clone needed since
`thumbnail` is not in-place (`b.iter` directly calls it on a shared `&src`). Bundled into
candidate 006's own `patch.diff`.

Candidate 020 added a new BMP corpus file, `tests/images/bmp/images/pal2_large.bmp` (700x700,
2-bit indexed, standard 40-byte-header, 4-color palette, repeating `(x+y) & 3` diagonal-stripe
index pattern, bottom-up row order), registered as a new entry in `benches/decode.rs`'s BMP
`BENCH_DEFS` list, plus a matching reference PNG (`tests/reference/bmp/images/pal2_large.bmp.png`,
saved with `PngEncoder::new_with_quality(_, CompressionType::Best, FilterType::Adaptive)` to keep
it small, 4.3 KiB) so `tests/reference_images.rs` covers it like every other corpus file. This
closed a gap candidate 011 left open: the repo's only pre-existing 2-bit BMP samples
(`pal2.bmp`/`pal2color.bmp`, 127x64) were correctness-only fixtures, not registered in the bench
and too small for a clean criterion signal, so `set_2bit_pixel_run`'s fixed-array fix (patched and
bit-exact-verified by candidate 011, but never separately benchmarked) had no direct measurement
until this candidate. Bundled into candidate 020's own `patch.diff`, same convention as
001/002/004/006 bundling their own new bench cases. The same gap still exists for RLE4/RLE8 (see
`IDEAS.md`) - this candidate only closed it for the plain (non-RLE) 2-bit path.

Candidate 033 is the first in this workspace to vendor a dependency rather than only patching
`image`'s own source: `vendor/pic-scale-safe/` (a full copy of pic-scale-safe 0.1.11's `src/`,
with one function - `resize_nearest` - changed) plugged in via a new `[patch.crates-io]` override
in `Cargo.toml`, per SKILL.md's "Vendoring a suspect dependency". Building against the vendored
copy needs no special `cargo` flags beyond what candidates 002/019-032 already required (the
`pic-scale-safe/rayon` feature forward) - `[patch.crates-io]` transparently swaps the registry
crate for the local path dependency for any build that resolves `pic-scale-safe` at all. The
vendored crate's own `Cargo.toml` needs an explicit empty `[workspace]` table so it isn't swept
into `image`'s own (implicit) workspace as a member.

Also learned this session: single repeated `cargo bench` invocations of one binary at a time were
not trustworthy on this machine even with `taskset` pinning - system load varied 3.7 to 18 during
the session and individual criterion runs swung by 2x+ run to run. Alternating two prebuilt
`--release` binaries (baseline vs. patched) A/B/A/B... and comparing paired runs seconds apart
(same idea as `tools/paired.sh`, done by hand this session rather than adapting that script) gave
consistent, trustworthy results where single-binary repeated runs did not - do this for any future
candidate in this workspace too, not just a "nice to have" for close calls.

Candidate 037 used `valgrind --tool=cachegrind --branch-sim=yes --cache-sim=yes` (with
`cg_annotate` for per-function breakdown) as a `perf`-substitute to get exact, deterministic
dynamic instruction/branch/cache counts per BMP-decoder function, since `perf` remains
unavailable in this container (see "Environment" below) - candidate 025 already used
`objdump`/`nm` for *static* disassembly, this added the *dynamic* counterpart. Needed
`bash -c 'ulimit -n 4096; valgrind ...'` first - this container's default `ulimit -n`
(~2^31) makes cachegrind's `libcfile` abort with "Private file creation failed" for reasons
unrelated to this project. The two temporary example drivers used
(`examples/_cachegrind_bmp_loop.rs`, a decode-N-times-in-a-loop driver; and
`examples/_correctness_check_bmp.rs`, a hash-based correctness checker reused from candidate
025's approach) were **not** kept in the tree - measurement tooling, not the fix - see candidate
037's `RESULT.md` and its `justfile`'s `add-drivers` recipe, which writes them into a fresh clone
directly so the result stays reproducible without leaving them as permanent workspace files.

## Environment

- **Update (2026-08-29, candidate 065):** on this session's host, `perf` **is** installed and
  usable (`perf stat` ran cleanly, `/proc/sys/kernel/perf_event_paranoid` reads `-1`, no
  `sudo`/privilege escalation needed) - the "not installed, cannot be installed" note below (from
  the candidates-025/037 session) evidently described a different, more locked-down container than
  this one. **Don't assume either state without checking `which perf` and a trial `perf stat -- ls`
  fresh each session** - this environment is not stable across sessions on this project. When
  available, `perf stat -e <counter>` gives directly-attributed hardware counter numbers (see
  candidate 065) instead of `valgrind`/`cachegrind`'s simulated counts.
- `perf` was **not installed** on the machine candidates 025/037 ran on (`perf: command not
  found`, `/proc/sys/kernel/perf_event_paranoid` reads `-1` meaning it'd be usable if installed),
  and could not be installed either - `sudo`/`apt` were both blocked by that container's "no new
  privileges" flag (confirmed candidates 025 and 037). `valgrind`/`cg_annotate` (cachegrind,
  callgrind) **are** installed and usable as a partial substitute (deterministic instruction/
  branch/cache counts, no execution-port/IPC model) - see candidate 037's addition above and
  `COMMANDS.md`'s cachegrind section. Hot-spot discovery otherwise relies on code-reading against
  `HEURISTICS.md` and criterion's own per-benchmark breakdown when `perf record`/`perf stat` isn't
  available.
- 24 logical cores available; `uptime` load average was ~2.7-3.75 at session start (well under
  core count, no re-baseline needed for that reason alone - re-check if it climbs during the
  session).
- `just` is installed and used for the workspace's own `justfile`/candidate justfiles.
- Toolchain: whatever `rustc`/`cargo` is on `$PATH` (stable per `Cargo.toml`'s
  `rust-version = "1.88.0"`); no nightly-only features observed in the benches.

## Candidate justfile convention: `just test` builds a real baseline, not just the patched side

Every candidate's `justfile` runs `clone` -> (harness-only hunks, if any) -> `bench-baseline` ->
(the actual fix) -> `bench-patched`, so a fresh `just test` always prints criterion's own real
before/after comparison (`change: ... Performance has {improved,regressed}`) rather than only
building and benching the patched code. For a candidate whose `patch.diff` bundles a bench
addition together with the actual fix (most of this workspace's candidates do, since the crate's
pre-existing benches often don't reach the changed function directly), `apply-harness`/`apply-fix`
split the patch by file with `git apply --include=<path>`/`--exclude=<path>` so the same benchmark
exists in both the baseline and patched build - `apply-harness` applies everything except the
fix's own source file(s), `apply-fix` applies the rest on top. Verified end-to-end (candidates 001
and 007, chosen as one harness-split and one non-split example) by actually running `just test`
with `repo_url` pointed at a local checkout instead of GitHub (`just repo_url="/path/to/image"
test`) - both produced a genuine unpatched baseline number followed by criterion's own comparison
against it, not just a single patched-side number.

## Root justfile: `pack`/`clean`

`___OPTIMIZATIONS___/justfile` (not a candidate's own) also has `just pack` (zips this whole
folder, excluding every `clone/`/`target/` and any previous pack zip, for sharing/attaching) and
`just clean` (removes every candidate's `clone/`/`target/` to reclaim disk space, safe any time -
both are fully regenerable). Both verified working this session.

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
