# pic-scale-safe rayon call-site gate - capped (not forced to 1) thread count

Follows up on candidates 021/022 and `IDEAS.md`'s row for this: those two candidates' call-site
workaround for pic-scale-safe's unconditional `rayon` fan-out (routing small resizes through a
cached `rayon::ThreadPool::install()` below a 500,000-source-pixel threshold) always capped the
pool at `num_threads(1)`, which fixed `Triangle`'s small-resize regression but (021, filter-blind
gate) badly regressed `Lanczos3` by throwing away a real multi-core win, or (022, filter-aware
gate) left `Triangle` itself only a noisy wash rather than a clear win. This candidate tests the
one thing both left open: keeping the same threshold/pool mechanism but capping the pool at 2 or 4
threads instead of 1, to see whether a small-but->1 thread count could recover `Lanczos3`'s lost
parallelism while still cutting the small-resize overhead for cheap filters like `Triangle`.

## What was tried

Same base as candidates 021/022: candidate 002's `pic-scale-safe/rayon` Cargo.toml forward +
candidate 019's `bench_resize_dynamic` criterion harness, bundled with this candidate's own change
- `run_resize` in `src/imageops/resize.rs`, filter-blind (like candidate 021, not candidate 022's
filter-aware gate), same `SMALL_RESIZE_SOURCE_PIXEL_THRESHOLD = 500_000`, but with
`rayon::ThreadPoolBuilder::new().num_threads(N)` for `N = 2` and `N = 4` instead of 021's
`num_threads(1)`. Both `N` values were built and measured as separate binaries in the same
session; `patch.diff` in this folder captures the `N = 4` variant (the one that measured
consistently at least as good as `N = 2` everywhere it mattered - see table).

"A" = candidate 002+019 state (Cargo.toml forward + bench harness, no gate at all - identical
baseline used by candidates 021/022). "C2"/"C4" = A plus this candidate's gate at
`num_threads(2)`/`num_threads(4)` respectively. Comparisons run via `raw/paired_pic_scale.sh`
(alternating A/B, `--bench <id> --exact` against prebuilt criterion binaries, taking the criterion
`time: [.. mid ..]` estimate per run) - 10 paired runs for the small (`400x300->32x24`) scenarios,
6 for the large-size controls, matching this workspace's established convention. `taskset -c 3` for
single-core; unpinned for multi-core (24 logical cores, `uptime` load average 2.7-4.4 during this
session).

| Variant / scenario | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | B-faster pairs |
|---|---|---|---|
| A, 400x300->32x24 Triangle | 107.660 us | 72.861 us | - |
| C2, 400x300->32x24 Triangle | 109.020 us (+1.3%) | 47.517 us (**-34.8%**) | single: 5/10; multi: 10/10 |
| C4, 400x300->32x24 Triangle | 74.975 us (+0.4%, different A-run) | 38.170 us (**-45.1%**) | single: 5/10; multi: 10/10 |
| A, 400x300->32x24 Lanczos3 | 215.840 us | 143.480 us | - |
| C2, 400x300->32x24 Lanczos3 | 215.950 us (+0.1%) | 140.140 us (-2.3%) | single: 4/10; multi: 7/10 |
| C4, 400x300->32x24 Lanczos3 | 211.170 us (+0.4%, different A-run) | 138.520 us (-4.7%) | single: 4/10; multi: 9/10 |
| A, 4032x3024->400x300 Lanczos3 (large control) | (not measured single-core) | 5226.600 / 5302.700 us | - |
| C2/C4, same (large control) | | 5259.500 (+0.6%) / 5338.100 us (+0.7%) | multi: 2/6, 4/6 |
| A, 4032x3024->64x48 Triangle (large-source control) | | 3870.900 / 3836.800 us | - |
| C2/C4, same (large-source control) | | 4012.900 (+3.7%) / 4000.800 us (+4.3%) | multi: 1/6, 2/6 |

Note the single-core Triangle "A" baseline differs between the C2 and C4 rows (107.660 us vs
74.661 us) - these were two separately re-launched `bench_A` runs on the same binary, minutes
apart in the same session; the workspace's own documented mode-lottery noise (see
`projects/image.md`) explains the gap, not a real change in "A" itself - each row's own A/B pairing
is still internally consistent since `paired_pic_scale.sh` alternates the same two binaries within
one run. Raw paired output: `raw/results_c2.txt`, `raw/results_c2_singlecore_triangle.txt` (a
same-session run captured before `raw/paired_pic_scale.sh` existed, using an equivalent by-hand
loop over the same two binaries), `raw/results_c4.txt`.

## Result

**Both capped-thread variants (2 and 4) are a clear, decisive win for `Triangle` on multi-core -
far better than candidate 021's `num_threads(1)` (which was a small +2.7% regression there) or
022's filter-aware wash (-0.7%) - while also recovering `Lanczos3`'s multi-core performance far
better than 021's severe +47.4% regression, landing as a small win (`num_threads(4)`, -4.7%,
9/10 pairs) or a near-wash (`num_threads(2)`, -2.3%, 7/10 pairs) instead.** `num_threads(4)`
measured at least as good as `num_threads(2)` on every scenario tried this session (Triangle
multi-core: -45.1% vs -34.8%; Lanczos3 multi-core: -4.7%/9-10 vs -2.3%/7-10), so `patch.diff`
captures the `N = 4` variant.

Single-core (`taskset -c 3`) is a wash for both filters and both `N` values (+0.1% to +1.3%, 4-5/10
pairs) - expected, since pinning to one core already means every worker thread in either pool
lands on that same core regardless of the pool's own configured size. The large-size controls
(`4032x3024->400x300` Lanczos3 and `4032x3024->64x48` Triangle, both far above the 500,000-pixel
threshold) show a small, consistent-direction-but-inconclusive-pair-count regression (+0.6% to
+4.3%, 1-4 of 6 pairs favoring the patch) for both `N` values - within this workspace's established
noise floor for a 6-repeat control case, and expected to be unaffected either way since the gate
never engages above the threshold (same code path as "A" there); this echoes candidates 021/022's
own large-control numbers (+0.4%/+2.0% and +0.1%/-0.7% respectively), all in the same noisy,
inconclusive range.

## Does it change behavior/output?

- **Bit-exact?** Yes. A temporary standalone example (`examples/_correctness_tmp_023.rs`, built
  and run, not committed - removed before the working tree was reverted) hashed
  (`DefaultHasher`, over pixel bytes and dimensions) `DynamicImage::resize_exact` output for all 5
  `FilterType` variants across 4 scenarios (`RgbImage`/`RgbaImage` at `400x300->32x24`, and a large
  4032x3024 `RgbImage` source resized to `64x48` and to `400x300`), built three times: once with no
  gate at all ("A"), once with the gate at `num_threads(2)`, once at `num_threads(4)`. All 20 hashes
  matched exactly across all three builds (`raw/hashes_A.txt`, `raw/hashes_C2.txt`,
  `raw/hashes_C4.txt`, identical apart from the unrelated compile-timing line) - expected, since
  changing how many rayon workers a call is dispatched to changes nothing about which chunks
  compute which output bytes.
- **Is this a breaking change?** No public API changes; behavior only differs in wall-clock time,
  not output, and only when the `rayon` feature is enabled (not currently forwarded into
  pic-scale-safe on `main` - this candidate's Cargo.toml change re-applies candidate 002's proposed
  forward, still not itself applied to `main`).
- Project test suite (`cargo test --release --features rayon`): 550/551 passed, both before
  (Cargo.toml forward + bench only, no gate - verified by `git stash`-ing just the `resize.rs`
  change and re-running) and after (full patch, `num_threads(4)` variant), with the same single
  pre-existing unrelated failure both times (`resizing_with_catmul` in `tests/regression.rs` - same
  failure noted in candidates 007/008/011/016/019/020/021/022's `RESULT.md`s, not touched by this
  candidate).

## Why the result looks the way it does

Candidate 021's own `RESULT.md` already identified the likely mechanism: routing a call through
`ThreadPool::install()` doesn't reproduce "serial, no rayon at all" - it still pays rayon's
task-dispatch/park-wake machinery to hand the whole call to the pool, but *on a much smaller pool*
than the default global one (24 threads on this machine) that pic-scale-safe's own
`par_chunks_exact`/`par_chunks_exact_mut` would otherwise fan out across uninvited. The surprising
part this candidate's numbers reveal is that even the *dispatch itself* is cheaper on a small
dedicated pool than on the large global one: `Triangle` multi-core wasn't just "no longer
regressed" the way 022's filter-aware gate left it (-0.7%, a wash) - it was dramatically *faster*
than "A" (-34.8% at `N=2`, -45.1% at `N=4`), which is more improvement than would be explained by
"skip the overhead of 20+ idle workers" alone if the *baseline* per-call dispatch cost itself were
fixed. The most likely explanation, not confirmed by `perf`/disassembly (unavailable in this
environment - see `README.md`): rayon's work-stealing dispatch has to publish/steal across however
many threads the pool actually has, so handing a tiny job to a 24-thread global pool costs more
synchronization (cache-line bouncing across whichever cores those threads are pinned/scheduled on)
than handing the same job to a 2- or 4-thread pool, independent of how much of that pool's capacity
is actually used for the tiny job's own chunks. `Lanczos3` benefits from the same reduced-dispatch-
cost effect, but starting from a smaller relative gap (it already has enough real per-row work to
partially hide dispatch cost), which is consistent with `N=4`'s larger benefit than `N=2`'s for
`Lanczos3` specifically (more real parallel workers available for a filter that has real
work to parallelize) while `Triangle` (which has little real work to parallelize regardless of
`N`) shows a much smaller `N=2`-to-`N=4` gap (-34.8% to -45.1%, both already large).

## What's not done / open questions

- Only `N=2` and `N=4` were tried, both filter-blind (candidate 021's gate shape, not candidate
  022's filter-aware one) - combining a capped-but->1 thread count with a filter-aware gate (only
  applying the cap to cheap-kernel filters, leaving costly filters on the full default pool
  unconditionally) was not tried, and per the "why" section above might do even better for
  `Lanczos3` if the mechanism really is "small-pool dispatch is cheaper independent of filter" -
  worth checking whether a filter-aware N=4 gate beats this candidate's filter-blind N=4 gate for
  `Lanczos3` specifically.
- `N` values between 2 and 4, or above 4 (e.g. `num_threads(8)`), were not swept - it's not known
  whether the improvement continues to grow with `N` up to some point and then reverses (more
  workers again costing more dispatch sync) or plateaus after 4.
- The 500,000-source-pixel threshold itself was not bisected (same open item carried over from
  candidates 019/021/022).
- Single-core numbers for the two large-size control scenarios were not collected (same choice
  candidates 021/022 made, for the same reason: multi-core is the configuration this mechanism
  actually depends on).
- No `perf`/disassembly confirmation of the "small pool = cheaper dispatch, not just fewer idle
  workers" theory in the "why" section above - `perf` is not installed in this environment (see
  `README.md`).
- `Nearest`/`CatmullRom`/`Gaussian` were correctness-checked (bit-exact, all three builds) but not
  separately timed at this size - same gap candidates 019/021/022 already left open.
