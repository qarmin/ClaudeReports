# 31 - A scale-factor change re-shapes the whole document

Everything in this workspace was measured at scale factor 1.0. `--scale-factor` and `--scale-toggle`
were added to cover HiDPI, which is what most displays actually are.

## HiDPI itself is healthy

Same *logical* viewport, so 4x the pixels (1280x860@1 against 2560x1720@2):

| scene | sf=1 | sf=2 | paragraph shapes | RSS |
| --- | --- | --- | --- | --- |
| `text` 100 000 | 3.93 ms | 11.23 ms | 13 -> 13 | 39.0 -> 51.8 MB |
| `list` 100 000 | 2.23 ms | 6.51 ms | 18 -> 18 | 37.0 -> 46.5 MB |
| `rects` 3 000 | 2.37 ms | 5.59 ms | 0 -> 0 | 20.4 -> 29.9 MB |
| `scroll` 10 000 | 8.67 ms | 15.58 ms | 0 -> 0 | 164.7 -> 173.9 MB |

**2.4-2.9x the time for 4x the pixels** - sub-linear, dominated by rasterization. Shape counts are
identical, so shaping is per logical paragraph and not per pixel. The RSS rise is the framebuffer
(2560x1720x3 = 13.2 MB). `scroll` scales least (1.8x) because it is traversal-bound, which is
consistent with everything else measured here.

## Changing it is another whole-document re-shape

`--scale-toggle` alternates 1.0 / 2.0 per frame - what dragging a window between a HiDPI and a
normal monitor does, usually several times as it crosses:

| document | per scale change | paragraphs shaped |
| --- | --- | --- |
| 1 000 lines | 19.89 ms | 1 065 |
| 10 000 lines | 172.00 ms | 10 065 |
| 50 000 lines | **863.12 ms** | 50 065 |

One shape per paragraph, exactly like candidate 20's typing.

## Pre-existing, and legitimate in a way typing is not

| document | before candidate 03 | after |
| --- | --- | --- |
| 10 000 lines | 186.26 ms | 172.00 ms |
| 50 000 lines | 937.32 ms | 863.12 ms |

Slightly better after 03, so not a regression from this session.

Unlike typing, this invalidation is **correct**: glyph advances are in physical pixels, so a new
scale factor genuinely invalidates every shaped paragraph. `TextLayoutCache::clear_if_scale_factor_changed`
says as much. Candidate 20's fix (diff the text, keep the unchanged paragraphs) does not apply here
- there is nothing to keep.

What could be kept is the *paragraph split*: the byte ranges do not depend on the scale factor, and
finding them is a memchr pass over the whole document that is currently redone. That is a small
part of the total, so the honest summary is that this cost is close to irreducible without making
shaping itself scale-independent.

**The number is worth knowing regardless**: an editor with a 50 000-line document open freezes for
~0.9 s per step while its window is dragged between monitors.

## Reproduce

```sh
../../target/rdebug/perf_test --bench --mode text --count 50000 --scale-toggle --scroll-step 0 --frames 5
```
