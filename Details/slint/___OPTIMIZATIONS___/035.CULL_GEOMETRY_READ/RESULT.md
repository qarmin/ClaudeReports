# 35 - don't read a culled item's geometry (idea 8, implemented)

Ported from `perf_test/___OPTIMIZATIONS___/35_cull_geometry_read/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

`TEN_IDEAS.md` sized this one by ablation and left it unimplemented: `filter_item` calls
`evaluate_no_tracking(|| item_rc.geometry())` for **every** item, but a culled item only needs its
*origin*, to translate before recursing into its children. The size is read solely by
`item.render(..)`, which `render_item_children` skips for culled items.

Candidate 032 answered the open question: `compute_dirty_regions` refreshes the cache entry for every
item every frame, so `CachedItemBoundingBoxAndTransform::RegularItem { offset }` **is** fresh - and
the geometry was being read twice per item per frame.

## The change (as described in the original session)

`filter_item` looks the cache entry up first, decides `draw` from the cached bounding rect, and for
a culled `RegularItem` returns the cached offset with a **zero size**, without touching the geometry.
`render_item_children` reads the geometry itself in the one branch that needs a size for a culled
item (the `BoxShadow`/`Transform`/`Opacity`/`Layer`/`clips_children` special cases it already tests
for). Clip items keep the old path: their cache entry stores a full geometry anyway.

## Result

Paired runs, alternating binaries, `tools/isolate.py off 08b-cull-geometry-read` for the OFF side:

| scene | OFF | ON | | paired wins |
| --- | --- | --- | --- | --- |
| `scroll` 100 000 `bare` | 19.63 ms | **17.24 ms** | **-12%** | 14/14 |
| `scroll` 20 000 (layout rows) | 24.59 ms | **21.54 ms** | **-12.4%** | 9/9 |
| `nest` depth 8 | 13.30 ms | **11.96 ms** | **-10.1%** | 9/9 |
| `scroll` 20 000 `--pointer-move` | 28.42 ms | **26.09 ms** | **-8.2%** | 7/9 |
| `list` 20 000 (ListView) | 2.25 ms | 2.23 ms | -0.9% | 4/9 |
| `rects` 3 000 | 2.44 ms | 2.42 ms | -0.8% | 7/9 |
| `text` 50 000 | 3.94 ms | 3.92 ms | -0.5% | 5/9 |
| `scroll` 5 000 `clipped` | 3.58 ms | 3.58 ms | 0.0% | 6/9 |

The gain is exactly where the theory says it should be - scenes with many culled items - and nothing
regresses. `clipped` rows are flat because a clip item's cache entry is a `ClipItem`, which has no
`offset` to reuse, so those items take the unchanged path.

Rendering is reported as unchanged on `scroll` (bare, layout and clipped rows), `list`, `nest`,
`text` and `rects --opacity 0.5`, checked with `tools/pixelcheck.sh`. 374 `i-slint-core` tests are
reported passing and `--features ffi` is reported clean.

## The first version was a wash, and the reason is worth keeping

The obvious implementation puts the special-case test **inside** `filter_item`:

```rust
if !draw && let Some(offset) = cached_offset
    && !(ItemRef::downcast_pin::<BoxShadow>(item).is_some()
      || ItemRef::downcast_pin::<Transform>(item).is_some()
      || ItemRef::downcast_pin::<Opacity>(item).is_some()
      || ItemRef::downcast_pin::<Layer>(item).is_some())
```

It is correct - pixel-identical on every scene - and it measured like this on `scroll` 100 000
`bare`, 16 paired runs:

```
OFF: 22.9 19.7 19.6 19.4 20.0 19.5 19.5 19.6 20.1 20.9 20.4 19.5 20.0 19.6 19.9 21.4
ON : 18.3 23.3 19.3 18.6 23.8 24.5 23.0 23.9 24.8 24.6 18.5 18.1 23.7 24.7 18.3 19.3
OFF median 19.80   ON median 23.15   ON faster in 7 of 16
```

**Bimodal**: every run landed near 18.3 or near 24.2, never in between, and the mode held for the
whole process. Meanwhile the same binary was a consistent -16% on the `layout` scene. Two things
followed from chasing that:

1. It was not the machine. The modes survived `taskset` pinning, and the OFF binary - identical
   except for a dead branch - stayed flat at 19.8 across the same runs.
2. Those four `downcast_pin` calls **duplicate a test the caller already makes**, on the hottest
   path in the traversal. Moving the size lookup into the caller's existing branch removed them,
   and the bimodality with them: variant B runs 16.4-18.8 ms and beat OFF in 14 of 14 pairs.

The lesson is not "downcasts are slow". It is that a *correct, pixel-identical* change that saves
work measured as no better than doing the work, and only a distribution - not a median, and not
three passes of a matrix - showed why.

## Only in `ReusedBuffer` - see candidate 037

This fix serves a culled item's origin out of the partial-rendering cache. `RepaintBufferType::NewBuffer`
- which is Slint's **default** - clears that cache every frame, so the entry is usually absent and the
old path runs. Measured per mode on `scroll` 100 000 `bare`: **-8.8% in `ReusedBuffer` (9/9 paired),
-1.7% in `NewBuffer` (6/9)**, the latter at the edge of the noise.

The numbers in the table above are `ReusedBuffer`. Candidate 036 has no such dependency and carries
over to both modes.

## Reproduce

```sh
tools/isolate.py off 08b-cull-geometry-read   # build -> /tmp/pt_i8off
tools/isolate.py restore                      # build -> /tmp/pt_i8b

# paired, alternating - a median of medians would have hidden the first version's two modes
for i in $(seq 1 14); do
  /tmp/pt_i8off --bench --mode scroll --count 100000 --row-style bare --frames 20
  /tmp/pt_i8b   --bench --mode scroll --count 100000 --row-style bare --frames 20
done

RUNS=3 tools/pixelcheck.sh /tmp/pt_i8off /tmp/pt_i8b -- --mode scroll --count 20000
```

## Does it change behavior/output?

This candidate is recorded in the original session's summary as "win, applied", and its RESULT.md
describes a real code change to `filter_item`/`render_item_children`, complete with measured
pixel-diff and test-suite results (374 `i-slint-core` tests passing, `--features ffi` clean, and
`tools/pixelcheck.sh` reporting no rendering difference on the scenes listed above). No
`patch.diff` or equivalent file was preserved from the original session for this candidate,
despite the applied status - the description above is prose-only, from the original session.

## Patch re-derived and verified (2026-08-25)

A fresh implementation of the described fix was written independently against this workspace's
pinned base commit (`internal/core/partial_renderer.rs`'s `PartialRenderer::filter_item`: look up
the cache entry first, and for a culled `RegularItem` return its already-fresh cached offset with
a zero size, without reading the item's full geometry; `ClipItem`/`ItemWithTransform` entries and
brand-new items keep the old full-geometry-read path), now `patch.diff` in this folder with a
working `justfile`. Verified: `cargo test -p i-slint-core --lib` 307/307 both before and after,
`cargo check --features ffi` clean, and `just test` builds and runs both a baseline and a patched
`--mode scroll --count 100000 --row-style bare` binary successfully (a single-run smoke test, not
the paired statistical comparison above - that methodology is `tools/paired.sh`, not this
justfile). Pixel/hash comparison was not re-run this pass.
