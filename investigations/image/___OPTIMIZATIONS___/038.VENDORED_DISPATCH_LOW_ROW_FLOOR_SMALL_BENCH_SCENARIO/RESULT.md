# Vendored pic-scale-safe dispatch: measuring candidate 036's row floor at a scenario it actually activates for

Follows candidate 036's own "What's not done" section and `IDEAS.md`'s backlog row dated
2026-08-26: candidate 036 built and bit-exact-verified its `MIN_ROWS_FOR_POOL = 8` floor at
`dst_height=6`, but never timed it there - this workspace's only benchmarked small-resize
scenario (`400x300->32x24`, `dst_height=24`) is already above any plausible floor value, so
candidate 036's own single-core-regression-vs-multi-core-win trade-off (vs. candidate 035, no
floor) was only ever measured at a size where the floor provably never activates. This candidate
adds a genuinely small destination size (`400x300->8x6`, `dst_height=6 < 8`) to
`benches/imageops.rs`'s `bench_resize_dynamic`, then re-measures `H` (candidate 036's floor) vs.
`E` (candidate 035's no-floor pool) at that size specifically.

## What was tried

Reused candidates 035/036's own `patch.diff` unmodified as the two source states under test (no
new source-level change - see "Does it change behavior/output?" below), and added one addon hunk
to `benches/imageops.rs`'s `bench_resize_dynamic`: the existing single `for filter in [...]`
loop over `400x300->32x24` became a nested `for (sw, sh) in [(32u32, 24u32), (8, 6)] { for
filter in [...] }`, so every filter now also runs at `400x300->8x6` (`dst_height=6`, below
candidate 036's floor of 8) alongside the pre-existing `32x24` case (`dst_height=24`, above the
floor, unchanged from candidates 002/031-036).

Two source states, same naming convention as candidates 019-036:

- **E (candidate 035's always-dispatch, no floor)** - candidate 035's own `patch.diff`, rebuilt
  fresh this session, plus this candidate's bench addon.
- **H (candidate 036's hybrid, 8-row floor)** - candidate 036's own `patch.diff`, rebuilt fresh
  this session, plus this candidate's bench addon (identical vendored source to E's, differing
  only by the `MIN_ROWS_FOR_POOL = 8` gate already recorded in 036's `patch.diff`).

Both built with `--features rayon`. Binaries alternated via `raw/paired_pic_scale.sh` (candidates
023-036's script, reused unmodified, invoked directly rather than through `cargo bench` per-run).
`taskset -c 3` for single-core, unpinned for multi-core (24 logical cores). `uptime` load average
2.3-8.1 during this session (well under the 24-core count). 5 paired runs per comparison, matching
candidate 036's own convention. A same-binary control (E vs. E) was run for both the smallest
(`Nearest`, ~0.1us/call) and a representative larger (`Triangle`) case to establish the noise
floor before trusting the real comparisons.

| Filter | Comparison | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) |
|---|---|---|---|
| Triangle | E -> H | 64.566 -> 55.159 us (**-14.6%**, 4/5 H-faster) | 68.906 -> 55.981 us (**-18.8%**, 5/5 H-faster) |
| CatmullRom | E -> H | 110.670 -> 100.830 us (-8.9%, 3/5 H-faster) | 121.020 -> 102.620 us (**-15.2%**, 5/5 H-faster) |
| Gaussian | E -> H | 104.360 -> 94.737 us (-9.2%, 3/5 H-faster) | 121.620 -> 96.392 us (**-20.7%**, 5/5 H-faster) |
| Lanczos3 | E -> H | 154.990 -> 144.860 us (**-6.5%**, 5/5 H-faster) | 169.960 -> 147.840 us (**-13.0%**, 5/5 H-faster) |
| Nearest | E -> H | 0.166 -> 0.165 us (-0.5%, 4/5 H-faster) | 0.103 -> 0.102 us (-1.8%, 4/5 H-faster) |
| Triangle (control) | E -> E | n/a | 67.705 -> 68.142 us (+0.6%, 1/5 "B"-faster) |
| Nearest (control) | E -> E (single-core) | 0.100 -> 0.100 us (+0.1%, 1/5 "B"-faster) | n/a |

Raw paired output under `raw/`: `singlecore_E_vs_H_<Filter>.log`, `multicore_E_vs_H_<Filter>.log`,
`control_singlecore_E_vs_E_Nearest.log`, `control_multicore_E_vs_E_Triangle.log`.

## Result

**At `dst_height=6` (below candidate 036's floor), `H` beats `E` on both axes for every filter
that actually routes through the patched dispatch functions.** Multi-core is decisive for all
four (`Triangle`/`CatmullRom`/`Gaussian`/`Lanczos3`, `-13.0%` to `-20.7%`, 5/5 pairs each) -
noticeably larger in magnitude than the same comparison's single-core numbers, i.e. skipping
`pool.install()` at a genuinely tiny output size helps *more* on multi-core here, not less, since
none of these filters have enough real work at `dst_height=6` for the pool dispatch to pay for
itself on any core count. Single-core is decisive for `Triangle` (`-14.6%`, 4/5) and `Lanczos3`
(`-6.5%`, 5/5), and a real-but-not-fully-decisive-by-pair-count improvement for `CatmullRom`
(`-8.9%`, 3/5) and `Gaussian` (`-9.2%`, 3/5) - both still comfortably outside the same-binary
control's noise floor (`+0.6%`, 1/5 "wins" for the control pair), just not clearing this
workspace's own `wins >= n-1` decisiveness bar at `n=5`.

`Nearest` shows no meaningful difference on either axis (`-0.5%`/`-1.8%`, both inside the
same-binary control's noise floor of `+0.1%`) - expected, since `Nearest` is dispatched through
`resize_nearest.rs`'s own separate row-gate (candidate 033), never through the
`convolve_trampoline_*`/`convolve_row_*`/`convolve_column_*` functions candidates 035/036 touch,
so `E` and `H` run identical code for `Nearest` regardless of the floor.

This resolves candidate 036's open question directly: **the floor's mechanism does what it was
designed to do** - it is a real, decisive-to-mostly-decisive win over candidate 035's always-dispatch
pool at a destination size small enough to actually cross it (`dst_height=6 < 8`), on both single-
and multi-core, for the three convolution-based filters that go through the gated dispatch
functions. Candidate 036's own finding - that the floor is a wash vs. `E` at `dst_height=24` - is
unaffected and still holds at that size; the two results describe different, non-overlapping
regions of the same parameter (below vs. above the floor), not a contradiction.

## Does it change behavior/output?

- **No new source-level fix in this candidate** - `patch.diff` in this folder is byte-identical
  in its vendored-source hunks to candidate 036's own `patch.diff` (diffed directly, only the
  `benches/imageops.rs` hunk differs, by exactly the new `8x6` scenario). The actual mechanism
  under test (`MIN_ROWS_FOR_POOL = 8` per dispatch function) was already fixed and recorded by
  candidate 036; this candidate only adds a new benchmark scenario and re-measures the existing
  fix at it. See `bench_dst6_addon.diff` in this folder for the isolated, minimal hunk (applies
  cleanly on top of either candidate 035's or candidate 036's own `patch.diff`).
- **Bit-exact?** Yes. A temporary standalone example (`examples/_correctness_tmp_038.rs`, built
  and run, not committed - removed before the working tree was reverted) hashed
  (`DefaultHasher`, over raw pixel bytes and dimensions) `DynamicImage::resize_exact` output for
  all 5 `FilterType` variants, `RgbImage`/`RgbaImage`, at both `400x300->8x6` and
  `400x300->32x24` (20 hashes total). Built three times: **S** (`--no-default-features --features
  default-formats`, true serial main), **E** (candidate 035's patch, `--features rayon`), and
  **H** (candidate 036's patch, `--features rayon`). All 20 hashes matched exactly across all
  three builds (`raw/hashes_S.txt`, `raw/hashes_E.txt`, `raw/hashes_H.txt`) - `dst_height=6` is
  bit-exact between the floor and no-floor dispatch paths, same as candidate 036 already found at
  its own dst_height=6 correctness scenario.
- **Is this a breaking change?** No public API changes to `image` itself - same as candidates
  033-036, the change lives entirely in the vendored `pic-scale-safe` copy plus the
  still-unmerged `pic-scale-safe/rayon` feature forward, neither of which is on `main` today.
- Project test suite: `cargo test --release --no-default-features --features default-formats`
  for **S**: 546 passed, 1 failed (`raw/test_S_full.txt`). `cargo test --release --features
  rayon` for **H**: 550 passed, 1 failed (`raw/test_H_full.txt`). Both share the identical single
  pre-existing unrelated failure (`resizing_with_catmul` in `tests/regression.rs`, recorded in
  every prior candidate in this line back through candidate 007) - no new failures. (E's own test
  counts were already recorded by candidate 035 at 550 passed/1 failed with the same failure; not
  independently re-run here since E's source is unchanged from that candidate.)

## Why the result looks the way it does

Candidate 036's own analysis already predicted this outcome structurally: "there is no floor
value that helps at `dst_height=24` specifically without also giving up the parallelism that
scenario already has enough real work to use productively... the floor idea only pays off for
outputs *smaller* than whatever scenario is being measured." This candidate is the direct
confirmation of that prediction's other half - at `dst_height=6`, genuinely below the floor, the
floor's serial fallback wins on both axes because `pool.install()`'s fixed per-call dispatch
overhead (thread-pool lookup, work partitioning, cross-thread synchronization) has no real
parallel work to amortize against at an 8x6 = 48-pixel output, regardless of core count -
multi-core doesn't help the pool-dispatch case here because there's nothing to parallelize
productively at this size, only overhead to pay.

## What's not done / open questions

- `CatmullRom`/`Gaussian` single-core results (3/5 pairs) don't clear this workspace's own
  decisiveness bar despite a magnitude (`-8.9%`/`-9.2%`) well outside the measured noise floor
  (`+0.6%`/`+0.1%` for same-binary controls) - a higher paired-run count (10+, matching candidates
  026-030's convention) would likely resolve this either way; not re-run here to keep this
  session's scope to the specific gap candidate 036 flagged.
- Only one below-floor size (`dst_height=6`) was benchmarked. The backlog's suggested range was
  "4-8 rows" for the floor itself (candidate 036 chose 8, unswept within that range) - a size
  sweep of a few destination heights between 1 and 8 (not just one point) was not done and would
  show whether the win scales smoothly or has its own threshold behavior.
- As in candidates 033-036, this fix was not given its own separate `___OPTIMIZATIONS___/` for
  the vendored crate - it continues the measurements already on file in *this* workspace
  (candidates 019-036). No decision is made here about upstream PR vs. local
  `[patch.crates-io]` vs. drop.
