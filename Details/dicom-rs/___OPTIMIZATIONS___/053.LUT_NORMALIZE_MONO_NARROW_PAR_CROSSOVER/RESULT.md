# Root-cause candidate 006's `mono_image_with_narrow_par` single-core regression: no crossover found, a genuine per-element codegen gap instead

Candidate 006's own RESULT.md ("Related finding from an independent re-derivation" section)
reported the 16-bit `mono_image_with_narrow_par` call site (`pixeldata/src/lib.rs`, the
`Vec<u16> = pixel_values.collect()` line inside `to_dynamic_image`'s monochrome/16-bit branch) as
a single-core win below ~262,144px but a regression somewhere between ~300K and ~1.3M px - never
pinned down more precisely, and never re-verified against the workspace's own real `SUMMARY.md`
(per `projects/dicom-rs.md`'s standing warning that narrative-only numbers from a differently-run
sub-derivation have drifted before). This candidate re-measures that exact site directly, with a
proper `taskset`-pinned, bit-exact-checked, multi-round harness sweep, and adds a callgrind
comparison isolating just the `collect()` step (no `ImageBuffer::from_raw`) to test whether the
regression is a per-call dispatch-overhead effect (would predict a crossover, shrinking as a
fraction of total cost as `n` grows) or a per-element codegen effect (would predict a roughly
constant relative gap, no crossover) between `Vec::from_iter` and rayon's parallel `collect()`.

## What was tried

**1) Wall-clock re-sweep of the real site**, single-core (`taskset -c 0`, so rayon's own
`sched_getaffinity`-based pool sizing collapses to 1 worker - the same mechanism `IDEAS.md`/
candidate 006 already documented). Two harness binaries built from the exact same unmodified
source (`___OPTIMIZATIONS___/harness/bench_pipeline`, `dicom-pixeldata` at
`df2b9e4236298efa3fa37a1f360fc4bf284d586a`): `bench-rayon` (default features, reaches
`mono_image_with_narrow_par`/rayon `collect()`) and `bench-norayon` (`--no-default-features`,
confirmed genuinely rayon-free via `nm | grep -c rayon_core` = 0, reaches `mono_image_with_narrow`/
`Vec::from_iter`). Fixture: `mono_below.dcm` (real `pydicom/MR_small.dcm`, 64x64=4,096px, 16-bit,
has `WindowCenter`/`WindowWidth` so it takes the O(1) `Lut::new_rescale_and_window` branch, not the
full-scan `new_rescale_and_normalize` path - ruled out as a confound explicitly, see below), tiled
vertically via the harness's own `gen-mono` mode to 9 sizes spanning 262,144px to 1,310,720px (a
separate single check at 32,768px, well below candidate 006's own reported win threshold, was also
run for a below-the-original-range data point - see the "+31.4%" figure below the main table).
Measured via the existing `colorspace` mode (`decode_pixel_data()` + `to_dynamic_image(0)`,
`fnv1a` checksum on output bytes) - same mode candidate 006's own sub-derivation used.

Pixel count = `4,096 * row-tile-multiplier` (base fixture is 64x64 = 4,096px; `gen-mono` tiles rows
only, so this is exact, not approximate):

| pixel count | bench-norayon min (round 1 / round 2) | bench-rayon (1-thread) min (round 1 / round 2) | rayon vs norayon |
|---|---|---|---|
| 262,144 | 555.7 / 582.9us | 669.6 / 699.4us | +20.5% / +20.0% |
| 393,216 | 735.9us (r1 only) | 865.4us (r1 only) | +17.6% |
| 524,288 | 926.6 / 981.8us | 1065.3 / 1116.6us | +15.0% / +13.7% |
| 655,360 | 1146.8us (r1 only) | 1263.0us (r1 only) | +10.1% |
| 786,432 | 1296.0 / 1347.8us | 1546.3 / 1576.0us | +19.3% / +16.9% |
| 917,504 | 1481.0us (r1 only) | 1655.9us (r1 only) | +11.8% |
| 1,048,576 | 1653.9 / 1786.3us | 1849.7 / 2002.3us | +11.8% / +12.1% |
| 1,179,648 | 1843.1us (r1 only) | 2060.2us (r1 only) | +11.8% |
| 1,310,720 | 2288.7 / 2131.7 / 2532.9us | 2239.2 / 2291.4 / 2797.7us | -2.2% / +7.5% / +10.5% |

All checksums matched exactly between `bench-rayon` and `bench-norayon` at every size (bit-exact).
Round 1/2 above used alternating invocation order (round 1: rayon-then-norayon; round 2:
norayon-then-rayon) to cancel systematic drift; a third, fully independent round (a fresh `git
clone` + fresh build via this candidate's own `justfile`, `just sweep`, norayon-then-rayon order)
re-confirmed the same shape end to end - the direction (rayon slower) is consistent across all
three rounds at 8 of the 9 sizes tested, from 262,144px through 1,179,648px (a stable +10% to +20%
overhead), and 2 of the 3 rounds at 1,310,720px (+7.5%, +10.5% - only round 1's -2.2% is an
outlier). Given the `justfile`'s own independent run also lands solidly on the "regressed" side at
1,310,720px, the overall picture is: **no win region was found anywhere in the
262,144px-1,310,720px range - contradicting candidate 006's own secondary "win below ~262,144px"
finding** - the relative overhead does shrink somewhat as `n` grows (from ~+20% at 262,144px
towards ~+8-11% at 1,310,720px across the 3 rounds) but never crosses into an actual win, the
opposite direction from candidate 006's own secondary finding, which placed the *regression* above
this point and a *win* only below 262,144px.

**2) Callgrind (`Ir` count) comparison isolating just the `collect()` step.** A standalone
`collect_bench` crate (`bench/collect_bench/`, depends only on `rayon`, no `dicom-*` crates)
reproduces exactly the two collect patterns without `ImageBuffer::from_raw`: `source.iter().copied()
.map(|i| lut[i as usize]).collect::<Vec<u16>>()` (`seq`) vs `source.par_iter().copied()
.map(|i| lut[i as usize]).collect::<Vec<u16>>()` (`par`, `taskset -c 0` forces the same 1-worker
pool). `lut` is a 65,536-entry `Vec<u16>` (matches `Lut<u16>`'s table size), `source` is a
`Vec<u16>` of the requested length - the same shapes as the real site's `samples`/`lut.get()`.

| n (elements) | seq `Ir` | par `Ir` | par/seq |
|---|---|---|---|
| 32,768 | 806,992 | 932,987 (933,605 on repeat run) | 1.156x |
| 262,144 | 3,845,987 | 4,660,386 | 1.212x |
| 1,310,720 | 17,739,660 | 21,700,480 (21,700,288 on repeat run) | 1.223x |

`seq`'s `Ir` count was bit-for-bit identical across two repeat runs (deterministic, no threads);
`par`'s varied by <0.1% between repeats (thread-startup noise), negligible next to the differences
being measured. Checksums matched between `seq`/`par` at every size (bit-exact).

**Marginal (per-element) `Ir` cost**, computed from the differences between consecutive `n` (this
cancels the fixed one-time cost of building the 65,536-entry LUT and process startup, which is
identical for both variants):

- 32,768 -> 262,144 (delta 229,376 elements): seq +3,038,995 Ir (13.25 Ir/elem), par +3,727,399 Ir
  (16.25 Ir/elem).
- 262,144 -> 1,310,720 (delta 1,048,576 elements): seq +13,893,673 Ir (13.25 Ir/elem), par
  +17,040,094 Ir (16.25 Ir/elem, using the first-run par figures; using the repeat-run figures
  changes this by <0.01 Ir/elem).

**The marginal per-element `Ir` cost is identical across both size intervals for each variant
(13.25 Ir/elem for `seq`, 16.25 Ir/elem for `par`) - i.e. a stable ~22.6% higher per-element
instruction count for the rayon `collect()` path, not a cost that shrinks as a fraction of the
total as `n` grows.** The fixed (n=0-extrapolated) intercepts are close (seq ~372,816 Ir, par
~400,507 Ir, a ~7.4% one-time difference dominated by rayon's own one-time pool-sizing/dispatch
setup) - and crucially, `par`'s intercept is *also* higher than `seq`'s, not lower, so there is no
point in this model, at any `n`, where `par` is cheaper than `seq`.

## Result

**No crossover into a win region was found in the 32,768px-1,310,720px range - `mono_image_with_narrow_par`
(rayon, 1-thread) is consistently slower than the sequential `mono_image_with_narrow` at every size
tested, across 3 independent measurement rounds (2 same-binary reordered rounds plus a 3rd fully
independent fresh-clone-and-build run via this candidate's own `justfile`), by roughly +10% to
+21% across 262,144px-1,179,648px (a separate check at 32,768px, below this range, showed the
largest gap, +31.4%), narrowing towards (but never crossing into) parity at the largest tested
size, 1,310,720px (+10.5% on the independent `justfile` run, +7.5% and a single outlier -2.2% on
the other two rounds). This direction - overhead shrinking as `n` grows rather than growing - is
consistent with a real per-element codegen gap plus a separate, roughly fixed dispatch-overhead
surcharge whose relative share shrinks as `n` grows, not with the "win below ~262,144px, regression
above" shape candidate 006's own secondary finding described.** The callgrind evidence directly
confirms and quantifies the mechanism: rayon's parallel `.collect()` costs a reproducible **~22.6% more instructions per
element** than `Vec::from_iter` for this exact `u16`-LUT-map-then-collect shape, even with a
1-worker pool that never actually splits work across threads - this is a pure codegen/dispatch-path
difference in `rayon::iter::Map::collect()`'s consumer machinery versus the plain iterator adapter
chain's `SpecFromIter`, not anything related to real multi-threading contention. This settles
candidate 006's own open question in favor of the "codegen difference" hypothesis it named as
"plausible, not confirmed" - confirmed here via a deterministic, reproducible `Ir`-count
measurement, not just symbol-size inspection (candidate 006's earlier `nm -S` follow-up).

For completeness, an **unpinned multi-core** re-run of three fixture sizes (real thread-pool
parallelism, 24 logical cores) shows the expected opposite shape: rayon is slower at 262,144px
(+41% min, dispatch overhead not yet worth it), turns into a real win by 1,310,720px (-24% min)
and 8,192,000px (-31% min) - i.e. the *multi-core* win/loss crossover (a genuinely different
question from this candidate's single-core one) sits somewhere between ~262K and ~1.3M px, roughly
where a prior, unconfirmed session note placed it - but that is a real-parallelism threshold
question, not the single-core codegen question this candidate was scoped to answer.

## Does it change behavior/output?

**N/A - this is a measurement/investigation-only candidate, no patch applied to `dicom-pixeldata`
or any other tested-project source.** No `patch.diff` is included, correctly - nothing in
`pixeldata/src/lib.rs` was changed for this candidate; the only new artifacts are the wall-clock
sweep (reusing the existing, unmodified `harness/bench_pipeline` binary and its existing `gen-mono`/
`colorspace` modes) and the standalone `collect_bench` crate (`bench/collect_bench/`, has no
dependency on `dicom-*` at all). Bit-exact confirmed throughout (see tables above). Project test
suite was not re-run since no source was changed (consistent with SKILL.md's "no exceptions other
than a pure investigation with no patch").

## Why the result looks the way it does

`rayon::iter::Map<I, F>::collect()` for `Vec<u16>` goes through rayon's own `Collect`/
`collect_into_vec` machinery (parallel producer/consumer split-and-merge scaffolding, plus a
`Vec::with_capacity` + parallel write-into-uninitialized-slice pattern), which - even when the
thread pool has exactly one worker and never actually splits into multiple physical threads -
still executes real, extra per-element instructions that plain `Iterator::collect()`'s
`SpecFromIter`/`SpecExtend` fast path for a `TrustedLen` source does not need: bounds/capacity
bookkeeping for the parallel merge structure, and less opportunity for LLVM to fuse the whole
`map`+`collect` chain into a single tight loop the way it can for a sequential `Iterator` (this is
the same "runtime `if` between `#[cfg]`-selected code defeats the compiler's ability to prove
branch-exclusivity" family of effect candidate 006 hypothesized, but demonstrated here directly at
the isolated `collect()` level via a from-scratch minimal repro, not inferred from `nm -S` symbol
size on the full pipeline binary). This is a real, structural per-element cost of going through
rayon's `ParallelIterator::collect()` even when parallelism buys nothing (1 thread), not a
transient JIT/warmup/dispatch-only artifact - it is present identically at every size tested and
its *marginal* rate (Ir/element beyond the fixed one-time setup cost) does not change with size.

## What's not done / open questions

- The exact instructions responsible for the ~22.6% Ir/element gap were not identified via
  `objdump -d` diffing (bounded time, per SKILL.md's guidance for regressions) - the mechanism
  above (rayon's parallel-collect merge bookkeeping vs. `Vec::from_iter`'s `SpecFromIter` fast
  path) is inferred from the shape of the numbers (constant marginal Ir/element, higher fixed
  intercept) and general knowledge of how `rayon::collect()` differs internally from
  `std::iter::Iterator::collect()`, not confirmed instruction-by-instruction.
- Only `u16` LUT values (`Lut<u16>`, matching `mono_image_with_narrow_par`'s exact output type)
  were tested in `collect_bench`. The sibling 8-bit sites (`mono_image_with_extend`/`_par`,
  `Vec<u8>`) were not re-measured with this isolated-collect technique - candidate 006's own notes
  reported those 5/6 "other" sites as clean wins, but per this candidate's finding for the 16-bit
  site, that claim is equally unverified against a real, current `SUMMARY.md` row and would be
  worth the same treatment in a future candidate.
- This candidate did not attempt a fix (e.g. a `rayon_worth_it()`-style guard for this specific
  site, restoring `mono_image_with_narrow`'s `Vec::from_iter` path whenever the thread pool has
  only 1 worker or `n` is below some threshold) - candidate 006 already tried the general
  guard-everywhere pattern and found it net-negative for a *different* site
  (`convert_pixel_slice`'s 16-bit LUT branch, regression *below* threshold, attributed to the same
  runtime-`if`-defeats-codegen family of effect). Whether a guard scoped to just this one
  `narrow_par` site nets positive (avoiding the ~11-31% single-core loss shown here without
  reintroducing a below-threshold codegen penalty of its own) is an open, testable follow-up, not
  attempted in this investigation-only candidate.
- The multi-core win/loss crossover (~262K-1.3M px per the informal 3-point check above) was not
  narrowed with the same rigor as the single-core question this candidate was scoped to answer -
  left as a distinct follow-up in `IDEAS.md`.
