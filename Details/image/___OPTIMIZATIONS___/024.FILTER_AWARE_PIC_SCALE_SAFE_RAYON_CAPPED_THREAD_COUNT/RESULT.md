# pic-scale-safe rayon call-site gate - filter-aware, capped (not single) thread count

Combines candidate 023's capped rayon pool (`num_threads(4)`, not 021's `num_threads(1)`) with
candidate 022's filter-aware gate (only cheap-kernel filters route through the small dedicated
pool at small sizes; `Lanczos3`/`CatmullRom`/`Gaussian` always take the default, unconditional
pool). Answers `IDEAS.md`'s row dated 2026-08-25 asking whether this combination beats candidate
023's filter-blind `num_threads(4)` gate for `Lanczos3` specifically, since 023 left it as an
open question in its own "What's not done" section.

## What was tried

Same base as candidates 021/022/023: candidate 002's `pic-scale-safe/rayon` Cargo.toml forward +
candidate 019's `bench_resize_dynamic` criterion harness, bundled with this candidate's own change
to `run_resize` in `src/imageops/resize.rs` - same `SMALL_RESIZE_SOURCE_PIXEL_THRESHOLD = 500_000`
and `rayon::ThreadPoolBuilder::new().num_threads(4)` cached pool as candidate 023, but gated on
`cheap_kernel = matches!(alg, ResamplingFunction::Bilinear | ResamplingFunction::Nearest)` (022's
filter-aware condition) instead of 023's filter-blind gate - `Lanczos3`/`CatmullRom`/`Gaussian`
always take the unconditional default (global) pool regardless of size, unchanged from the "A"
baseline.

"A" = candidate 002+019 state (Cargo.toml forward + bench harness, no gate at all - identical
baseline used by candidates 021/022/023). "D" = A plus this candidate's filter-aware `num_threads(4)`
gate. Comparisons run via `raw/paired_pic_scale.sh` (candidate 023's script, reused unmodified;
alternating A/B, `--bench <id> --exact` against prebuilt criterion binaries, taking the criterion
`time: [.. mid ..]` estimate per run) - 10 paired runs for the small (`400x300->32x24`) scenarios,
6 for the large-size controls, matching this workspace's established convention. `taskset -c 3`
for single-core; unpinned for multi-core (24 logical cores, `uptime` load average 4.7-5.2 during
this session).

| Variant / scenario | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | B-faster pairs |
|---|---|---|---|
| A, 400x300->32x24 Triangle | 74.760 us | 70.623 us | - |
| D, 400x300->32x24 Triangle | 75.081 us (+0.4%) | 41.751 us (**-40.9%**) | single: 2/10; multi: 10/10 |
| A, 400x300->32x24 Lanczos3 | 210.820 us | 141.130 us | - |
| D, 400x300->32x24 Lanczos3 | 210.370 us (-0.2%) | 142.960 us (+1.3%) | single: 9/10; multi: 3/10 |
| A, 4032x3024->400x300 Lanczos3 (large control) | (not measured single-core) | 5449.800 us | - |
| D, same (large control) | | 5294.700 us (-2.8%) | multi: 5/6 |
| A, 4032x3024->64x48 Triangle (large-source control) | | 4263.200 us | - |
| D, same (large-source control) | | 4243.800 us (-0.5%) | multi: 4/6 |

Raw paired output: `raw/singlecore_triangle.txt`, `raw/multicore_triangle.txt`,
`raw/singlecore_lanczos.txt`, `raw/multicore_lanczos.txt`, `raw/multicore_large_lanczos.txt`,
`raw/multicore_large_triangle.txt`.

## Result

**`Triangle` (the filter this gate actually engages for at this size) gets the same decisive
multi-core win candidate 023 measured for its filter-blind `num_threads(4)` gate: -40.9% here vs.
023's -45.1%, both far better than 021's `num_threads(1)` regression (+2.7%) or 022's filter-aware-
but-single-thread wash (-0.7%).** `Lanczos3` multi-core is a wash here (+1.3%, 3/10 pairs), because
this gate never touches it - it takes the exact same unconditional default-pool code path as
baseline "A" for every size. This directly answers `IDEAS.md`'s open question: making the capped
pool filter-aware does **not** additionally help `Lanczos3` beyond what "A" already gets - it just
leaves `Lanczos3` exactly where "A" already was, giving up candidate 023's own filter-blind
`Lanczos3` win (-4.7%, 9/10 pairs) in exchange for never risking a `Lanczos3` regression from the
capped pool at all. Single-core and both large-size controls are within this workspace's
established noise floor for both filters, same pattern as candidates 021/022/023.

## Does it change behavior/output?

- **Bit-exact?** Yes. A temporary standalone example (`examples/_correctness_tmp_024.rs`, built
  and run, not committed - removed before the working tree was reverted) hashed
  (`DefaultHasher`, over pixel bytes and dimensions) `DynamicImage::resize_exact` output for all 5
  `FilterType` variants across 4 scenarios (`RgbImage`/`RgbaImage` at `400x300->32x24`, and a large
  4032x3024 `RgbImage` source resized to `64x48` and to `400x300`), built twice: once with no gate
  at all ("A"), once with this candidate's filter-aware `num_threads(4)` gate ("D"). All 20 hashes
  matched exactly between the two builds (`raw/hashes_A.txt`, `raw/hashes_D.txt`, identical apart
  from the unrelated compile-timing line) - expected, since changing how many rayon workers a call
  is dispatched to, or whether it's dispatched to a small pool at all, changes nothing about which
  chunks compute which output bytes.
- **Is this a breaking change?** No public API changes; behavior only differs in wall-clock time,
  not output, and only when the `rayon` feature is enabled (not currently forwarded into
  pic-scale-safe on `main` - this candidate's Cargo.toml change re-applies candidate 002's proposed
  forward, still not itself applied to `main`).
- Project test suite (`cargo test --release --features rayon`): 550/551 passed, both before
  (`raw/test_A_full.txt` - Cargo.toml forward + bench only, no gate, verified via `git stash`-ing
  just the `resize.rs` change) and after (`raw/test_D_full.txt` - full patch), with the same
  single pre-existing unrelated failure both times (`resizing_with_catmul` in
  `tests/regression.rs` - same failure noted in candidates 007/008/011/016/019/020/021/022/023's
  `RESULT.md`s, not touched by this candidate).

## Why the result looks the way it does

The `Triangle` result reproduces candidate 023's core finding almost exactly (-40.9% here vs.
023's -45.1%, both filter-blind and filter-aware routing `Triangle` through the same
`num_threads(4)` cached pool at this size) - see 023's own "Why the result looks the way it does"
for the underlying mechanism (a small dedicated pool appears to have lower per-call dispatch cost
than the large 24-thread global pool, not merely "fewer idle workers"). The `Lanczos3` number here
is simply unchanged from "A" by construction: this gate's `cheap_kernel` check
(`matches!(alg, ResamplingFunction::Bilinear | ResamplingFunction::Nearest)`) is `false` for
`Lanczos3`, so `run_resize` takes its `else` branch (`f()`, no pool install) every time, identical
to what "A" always did. Candidate 023's own -4.7% `Lanczos3` win came specifically from *also*
routing `Lanczos3` through the same small pool (which apparently has lower dispatch cost even for
a filter with real per-row work to parallelize) - a benefit this filter-aware variant deliberately
declines to take, on the theory (per `IDEAS.md`'s framing and candidate 019's finding that the
default pool already benefits `Lanczos3` at this size) that leaving costly filters on the default
pool was the safer, more conservative choice. The measurement here shows that conservatism has a
real, measured cost: it gives up a small but repeatable `Lanczos3` multi-core win (023's -4.7%)
for no measured benefit over the filter-blind version anywhere in this session's numbers.

## What's not done / open questions

- This candidate directly answers `IDEAS.md`'s question ("worth checking whether a filter-aware
  N=4 gate beats this candidate's filter-blind N=4 gate for `Lanczos3` specifically") with "no" -
  the filter-aware gate is not better than the filter-blind one for `Lanczos3`; it is strictly a
  wash where 023 was a small win. Filter-blind (023) therefore looks like the more complete
  version of this mechanism  so far measured in this workspace, not filter-aware (this candidate).
- `N` values other than 4 were not tried in this filter-aware shape (same gap 023 itself left for
  its own filter-blind shape).
- `Nearest` (included in this candidate's `cheap_kernel` predicate alongside `Bilinear`/`Triangle`)
  was correctness-checked (bit-exact, both builds) but not separately timed at this size - no
  existing bench case exercises `DynamicImage::resize_exact` with `FilterType::Nearest` at
  `400x300->32x24`; same gap candidates 019/021/022/023 left open for `Nearest`/`CatmullRom`/
  `Gaussian` generally.
- The 500,000-source-pixel threshold itself was not bisected (same open item carried over from
  candidates 019/021/022/023).
- Single-core numbers for the two large-size control scenarios were not collected (same choice
  candidates 021/022/023 made, for the same reason: multi-core is the configuration this
  mechanism actually depends on).
- No `perf`/disassembly confirmation of candidate 023's "small pool = cheaper dispatch" theory -
  `perf` is not installed in this environment (see `README.md`).
