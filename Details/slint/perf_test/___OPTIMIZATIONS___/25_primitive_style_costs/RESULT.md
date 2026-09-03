# 25 - What styling a rectangle costs (software renderer)

`rects` always drew the same rounded, bordered rectangle, so the cost of each *style* was never
separated from the cost of drawing at all. New flags: `--opacity`, `--radius`, `--border`.

## Per-item opacity: +18%, and no layer

3 000 rectangles, all visible:

| | frame time | allocations/frame | RSS |
| --- | --- | --- | --- |
| opaque | 2.30 ms | 14 | 19.8 MB |
| `opacity: 0.99` | 2.71 ms | 14 | 19.6 MB |
| `opacity: 0.5` | 2.72 ms | 14 | 19.7 MB |
| `opacity: 0.1` | 2.71 ms | 14 | 19.7 MB |

**+18%, flat in the opacity value**, with no change in allocations or resident memory. The software
renderer does not push a subtree into an offscreen buffer for opacity - it blends per pixel. Worth
knowing because on GPU backends a partial opacity often *does* force a render target, which would
be a different shape entirely (untested here, no display).

## Corner radius costs, border width barely does

| | frame time | vs sharp fill |
| --- | --- | --- |
| `radius: 0`, `border: 0` | 2.08 ms | - |
| `radius: 0`, `border: 1` | 2.16 ms | +4% |
| `radius: 3`, `border: 0` | 2.40 ms | **+15%** |
| `radius: 3`, `border: 1` | 2.54 ms | +22% |
| `radius: 3`, `border: 4` | 2.40 ms | +15% |

A rounded corner is the anti-aliased path; a border of any thickness is nearly free next to it.

**A caveat, stated because it did not reproduce:** a first run showed `radius: 12` at 3.41 ms
(+64%), suggesting the cost grows with the radius. A second run across radii 0-24 was flat past
r=2 (2.82, 3.04, 2.79, 2.97, 2.99 ms). The cells in this scene are 34x26 px, so any radius past ~13
is clamped to half the shorter side and the larger values are not actually different - the +64% was
a noisy sample and the scaling claim does not survive. **Rounded corners cost ~15-20% over a sharp
fill; whether that grows with the radius is not answered here** and needs a scene with cells large
enough that the radius is not clamped.

## Reproduce

```sh
tools/matrix.sh \
  "sharp|--mode rects --count 3000 --radius 0 --border 0" \
  "rounded|--mode rects --count 3000 --radius 3 --border 0" \
  "opacity|--mode rects --count 3000 --opacity 0.5"
```
