# Filter-aware pic-scale-safe rayon call-site gate

Follows up on candidate 021, which found that wrapping `src/imageops/resize.rs`'s pic-scale-safe
call in a single-thread `rayon::ThreadPool::install()` below a size threshold (fixing candidate
019's small-`Triangle`-resize regression) was filter-blind and badly regressed `Lanczos3`
(+47.4% multi-core) by discarding a real multi-core parallel win candidate 019 had measured for
`Lanczos3` at that same size. `IDEAS.md`'s row for this candidate proposed making the gate
filter-aware instead: only route cheap-kernel filters (`Triangle`, i.e.
`pic_scale_safe::ResamplingFunction::Bilinear`) through the single-thread workaround, leaving
`Lanczos3`/`CatmullRom`/`Gaussian` on the default multi-threaded path unconditionally regardless
of size.

## What was tried

Same bundling as candidate 021 (Cargo.toml `pic-scale-safe/rayon` forward + `bench_resize_dynamic`
harness + this candidate's own fix - see `justfile` header). The fix itself: `run_resize` (in
`src/imageops/resize.rs`) now takes the resolved `ResamplingFunction` and only engages the cached
single-thread pool when `matches!(alg, ResamplingFunction::Bilinear)` **and** `source_pixels <
500_000` (same threshold value as candidate 021, not independently rebisected - see "What's not
done"). Every other filter takes the unchanged, unconditional default-pool path at every size.

"A" = candidate 002+019 state (Cargo.toml forward + bench harness, no gate at all - the same
regressed-for-Triangle/mixed-for-Lanczos3 state candidate 019 characterized and candidate 021
also compared against). "B" = A plus this candidate's filter-aware gate. 10 paired runs for the
small (400x300->32x24) scenarios, 6 for the large-size controls (matches this workspace's reduced-
repeat-count convention for control cases). `taskset -c 3` for single-core; unpinned for
multi-core (24 logical cores, `uptime` load average 2.15-5.79 during this session).

| Variant / scenario | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | B-faster pairs |
|---|---|---|---|
| A, 400x300->32x24 Triangle | 75.012 us | 72.600 us | - |
| B, 400x300->32x24 Triangle | 75.135 us (+0.2%) | 72.103 us (-0.7%) | single: 4/10; multi: 3/10 |
| A, 400x300->32x24 Lanczos3 | 209.410 us | 141.280 us | - |
| B, 400x300->32x24 Lanczos3 | 209.010 us (-0.2%) | 141.670 us (+0.3%) | single: 5/10; multi: 4/10 |
| A, 4032x3024->400x300 Lanczos3 (large control) | (not measured single-core) | 4721.000 us | multi: 4/6 |
| B, 4032x3024->400x300 Lanczos3 (large control) | | 4725.200 us (+0.1%) | |
| A, 4032x3024->64x48 Triangle (large-source control) | | 3703.800 us | multi: 3/6 |
| B, 4032x3024->64x48 Triangle (large-source control) | | 3676.300 us (-0.7%) | |

Raw paired samples: `raw/small_triangle_singlecore.txt`, `raw/small_triangle_multicore.txt`,
`raw/small_lanczos_singlecore.txt`, `raw/small_lanczos_multicore.txt`,
`raw/large_lanczos_multicore.txt`, `raw/large_triangle_multicore.txt`.

## Result

**Fixes the filter-blindness problem this candidate set out to fix: `Lanczos3` at
400x300->32x24 is now a wash on multi-core (+0.3%, 4/10 pairs) instead of candidate 021's +47.4%
regression**, because the gate never engages for it (`ResamplingFunction::Lanczos3 !=
ResamplingFunction::Bilinear`, so it always takes the default multi-threaded path, identical
code to "A"). The large-size controls remain unaffected either way (+0.1%/-0.7%, both well
within this workspace's established noise floor, 4/6 and 3/6 pairs - the 3703.8/3676.3 us
Triangle-control pair includes two ~30-60% outliers in the raw data matching this workspace's
documented mode-lottery noise, not a real effect).

`Triangle` itself, however, is **not clearly fixed either** - it remains a wash in both
directions (single-core: +0.2%, 4/10; multi-core: -0.7%, 3/10), not decisively better than "A"
the way candidate 019's original regression characterization (~9-12% slower with rayon enabled)
would need to be reversed for the fix to count as "fixed" rather than "no longer actively
regressed further by a subsequent patch attempt". Neither pair count (4/10, 3/10) crosses this
workspace's `paired.sh`-style decisiveness bar (`wins >= n-1` or `wins <= 1`) in either direction.
Note "A" here is *not* the same 019-regressed number line-for-line: 019's own `Triangle` multi-
core regression was measured against a serial (`rayon` feature entirely absent) build, while this
candidate's "A" already has the `rayon` feature forwarded (unconditional parallel dispatch) - so
"A" already contains the regression, and this candidate's job was to see whether the filter-aware
gate improves on it, which it did not do decisively in either direction this session.

## Does it change behavior/output?

- **Bit-exact?** Yes. A temporary standalone example (`examples/_correctness_tmp_022.rs`, built
  and run, not committed - removed before the working tree was reverted) hashed
  (`DefaultHasher`, over pixel bytes and dimensions) `DynamicImage::resize_exact`/`resize`
  output for all 5 `FilterType` variants across 4 scenarios (`RgbImage` and `RgbaImage` sources at
  400x300->32x24, plus a large 4032x3024 source resized to 64x48 and to 400x300), built once
  with only the Cargo.toml+bench changes ("A") and once with this candidate's filter-aware gate
  added ("B"). All 20 hashes matched exactly between the two builds (`raw/hashes_A.txt` /
  `raw/hashes_B.txt`, identical apart from the unrelated compile-timing line) - expected, since
  routing through a different rayon thread pool changes nothing about which chunks compute which
  output bytes.
- **Is this a breaking change?** No public API changes; behavior only differs in wall-clock time,
  not output, and only when the `rayon` feature is enabled (not currently forwarded into
  pic-scale-safe on `main` - this candidate's Cargo.toml change re-applies candidate 002's
  proposed forward, still not itself applied to `main`).
- Project test suite (`cargo test --release --features rayon`): 550/551 passed, both before ("A":
  Cargo.toml forward + bench only, no gate) and after ("B": full patch), with the same single
  pre-existing unrelated failure both times (`resizing_with_catmul` in `tests/regression.rs` -
  same failure noted in candidates 007/008/011/016/019/020/021's `RESULT.md`s, not touched by
  this candidate).

## Why the result looks the way it does

The gate is a pure filter dispatch: `run_resize` only special-cases
`ResamplingFunction::Bilinear` (the value `convert_filter_type` maps `FilterType::Triangle` to),
so any code path taking a different filter is byte-for-byte identical to "A" regardless of size -
which is exactly why `Lanczos3` stopped regressing (it never touches the changed branch at all,
unlike candidate 021's blanket `num_threads(1)` wrap which applied to every filter below the size
threshold). This directly confirms candidate 021's own explanation for *why* it regressed
`Lanczos3`: the problem was never "small resizes categorically shouldn't use the default pool," it
was "cheap-kernel filters specifically don't have enough per-row work to hide rayon's dispatch
overhead at this size," and gating on the filter that's actually cheap avoids taking away a real
win from filters that aren't.

`Triangle` remaining a wash rather than a clear win is more surprising given the mechanism should
be identical to candidate 021's own "A vs B" comparison for `Triangle` (021 measured 021's B
+2.7% slower than its own A for `Triangle` multi-core; this candidate's own B/A for the same
scenario landed at -0.7%, i.e. also close to a wash but in the opposite direction) - both readings
are inside the same noisy, inconclusive range for this specific filter/size, not a contradiction,
just two inconclusive measurements from two different sessions rather than one clean confirmation
and one clean regression. The single-thread-pool-install approach itself (not particular to this
candidate) still pays rayon's own task hand-off/park-wake cost even with only one worker, which
candidate 021's `RESULT.md` already identified as the likely reason it doesn't cleanly outperform
"do nothing extra" for a filter this cheap - this candidate's contribution is narrowing *which*
filters pay that cost at all, not eliminating the cost itself for the one filter that still does.

## What's not done / open questions

- `Triangle`'s own regression (candidate 019's original ~9-12% single/multi-core finding, measured
  against a fully-serial `rayon`-feature-absent build) is not decisively reversed by this
  candidate's mechanism - both this session's and candidate 021's own "gate vs. no-gate, rayon
  already enabled" comparisons for `Triangle` land in a noisy near-wash, not a clear win. A capped-
  but->1 thread count (IDEAS.md's existing follow-up, still not tried by either candidate) or a
  vendored fix inside pic-scale-safe itself (per candidate 021's "what's not done", also still not
  attempted) remain the two next things to try if `Triangle`'s regression specifically needs
  closing, not just `Lanczos3`'s regression-from-021 avoided.
- `Nearest` (cheaper than even `Triangle`) is not included in the cheap-kernel gate and was not
  separately measured at this size - `IDEAS.md` (candidate 019's entry) already flagged that
  `Nearest`/`Gaussian`/`CatmullRom` were correctness-checked but not separately timed at
  400x300->32x24 by candidate 019; this candidate doesn't close that gap either, since it only
  needed to confirm `Bilinear` specifically for its own fix.
- The 500,000-source-pixel threshold itself was not bisected (same open item carried over from
  candidates 019 and 021).
- Single-core numbers for the two large-size control scenarios were not collected (same choice
  candidate 021 made, for the same reason: multi-core is the configuration the underlying
  mechanism actually depends on).
- No `perf`/disassembly confirmation of the rayon dispatch/park-wake cost theory in the "why"
  section above - `perf` is not installed in this environment (see `README.md`).
