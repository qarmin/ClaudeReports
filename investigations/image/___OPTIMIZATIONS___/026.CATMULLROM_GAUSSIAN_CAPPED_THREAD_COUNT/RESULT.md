# pic-scale-safe rayon call-site gate - filter-blind capped thread count, CatmullRom/Gaussian

Follows up on `IDEAS.md`'s row dated 2026-08-25 (candidate 024's open question): candidate 023
established that a filter-*blind* `num_threads(4)` capped rayon pool at the pic-scale-safe small-
resize call site is a large, decisive multi-core win for `Triangle` (-45.1%) and a small win for
`Lanczos3` (-4.7%) at `400x300->32x24`, but only bit-exact-verified (not timed) `CatmullRom` and
`Gaussian` at that size. This candidate adds `bench_resize_dynamic` cases for those two filters at
the same size and measures them with the exact same gate (candidate 023's patch, unmodified) to
check whether they follow `Lanczos3`'s pattern (wider-kernel filter, real per-row work, still
benefits) as candidate 023's own dispatch-cost theory predicts.

## What was tried

Same base as candidates 021/022/023/024: candidate 002's `pic-scale-safe/rayon` Cargo.toml forward
+ candidate 019's `bench_resize_dynamic` criterion harness, extended by this candidate with two new
`400x300->32x24` cases (`CatmullRom`, `Gaussian`) alongside the existing `Lanczos3`/`Triangle`
ones, bundled with candidate 023's exact `run_resize` gate in `src/imageops/resize.rs`
(`SMALL_RESIZE_SOURCE_PIXEL_THRESHOLD = 500_000`, filter-blind, `rayon::ThreadPoolBuilder::new().num_threads(4)`
cached pool) - unchanged from 023's own `patch.diff`, no new gate logic written for this candidate.

"A" = candidate 002+019 state plus this candidate's two new bench cases (Cargo.toml forward +
extended bench harness, no gate at all - same baseline shape as 021/022/023/024, extended with the
two new filters). "E" = A plus candidate 023's unmodified `num_threads(4)` gate. Comparisons run
via `raw/paired_pic_scale.sh` (candidate 023's script, reused unmodified apart from invoking the
binaries via an explicit `./` path - the original script's bare binary names silently failed to
execute and printed empty timings, a script bug fixed here, see "What's not done" below):
alternating A/B, `--bench <id> --exact` against prebuilt criterion binaries, taking the criterion
`time: [.. mid ..]` estimate per run - 10 paired runs per scenario, plus a second independent
10-run repeat for `CatmullRom` multi-core to check the noise floor (matching this workspace's
established convention). `taskset -c 3` for single-core; unpinned for multi-core (24 logical
cores, `uptime` load average 2.7-11.6 across this session, re-checked before each run to stay
below the 24-core threshold).

| Variant / scenario | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | E-faster pairs |
|---|---|---|---|
| A, 400x300->32x24 CatmullRom | 135.890 us | 94.914 us (run 1) / 97.288 us (run 2) | - |
| E, 400x300->32x24 CatmullRom | 136.750 us (+0.6%) | 71.763 us (**-24.4%**, run 1) / 77.092 us (**-20.8%**, run 2) | single: 2/10; multi: 9/10 (run 1), 10/10 (run 2) |
| A, 400x300->32x24 Gaussian | 132.720 us | 92.805 us | - |
| E, 400x300->32x24 Gaussian | 133.740 us (+0.8%) | 67.624 us (**-27.1%**) | single: 5/10; multi: 10/10 |

## Result

**Both `CatmullRom` and `Gaussian` follow candidate 023's `Lanczos3` pattern, not its `Triangle`
pattern: a clear, decisive multi-core win (-24.4%/-20.8% for `CatmullRom` across two independent
10-run repeats, -27.1% for `Gaussian`, all with 9-10 of 10 pairs favoring the patch) with a
single-core wash (+0.6% to +0.8%, 2-5 of 10 pairs) - the same shape candidate 023 measured for
`Lanczos3` (-4.7% multi-core, +0.4% single-core) but noticeably larger in magnitude, closer to (in
`Gaussian`'s case, exceeding) `Triangle`'s own multi-core win (-45.1%).** This confirms candidate
023's own theory (a small dedicated rayon pool has lower per-call dispatch overhead than the
default large pool, largely independent of how much of that pool's capacity the call's own work
actually uses) generalizes cleanly to these two wider-kernel filters, and does so with a bigger win
than `Lanczos3` got - not just a marginal extension of it.

## Does it change behavior/output?

- **Bit-exact?** Yes. A temporary standalone example (`examples/_correctness_tmp_026.rs`, built
  and run, not committed - removed before the working tree was reverted) hashed
  (`DefaultHasher`, over pixel bytes and dimensions) `DynamicImage::resize_exact` output for all 5
  `FilterType` variants across 4 scenarios (`RgbImage`/`RgbaImage` at `400x300->32x24`, and a large
  4032x3024 `RgbImage` source resized to `64x48` and to `400x300`), built twice: once with no gate
  at all ("A"), once with candidate 023's `num_threads(4)` gate applied ("E"). All 20 hashes
  matched exactly between the two builds (`raw/hashes_A.txt`, `raw/hashes_E.txt`, identical apart
  from the unrelated compile-timing line) and match candidate 023's own recorded hashes for the
  same 20 cases exactly - expected, since this candidate reuses 023's gate unmodified and changing
  how many rayon workers a call is dispatched to changes nothing about which chunks compute which
  output bytes.
- **Is this a breaking change?** No public API changes; behavior only differs in wall-clock time,
  not output, and only when the `rayon` feature is enabled (not currently forwarded into
  pic-scale-safe on `main` - this candidate's Cargo.toml change re-applies candidate 002's proposed
  forward, still not itself applied to `main`).
- Project test suite (`cargo test --release --features rayon`): 550/551 passed, both before
  (`raw/test_A_full.txt` - Cargo.toml forward + extended bench harness, no gate, verified via
  `git stash`-ing just the `resize.rs` change and re-running) and after (`raw/test_E_full.txt` -
  full patch), with the same single pre-existing unrelated failure both times
  (`resizing_with_catmul` in `tests/regression.rs` - same failure noted in candidates
  007/008/011/016/019/020/021/022/023/024's `RESULT.md`s, not touched by this candidate).

## Why the result looks the way it does

Candidate 023's own "why" section already laid out the mechanism: routing a small resize call
through a small, dedicated `num_threads(4)` rayon pool via `ThreadPool::install()` appears to have
lower per-call dispatch/synchronization cost than letting pic-scale-safe's internal
`par_chunks_exact`/`par_chunks_exact_mut` fan out across the large (24-thread) default global pool
- an effect visible even for filters with real per-row work to parallelize (`Lanczos3`), not just
cheap filters with little real work (`Triangle`). `CatmullRom` and `Gaussian` (both wider-kernel,
similar in per-pixel arithmetic cost shape to `Lanczos3` per candidates 018/019's own filter-cost
characterization) land in the same "real multi-core win, single-core wash" bucket as `Lanczos3`,
consistent with that theory. The larger magnitude here (-20.8% to -27.1%, vs. `Lanczos3`'s -4.7%)
is not explained by this session's measurements alone: one plausible reading is that `CatmullRom`/
`Gaussian`'s per-row convolution cost sits somewhere between `Triangle`'s (cheap, dispatch-cost-
dominated, hence a `Triangle`-sized win) and `Lanczos3`'s (already has enough real work at this
size to partially hide dispatch cost, hence a smaller win) - but this is inferred from the relative
magnitudes, not confirmed by `perf`/disassembly (unavailable in this environment, see `README.md`).

## What's not done / open questions

- The relative-magnitude ordering guess above (`CatmullRom`/`Gaussian` between `Triangle` and
  `Lanczos3` in per-row cost) is not confirmed by profiling - `perf` is unavailable in this
  environment.
- `Nearest` is still only bit-exact-checked, never separately timed at this size - same gap
  candidates 019/021/022/023/024 already left open, not closed by this candidate.
- The 500,000-source-pixel threshold and the `N=4` thread count itself were not re-swept for these
  two filters specifically (both reused unmodified from candidate 023); it's not known whether a
  different `N` would do even better for `CatmullRom`/`Gaussian` given their larger measured win
  here relative to `Lanczos3`.
- Single-core and large-size control scenarios for `CatmullRom`/`Gaussian` specifically were not
  collected (same choice candidates 021-024 made for `Lanczos3`/`Triangle`, for the same reason:
  the large-size controls already established in those candidates exercise the same "does the gate
  regress above-threshold resizes" question and are filter-blind by construction, so they cover
  `CatmullRom`/`Gaussian` too without re-running them).
- `raw/paired_pic_scale.sh` (candidate 023's script, reused here) silently produces all-zero/`-nan%`
  output if invoked with bare binary names not on `PATH` (the underlying `bench_A`/`bench_E`
  commands fail with "command not found," which `2>/dev/null` swallows) - worth fixing the script
  itself (quote/resolve the binary path, or fail loudly if `run_one` returns empty) so a future
  session doesn't lose time to the same silent-empty-result trap; not fixed in the shared
  `tools/`-equivalent copy this session, only worked around locally by invoking it with explicit
  `./` prefixes.
