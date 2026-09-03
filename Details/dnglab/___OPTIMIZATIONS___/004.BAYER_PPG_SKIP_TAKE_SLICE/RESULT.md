# Bayer PPG: replace `.skip().take()` iterator chains with direct slicing

`interpolate_green`, `interpolate_rb_at_green`, and `interpolate_rb_at_non_green`
(`rawler/src/imgop/sensor/bayer/ppg.rs:111-249`) all iterate the interior of the image (rows
`3..h-3`, columns `3..w-3`) via `par_chunks_exact_mut(w).enumerate().skip(3).take(h-6)` for rows
and `.iter_mut().enumerate().skip(3).take(w-6)` for columns inside each row. A callgrind profile
of a Canon EOS R5 CRAW sample (full decode + develop) attributed ~17% of total instructions to
these three functions together, with `.take()` itself showing up as its own attributed cost
line - suggesting the `Skip`/`Take` adapter chain wrapping the already `multiversion`-tagged
closures might be blocking auto-vectorization of the inner column loop. Tried replacing both the
row-level and column-level `.skip().take()` chains with direct slice indexing
(`pixels_mut()[3*w..(h-3)*w]` for rows, `buf[3..w-3]` for columns) so the compiler sees a plain
bounded slice instead of an iterator-adapter chain.

## What was tried

One variant: for each of the three functions, replace
`img.pixels_mut().par_chunks_exact_mut(w).enumerate().skip(3).take(h - 6).for_each(|(row, buf)| { for (col, pixel) in buf.iter_mut().enumerate().skip(3).take(w - 6) { ... } })`
with
`img.pixels_mut()[3 * w..(h - 3) * w].par_chunks_exact_mut(w).enumerate().for_each(|(row_idx, buf)| { let row = row_idx + 3; for (col_idx, pixel) in buf[3..w - 3].iter_mut().enumerate() { let col = col_idx + 3; ... } })`.
Same iteration order and same set of `(row, col)` pairs visited (verified bit-exact, see below) -
purely a change in how the bounds are expressed to the compiler.

Measured with `rawler/benches/demosaic.rs`'s existing `full_image_canon_r5` case (Canon EOS R5
CRAW, criterion, `--features rawdb`), plus a small scratch example (`rawler/examples/ppg_dump.rs`,
added and removed for this candidate only - not part of the patch) that hashes (md5) the full
`Intermediate::ThreeColor` output buffer to check bit-exactness, run alongside the criterion
benches for single-core control (`RAYON_NUM_THREADS=1 taskset -c 0`).

| Variant | single-core p50 (RAYON_NUM_THREADS=1, taskset -c 0) | multi-core p50 (unpinned, 24 cores) | notes |
|---|---|---|---|
| baseline (criterion, run 1) | - | 289.76 ms [284.92, 294.89] | `cargo bench --bench demosaic -- full_image_canon_r5` |
| baseline (criterion, run 2) | - | 288.55 ms [284.61, 294.25] | repeat run, tight agreement with run 1 |
| baseline (criterion, single-core, run 1) | 1.4662 s [1.4557, 1.4784] | - | |
| baseline (criterion, single-core, run 2) | 1.4527 s [1.4495, 1.4563] | - | repeat |
| patched (criterion, run 1) | - | 269.98 ms [266.81, 272.89] | |
| patched (criterion, run 2, vs base2) | - | 296.03 ms [275.98, 330.57] | criterion: change +1.87% [-4.50%, +10.78%], p=0.72 - "No change in performance detected"; 3 outliers (2 high severe) |
| patched (criterion, single-core, run 1) | 1.4493 s [1.4386, 1.4623] | - | |
| patched (criterion, single-core, run 2, vs base2_sc) | 1.4341 s [1.4313, 1.4371] | - | criterion: change -1.28% [-1.59%, -0.97%], p<0.10 - "Change within noise threshold" |

## Result

Multi-core: one patched run (269.98 ms) landed below both baseline runs (~289 ms), but the second
patched run (296.03 ms, wider CI, 3 outliers including 2 severe) landed back inside the baseline
range, and criterion's own paired comparison against the same-session `base2` baseline reports "No
change in performance detected" (p=0.72). Single-core: criterion reports a statistically
significant but tiny -1.28% change on the second run, explicitly classified by criterion itself as
"Change within noise threshold". Across both baseline and patched runs, single-core p50 stays in a
tight 1.43-1.47 s band and multi-core p50 stays in a 267-296 ms band - the two measurement
distributions overlap. No consistent, reproducible speedup in either mode.

## Does it change behavior/output?

- **Bit-exact?** Yes. Full `Intermediate::ThreeColor` output buffer for the Canon EOS R5 CRAW
  sample (13056x8784 minus non-multiple-of-chunk remainder handling, 559,851,264 bytes, 3 f32
  channels) hashed identically (md5 `c27e167f1732eab0fd277971a9cc3987`) before and after the
  patch, confirmed deterministic across 2 runs each side before comparing.
- **Is this a breaking change?** No public API change - all three touched functions are private
  (`fn`, not `pub fn`) to `rawler::imgop::sensor::bayer::ppg`.
- Project test suite: `cargo test -p rawler --lib` - 66 passed, 0 failed, both before and after.

## Why the result looks the way it does

The row-level `.skip(3).take(h-6)` wraps a `rayon` parallel iterator (`par_chunks_exact_mut`) -
row-to-row parallelism, not something LLVM auto-vectorizes across in the first place, so removing
that layer's `Skip`/`Take` wrapper was never going to change codegen for the row loop itself. The
column-level `.skip(3).take(w-6)` is the one actually inside each per-row closure body, but the
per-pixel work in all three functions is a scalar, branch-heavy computation (an `if
shifted.color_at(...) != CFA_COLOR_G` gate per pixel, followed by several data-dependent unsafe
loads from non-adjacent offsets - `row-1`, `row-2`, `col+1`, `col+2`, diagonal neighbors, etc. -
feeding into `abs()`/comparison chains) rather than a tight, branch-free numeric loop over
contiguous memory. That shape is a poor auto-vectorization candidate regardless of how the loop
bounds are expressed (iterator chain vs. plain slice range) - `multiversion` on the outer `fn`
means a target-specific version gets selected at runtime, but it can't force the compiler to
vectorize a branchy, gather-style access pattern. The candidate's premise (that the `.skip/.take`
*chain itself* was the blocker) doesn't hold up under measurement: with the adapter removed, both
single-core and multi-core numbers stay statistically indistinguishable from baseline. The ~17%
callgrind instruction share for these three functions combined is very plausibly real cost, but it
looks structural to the branchy per-pixel access pattern rather than caused by the loop-bound
expression form.

## What's not done / open questions

- No disassembly comparison was done to directly confirm whether LLVM emitted different code for
  the inner loop before vs. after (the measured wall-clock numbers being indistinguishable made
  this a low-priority follow-up, but it would be the definitive way to confirm/refute "same
  codegen" vs. "different codegen, same wall-clock cost anyway").
- Not tried: restructuring the per-pixel branchy logic itself (e.g. splitting into two passes -
  one over G-channel pixels, one over R/B-channel pixels, so each pass has no per-pixel branch and
  a denser, more uniform access pattern) - a heavier, more invasive change than this candidate's
  scope, but the more likely place a real win is hiding in this code, per the "why" section above.
- Multi-core numbers were collected on a machine with background load (`uptime` load average
  ~6-7 out of 24 cores) rather than a fully idle one; noted as context for the noisier of the two
  multi-core patched runs (3 outliers, 2 severe).
