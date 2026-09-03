# 33 - how much of the dirty-region walk is *dependency registration*?

Candidate 32 measured what the whole walk costs (11.07 ms of a 19.28 ms frame on `scroll` 100 000)
and established that it cannot be removed, because it is also what registers the geometry
dependencies that schedule the next frame for culled items.

That leaves a design question for the subtree-bounds work: does a fix have to reduce the number of
**visits**, or only the number of **dependency registrations**? Those call for different designs, so
the split is worth measuring before anyone builds either.

## The ablation

One line, and it removes *only* the registration - every read, every bounding-box calculation, every
cache lookup and every transform stays exactly where it was:

```rust
// internal/core/partial_renderer.rs, inside compute_dirty_regions' visitor
let new_geom = crate::properties::evaluate_no_tracking(|| {
    CachedItemBoundingBoxAndTransform::new::<T>(&item_rc, &self.window_adapter)
});
```

`ItemRc::geometry()` reads four properties (x, y, width, height), so on a 100 000-item tree this is
400 000 dependency registrations per frame that the ablation removes and nothing else.

## Result

Interleaved, two passes of `RUNS=5` alternating between the two binaries, medians:

| config | tracked (baseline) | untracked | |
|---|---|---|---|
| `scroll` 100k `bare`, scrolling | 19.05 ms | **16.16 ms** | **-15%** |
| `scroll` 100k `bare`, static | 17.70 ms | **14.83 ms** | **-16%** |
| first frame | 35.8 ms | **26.7 ms** | **-25%** |
| resident memory | 138.4 MB | **124.6 MB** | **-13.8 MB** |

Both passes agreed to within 0.03 ms on the scrolling row (19.12/18.99 and 16.15/16.16), so the
separation is far outside the noise.

## What it means

**Registration is about a quarter of the walk, not the bulk of it.** The walk costs 11.07 ms
(candidate 32); registration accounts for 2.9 ms of that, so **~26%**. The other ~74% is the visit
itself: the tree walk, `geometry()`'s property reads, the bounding-box arithmetic, the cache lookup
and the transform composition.

So a fix that only made registration cheaper - a coarser dependency, one node per subtree instead of
four per item - would return roughly a quarter of the traversal. Reaching the rest requires not
visiting the items at all. **Subtree bounds have to prune visits; making the dependencies coarser is
a bonus, not the mechanism.**

The memory figure is a separate finding worth keeping: **~14 MB of the 138 MB resident set is
dependency nodes created by this one traversal**, about 140 bytes per item, and 9 ms of the 36 ms
first frame is spent building them.

## This is attribution, not a fix

The walk's dependency registration is what schedules the next frame for *culled* items
(candidate 32). Removing it cannot be shipped, whatever it measures.

### A pixel claim retracted

This write-up originally reported that the ablation left a stale 26x10 pixel patch at the bottom of
the window - a label whose geometry changed and was never repainted - and presented it as candidate
32's predicted failure mode caught in the act. **That was wrong, and it was my own error.**

The `perf_test` status label prints a measured duration, so a ~15x10 px block at the bottom of the
window differs between two runs of the **same binary**. Comparing one snapshot against one snapshot
cannot tell that apart from a rendering change, and I read it as one. Re-checked with a same-binary
control (`tools/pixelcheck.sh`, written because of this):

```
### 33 ablation: --mode scroll --count 5000 --row-style layout
noise floor (same binary, 4 runs each): 128 px
raw difference between binaries:            0 px
difference outside the noise floor:         0 px       VERDICT: pixel-identical
### 33 ablation: --mode scroll --count 20000 --row-style layout
noise floor (same binary, 4 runs each): 192 px
raw difference between binaries:            107 px
difference outside the noise floor:         0 px       VERDICT: pixel-identical
```

The ablation is pixel-identical. The timing figures above are unaffected - if anything they are
cleaner, since no drawing was skipped at all.

What this costs the argument: nothing, but it removes a piece of evidence that was never real. The
reason the walk cannot be skipped is the source's own documented design (candidate 32), not a
visible artifact. **The benchmark cannot show that failure mode**, because it drives frames on a
fixed schedule rather than waiting to be asked - candidate 32 said exactly that, and I should not
then have accepted a pixel diff as a demonstration of it.

The same tool was pointed at candidate 32's *first*, invalid ablation as a control, and that one is
real by a wide margin: **621 626 px differ, 56% of the screen**, against a 126 px noise floor.

## Reproduce

```sh
# apply the one-line change above, build both binaries, then interleave:
for pass in 1 2; do for b in base notrack; do
  BIN=/tmp/pt_$b RUNS=5 tools/matrix.sh \
    "scroll 100k bare|--mode scroll --count 100000 --row-style bare" \
    "scroll 100k static|--mode scroll --count 100000 --row-style bare --scroll-step 0"
done; done

# and the pixel check:
for b in base notrack; do
  /tmp/pt_$b --bench --mode scroll --count 5000 --row-style layout --frames 6 \
             --snapshot /tmp/snap_$b.ppm; done
md5sum /tmp/snap_base.ppm /tmp/snap_notrack.ppm
```
