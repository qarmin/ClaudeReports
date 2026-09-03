# 16 - Image items: null result (software renderer)

Ported from `perf_test/___OPTIMIZATIONS___/16_image_scaling/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

New `images` scene (`--mode images`): a grid of `Image` items over procedurally generated RGBA
buffers, so it needs no asset file. Nothing here had ever measured the image path.

## Item count does not matter

The page clips, so off-screen items are pruned like any other clipped subtree:

| items | frame time | allocations/frame |
| --- | --- | --- |
| 300 | 4.92 ms | 11 |
| 1 000 | 5.10 ms | 12 |

## Sharing a source does not matter

64 distinct sources cycled over the grid against one shared by every item: 4.92 ms either way.
Whatever the renderer caches, it is not the bottleneck, and not sharing costs nothing.

## Source resolution does not matter

Each item is drawn at ~64x48 px; the source is scaled to it:

| source | 64px | 128px | 256px | 512px | 1024px |
| --- | --- | --- | --- | --- | --- |
| frame time | 6.20 ms | 5.00 ms | 5.08 ms | 4.99 ms | 5.03 ms |

Flat. The software renderer's scaling is O(destination pixels), not O(source pixels), which is the
correct shape - the classic "4000 px photo in a 64 px thumbnail" trap does not cost frame time here.
It still costs **resident memory**: 14.9 MB at 64px against 18.9 MB at 1024px for 64 sources, which
is the decoded buffers and nothing to do with drawing.

**11 allocations per frame** for the whole scene: image drawing allocates nothing per item.

## Not covered

FemtoVG/Skia were not measured (no display in this environment). A GPU renderer uploads textures,
so source resolution plausibly matters there in a way it does not here - that is the run to do
before treating this as settled for all renderers.

## Does it change behavior/output?

N/A - measured/quantified only, no patch applied to real source in the original session.

## Reproduce

```sh
tools/matrix.sh \
  "300 shared|--mode images --count 300 --image-rescale" \
  "300 distinct|--mode images --count 300 --image-distinct --image-rescale" \
  "src 1024px|--mode images --count 300 --image-rescale --image-size 1024"
```
