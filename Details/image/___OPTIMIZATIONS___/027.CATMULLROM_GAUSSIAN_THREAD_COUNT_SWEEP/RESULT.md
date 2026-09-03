# pic-scale-safe rayon call-site gate - thread-count sweep for CatmullRom/Gaussian

Follows up on `IDEAS.md`'s row dated 2026-08-25 ("The `N=4` thread count ... was never re-swept
specifically for `CatmullRom`/`Gaussian`"): candidate 023 swept `num_threads(2)`/`num_threads(4)`
for `Triangle`/`Lanczos3` only; candidate 026 reused `N=4` unmodified for `CatmullRom`/`Gaussian`
and measured a much larger win there (-20.8% to -27.1%) than `Lanczos3` got with the same `N`
(-4.7%), but never re-swept `N` for these two filters specifically. This candidate sweeps
`N = 2/3/4/6/8` for `CatmullRom`/`Gaussian` at the same `400x300->32x24` scenario, reusing candidate
026's already-built bench harness and candidate 023's `raw/paired_pic_scale.sh` unmodified (apart
from invoking the binaries via explicit `./` paths, working around the known bare-binary-name bug
noted in candidate 026's own "What's not done").

## What was tried

Same base as candidates 021/022/023/024/026: candidate 002's `pic-scale-safe/rayon` Cargo.toml
forward + candidate 026's `bench_resize_dynamic` criterion harness (already has `CatmullRom`/
`Gaussian` cases at `400x300->32x24`, unmodified - no new bench work needed for this candidate).
"A" = that base with no gate at all. "N2"/"N3"/"N4"/"N6"/"N8" = A plus candidate 023's `run_resize`
gate (`SMALL_RESIZE_SOURCE_PIXEL_THRESHOLD = 500_000`, filter-blind,
`rayon::ThreadPoolBuilder::new().num_threads(N)` cached pool), varying only the `N` literal. Each
of the 5 `N` values was built as its own binary from the same source tree (only `resize.rs`'s
`num_threads(N)` line changed between builds). Comparisons run via `raw/paired_pic_scale.sh`
(alternating A/B, `--bench <id> --exact` against prebuilt criterion binaries, taking the criterion
`time: [.. mid ..]` estimate per run) - 10 paired runs per (N, filter, core-mode) combination.
`taskset -c 3` for single-core; unpinned for multi-core (24 logical cores, `uptime` load average
2.2-11.6 across this session, re-checked periodically to stay below the 24-core threshold). A
second independent 10-run repeat of A-vs-N6 (both filters, multi-core) and a direct N4-vs-N6
head-to-head (both filters, multi-core, 10 runs each) were run afterward to confirm the winning
`N` against noise and quantify the improvement over the untuned `N=4` directly.

| Variant / scenario | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | B-faster pairs |
|---|---|---|---|
| A, CatmullRom | 136.220-135.680 us (varies by run) | 95.909-114.630 us (varies by run) | - |
| N2, CatmullRom | 138.960 us (+2.0%) | 88.887 us (**-7.3%**) | single: 3/10; multi: 9/10 |
| N3, CatmullRom | 136.230 us (+0.4%) | 78.097 us (**-21.4%**) | single: 1/10; multi: 8/10 |
| N4, CatmullRom | 135.970 us (+0.5%) | 74.103 us (**-25.3%**) | single: 3/10; multi: 10/10 |
| N6, CatmullRom | 135.300 us (+0.7%) | 77.314 us (**-32.6%**) | single: 2/10; multi: 10/10 |
| N8, CatmullRom | 135.010 us (+0.9%) | 82.285 us (**-16.9%**) | single: 2/10; multi: 10/10 |
| A, Gaussian | 138.930-131.260 us (varies by run) | 90.677-112.260 us (varies by run) | - |
| N2, Gaussian | 132.140 us (-4.9%) | 103.330 us (**-8.0%**) | single: 7/10; multi: 7/10 |
| N3, Gaussian | 131.190 us (-0.1%) | 87.577 us (**-19.5%**) | single: 6/10; multi: 10/10 |
| N4, Gaussian | 131.560 us (+0.4%) | 82.487 us (**-24.0%**) | single: 2/10; multi: 10/10 |
| N6, Gaussian | 131.550 us (+1.0%) | 83.857 us (**-23.6%**) | single: 1/10; multi: 10/10 |
| N8, Gaussian | 131.530 us (+1.0%) | 74.554 us (**-21.7%**) | single: 2/10; multi: 10/10 |
| N6 vs A, CatmullRom (repeat, multi-core) | - | 67.371 us (**-28.2%**) | multi: 10/10 |
| N6 vs A, Gaussian (repeat, multi-core) | - | 60.940 us (**-32.8%**) | multi: 10/10 |
| N6 vs N4 direct, CatmullRom (multi-core) | - | 65.784 us (**-6.7%**) | multi: 9/10 |
| N6 vs N4 direct, Gaussian (multi-core) | - | 60.508 us (**-9.6%**) | multi: 9/10 |

Note the "A" baseline's own absolute numbers vary between rows in the first table (e.g. CatmullRom
95.9-114.6 us multi-core) because each `N` was measured as a separate `paired_pic_scale.sh`
invocation, minutes apart, and this machine's own documented mode-lottery noise (see
`projects/image.md`) shifts the baseline between invocations - each row's own A/B pairing is still
internally consistent since the script alternates the same two binaries within one run, which is
why the direct N6-vs-N4 and repeated N6-vs-A comparisons (run without an intervening baseline
re-measurement) are included as the more trustworthy same-session confirmation.

## Result

**`N=6` is the best or joint-best thread count found for both filters, and is a real, repeatable
improvement over the untuned `N=4` used by candidates 023/026, not just noise: a same-session
direct head-to-head (N6 vs N4, no intervening "A" re-baseline) shows N6 faster by -6.7% for
CatmullRom (9/10 pairs) and -9.6% for Gaussian (9/10 pairs), and an independent repeat of N6-vs-A
found -28.2% (CatmullRom) and -32.8% (Gaussian), both exceeding the original N4-vs-A results from
this same sweep (-25.3%/-24.0%) and from candidate 026 (-20.8% to -27.1%).** The full sweep's shape
is non-monotonic for CatmullRom (N2 -7.3%, N3 -21.4%, N4 -25.3%, N6 -32.6%, N8 -16.9% - peaking at
N6 then falling off at N8) and roughly flat from N4 through N8 for Gaussian (-24.0%, -23.6%,
-21.7%, all within a few percent of each other and of N6). Single-core is a wash for every `N` and
both filters (-4.9% to +2.0%, no `N` showing a decisive single-core cost), matching the pattern
candidates 023/026 already established. All multi-core wins are decisive by this workspace's own
9-10-of-10 bar; single-core reads are inconclusive by the same bar (1-7 of 10), consistent with
being a wash rather than a real single-core effect.

## Does it change behavior/output?

- **Bit-exact?** Yes. A temporary standalone example (`examples/_correctness_tmp_027.rs`, built and
  run, not committed - removed before the working tree was reverted) hashed (`DefaultHasher`, over
  pixel bytes via `to_rgba8()` and dimensions) `DynamicImage::resize_exact` output for all 5
  `FilterType` variants across 4 scenarios (`RgbImage`/`RgbaImage` at `400x300->32x24`, and a large
  4032x3024 `RgbImage` source resized to `64x48` and to `400x300`), built twice: once with no gate
  at all ("A"), once with the gate at `num_threads(6)` ("N6", the winning value). All 20 hashes
  matched exactly between the two builds (`raw/hashes_A.txt`, `raw/hashes_N6.txt`, identical apart
  from the unrelated compile-timing line) - expected, since changing how many rayon workers a call
  is dispatched to changes nothing about which chunks compute which output bytes.
- **Is this a breaking change?** No public API changes; behavior only differs in wall-clock time,
  not output, and only when the `rayon` feature is enabled (not currently forwarded into
  pic-scale-safe on `main` - this candidate's Cargo.toml change re-applies candidate 002's proposed
  forward, still not itself applied to `main`).
- Project test suite (`cargo test --release --features rayon`): 550/551 passed, both before
  (`raw/test_A_full.txt` - Cargo.toml forward + bench harness, no gate) and after
  (`raw/test_E_full.txt` - full patch with `num_threads(6)`), with the same single pre-existing
  unrelated failure both times (`resizing_with_catmul` in `tests/regression.rs` - same failure
  noted in candidates 007/008/011/016/019/020/021/022/023/024/026's `RESULT.md`s, not touched by
  this candidate).

## Why the result looks the way it does

Candidate 023's own theory (a small dedicated rayon pool has lower per-call dispatch/
synchronization cost than the large default global pool, largely independent of how much of the
small pool's own capacity the call's work actually uses) continues to explain the overall shape:
every capped `N` from 2 to 8 beats the uncapped default pool by a wide margin for both filters.
Why `N=6` specifically outperforms both `N=4` (candidates 023/026's untuned choice) and `N=8`
rather than the improvement monotonically tracking `N` upward is not confirmed by profiling in this
environment (`perf` unavailable, see `README.md`) - a plausible reading is that `N=6` sits closest
to whatever number of *chunks* pic-scale-safe's own `par_chunks_exact`/`par_chunks_exact_mut`
actually splits a 32-24-pixel output into for these two filters' row-height, so the pool's threads
are close to fully (not over- or under-) subscribed for the real work available, while `N=8`
reintroduces some of the larger pool's own dispatch/park-wake overhead for threads that end up with
little or no chunk to process. This is inferred from the measured curve's shape alone, not verified
against pic-scale-safe's own chunking logic or via disassembly/tracing.

## What's not done / open questions

- The non-monotonic N2->N3->N4->N6->N8 shape (peaking at N6, falling at N8) is not confirmed by
  profiling or by reading pic-scale-safe's own chunk-count logic for a 32x24 output - `perf` is
  unavailable in this environment (see `README.md`).
- `N` values above 8, or non-power/non-round values between 4 and 6 (e.g. 5, 7), were not tried -
  it's not known whether 6 is the true optimum or a coincidentally strong sample point in a curve
  with more structure between 4 and 8.
- `N=6` was not swept for `Triangle`/`Lanczos3` (candidate 023's original two filters) - it's not
  known whether `N=6` would also beat their own already-established `N=4` win, or whether the
  per-filter optimum genuinely differs (candidate 023 found `N=4` at least as good as `N=2` for
  both of those filters, but never tried `N=6`/`N=8` for them).
- Only the `400x300->32x24` scenario was swept; whether a different source/output size shifts the
  optimal `N` (e.g. toward a value proportional to how many chunks the actual pixel count divides
  into) was not investigated - same open item candidate 034 (`IDEAS.md`, the size-threshold-bisect
  row) already left open for the threshold itself.
- `Nearest` is still only bit-exact-checked (via the correctness harness), never separately timed
  at this size or swept over `N` - same gap candidates 019/021/022/023/024/026 already left open.
- `raw/paired_pic_scale.sh`'s bare-binary-name silent-failure bug (candidate 026's "What's not
  done") was worked around here (explicit `./` paths) but not fixed in the script itself.
