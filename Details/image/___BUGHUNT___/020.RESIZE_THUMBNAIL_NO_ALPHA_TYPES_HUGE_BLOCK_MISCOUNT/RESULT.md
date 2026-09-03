# RESIZE_THUMBNAIL_NO_ALPHA_TYPES_HUGE_BLOCK_MISCOUNT

Backlog item from `IDEAS.md` (top row, sourced from candidate 016): sweep `imageops::thumbnail`/
`DynamicImage::thumbnail_exact` for the uniform-color-preservation invariant across the 5 no-alpha
color types (`Luma8`/`Luma16`/`Rgb16`/`Luma32F`/`Rgb32F`) - `thumbnail`'s accumulator had only been
checked for the seed-value bug class (candidate 005) and, for `LumaA8` only, by candidate 010, never
empirically for these 5 types through `thumbnail` itself.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value/differential-invariant construction: temporary `#[test]` fn in `src/imageops/sample.rs`'s `mod tests`, a fixed grid of 6 source sizes x 6 destination sizes x the 5 no-alpha types through `imageops::thumbnail` (the shared implementation behind both `DynamicImage::thumbnail` and `thumbnail_exact`) | `cargo test --lib scratch_thumbnail_uniform_color_probe_no_alpha_types`, debug build, 3 repeat runs | 0/930 mismatches, 3/3 identical runs. |
| Randomized property sweep (escalation): 500 iterations of random source/destination size (1-40), random pixel type among the 5, and random value, through the real public `DynamicImage::thumbnail`/`thumbnail_exact` entry points, via a temporary `#[test]` fn using a seeded xorshift32 PRNG | `cargo test --lib scratch_thumbnail_uniform_color_probe_randomized`, debug build, 3 repeat runs | 0/147640 mismatches, 3/3 identical runs. |
| Boundary-value construction targeting the `pixel_count > 256 * 256` STEP_SIZE-splitting branch of `thumbnail_sample_block` specifically (untouched by the two probes above, whose max source size was 40x40 = 1600 pixels, far below the 65536-pixel split threshold): a 600x600 uniform-color source (not a multiple of 256 in either dimension) thumbnailed to 1x1, for all 5 no-alpha types | `cargo test --lib scratch_thumbnail_uniform_color_probe_huge_block_boundary`, debug build, 1 run | **FAILED**: all 5 types returned a wrong, inflated value (e.g. `Luma8` 137 -> 255, `Luma32F` 0.6789 -> 1.527525) - a live bug. |
| Repeat + generalize: 3 repeat runs of the above, plus 5 additional boundary shapes (`257x257`, `300x1000`, `1000x300`, `65537x2`, `2x65537`) for `Luma8`, plus isolation (reverting only the fix hunk and re-running to confirm the failure returns, then re-applying and confirming it passes again) | `cargo test --lib`, debug build | Deterministic: 3/3 runs failed identically before the fix, all 5 extra shapes failed the same way before the fix (255 instead of 91 in every case), all pass after the fix; isolation confirmed the fix hunk (not something else) is what makes the difference. |
| Standalone `repro/` (own `Cargo.toml`, real public API `image::DynamicImage::thumbnail`/`thumbnail_exact`) run via `just clone`/`reproduce-before`/`apply`/`reproduce-after` against a pristine local clone pinned at `base_commit`, 2 repeat runs each before and after | `cargo run --release` in `repro/`, via `just` | Before: reproduces 2/2 runs (exit code 1, wrong pixel values for all 5 types plus a 6th non-square/aspect-preserving `thumbnail()` check). After (patched clone): 2/2 runs pass (exit code 0). See `output.txt`. |

The first two approaches (fixed-grid, randomized) reached the soft cap of "escalating approaches"
with zero signal, matching every prior sibling candidate (009/010/012/016) for this exact suspect -
but per `SKILL.md`'s "any approach turns up even a partial signal... keep digging," the *scope* of
those two probes (max source size 40x40) never touched `thumbnail_sample_block`'s separate
"huge-block" code path (guarded by `pixel_count > 256 * 256`, i.e. only reachable above a
65536-pixel source-block size), which is exactly the code path that had never been checked for
*correctness* by any prior candidate (the existing `thumbnail_huge_block` test only checked for
absence of overflow/panic on power-of-256-friendly sizes 4096x4113 and 1024x1024, not the actual
returned value). Escalating specifically into that boundary is what surfaced the bug.

## Status

**Confirmed.**

Reproduces deterministically, every run: 3/3 in-tree debug-build runs (plus 5/5 additional boundary
shapes, each individually deterministic), and 2/2 release-build runs of the standalone `repro/`
against a pristine clone at `base_commit`. All 5 no-alpha types (`Luma8`, `Luma16`, `Rgb16`,
`Luma32F`, `Rgb32F`) plus a 6th check through the aspect-preserving `thumbnail()` entry point (not
just `thumbnail_exact()`) show the bug identically.

**Fully reproducible: Yes.** The repro calls the crate's real public API
(`image::DynamicImage::thumbnail_exact`/`thumbnail`) with an `ImageBuffer` constructed directly by
the repro binary - a uniform-color image *is* the genuine external input a real caller would supply
(e.g. any real photo being reduced to a thumbnail; uniformity is just what makes the wrong result
trivially detectable by equality, not an artificial construction of internal state). No second
participant/peer/wire input is involved, so `CONFIRMATION.md` section 11's stricter bar does not
apply here (see that section's own carve-out: "a malformed file handed to a parser's real public
entry point *is* the genuine external input... there's no 'other side' being impersonated there" -
the same reasoning applies to a plain image buffer handed to a resize function).

## Class

logic

## Root cause

`src/imageops/sample.rs`, `thumbnail_sample_block` (around line 695-727, specifically the divisor
computed at what was line 716 before the fix). When a sampled input block (`(right-left) *
(top-bottom)` pixels) exceeds `256 * 256 = 65536` pixels, the function splits it into `STEP_SIZE =
256`-sized sub-blocks via nested `step_by(STEP_SIZE)` loops, computes each sub-block's own average
recursively, and sums those sub-block averages into `sum`. It then divides `sum` by:

```rust
((right - left) as u64 / STEP_SIZE as u64) * ((top - bottom) as u64 / STEP_SIZE as u64)
```

This is a **floor**-division estimate of "how many `STEP_SIZE x STEP_SIZE` sub-blocks fit", but
`step_by(STEP_SIZE)` over a range `bottom..top` (or `left..right`) that is **not** an exact
multiple of `STEP_SIZE` still yields one extra iteration for the smaller trailing sub-block per
dimension (e.g. `step_by(256)` over `0..600` yields starts `0, 256, 512` - 3 iterations, not
`600/256 = 2`). So whenever either span isn't an exact multiple of 256 - true for essentially any
real-world image, since dimensions being exact multiples of 256 is the unusual case - the loop
performs `sum.add_pixel(k)` more times than the computed divisor accounts for, and the resulting
average is inflated. For a 600x600 uniform source (9 sub-blocks summed, divisor computed as `2*2=4`
instead of `3*3=9`), the result is inflated by exactly `9/4 = 2.25x` (confirmed directly: `Luma32F`
0.6789 -> 1.527525 = 0.6789 * 2.25 to float precision), saturating to the type's maximum for
integer types (`Luma8` 137 -> 255, not 308 as the raw 2.25x arithmetic would give, because of the
subsequent `S::clamp_from` step).

This affects **any** pixel type going through `thumbnail_sample_block` (the code has no
type-specific branching in the buggy divisor), not just the 5 no-alpha types this candidate's scope
named - it was simply this candidate's boundary-value escalation that happened to be the first to
construct an input large enough (and non-256-aligned) to reach this specific branch. The condition
is exactly "source image large enough that a single output pixel's input window exceeds 65536
source pixels" - straightforwardly reachable by thumbnailing any real photo (a source many
megapixels in size, being reduced to a small thumbnail, gives per-output-pixel input windows well
over 65536 pixels whenever the reduction ratio is large) down to a small enough thumbnail, with
source dimensions not exact multiples of 256 (i.e. almost always).

## Introduced by

61c7622d "Fix overflow in `thumbnail`" (2026-05-28)

## The fix

Track the actual number of `sum.add_pixel` calls made (`block_count`, incremented once per loop
iteration) instead of recomputing an estimate from a floor division of the block's dimensions by
`STEP_SIZE`. This makes the divisor exactly match the number of sub-block averages actually summed,
regardless of whether either dimension is a multiple of 256.

```rust
let mut block_count: u64 = 0;
for y in (bottom..top).step_by(STEP_SIZE as usize) {
    for x in (left..right).step_by(STEP_SIZE as usize) {
        let k = thumbnail_sample_block(image, x, x.saturating_add(STEP_SIZE).min(right), y, y.saturating_add(STEP_SIZE).min(top));
        sum.add_pixel(k);
        block_count += 1;
    }
}
block_count
```

Verified:
- The exact triggering case (600x600 uniform source, all 5 no-alpha types, both `thumbnail_exact`
  and `thumbnail`) now returns the correct, unchanged value - confirmed 3/3 in-tree runs and 2/2
  standalone `repro/` runs against a freshly patched pristine clone.
- Per `CONFIRMATION.md` section 7 ("vary the input within the same bug class"): 5 additional
  boundary shapes (`257x257`, `300x1000`, `1000x300`, `65537x2`, `2x65537`) - none of them equal to
  the original 600x600 trigger, all non-multiples-of-256 - also now pass; before the fix, all 5
  failed identically to the original case.
- Isolation per `SKILL.md` pitfall #6 ("confounded fix"): reverting *only* the fix hunk (keeping
  everything else, including the new test) reproduces the failure again on the exact same boundary
  test; re-applying the fix hunk alone makes it pass again - confirms the specific 6-line change is
  what fixes it, not some other incidental change.
- The pre-existing `thumbnail_huge_block` overflow-only regression test (4096x4113 and 1024x1024)
  still passes after the fix.

- Project test suite: `cargo test --no-fail-fast` (default features, `dav1d`/`avif-native` system
  library unavailable in this environment so `--all-features` could not be run either before or
  after - unrelated to this change). **Before fix (pristine tree)**: 622 passed, 1 failed, 4
  ignored, across all binaries (331 lib + 0 avif_termination + 1 bad_images + 7 conversions + 6
  limits + 2 limits_anim + 6 metadata + 189 reference_images + 8 regression [1 failing] + 1
  save_jpeg + 2 save_pnm + 14 truncate_images + 55 doctests). **After fix**: identical counts, 622
  passed, 1 failed, 4 ignored - no new failures, no fixed-by-accident failures. The one failing
  test both before and after, `regression::resizing_with_catmul`, is a pre-existing, unrelated
  failure (confirmed via `git stash` to fail identically on the pristine tree with no changes at
  all) - not caused or affected by this candidate's fix.

## What's not done / open questions

- **Not checked**: whether the same bug is independently reachable through the alpha-bearing
  pixel types (`LumaA8`, `Rgba8`, `Rgba16`, `Rgba32F`) - the buggy divisor computation has no
  type-specific logic, so by the same code-reading argument used elsewhere in this session's sibling
  candidates it's expected to reproduce identically for those types too, but this candidate stayed
  scoped to the 5 no-alpha types named in its `IDEAS.md` backlog entry and did not empirically
  re-verify the alpha-bearing types against this specific boundary. Added to `IDEAS.md` as a new,
  narrowly-scoped follow-up.
- **Not checked**: the exact threshold/scaling behavior for even larger images (multi-megapixel,
  i.e. many STEP_SIZE sub-blocks per dimension) - only single-digit-sub-block-count shapes were
  tried. The bug's mechanism (a fixed per-dimension undercount of at most 1 sub-block whenever a
  span isn't a multiple of 256) means the relative error shrinks as the true sub-block count grows,
  but this wasn't empirically measured for e.g. a 10000x10000 source.
- Not otherwise re-litigated: the fixed-grid and randomized sweeps in this candidate re-confirm (a
  4th and 5th time, after candidates 009/010/012/016) that the *non-huge-block* averaging path
  (`ThumbnailSum`'s zero-seeding, checked by candidate 005) remains correct for these 5 types - no
  new information there, included for completeness of this candidate's own record.
