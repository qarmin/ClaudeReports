# pic-scale-safe rayon parallelism at small resize sizes - regression check

Candidate 002 forwarded the `rayon` feature into `pic-scale-safe` (`"pic-scale-safe/rayon"` added
to `Cargo.toml`'s `rayon` feature list), giving `DynamicImage::resize`/`resize_exact` a large
multi-core win at a 4032x3024 -> 400x300 downscale. `IDEAS.md` flagged that pic-scale-safe's
`fixed_point_dispatch.rs`/`floating_point_dispatch.rs` (v0.1.11) use
`par_chunks_exact`/`par_chunks_exact_mut` unconditionally whenever the `rayon` feature is enabled,
with no minimum-row-count/area gate, and that this was never checked at a small output size. This
candidate re-applies candidate 002's exact Cargo.toml change and measures it at several smaller
sizes to check for the suspected regression.

## What was tried

Re-used candidate 002's harness (`benches/imageops.rs`'s `dynamic_resize` criterion group,
`DynamicImage::resize_exact`), extended with two more cases beyond the existing 400x300 control:

- `4032x3024 -> 64x48` (large source, small output - output row count is small, but the
  horizontal convolution pass still scans all 3024 source rows regardless of output height)
- `400x300 -> 32x24` (small source, small output - genuinely small total work, the scenario the
  idea's "icon/thumbnail" framing actually implies)

Two prebuilt `--release` `imageops` bench binaries were alternated (same binary alternation
practice as every other candidate in this workspace, per `README.md`): "serial" (pristine
`Cargo.toml`, pic-scale-safe's own `chunks_exact` path) vs. "parallel" (candidate 002's one-line
`Cargo.toml` change, pic-scale-safe's `par_chunks_exact` path - `rayon` is a default feature of
this crate, so both binaries actually use rayon internally for `image`'s own default-enabled
`rayon` feature, only pic-scale-safe's own internal dispatch differs). `taskset -c 3` for
single-core; unpinned for multi-core (24 logical cores, load average 1.7-7.0 during this
session's measurements, machine otherwise idle).

| Variant / scenario | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | serial-faster pairs |
|---|---|---|---|
| baseline (serial), 4032x3024->400x300 Lanczos3 | ~19.4 ms | (not re-measured this candidate, matches candidate 002) | - |
| patched (parallel), 4032x3024->400x300 Lanczos3 | ~19.5 ms | (matches candidate 002: ~5.4-5.7 ms) | 4/8 (wash) |
| baseline (serial), 4032x3024->64x48 Lanczos3 | ~20.8 ms | ~21.0 ms | single-core: 8/10; multi-core: 0/10 |
| patched (parallel), 4032x3024->64x48 Lanczos3 | ~21.1 ms | ~6.3 ms | |
| baseline (serial), 4032x3024->64x48 Triangle | (not single-core measured) | ~10.2 ms | multi-core: 0/8 |
| patched (parallel), 4032x3024->64x48 Triangle | | ~3.7 ms | |
| baseline (serial), 400x300->32x24 Lanczos3 | ~0.206 ms | ~0.230 ms | single-core: 9/10; multi-core: 2/10 |
| patched (parallel), 400x300->32x24 Lanczos3 | ~0.215 ms | ~0.205 ms | |
| baseline (serial), 400x300->32x24 Triangle | ~0.0758 ms | ~0.0660 ms (2 runs, both consistent) | single-core: 10/10 both runs; multi-core: 10/10 both runs |
| patched (parallel), 400x300->32x24 Triangle | ~0.0830 ms | ~0.0734 ms | |

"serial-faster pairs" = how many of the N alternated pairs had the serial (unpatched) binary
win - i.e. how often turning rayon on for pic-scale-safe actually made this specific case slower.

Raw paired data (10 pairs unless noted) saved under `raw/` in this folder
(`lanczos_small_singlecore.txt`, `lanczos_large_singlecore.txt`, `lanczos_small_multicore.txt`,
`triangle_small_multicore.txt`, `small_source_lanczos_singlecore.txt`,
`small_source_lanczos_multicore.txt`, `small_source_triangle_singlecore.txt`,
`small_source_triangle_multicore.txt`, `small_source_triangle_multicore_run2.txt` - the last is a
repeat of the same case to confirm reproducibility per MEASUREMENT.md's repeat-baseline
discipline).

## Result

**No regression at `4032x3024 -> 64x48`** (small output, large source): multi-core still gets
candidate 002's full win (~3.3x for Lanczos3, ~2.8x for Triangle, 10/10 and 8/8 pairs favoring
parallel) - the horizontal convolution pass scans all 3024 source rows regardless of how few
output rows there are, so there is still plenty of independent per-row work to parallelize even
though the *output* is small. Single-core shows only a small, borderline-noise effect (Lanczos3:
parallel ~1.6% slower on average, 8/10 pairs favoring serial, but the 400x300 control case run in
the same session was an even closer wash at 4/8) - consistent with candidate 002's original
finding that single-core thread-pool-pinned overhead is present but small, not a real added cost
worth calling a regression by itself.

**A real, small, reproducible regression exists at `400x300 -> 32x24`** (small source, small
output - the case where total resize work, not just output row count, is genuinely small):

- `Triangle` at this size: parallel was slower in **every** paired run, both single-core (10/10,
  ~9.4% slower on average) and multi-core (10/10 in each of two separate runs done ~10 minutes
  apart, ~11-12% slower on average) - the most airtight signal in this candidate, since it holds
  unpinned (the realistic-usage configuration) and reproduced identically on a second, independent
  run.
- `Lanczos3` at this size: single-core also regressed consistently (9/10 pairs favoring serial,
  ~4.6% slower on average) but multi-core did not (8/10 pairs favoring *parallel*, ~11% faster on
  average) - noisier and smaller-magnitude than Triangle's, and in the opposite direction on the
  configuration that matters most (unpinned).

So the regression is real but filter-dependent at this size: consistently present for the cheap
`Triangle` kernel (less per-row work to amortize thread-dispatch overhead against), inconsistent
and possibly net-neutral-or-positive for the costlier `Lanczos3` kernel even at the same tiny
32x24 output. Absolute magnitudes throughout this scenario are extremely small in real terms
(tens to low hundreds of *microseconds* total either way - 0.066 ms vs 0.073 ms for Triangle), so
even a consistent ~10% relative regression is unlikely to be perceptible in a real application,
but it is a genuine, reproducible, measured direction reversal from candidate 002's headline
result, not noise.

## Does it change behavior/output?

- **Bit-exact?** Yes. A temporary standalone example (`examples/_correctness_tmp_019.rs`, built
  and run, not committed - removed before the working tree was reverted) hashed
  (`DefaultHasher`, over both pixel bytes and dimensions) `DynamicImage::resize_exact` output for
  all 5 `FilterType` variants at both new sizes used in this candidate (`4032x3024 -> 64x48` and
  `400x300 -> 32x24`, the latter cascaded from a `400x300` `Triangle`-resized copy of the same
  source JPEG), built once against the unpatched Cargo.toml and once against the patched one. All
  10 hashes matched exactly between the two builds (see raw hex values in `raw/hashes.txt`) -
  consistent with candidate 002's original finding and expected for the same reason: pic-scale-safe's
  rayon split has no shared mutable state or floating-point reduction across chunks.
- **Is this a breaking change?** No, same as candidate 002 - this is the same one-line
  `Cargo.toml` feature-forwarding change (only re-applied here to measure a different size), only
  active when the crate's own already-public, already-default `rayon` feature is enabled.
- Project test suite: 550/551 passed, both before (unpatched Cargo.toml) and after (patched), with
  the same single pre-existing unrelated failure both times (`resizing_with_catmul` in
  `tests/regression.rs`, `assertion left == right failed` on two `ImageRgba8` buffers of matching
  dimensions - same failure noted as pre-existing in candidates 007/008/011/016's `RESULT.md`s,
  not touched by this candidate's change). No new failures introduced by the patch at either
  build.

## Why the result looks the way it does

pic-scale-safe's rayon path (`fixed_point_dispatch.rs`/`floating_point_dispatch.rs` in the
resolved 0.1.11) unconditionally uses `par_chunks_exact`/`par_chunks_exact_mut` with no
minimum-size gate (as `IDEAS.md`'s entry described). Whether this is a net win or loss depends on
how much independent per-chunk work there is to amortize the rayon thread-pool dispatch/join cost
against - not on the *output* size alone:

- `4032x3024 -> 64x48` still has 3024 source rows' worth of horizontal-convolution work
  regardless of output height, so there's ample parallel work and no regression shows up.
- `400x300 -> 32x24` has only 300 source rows for the horizontal pass and just 24 output rows for
  the vertical pass - genuinely little work per chunk, so the fixed cost of spinning up/joining
  the thread pool (already measured elsewhere in this workspace, e.g. `premultiply_rgba8`'s own
  overhead in candidate 014, as being non-trivial in this dependency) can exceed what parallelism
  saves. `Triangle`'s narrower/cheaper kernel has even less per-row arithmetic to hide the
  overhead behind than `Lanczos3`'s wider kernel, which plausibly explains why `Triangle`
  regressed consistently while `Lanczos3` did not on the multi-core configuration.

## What's not done / open questions

- The exact row/area threshold at which the regression flips to a win was not bisected - only
  two source/output size combinations were measured (`4032x3024->64x48` and `400x300->32x24`),
  not a sweep across intermediate sizes.
- No fix was implemented. The regression lives entirely inside `pic-scale-safe`'s own dispatch
  code (`fixed_point_dispatch.rs`/`floating_point_dispatch.rs`), not in this repo's source - a
  size-threshold gate could only be added either upstream in pic-scale-safe itself, or in this
  repo's `src/imageops/resize.rs` by choosing at the call site whether to enable/bypass
  pic-scale-safe's parallel path (not currently exposed as a per-call option by pic-scale-safe's
  public API - not investigated whether one exists). Per SKILL.md's "vendoring a suspect
  dependency" guidance, actually implementing and measuring a fix would mean cloning
  pic-scale-safe with its own `___OPTIMIZATIONS___/` - not done this session, this candidate is a
  measurement/characterization only.
- `Gaussian`/`CatmullRom`/`Nearest` were correctness-checked (bit-exact, in the hash comparison
  above) at both new sizes but not separately performance-measured at the small `400x300->32x24`
  size - only `Lanczos3` and `Triangle` were.
- Whether the regression's magnitude/direction is stable across machines (vs. this session's
  specific CPU/thread-scheduling behavior) was not checked - see this workspace's other notes on
  machine-specific noise (`projects/image.md`, `README.md`).
