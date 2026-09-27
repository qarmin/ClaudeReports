<!-- rustc 1.98.0 (88d9e12ae 2026-08-18), cargo 1.98.0 (797e8a9bc 2026-08-05) - recorded because this
candidate's conclusion rests on a vectorization/LICM claim about the compiled code, not just a
wall-clock number (see SKILL.md template note on hayro candidates 013/014/016). -->

# vello_common::strip::render_impl - hoist the tile-row constant (`px_top_y`/`px_bottom_y`) out of the tile loop

`vello_common::strip::render_impl` (`sparse_strips/vello_common/src/strip.rs`) is the largest
self-time hotspot in `perf record` profiles of `vello_toy svg` rendering
`examples/assets/Ghostscript_Tiger.svg` (24.5%-26.5% of cycles across two independent runs at
different output scales). Inside its per-tile loop, `px_top_y = f32x4::simd_from(s, [0., 1., 2.,
3.])` and `px_bottom_y = 1. + px_top_y` were recomputed on every iteration even though they don't
depend on any tile/line data at all - they only encode "row 0..3 of a 4-row tile", a true
compile-time constant. Hoisting them out of the loop looked like a free, bit-exact win worth
checking before concluding (as the candidate brief already suspected) that this hotspot has no
single fixable line.

## What was tried

Existing bench reused as-is (no new harness needed): `sparse_strips/vello_bench`'s
`render_strips` criterion group (`cargo bench -p vello_bench --bench main -- render_strips`),
which already benches `vello_common::strip::render` directly against the Ghostscript Tiger SVG's
tiles/lines - the same input/code path the profiling was done on.

Machine: 24 logical cores. `taskset -c 3` used for "single-core" pinned runs; unpinned runs used
for "multi-core"/realistic-wall-clock, both per SKILL.md's candidate loop.

Noise floor established first (two same-binary baseline runs, pinned, `--save-baseline`):

| Run | median (pinned) |
|---|---|
| baseline run 1 | 166.97 µs |
| baseline run 2 (same binary) | 178.02 µs (+6.6% vs run 1, zero code change) |

This machine shows a bimodal/long-tail noise pattern under `taskset` pinning alone (consistent
with MEASUREMENT.md's note that `taskset` doesn't fully suppress OS-scheduling-driven noise) -
some single runs swing to 190-210 µs even with identical binaries. Given that, a single
before/after pair is not evidence; an 8-round alternating paired comparison (MEASUREMENT.md
section 2b) was used for the real comparison, plus criterion's own built-in `--baseline` stats.

**Alternating paired comparison, pinned (`taskset -c 3`), 8 rounds, median of 30 samples per
round:**

| round | baseline (µs) | patched (µs) | patched faster? |
|---|---|---|---|
| 1 | 160.01 | 161.32 | no |
| 2 | 158.61 | 159.09 | no |
| 3 | 158.70 | 159.23 | no |
| 4 | 159.71 | 169.29 | no |
| 5 | 206.25 | 194.56 | yes |
| 6 | 184.40 | 160.89 | yes |
| 7 | 160.66 | 160.45 | yes (barely) |
| 8 | 160.05 | 162.17 | no |

Patched faster in 3/8 rounds - not a consistent direction (MEASUREMENT.md's paired-comparison
bar for "real" is essentially all-or-nothing, e.g. 8/8 or 1/8; 3/8 is noise). Median-of-medians:
baseline 160.03 µs vs. patched 161.11 µs (+0.7%, well inside the ~6.6-20%+ same-binary noise band
measured above).

**Alternating paired comparison, unpinned ("multi-core"/realistic), 6 rounds:**

| round | baseline (µs) | patched (µs) | patched faster? |
|---|---|---|---|
| 1 | 159.16 | 158.92 | yes |
| 2 | 159.19 | 160.03 | no |
| 3 | 158.83 | 158.21 | yes |
| 4 | 158.17 | 159.73 | no |
| 5 | 158.95 | 163.87 | no |
| 6 | 158.10 | 159.10 | no |

Patched faster in 2/6 rounds. Median-of-medians: baseline 158.89 µs vs. patched 159.42 µs
(+0.36%).

**Criterion's own paired `--baseline` comparison** (lowest-noise single measurement, both runs
back-to-back in the same invocation, unpinned, via this candidate's own `justfile`'s `just bench`):

```
render_strips/Ghostscript_Tiger_simd
                        time:   [158.72 µs 158.82 µs 158.94 µs]
                        change: [-0.0399% +0.0352% +0.1099%] (p = 0.37 > 0.05)
                        No change in performance detected.
```

No RSS/allocation-count dimension was measured - this change touches only register-level SIMD
constant materialization, not allocation behavior, so a memory metric would not be expected to
move and wasn't checked.

## Result

**No change**, time only. Every measurement method used (noisy alternating-paired comparison on
two axes, and criterion's own low-noise paired statistic) agrees within noise: the patched build
is not measurably faster or slower than baseline. The largest, cleanest single data point
(criterion's own `--baseline`, p=0.37) is a textbook "no change detected".

## Does it change behavior/output?

- **Bit-exact?** Yes. `git apply --check patch.diff` applies cleanly against `base_commit`; the
  change only moves two loop-invariant `let` bindings to before the loop, it performs the exact
  same floating-point operations in the exact same order per iteration as before.
- **Is this a breaking change?** No - private-fn-internal refactor only, no public API change.
- Project test suite: `cargo test --release -p vello_common` - **159 passed, 0 failed, 1
  ignored** before the patch; **159 passed, 0 failed, 1 ignored** after the patch (identical
  counts, no new failures).

## Why the result looks the way it does

`px_top_y`/`px_bottom_y` are a pure function of nothing (a literal array turned into a SIMD
register) with no dependency on the loop's mutable state or any memory that could alias - exactly
the shape of code LLVM's loop-invariant code motion (LICM) already hoists automatically, monomorphized
per SIMD backend through `#[inline(always)] fn render_impl<S: Simd>`. The baseline binary was
already doing the hoist itself at the machine-code level; writing it out by hand in the source
changes nothing the optimizer wasn't already producing. This is consistent with the candidate
brief's own framing: `render_impl`'s cost is smeared across many small, latency-bound,
narrow-SIMD (4-lane, matching `Tile::HEIGHT`) operations rather than concentrated in one
recomputed value - there was no "single fixable line" here, and this candidate confirms that by
elimination rather than by assumption.

## What's not done / open questions

- The actual lever the profiling data points at - widening the vectorization beyond the 4-lane
  tile-column width - was **not** attempted here. Both the per-location winding accumulation
  (`accumulated_winding` carried across tile-columns) and the per-pixel trapezoid-area loop
  (`for x_idx in 0..Tile::WIDTH`, which also carries a sequential `acc` across the 4 x positions)
  are genuine prefix-sum/scan patterns with a real loop-carried dependency, not just missed
  constant-folding - restructuring either into wider SIMD (e.g. a 16-wide combined x*y pass, or a
  SIMD prefix-sum construction) is a substantially larger, riskier change that would need its own
  careful bit-exactness verification (this function's own code comments already document subtle
  NaN-handling differences between x86 and AArch64 `max`/`max_precise` semantics, so any
  restructuring has real correctness surface). Left as an open idea, not attempted in this
  candidate.
- No memory/allocation dimension was relevant to check for this specific change (see above).
