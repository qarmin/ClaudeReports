# 40 - there are two software renderers, and only one had ever been measured

`SoftwareRenderer::render(buffer, stride)` and `SoftwareRenderer::render_by_line(provider)` are not
two entry points to the same code. They are **two implementations**:

* `render` -> `render_buffer_impl` -> a `RenderToBuffer` processor that draws straight into the
  target (`software/lib.rs:554-605`);
* `render_by_line` -> `render_window_frame_by_line` -> `prepare_scene` builds a `Scene` of spans,
  `Scene::new` sorts them, and each display line is composed from that (`lib.rs:756-771, 1349-1356`).

The harness has always called `render`. Everything in this workspace measured the first one.

The by-line path is what an embedded target without a full framebuffer uses, so it is not a corner
case - it is the reason the software renderer exists.

Added `--render-by-line`, with a `LineBufferProvider` that writes into the same buffer `render`
would have, so the two are compared on identical output.

## The two paths agree, and the by-line one is a few percent slower

Pixel comparison with a same-binary noise floor: `rects`, `scroll`, `text` and an image scene all
show **0 px outside the noise**. (The `text` scene showed 2 px at 3 runs per side and 0 px at 6 -
the undersampled-floor pattern from candidate 36 again.)

9 paired alternating runs each:

| scene | `render` | `render_by_line` | | paired wins for by-line |
| --- | --- | --- | --- | --- |
| `text` 20 000 | 4.15 ms | 4.51 ms | **+8.7%** | 0/9 |
| `list` 20 000 | 2.22 ms | 2.37 ms | +6.8% | 0/9 |
| `scroll` 20 000 | 22.44 ms | 23.84 ms | +6.2% | 2/9 |
| `rects` 3 000 | 2.05 ms | 2.14 ms | +4.4% | 1/9 |
| `images` 30x200 | 1.21 ms | 1.23 ms | +1.7% | 0/9 |

Consistent rather than large. Worth knowing that the mode embedded targets are pushed towards is the
slower of the two, and worth re-checking any fix that touches only one of them.

## A1 is a null result: the scene sort is 0.1-1.4% of the frame

`Scene::new` (`software/scene.rs:50-60`) does a full `sort_unstable_by` plus a partial re-sort by `z`
on every frame. Instrumented (item count, whether the input was already ordered, and the sort's own
duration):

| scene | scene items | sort | share of the frame |
| --- | --- | --- | --- |
| `text` 20 000 | 4 910 | 62.8 us | **1.4%** |
| `list` 20 000 | 1 692 | 20.6 us | 0.87% |
| `rects` 3 000 | 1 197 | 10.2 us | 0.48% |
| `scroll` 20 000 | 1 439 | 21.5 us | 0.09% |

Below the 2% null condition written down in `NEXT_CANDIDATES.md` before measuring. The input is also
**never already sorted** (`already_sorted=false` in every scene), so the obvious cheap trick - check
first, sort only if needed - has nothing to collect either.

The reason the totals stay small is that scene items are bounded by what is *visible*, not by the
item tree: 4 910 items for a full screen of text, 1 439 for a 20 000-row list. This is the one
traversal in the renderer that is already proportional to the screen.

## It retroactively explains a refutation in candidate 39

Candidate 39 tried relaxing `is_guaranteed_opaque` (`software/scene.rs:235-253`) to let an opaque
RGBA image occlude what is behind it, and recorded that it changed nothing - **not even the pixels of
a translucent image**, which was the odd part. The explanation is here: `is_guaranteed_opaque` is part
of the `Scene`, and the benchmark was calling `render`. The ablation was never executed.

That is worth generalising: **an ablation that changes nothing may be unreachable rather than
unimportant**, and the cheapest way to tell the difference is to make it deliberately wrong first and
confirm the output breaks. Candidate 32 learned the same lesson from the opposite direction.

## Reproduce

```sh
for scene in "--mode text --count 20000 --repaint-buffer new" "--mode list --count 20000"; do
  for i in $(seq 1 9); do
    perf_test --bench $scene --frames 20
    perf_test --bench $scene --render-by-line --frames 20
  done
done
```
