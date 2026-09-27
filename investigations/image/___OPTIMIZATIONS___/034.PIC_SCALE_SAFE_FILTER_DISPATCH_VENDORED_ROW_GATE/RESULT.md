# Extend candidate 033's vendored per-function row/size gate from resize_nearest to fixed_point_dispatch.rs/floating_point_dispatch.rs (Triangle/CatmullRom/Gaussian/Lanczos3)

Follows `IDEAS.md` row 49 (2026-08-25, sourced from candidate 033's own "What's not done"
section): candidate 033 vendored pic-scale-safe 0.1.11 and added a `dst_height >= 32`
minimum-row gate inside `resize_nearest`'s own rayon dispatch, closing a 26-37x regression for
`FilterType::Nearest` at small output sizes, but left every other filter's own dispatch site
(`convolve_trampoline_fixed_point`/`convolve_row_fixed_point`/`convolve_column_fixed_point` in
`fixed_point_dispatch.rs`, and their floating-point mirrors in `floating_point_dispatch.rs`)
untouched, even though candidate 019's own data showed some of them (`Triangle`) also regress at
small sizes under pic-scale-safe's unconditional `rayon` dispatch. This candidate applies the
identical style of gate (same `dst_height >= 32` threshold, same "fall back to the crate's own
existing serial loop below the threshold" mechanism) to all three dispatch functions in both
files, and measures `Triangle`/`CatmullRom`/`Gaussian`/`Lanczos3` at the same `400x300->32x24`
scenario this whole 019-033 line has always used.

## What was tried

Re-applied candidate 033's vendoring (`vendor/pic-scale-safe/` + `Cargo.toml`'s
`[patch.crates-io]` override and `pic-scale-safe/rayon` feature forward) as the starting point,
then edited `vendor/pic-scale-safe/src/fixed_point_dispatch.rs` and
`floating_point_dispatch.rs`: each of the three dispatch functions per file
(`convolve_trampoline_*`, `convolve_row_*`, `convolve_column_*`) had its
`#[cfg(feature = "rayon")]` block turned into `#[cfg(feature = "rayon")] if destination_size.height
>= 32 { ...unchanged parallel code...; return; }`, and its `#[cfg(not(feature = "rayon"))]` block
(previously compiled only when the feature was absent) made unconditional as the fallback taken
either when the feature is off, or when it's on but the row count is below the threshold - the
same mechanism candidate 033 used for `resize_nearest`, just applied to 6 more call sites (3
functions x 2 files) instead of 1.

Three source states, same naming convention as candidates 029-033:

- **S (serial / current main)** - pristine `Cargo.toml`, registry pic-scale-safe, built with
  `--no-default-features --features default-formats` (`pic-scale-safe`'s own `rayon` feature
  never compiled in - literally what `main` does today).
- **A (parallel, ungated)** - candidate 002's `Cargo.toml` rayon-forward only, registry
  pic-scale-safe (no vendoring, no gate) - the already-measured regression baseline from
  candidates 019-032, rebuilt fresh this session as a same-session control.
- **E (this candidate)** - the vendored, gated `pic-scale-safe` (all 4 non-Nearest filters'
  dispatch sites now gated, not just `resize_nearest`), built with `--features rayon`.

Binaries alternated via `raw/paired_pic_scale.sh` (unchanged from candidates 023-033).
`taskset -c 3` for single-core, unpinned for multi-core (24 logical cores). `uptime` load average
1.0-7.3 during this session (well under the 24-core count). 6 paired runs per comparison (fewer
than 033's 8/10-pair convention - reduced to fit four filters x four comparisons in one session;
still enough to read decisiveness via this workspace's own `wins >= n-1` / `<= 1` bar). `bin_S` was
also rebuilt with the harness added via candidate 033's own `patch.diff` (unchanged this session)
since it isn't on `main`.

| Filter | Comparison | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) |
|---|---|---|---|
| Triangle | S -> E | 64.055 -> 64.265 us (+0.3%, 4/6 E-faster) | 64.644 -> 63.590 us (-1.6%, 5/6 E-faster) |
| Triangle | A -> E | 72.820 -> 64.371 us (**-11.6%**, 6/6 E-faster) | 66.927 -> 64.778 us (**-3.2%**, 6/6 E-faster) |
| CatmullRom | S -> E | 124.940 -> 124.980 us (+0.0%, 2/6 E-faster) | 125.690 -> 124.100 us (-1.3%, 3/6 E-faster) |
| CatmullRom | A -> E | 133.690 -> 125.990 us (**-5.8%**, 6/6 E-faster) | 93.720 -> 124.180 us (**+32.5%**, 1/6 E-faster) run 1; 95.282 -> 126.540 us (+32.8%, 1/6) run 2 |
| Gaussian | S -> E | 120.530 -> 120.990 us (+0.4%, 1/6 E-faster) | 118.810 -> 120.700 us (+1.6%, 2/6 E-faster) |
| Gaussian | A -> E | 131.460 -> 121.190 us (**-7.8%**, 6/6 E-faster) | 114.800 -> 143.460 us (**+25.0%**, 0/6 E-faster) |
| Lanczos3 | S -> E | 197.970 -> 197.550 us (-0.2%, 3/6 E-faster) | 193.830 -> 194.600 us (+0.4%, 2/6 E-faster) |
| Lanczos3 | A -> E | 204.790 -> 196.450 us (**-4.1%**, 6/6 E-faster) | 138.760 -> 195.940 us (**+41.2%**, 0/6 E-faster) |

Raw paired output under `raw/`: `singlecore_{S,A}_vs_E_<Filter>.log`,
`multicore_{S,A}_vs_E_<Filter>.log` (plus a `multicore_A_vs_E_CatmullRom_run2.log` repeat that
confirmed the CatmullRom multi-core regression wasn't a one-off).

## Result

**Mixed, and filter-dependent in a way `resize_nearest`'s own result (candidate 033) did not
predict.** Single-core: the gate is a decisive win vs. ungated `A` for all four filters
(`-11.6%` to `-4.1%`, 6/6 pairs each) and a wash vs. true serial `S` for all four (none clears
this workspace's `wins >= n-1`/`<=1` decisiveness bar) - consistent with candidate 033's
`Nearest` result, since single-core has no parallelism to lose by taking the serial fallback.
Multi-core is where this diverges from `Nearest`: `Triangle` still shows a small, decisive win vs.
`A` (`-3.2%`, 6/6), but `CatmullRom` (`+32.5%`/`+32.8%` across two runs, 1/6), `Gaussian`
(`+25.0%`, 0/6), and `Lanczos3` (`+41.2%`, 0/6) all show a **decisive regression** vs. `A` -
i.e. for these three filters, taking the serial fallback below `dst_height=32` throws away a real,
already-multi-core-parallel win pic-scale-safe's own rayon dispatch was providing, rather than
just avoiding dispatch overhead the way it did for `Nearest`. vs. `S` (true serial, what `main`
actually ships today, since the `rayon` forward this whole line depends on isn't merged), all four
filters land as a wash on both axes - so this candidate does not regress anything relative to
`main` as it ships today, only relative to the *ungated* `rayon`-forwarded alternative (`A`) that
candidates 002/019-030 measured as a real win for these three filters.

## Does it change behavior/output?

- **Bit-exact?** Yes. A temporary standalone example (`examples/_correctness_tmp_034.rs`, built
  and run, not committed - removed before the working tree was reverted) hashed
  (`DefaultHasher`, over `to_rgb8()`/`to_rgba8()` bytes and dimensions)
  `DynamicImage::resize_exact` output for all 4 filters across 4 scenarios each (16 hashes total):
  `RgbImage`/`RgbaImage` at `400x300->32x24` (below the gate's threshold, `dst_height=24`), and a
  `4032x3024` `RgbImage` resized to `64x48` and to `400x300` (both `dst_height >= 32`, exercising
  the gate's *other*, unchanged-parallel branch). Built twice: once as **S**
  (`--no-default-features --features default-formats`), once as **E** (`--features rayon`,
  vendored+gated `pic-scale-safe`). All 16 hashes matched exactly between the two builds
  (`raw/hashes_S.txt`, `raw/hashes_E.txt`).
- **Is this a breaking change?** No public API changes to `image` itself - same as candidate 033,
  the change lives entirely in the vendored `pic-scale-safe` copy plus the still-unmerged
  `pic-scale-safe/rayon` feature forward, neither of which is on `main` today.
- Project test suite: `cargo test --release --no-default-features --features default-formats` for
  S: **546 passed, 1 failed** (`raw/test_S_full.txt`). `cargo test --release --features rayon`
  for E: **550 passed, 1 failed** (`raw/test_E_full.txt`). Both share the identical single
  pre-existing unrelated failure (`resizing_with_catmul` in `tests/regression.rs`, recorded in
  every prior candidate in this line back through candidate 007) - no new failures. The 4-test
  difference in total count (546 vs. 550) is `default-formats` vs. the crate's true default
  feature set, not this candidate's change (candidate 033 recorded the same 550-vs-similar split).

## Why the result looks the way it does

`resize_nearest`'s per-row cost is negligible (~1us total for a 24-row output, candidate
031/033), so below any reasonable row-count threshold, `par_chunks_exact_mut`'s own
dispatch/join overhead (candidate 032's ~9-26us fixed cost) dominates completely regardless of
core count - gating it off is close to a pure win with no real parallelism given up. The other
four filters convolve with much wider kernels (CatmullRom/Gaussian/Lanczos3 all in the
100-200us range for the same `400x300->32x24` job, an order of magnitude more per-row work than
`Nearest`), so at `dst_height=24` there is already enough real work to productively spread across
multiple cores - candidates 023-030's whole capped-thread-count sweep line exists precisely
because these filters showed large, genuine multi-core wins from `rayon` dispatch even at this
same small size. Copying `resize_nearest`'s `32`-row threshold verbatim onto these filters'
dispatch functions asks the wrong question: `Nearest`'s regression was "is there enough total
work to be worth dispatching to any pool at all", while these filters' already-established
optimum (candidates 023-030) is "how many threads should absorb this work", a per-filter tuning
problem the flat gate structurally cannot express (it only ever chooses between 0 or "however
many `rayon::current_num_threads()` returns" threads, never something in between like the
already-shipped-in-this-workspace `N=4`/`N=6` capped pools from candidate 029).

## What's not done / open questions

- This makes `IDEAS.md`'s already-open row 48 (bisecting `resize_nearest`'s own threshold) more
  urgent than a refinement: a single flat `dst_height` threshold reused across filters is now
  shown to be actively harmful (not just suboptimal) for filters with substantial per-row cost.
  A real fix for these three filters would need either a per-filter threshold (high below which
  no filter benefits from any parallelism) or, better, a per-filter thread-count cap inside the
  vendored dispatch itself (mirroring candidate 029's already-validated caller-side `N=4`/`N=6`
  pools) rather than a binary serial-or-full-parallelism choice.
- `Triangle` was the only one of the four filters where the flat gate was a genuine (if small)
  multi-core win, not just a single-core one - not investigated why `Triangle`'s per-row cost
  sits close enough to `Nearest`'s regime for the same threshold to work, while
  `CatmullRom`/`Gaussian`/`Lanczos3` do not; candidate 018/019's own explanation (`Triangle`'s
  bilinear kernel is cheap/narrow relative to the other three's wider kernels) is consistent with
  this but wasn't re-confirmed via profiling here (`perf` unavailable in this environment, see
  `README.md`).
- Given the multi-core regression found for 3 of 4 filters, this candidate's own vendored change
  is *not* recommended as a drop-in replacement for candidates 021-030's caller-side
  thread-count-gate approach for those filters - if this were to be pursued upstream or shipped
  locally, it would need a per-filter threshold/cap, not the single constant reused here. Per
  SKILL.md's "vendoring a suspect dependency" guidance, no decision is made here about upstream
  PR vs. local `[patch.crates-io]` vs. drop - this is a measured, characterized result, not a
  recommendation.
- As in candidate 033, the vendored copy was not given its own separate `___OPTIMIZATIONS___/` -
  this fix is directly motivated by, and continues, the measurements already on file in *this*
  workspace (candidates 019-033).
