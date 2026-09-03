# pic-scale-safe rayon call-site gate - per-filter capped thread count

Follows up on `IDEAS.md`'s row dated 2026-08-25 (candidate 028's open question): candidate 028
found `N=4` stays best for `Triangle`/`Lanczos3`, while candidate 027 separately found `N=6` a
real, repeatable win for `CatmullRom`/`Gaussian` on the identical `400x300->32x24` scenario and
gate mechanism - but every prior candidate in this line (021/022/023/024/026/027/028) only ever
built a gate with a single constant `N` applied to every filter it covered. This candidate builds
an actual per-filter gate (two cached rayon pools, keyed on `ResamplingFunction` at the call site:
`num_threads(4)` for `Nearest`/`Bilinear`/`Lanczos3`, `num_threads(6)` for
`CatmullRom`/`Gaussian`) and measures all four timed filters in one build to confirm both
per-filter wins hold simultaneously rather than being a wash/regression trade-off.

## What was tried

Same base as candidates 021-024/026-028: candidate 002's `pic-scale-safe/rayon` Cargo.toml forward
+ candidate 026's `bench_resize_dynamic` criterion harness (already has all four filters -
`Triangle`, `Lanczos3`, `CatmullRom`, `Gaussian` - at `400x300->32x24`, unmodified, no new bench
work needed), combined with a new `run_resize` in `src/imageops/resize.rs`: same
`SMALL_RESIZE_SOURCE_PIXEL_THRESHOLD = 500_000` gate from candidate 021, but two separate cached
`OnceLock<rayon::ThreadPool>` instances instead of one - a `match` on the `ResamplingFunction`
passed in from `resize_impl` routes `CatmullRom`/`Gaussian` to a `num_threads(6)` pool and
everything else (`Nearest`, `Bilinear` i.e. `Triangle`, `Lanczos3`) to a `num_threads(4)` pool.
"A" = harness base with no gate at all. "E" = A plus this per-filter gate. Comparisons run via
`raw/paired_pic_scale.sh` (candidates 023/026/027/028's script, reused unmodified, invoked via
explicit `./` paths per the known bare-binary-name workaround) - 6 paired runs per filter for
single-core, 10 per filter for multi-core, matching this workspace's established convention.
`taskset -c 3` for single-core; unpinned for multi-core (24 logical cores, `uptime` load average
2.9-7.6 across this session, re-checked before starting the run, stayed below the 24-core
threshold throughout).

| Variant / scenario | single-core p50 (`taskset -c 3`, 6 runs) | multi-core p50 (unpinned, 10 runs) | E-faster pairs |
|---|---|---|---|
| A, Triangle | 75.631 us | 71.508 us | - |
| E, Triangle (N4 pool) | 76.069 us (+0.6%) | 39.852 us (**-44.3%**) | single: 2/6; multi: 10/10 |
| A, Lanczos3 | 210.970 us | 144.790 us | - |
| E, Lanczos3 (N4 pool) | 212.590 us (+0.8%) | 120.690 us (**-16.6%**) | single: 0/6; multi: 10/10 |
| A, CatmullRom | 136.740 us | 98.886 us | - |
| E, CatmullRom (N6 pool) | 137.970 us (+0.9%) | 72.388 us (**-26.8%**) | single: 1/6; multi: 10/10 |
| A, Gaussian | 132.980 us | 93.544 us | - |
| E, Gaussian (N6 pool) | 134.690 us (+1.3%) | 67.187 us (**-28.2%**) | single: 2/6; multi: 10/10 |

Raw paired output: `raw/singlecore_*.log`, `raw/multicore_*.log`.

## Result

**All four timed filters keep their own already-established multi-core win simultaneously in one
build: `Triangle` -44.3% (10/10 pairs, matching candidate 023's earlier `N=4`-pool measurement of
-45.1%), `Lanczos3` -16.6% (10/10 pairs, in the same direction as candidate 023's earlier -4.7%
though noisier this run), `CatmullRom` -26.8% (10/10 pairs, in candidate 027's `N=6` range of
-6.7% to -32.6% depending on run), `Gaussian` -28.2% (10/10 pairs, close to candidate 026/027's
-20.8% to -32.8% range).** No filter shows the wash/regression risk candidate 028 found for
`Triangle`/`Lanczos3` at `N=6`/`N=8` - because those two filters are routed to the `N=4` pool by
this gate's `match`, never the `N=6` one. Single-core is a wash for every filter (+0.6% to +1.3%,
0-2 of 6 pairs favoring the patch), matching the pattern every prior candidate in this line
(021-024, 026-028) already established. This is a combination-and-confirmation result, not a new
measurement of either `N` value in isolation - the `N=4`/`N=6` numbers themselves were already
established by candidates 023/026/027/028; what this candidate adds is that a single gate can
route each filter to its own already-known-best `N` and both effects hold at once, in the same
binary, rather than one constant needing to compromise between them.

## Does it change behavior/output?

- **Bit-exact?** Yes. A temporary standalone example (`examples/_correctness_tmp_029.rs`, built and
  run, not committed - removed before the working tree was reverted) hashed (`DefaultHasher`, over
  pixel bytes via `to_rgba8()` and dimensions) `DynamicImage::resize_exact` output for all 5
  `FilterType` variants across 4 scenarios (`RgbImage`/`RgbaImage` at `400x300->32x24`, and a large
  4032x3024 `RgbImage` source resized to `64x48` and to `400x300`), built twice: once with no gate
  at all ("A"), once with the per-filter gate ("E"). All 20 hashes matched exactly between the two
  builds (`raw/hashes_A.txt`, `raw/hashes_E.txt`) and match candidate 027's own recorded "A" hashes
  for the same 20 cases exactly - expected, since changing how many rayon workers a call is
  dispatched to, or which of two pools it's dispatched to, changes nothing about which chunks
  compute which output bytes.
- **Is this a breaking change?** No public API changes; behavior only differs in wall-clock time,
  not output, and only when the `rayon` feature is enabled (not currently forwarded into
  pic-scale-safe on `main` - this candidate's Cargo.toml change re-applies candidate 002's proposed
  forward, still not itself applied to `main`).
- Project test suite (`cargo test --release --features rayon`): 550/551 passed, both before
  (`raw/test_A_full.txt` - Cargo.toml forward + bench harness only, no gate) and after
  (`raw/test_E_full.txt` - full patch with the per-filter gate), with the same single pre-existing
  unrelated failure both times (`resizing_with_catmul` in `tests/regression.rs` - same failure
  noted in candidates 007/008/011/016/019/020/021/022/023/024/026/027/028's `RESULT.md`s, not
  touched by this candidate).

## Why the result looks the way it does

Candidates 023/027/028 already established that the per-call dispatch/synchronization cost of a
small dedicated rayon pool beats the default large global pool for a small resize regardless of
which capped `N` is used (2 through 8), and that the specific optimal `N` differs by filter
(027's `N=6` peak for `CatmullRom`/`Gaussian`'s row-height/kernel-width combination vs. 028's `N=4`
staying best for `Triangle`/`Lanczos3`). This candidate's result is the direct, expected
consequence of combining those two already-measured facts into one gate: since the `match` on
`ResamplingFunction` sends each filter to the pool candidates 027/028 already found best for it,
and pic-scale-safe's own chunking/output computation is unaffected by which pool a call runs on
(confirmed bit-exact above), there is no interaction effect to discover here - the four filters'
wins are independent of each other (each filter's own resize call only ever touches its own
matched pool, never the other one), so all four hold simultaneously. No new mechanism was
introduced beyond the two-pool `match`, so no further "why" investigation was needed beyond
confirming this expectation held in practice.

## What's not done / open questions

- Only `Nearest`/`Bilinear` (i.e. `Triangle`)/`Lanczos3` were routed to the `N=4` pool by this
  gate's `match`; `Nearest` itself is still only bit-exact-checked, never separately timed - same
  gap every prior candidate in this line (019/021-024/026-028) already left open. If `Nearest`
  were ever found to want a different `N` than `Bilinear`/`Lanczos3`, this gate's `match` would
  need a third pool.
- Only the `400x300->32x24` scenario was checked (same open item every prior thread-count-sweep
  candidate in this line - 021-024, 026-028 - already left open); whether the optimal per-filter
  `N` shifts with a differently-shaped small output was not investigated here either.
- The "pool size should roughly match the real chunk count for this output shape" theory (carried
  over from candidates 027/028) was not confirmed by reading pic-scale-safe's own
  `par_chunks_exact`/`par_chunks_exact_mut` chunk-size calculation or via `perf`/disassembly
  (unavailable in this environment, see `README.md`) - same gap left open by every prior candidate
  in this line.
- `N` values other than 4/6 were not re-swept for any filter in this candidate - it reuses
  candidates 027/028's already-established per-filter optimum without re-deriving it.
